use cgmath::{Rad, Deg};
use cgmath::Vector2;
use cgmath::Vector4;

const NUM_VERTICES_WIDTH: usize = 3;
const NUM_VERTICES_HEIGHT: usize = 3;
const NUM_VERTICES: usize = 4 + 2*NUM_VERTICES_WIDTH + 2*NUM_VERTICES_HEIGHT;

pub struct FieldOfView {
    vertices_homo_space: [Vector2<f32>; NUM_VERTICES],
    vertices_local_space: [Vector4<f32>; NUM_VERTICES],
    vertices_world_space: [Vector4<f32>; NUM_VERTICES],

    aperture_angle: Rad<f32>, // fov can be None if the camera is out of the projection
    screen_scaling: Vector2<f32>,

    cells: BTreeSet<HEALPixCell>,
    current_depth: u8,
    max_num_tiles: usize,
}

use itertools_num;
use std::iter;
use crate::projection::ProjectionType;
use crate::math;
use crate::MAX_DEPTH;
use crate::window_size_f32;

use std::sync::atomic;
use std::cmp;

#[derive(Clone, Copy)]
#[derive(Debug)]
#[derive(PartialEq, Eq, Hash)]
pub struct HEALPixCell(pub u8, pub u64);

impl PartialOrd for HEALPixCell {
    fn partial_cmp(&self, other: &Self) -> Option<cmp::Ordering> {
        let uniq = (1 << (2*((self.0 as u64) + 1))) + self.1;
        let uniq_other = (1 << (2*((other.0 as u64) + 1))) + other.1;

        uniq.partial_cmp(&uniq_other)
    }
}
impl Ord for HEALPixCell {
    fn cmp(&self, other: &Self) -> cmp::Ordering {
        self.partial_cmp(&other).unwrap()
    }
}

use crate::texture::Tile;
impl From<Tile> for HEALPixCell {
    fn from(tile: Tile) -> Self {
        tile.cell
    }
}
use crate::texture::TileRequest;
impl From<TileRequest> for HEALPixCell {
    fn from(tile_request: TileRequest) -> Self {
        tile_request.cell
    }
}

use std::collections::BTreeSet;
use std::sync::Arc;
use std::sync::Mutex;
lazy_static! {
    pub static ref ALLSKY: Arc<Mutex<BTreeSet<HEALPixCell>>> = {
        let mut allsky = BTreeSet::new();
        allsky.insert(HEALPixCell(0, 0));
        allsky.insert(HEALPixCell(0, 1));
        allsky.insert(HEALPixCell(0, 2));
        allsky.insert(HEALPixCell(0, 3));
        allsky.insert(HEALPixCell(0, 4));
        allsky.insert(HEALPixCell(0, 5));
        allsky.insert(HEALPixCell(0, 6));
        allsky.insert(HEALPixCell(0, 7));
        allsky.insert(HEALPixCell(0, 8));
        allsky.insert(HEALPixCell(0, 9));
        allsky.insert(HEALPixCell(0, 10));
        allsky.insert(HEALPixCell(0, 11));

        Arc::new(Mutex::new(allsky))
    };
}

use crate::texture::BufferTiles;
use web_sys::console;

use crate::renderable::Renderable;
use crate::renderable::hips_sphere::HiPSSphere;
impl FieldOfView {
    pub fn new(buffer: &BufferTiles) -> FieldOfView {
        let mut x_homo_space = itertools_num::linspace::<f32>(-1., 1., NUM_VERTICES_WIDTH + 2)
            .collect::<Vec<_>>();

        x_homo_space.extend(iter::repeat(1_f32).take(NUM_VERTICES_HEIGHT));
        x_homo_space.extend(itertools_num::linspace::<f32>(1., -1., NUM_VERTICES_WIDTH + 2));
        x_homo_space.extend(iter::repeat(-1_f32).take(NUM_VERTICES_HEIGHT));

        let mut y_homo_space = iter::repeat(-1_f32).take(NUM_VERTICES_WIDTH + 1)
            .collect::<Vec<_>>();

        y_homo_space.extend(itertools_num::linspace::<f32>(-1., 1., NUM_VERTICES_HEIGHT + 2));
        y_homo_space.extend(iter::repeat(1_f32).take(NUM_VERTICES_WIDTH));
        y_homo_space.extend(itertools_num::linspace::<f32>(1., -1., NUM_VERTICES_HEIGHT + 2));
        y_homo_space.pop();

        let mut vertices_homo_space = [Vector2::new(0_f32, 0_f32); NUM_VERTICES];
        for idx_vertex in 0..NUM_VERTICES {
            vertices_homo_space[idx_vertex] = Vector2::new(
                x_homo_space[idx_vertex],
                y_homo_space[idx_vertex],
            );
        }
        
        let vertices_local_space = [Vector4::new(0_f32, 0_f32, 0_f32, 1_f32); NUM_VERTICES];
        let vertices_world_space = vertices_local_space.clone();

        let aperture_angle = Deg(30_f32).into();
        let screen_scaling = Vector2::new(1_f32, 1_f32);

        let cells = ALLSKY.lock()
            .unwrap()
            .clone();
        let current_depth = 0;

        let max_num_tiles = buffer.len_variable_tiles();
        FieldOfView {
            vertices_homo_space,
            vertices_local_space,
            vertices_world_space,

            aperture_angle,
            screen_scaling,

            cells,
            current_depth,

            max_num_tiles
        }
    }

