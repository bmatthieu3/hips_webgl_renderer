use cgmath::{Rad, Deg};
use cgmath::Vector2;
use cgmath::Vector4;
use cgmath::Matrix4;
use cgmath::SquareMatrix;

const NUM_VERTICES_WIDTH: usize = 3;
const NUM_VERTICES_HEIGHT: usize = 3;
const NUM_VERTICES: usize = 4 + 2*NUM_VERTICES_WIDTH + 2*NUM_VERTICES_HEIGHT;

pub struct FieldOfView {
    pos_ndc_space: [Vector2<f32>; NUM_VERTICES],
    pos_world_space: Option<[Vector4<f32>; NUM_VERTICES]>,
    pos_transformed_space: Option<[Vector4<f32>; NUM_VERTICES]>,

    aperture_angle: Rad<f32>, // fov can be None if the camera is out of the projection
    r: Matrix4<f32>, // Rotation matrix of the FOV (i.e. same as the HiPS sphere model matrix)

    ndc_to_clip: Vector2<f32>,
    clip_zoom_factor: f32,

    cells: BTreeSet<HEALPixCell>,
    current_depth: u8,

    // The width over height ratio
    aspect: f32,

    // The width of the screen in pixels
    width: f32,
    // The height of the screen in pixels
    height: f32,

    // Canvas HtmlElement
    canvas: web_sys::HtmlCanvasElement,

    // WebGL2 context
    gl: WebGl2Context,
}

use itertools_num;
use std::iter;
use crate::math;
use crate::MAX_DEPTH;

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

use wasm_bindgen::JsCast;

use crate::renderable::Renderable;
use crate::renderable::hips_sphere::HiPSSphere;
use crate::projection::Projection;
use crate::WebGl2Context;

impl FieldOfView {
    pub fn new<P: Projection>(gl: &WebGl2Context, aperture_angle: Rad<f32>) -> FieldOfView {
        let mut x_ndc_space = itertools_num::linspace::<f32>(-1., 1., NUM_VERTICES_WIDTH + 2)
            .collect::<Vec<_>>();

        x_ndc_space.extend(iter::repeat(1_f32).take(NUM_VERTICES_HEIGHT));
        x_ndc_space.extend(itertools_num::linspace::<f32>(1., -1., NUM_VERTICES_WIDTH + 2));
        x_ndc_space.extend(iter::repeat(-1_f32).take(NUM_VERTICES_HEIGHT));

        let mut y_ndc_space = iter::repeat(-1_f32).take(NUM_VERTICES_WIDTH + 1)
            .collect::<Vec<_>>();

        y_ndc_space.extend(itertools_num::linspace::<f32>(-1., 1., NUM_VERTICES_HEIGHT + 2));
        y_ndc_space.extend(iter::repeat(1_f32).take(NUM_VERTICES_WIDTH));
        y_ndc_space.extend(itertools_num::linspace::<f32>(1., -1., NUM_VERTICES_HEIGHT + 2));
        y_ndc_space.pop();

        let mut pos_ndc_space = [Vector2::new(0_f32, 0_f32); NUM_VERTICES];
        for idx_vertex in 0..NUM_VERTICES {
            pos_ndc_space[idx_vertex] = Vector2::new(
                x_ndc_space[idx_vertex],
                y_ndc_space[idx_vertex],
            );
        }
        
        let pos_world_space = None;
        let pos_transformed_space = None;

        let cells = ALLSKY.lock()
            .unwrap()
            .clone();

        let width = web_sys::window()
            .unwrap()
            .inner_width()
            .unwrap()
            .as_f64()
            .unwrap() as f32;
        let height = web_sys::window()
            .unwrap()
            .inner_height()
            .unwrap()
            .as_f64()
            .unwrap() as f32;

        let aspect = width / height;
        let ndc_to_clip = Self::compute_ndc_to_clip_factor(width, height);
        let clip_zoom_factor = 0_f32;
        let current_depth = 0;

        // Canvas definition
        let canvas = gl.canvas().unwrap()
            .dyn_into::<web_sys::HtmlCanvasElement>()
            .unwrap();
        canvas.set_width(width as u32);
        canvas.set_height(height as u32);
        gl.viewport(0, 0, width as i32, height as i32);
        gl.scissor(0, 0, width as i32, height as i32);

        let r = Matrix4::identity();

        let gl = gl.clone();
        let mut fov = FieldOfView {
            pos_ndc_space,
            pos_world_space,
            pos_transformed_space,

            aperture_angle,
            r,
            
            ndc_to_clip,
            clip_zoom_factor,

            cells,
            current_depth,

            aspect,

            width,
            height,

            canvas,
            gl,
        };

        console::log_1(&format!("SET FOV").into());
        fov.set_aperture::<P>(aperture_angle);
        console::log_1(&format!("FOV end init").into());
        fov
    }

    pub fn resize_window<P: Projection>(&mut self, width: f32, height: f32) {
        self.width = width;
        self.height = height;

        self.aspect = width / height;

        self.canvas.set_width(width as u32);
        self.canvas.set_height(height as u32);
        self.gl.viewport(0, 0, width as i32, height as i32);
        self.gl.scissor(0, 0, width as i32, height as i32);

        // Compute the new clip zoom factor
        self.ndc_to_clip = Self::compute_ndc_to_clip_factor(width, height);

        self.deproj_field_of_view::<P>();
    }

