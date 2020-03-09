use cgmath::Rad;
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

    cells: Vec<HEALPixCell>,
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

use crate::healpix_cell::HEALPixCell;

/*
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
*/
use std::sync::Arc;
use std::sync::Mutex;
lazy_static! {
    pub static ref ALLSKY_ZERO_DEPTH: Arc<Mutex<Vec<HEALPixCell>>> = {
        let mut allsky = Vec::with_capacity(12);
        for idx in 0..12 {
            allsky.push(HEALPixCell(0, idx));
        }

        Arc::new(Mutex::new(allsky))
    };
    pub static ref ALLSKY_ONE_DEPTH: Arc<Mutex<Vec<HEALPixCell>>> = {
        let mut allsky = Vec::with_capacity(48);
        for idx in 0..48 {
            allsky.push(HEALPixCell(1, idx));
        }

        Arc::new(Mutex::new(allsky))
    };
}

use web_sys::console;

use wasm_bindgen::JsCast;
use crate::projection::Projection;
use crate::WebGl2Context;

impl FieldOfView {
    pub fn new<P: Projection>(gl: &WebGl2Context, aperture_angle: Rad<f32>, max_depth_hips: u8) -> FieldOfView {
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

        let cells = ALLSKY_ZERO_DEPTH.lock()
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

        fov.set_aperture::<P>(aperture_angle, max_depth_hips);
        fov
    }

    pub fn resize_window<P: Projection>(&mut self, width: f32, height: f32, max_depth_hips: u8) {
        self.width = width;
        self.height = height;

        self.aspect = width / height;

        self.canvas.set_width(width as u32);
        self.canvas.set_height(height as u32);
        self.gl.viewport(0, 0, width as i32, height as i32);
        self.gl.scissor(0, 0, width as i32, height as i32);

        // Compute the new clip zoom factor
        self.ndc_to_clip = Self::compute_ndc_to_clip_factor(width, height);

        self.deproj_field_of_view::<P>(max_depth_hips);
    }

    fn compute_clip_zoom_factor<P: Projection>(fov: Rad<f32>) -> f32 {
        let lon = fov.0.abs() / 2_f32;

        // Vertex in the WCS of the FOV
        let v0 = math::radec_to_xyzw(Rad(lon), Rad(0_f32));

        // Project this vertex into the screen
        let p0 = P::world_to_clip_space(v0);
        p0.x.abs()
    }

    fn compute_ndc_to_clip_factor(width: f32, height: f32) -> Vector2<f32> {
        Vector2::new(1_f32, height / width)
    }

    pub fn set_aperture<P: Projection>(&mut self, angle: Rad<f32>, max_depth_hips: u8) {
        self.aperture_angle = angle;
        // Compute the new clip zoom factor
        self.clip_zoom_factor = Self::compute_clip_zoom_factor::<P>(self.aperture_angle);
        
        self.deproj_field_of_view::<P>(max_depth_hips);
    }

    pub fn get_aperture(&self) -> Rad<f32> {
        self.aperture_angle
    }

    pub fn area_clip_zoomed_space(&self) -> f32 {
        let pos_normalized_device = Vector2::new(1_f32, 1_f32);

        let pos_clip_space = Vector2::new(
            pos_normalized_device.x * self.ndc_to_clip.x * self.clip_zoom_factor,
            pos_normalized_device.y * self.ndc_to_clip.y * self.clip_zoom_factor,
        );

        pos_clip_space.x.abs() * pos_clip_space.y.abs() * 4_f32
    }

    fn deproj_field_of_view<P: Projection>(&mut self, max_depth_hips: u8) {
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
        self.rotate::<P>(max_depth_hips);
    }

    pub fn set_rotation_mat<P: Projection>(&mut self, r: &Matrix4<f32>, max_depth_hips: u8) {
        self.r = *r;

        self.rotate::<P>(max_depth_hips);
    }

