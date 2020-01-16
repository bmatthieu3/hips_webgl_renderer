use cgmath::Rad;
use cgmath::Deg;
use cgmath::Vector2;
use cgmath::Vector4;
use cgmath::Matrix4;

pub struct FieldOfView {
    num_vertices: usize,
    num_vertices_width: usize,
    num_vertices_height: usize,

    vertices_screen_space: Vec<Vector2<f32>>,
    vertices_world_space: Vec<Vector4<f32>>,

    value: Option<Rad<f32>>, // fov can be None if the camera is out of the projection

    cells: BTreeSet<HEALPixCell>,
    current_depth: u8,
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

impl FieldOfView {
    pub fn new() -> FieldOfView {
        let num_vertices_width = 3;
        let num_vertices_height = 3;
        let num_vertices = 4 + 2*num_vertices_width + 2*num_vertices_height;

        let mut x_screen_space = itertools_num::linspace::<f32>(-1., 1., num_vertices_width + 2)
            .collect::<Vec<_>>();

        x_screen_space.extend(iter::repeat(1_f32).take(num_vertices_height));
        x_screen_space.extend(itertools_num::linspace::<f32>(1., -1., num_vertices_width + 2));
        x_screen_space.extend(iter::repeat(-1_f32).take(num_vertices_height));

        let mut y_screen_space = iter::repeat(-1_f32).take(num_vertices_width + 1)
            .collect::<Vec<_>>();

        y_screen_space.extend(itertools_num::linspace::<f32>(-1., 1., num_vertices_height + 2));
        y_screen_space.extend(iter::repeat(1_f32).take(num_vertices_width));
        y_screen_space.extend(itertools_num::linspace::<f32>(1., -1., num_vertices_height + 2));
        y_screen_space.pop();

        let vertices_screen_space = x_screen_space.into_iter().zip(y_screen_space.into_iter()).map(|(x, y)| {
            Vector2::new(x, y)
        }).collect::<Vec<_>>();
        let vertices_world_space = vec![Vector4::new(0_f32, 0_f32, 0_f32, 1_f32); vertices_screen_space.len()];
        let value = None;

        let cells = ALLSKY.lock().unwrap().clone();
        let current_depth = 0;
        FieldOfView {
            num_vertices,
            num_vertices_width,
            num_vertices_height,

            vertices_screen_space,
            vertices_world_space,

            value,

            cells,
            current_depth,
        }
    }

    pub fn update(&mut self, model: &Matrix4<f32>, zoom: f32, projection: &ProjectionType) {
        self.vertices_world_space = self.vertices_screen_space.iter()
            .filter_map(|vertex_screen_space| {
                let vertex_homogeneous_space = vertex_screen_space / zoom;
                let vertex_world_space = projection.screen_to_world_space(&vertex_homogeneous_space);
                vertex_world_space
            })
            .collect::<Vec<_>>();

        self.value = if self.vertices_world_space.len() == self.num_vertices {
            let idx_r = self.num_vertices_width + 1 + (((self.num_vertices_height as f32)/2_f32).ceil() as usize);
            let idx_l = 2 * self.num_vertices_width + 3 + self.num_vertices_height + (((self.num_vertices_height as f32)/2_f32).ceil() as usize);

            let pos_r_world_space = cgmath::Vector3::new(
                self.vertices_world_space[idx_r].x,
                self.vertices_world_space[idx_r].y,
                self.vertices_world_space[idx_r].z
            );
            let pos_l_world_space = cgmath::Vector3::new(
                self.vertices_world_space[idx_l].x,
                self.vertices_world_space[idx_l].y,
                self.vertices_world_space[idx_l].z
            );

            let fov = math::angular_distance_xyz(pos_r_world_space, pos_l_world_space);
            Some(fov)
        } else {
            None
        };

        
        let allsky = ALLSKY.lock().unwrap();
        self.cells = if let Some(fov) = self.value {
            // The fov does not cross the border of the projection
            if fov >= Deg(150_f32).into() {
                // The fov is >= 150°
                self.current_depth = 0;
                allsky.clone()
            } else {
                let (width, _) = window_size_f32();
                let l = width;
                // Compute the depth corresponding to the angular resolution of a pixel
                // along the width of the screen
                let mut depth = std::cmp::min(math::ang_per_pixel_to_depth(fov.0 / l), MAX_DEPTH.load(atomic::Ordering::Relaxed));
                //console::log_1(&format!("depth {:?}", depth).into());
                let cells = if depth == 0 {
                    allsky.clone()
                } else {
                    // The fov is not too big so we can get the HEALPix cells
                    // being in the fov
                    //console::log_1(&format!("VERTICES world space, {:?}", self.vertices_world_space).into());
                    let lon_lat_world_space = self.vertices_world_space.iter()
                        .map(|vertex_world_space| {
                            // Take into account the rotation of the sphere
                            let vertex_model_space = (*model) * vertex_world_space;

                            let (ra, dec) = math::xyzw_to_radec(vertex_model_space);
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
                        if num_tiles <= 64 {
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
                self.current_depth = depth + 1;
                cells
            }
        } else {
            // The fov is out the projection
            self.current_depth = 0;
            allsky.clone()
        };
    }

    // Returns the HEALPix cells located in the
    // field of view
    pub fn cells(&self) -> &BTreeSet<HEALPixCell> {
        &self.cells
    }

    pub fn get_current_depth(&self) -> u8 {
        self.current_depth
    }

    pub fn value(&self) -> &Option<Rad<f32>> {
        &self.value
    }
}