    fn compute_clip_zoom_factor<P: Projection>(fov: Rad<f32>) -> f32 {
        let lon = fov.0.abs() / 2_f32;

        // Vertex in the WCS of the FOV
        let v0 = math::radec_to_xyz(Rad(lon), Rad(0_f32));

        // Project this vertex into the screen
        let p0 = P::world_to_clip_space(v0);
        p0.x.abs()
    }

    fn compute_ndc_to_clip_factor(width: f32, height: f32) -> Vector2<f32> {
        Vector2::new(1_f32, height / width)
    }

    pub fn set_aperture<P: Projection>(&mut self, angle: Rad<f32>) {
        self.aperture_angle = angle;
        // Compute the new clip zoom factor
        self.clip_zoom_factor = Self::compute_clip_zoom_factor::<P>(angle);
        
        self.deproj_field_of_view::<P>();
    }

    fn deproj_field_of_view<P: Projection>(&mut self) {
        // Deproject the FOV from ndc to the world space
        let mut vertices_world_space = [Vector4::new(0_f32, 0_f32, 0_f32, 0_f32); NUM_VERTICES];
        let mut out_of_fov = false;
        for idx_vertex in 0..NUM_VERTICES {
            let pos_ndc_space = &self.pos_ndc_space[idx_vertex];

            let pos_clip_space = Vector2::new(
                pos_ndc_space.x * self.ndc_to_clip.x * self.clip_zoom_factor,
                pos_ndc_space.y * self.ndc_to_clip.y * self.clip_zoom_factor,
            );

            let pos_world_space = P::clip_to_world_space(pos_clip_space);
            if let Some(pos_world_space) = pos_world_space {
                vertices_world_space[idx_vertex] = pos_world_space;
            } else {
                out_of_fov = true;
                break;
            }
        }
        if out_of_fov {
            self.pos_world_space = None;
        } else {
            self.pos_world_space = Some(vertices_world_space);
        }

        // Rotate the FOV
        self.rotate();
    }

    pub fn set_rotation_mat(&mut self, r: &Matrix4<f32>) {
        self.r = *r;

        self.rotate();
    }

    fn rotate(&mut self) {
        if let Some(pos_world_space) = self.pos_world_space {
            let mut pos_transformed_space = [Vector4::new(0_f32, 0_f32, 0_f32, 0_f32); NUM_VERTICES];
            for idx_vertex in 0..NUM_VERTICES {
                pos_transformed_space[idx_vertex] = self.r * pos_world_space[idx_vertex];
            }

            self.pos_transformed_space = Some(pos_transformed_space);
        } else {
            self.pos_transformed_space = None;
        }

        self.compute_healpix_cells();
    }

    fn compute_healpix_cells(&mut self) {
        // The field of view has changed (zoom or translation, so we recompute the cells)
        let allsky = ALLSKY.lock().unwrap();

        if let Some(pos_transformed_space) = self.pos_transformed_space {
            // Compute the depth corresponding to the angular resolution of a pixel
            // along the width of the screen
            let mut depth = std::cmp::min(
                math::fov_to_depth(self.aperture_angle, self.width),
                MAX_DEPTH.load(atomic::Ordering::Relaxed)
            );
            let cells = if depth == 0 {
                allsky.clone()
            } else {
                // The fov is not too big so we can get the HEALPix cells
                // being in the fov
                let lon_lat_world_space = pos_transformed_space.iter()
                    .map(|pos_transformed_space| {
                        let (ra, dec) = math::xyzw_to_radec(*pos_transformed_space);
                        (ra as f64, dec as f64)
                    })
                    .collect::<Vec<_>>();

                let mut cells = BTreeSet::new();
                while depth > 0 {
                    let moc = healpix::nested::polygon_coverage(depth, &lon_lat_world_space, true);
                    let num_tiles = moc.entries.len();

                    // Stop when the number of tiles for this depth
                    // can be contained in the tile buffer
                    if num_tiles <= 52 {
                        cells = moc.flat_iter()
                            .map(|idx| {
                                HEALPixCell(depth, idx)
                            })
                            .collect::<BTreeSet<_>>();
                        break;
                    }
                    console::log_1(&format!("buffer too small!").into());


                    depth -= 1;
                }

                if depth == 0 {
                    cells = allsky.clone();
                }
                
                cells
            };

            self.current_depth = depth;
            self.cells = cells;
        } else {
            self.current_depth = 0;
            self.cells = allsky.clone();
        }

        console::log_1(&format!("current depth {:?}", self.current_depth).into());
    }

    // Returns the HEALPix cells in the field of view
    pub fn healpix_cells(&self) -> &BTreeSet<HEALPixCell> {
        &self.cells
    }

    pub fn current_depth(&self) -> u8 {
        self.current_depth
    }

    pub fn get_ndc_to_clip(&self) -> &Vector2<f32> {
        &self.ndc_to_clip
    }

    pub fn get_clip_zoom_factor(&self) -> f32 {
        self.clip_zoom_factor
    }

    pub fn get_aspect(&self) -> f32 {
        self.aspect
    }

    pub fn get_width_screen(&self) -> f32 {
        self.width
    }

    pub fn get_height_screen(&self) -> f32 {
        self.height
    }

    pub fn get_size_screen(&self) -> (f32, f32) {
        (self.width, self.height)
    }
}