    fn to_screen_factor(fov: Rad<f32>, projection: &ProjectionType) -> Vector2<f32> {
        let (width, height) = window_size_f32();
        let aspect = width / height;

        let lon = fov.0.abs() / 2_f32;
        let lat = lon / aspect;

        // Vertex in the WCS of the FOV
        let v0 = math::radec_to_xyz(Rad(lon), Rad(0_f32));
        let v1 = math::radec_to_xyz(Rad(0_f32), Rad(lat));

        // Project this vertex into the screen
        let p0 = projection.world_to_screen_space(v0)
            .unwrap();
        let p1 = projection.world_to_screen_space(v1)
            .unwrap();

        Vector2::new(p0.x.abs(), p1.y.abs())
    }

    pub fn set_aperture(&mut self, angle: Rad<f32>, projection: &ProjectionType, hips_sphere: &Renderable<HiPSSphere>) {
        self.aperture_angle = angle;
        let scaling_screen_factor = *self.get_scaling_screen_factor();
        // Update the local coordinates w.r.t the newly computed
        // screen scaling vector 
        for idx_vertex in 0..NUM_VERTICES {
            let homogeneous_vertex = &self.vertices_homo_space[idx_vertex];
            self.vertices_local_space[idx_vertex] = projection
                .homogeneous_to_world_space(homogeneous_vertex.clone(), &scaling_screen_factor)
                .unwrap();
        }

        // Compute the world space vertices
        self.translate(hips_sphere);
    }

    pub fn translate(&mut self, hips_sphere: &Renderable<HiPSSphere>) {
        let model = hips_sphere.get_model_mat();
        for idx_vertex in 0..NUM_VERTICES {
            self.vertices_world_space[idx_vertex] = *model * self.vertices_local_space[idx_vertex];
        }
        console::log_1(&format!("current depth {:?}", self.current_depth).into());

        self.compute_healpix_cells();
    }

    fn compute_healpix_cells(&mut self) {
        // The field of view has changed (zoom or translation, so we recompute the cells)
        let allsky = ALLSKY.lock().unwrap();

        // Compute the depth corresponding to the angular resolution of a pixel
        // along the width of the screen
        let mut depth = std::cmp::min(
            math::fov_to_depth(self.aperture_angle),
            MAX_DEPTH.load(atomic::Ordering::Relaxed)
        );
        let cells = if depth == 0 {
            allsky.clone()
        } else {
            // The fov is not too big so we can get the HEALPix cells
            // being in the fov
            //console::log_1(&format!("VERTICES world space, {:?}", self.vertices_world_space).into());
            let lon_lat_world_space = self.vertices_world_space.iter()
                .map(|vertex_world_space| {
                    // Take into account the rotation of the sphere
                    //let vertex_model_space = (*model) * vertex_world_space;

                    let (ra, dec) = math::xyzw_to_radec(*vertex_world_space);
                    (ra as f64, dec as f64)
                })
                .collect::<Vec<_>>();

            //console::log_1(&format!("LONLAT, {:?}", lon_lat_world_space).into());
            let mut cells = BTreeSet::new();
            while depth > 0 {
                let moc = healpix::nested::polygon_coverage(depth, &lon_lat_world_space, true);
                let num_tiles = moc.entries.len();
                // Stop when the number of tiles for this depth
                // can be contained in the tile buffer
                if num_tiles <= self.max_num_tiles {
                    cells = moc.flat_iter()
                        .map(|idx| {
                            HEALPixCell(depth, idx)
                        })
                        .collect::<BTreeSet<_>>();
                    break;
                }

                depth -= 1;
            }

            if depth == 0 {
                cells = allsky.clone();
            }
            
            cells
        };
        self.current_depth = depth;
        self.cells = cells;
        console::log_1(&format!("current depth {:?}", self.current_depth).into());
    }

    // Returns the HEALPix cells in the field of view
    pub fn healpix_cells(&self) -> &BTreeSet<HEALPixCell> {
        &self.cells
    }

    pub fn current_depth(&self) -> u8 {
        self.current_depth
    }

    pub fn get_scaling_screen_factor(&self) -> &Vector2<f32> {
        &self.screen_scaling
    }
}