    fn rotate<P: Projection>(&mut self, max_depth_hips: u8) {
        if let Some(pos_world_space) = self.pos_world_space {
            let mut pos_transformed_space = [Vector4::new(0_f32, 0_f32, 0_f32, 0_f32); NUM_VERTICES];
            for idx_vertex in 0..NUM_VERTICES {
                pos_transformed_space[idx_vertex] = self.r * pos_world_space[idx_vertex];
            }

            self.pos_transformed_space = Some(pos_transformed_space);
        } else {
            self.pos_transformed_space = None;
        }

        self.compute_healpix_cells::<P>(max_depth_hips);
    }

    pub fn get_cells_in_fov(&self, depth: u8) -> Vec<HEALPixCell> {
        if let Some(pos_transformed_space) = self.pos_transformed_space {
            let lon_lat_world_space = pos_transformed_space.iter()
                .map(|pos_transformed_space| {
                    let (ra, dec) = math::xyzw_to_radec(*pos_transformed_space);
                    (ra as f64, dec as f64)
                })
                .collect::<Vec<_>>();

            let moc = healpix::nested::polygon_coverage(depth, &lon_lat_world_space, true);
                moc.flat_iter()
                .map(|idx| {
                    HEALPixCell(depth, idx)
                })
                .collect()
        } else {
            if depth > 3 {
                // Allsky happens for small depths!
                unreachable!();
            } 

            let npix = 12 << (2*depth);
            let mut cells = Vec::with_capacity(npix);
            for idx in 0..npix {
                cells.push(HEALPixCell(depth, idx as u64));
            }
            cells
        }
    }

    fn compute_healpix_cells<P: Projection>(&mut self, max_depth_hips: u8) {
        // The field of view has changed (zoom or translation, so we recompute the cells)
        if let Some(pos_transformed_space) = self.pos_transformed_space {
            // Compute the depth corresponding to the angular resolution of a pixel
            // along the width of the screen
            let depth = std::cmp::min(
                math::fov_to_depth(self.aperture_angle, self.width),
                max_depth_hips
            );

            if let Some(allsky) = P::check_for_allsky_fov(depth) {
                self.cells = allsky;
                self.current_depth = depth;
                console::log_1(&format!("current depth {:?}", self.current_depth).into());
                return;
            }

            // It is not an allsky fov at this point
            // We can get the HEALPix cells being in the fov
            let lon_lat_world_space = pos_transformed_space.iter()
                .map(|pos_transformed_space| {
                    let (ra, dec) = math::xyzw_to_radec(*pos_transformed_space);
                    (ra as f64, dec as f64)
                })
                .collect::<Vec<_>>();

            let moc = healpix::nested::polygon_coverage(depth, &lon_lat_world_space, true);
            let mut cells = moc.flat_iter()
                .map(|idx| {
                    HEALPixCell(depth, idx)
                })
                .collect::<Vec<_>>();
            /*let mut cells = vec![];
            while depth > 0 {
                let moc = healpix::nested::polygon_coverage(depth, &lon_lat_world_space, true);
                let num_tiles = moc.entries.len();

                // Stop when the number of tiles for this depth
                // can be contained in the tile buffer
                if num_tiles <= 32 {
                    cells = moc.flat_iter()
                        .map(|idx| {
                            HEALPixCell(depth, idx)
                        })
                        .collect::<Vec<_>>();
                    break;
                }

                depth -= 1;
            }*/

            if depth == 0 {
                cells = ALLSKY_ZERO_DEPTH.lock().unwrap().clone();
            }

            self.current_depth = depth;
            self.cells = cells;

            return;
        }

        // We are out of the FOV
        self.current_depth = 0;
        self.cells = ALLSKY_ZERO_DEPTH.lock().unwrap().clone();
    }

    // Returns the HEALPix cells in the field of view
    pub fn healpix_cells(&self) -> &Vec<HEALPixCell> {
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