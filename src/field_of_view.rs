use cgmath::Rad;
use cgmath::{Vector2, Vector3, Vector4};
use cgmath::Matrix4;
use cgmath::SquareMatrix;

const NUM_VERTICES_WIDTH: usize = 5;
const NUM_VERTICES_HEIGHT: usize = 5;
const NUM_VERTICES: usize = 4 + 2*NUM_VERTICES_WIDTH + 2*NUM_VERTICES_HEIGHT;

use std::collections::HashSet;
use crate::buffer::HiPSConfig;
pub struct FieldOfView {
    pos_ndc_space: [Vector2<f32>; NUM_VERTICES],
    pos_world_space: Option<[Vector4<f32>; NUM_VERTICES]>,
    pos_transformed_space: Option<[Vector4<f32>; NUM_VERTICES]>,

    aperture_angle: Rad<f32>, // fov can be None if the camera is out of the projection
    r: Matrix4<f32>, // Rotation matrix of the FOV (i.e. same as the HiPS sphere model matrix)

    ndc_to_clip: Vector2<f32>,
    clip_zoom_factor: f32,

    // The set of cells being in the current field of view
    cells: HashSet<HEALPixCell>,
    // A map describing the cells in the current field of view
    // A boolean is associated with the cells telling if the
    // cell is new (meaning it was not in the previous field of view).
    // ``cells`` is always equal to its keys!
    new_cells: HashMap<HEALPixCell, bool>,
    is_there_new_cells: bool,
    // The current depth of the cells in the field of view
    current_depth: u8,

    // The width over height ratio
    aspect: f32,

    // The width of the screen in pixels
    width: f32,
    // The height of the screen in pixels
    height: f32,

    // Canvas HtmlElement
    canvas: web_sys::HtmlCanvasElement,
    // HiPS config
    config: HiPSConfig,

    // WebGL2 context
    gl: WebGl2Context,
}

use itertools_num;
use std::iter;
use crate::math;

use crate::healpix_cell::HEALPixCell;

use wasm_bindgen::JsCast;
use crate::projection::Projection;
use crate::WebGl2Context;

use std::collections::HashMap;
use crate::healpix_cell;
use web_sys::console;
impl FieldOfView {
    pub fn new<P: Projection>(gl: &WebGl2Context, aperture_angle: Rad<f32>, config: &HiPSConfig) -> FieldOfView {
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

        let cells = healpix_cell::allsky(0);
        let new_cells = cells.iter()
            .cloned()
            .map(|cell| (cell, true))
            .collect();
        let is_there_new_cells = true;

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
        let config = config.clone();
        let mut fov = FieldOfView {
            pos_ndc_space,
            pos_world_space,
            pos_transformed_space,

            aperture_angle,
            r,
            
            ndc_to_clip,
            clip_zoom_factor,

            cells,
            new_cells,
            is_there_new_cells,
            current_depth,

            aspect,

            width,
            height,

            canvas,
            config,
            gl,
        };

        fov.set_aperture::<P>(aperture_angle);
        fov
    }

    pub fn set_config(&mut self, config: &HiPSConfig) {
        self.config = config.clone();
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
        let v0 = math::radec_to_xyzw(Rad(lon), Rad(0_f32));

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
        self.clip_zoom_factor = Self::compute_clip_zoom_factor::<P>(self.aperture_angle);
        
        self.deproj_field_of_view::<P>();
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
        self.rotate::<P>();
    }

    pub fn set_rotation_mat<P: Projection>(&mut self, r: &Matrix4<f32>) {
        self.r = *r;

        self.rotate::<P>();
    }

    fn rotate<P: Projection>(&mut self) {
        if let Some(pos_world_space) = self.pos_world_space {
            let mut pos_transformed_space = [Vector4::new(0_f32, 0_f32, 0_f32, 0_f32); NUM_VERTICES];
            for idx_vertex in 0..NUM_VERTICES {
                pos_transformed_space[idx_vertex] = self.r * pos_world_space[idx_vertex];
            }

            self.pos_transformed_space = Some(pos_transformed_space);
        } else {
            self.pos_transformed_space = None;
        }

        self.compute_healpix_cells::<P>();
    }
    
    fn compute_healpix_cells<P: Projection>(&mut self) {
        // Compute the depth corresponding to the angular resolution of a pixel
        // along the width of the screen
        let max_depth = self.config
            // Max depth of the current HiPS tiles
            .max_depth();
        
        let depth = std::cmp::min(
            math::fov_to_depth(self.aperture_angle, self.width, self.config.tile_config()),
            max_depth,
        );
        //console::log_1(&format!("max depth {:?}", max_depth).into());

        let cells = self.get_cells_in_fov::<P>(depth);

        // Look for the newly added cells in the field of view
        // by doing the difference of the new cells set with the previous one
        let new_cells = cells.difference(&self.cells).collect::<HashSet<_>>();
        self.is_there_new_cells = new_cells.len() > 0;
        self.new_cells = cells.iter().cloned()
            .map(|cell| {
                (cell, new_cells.contains(&cell))
            })
            .collect::<HashMap<_, _>>();
        self.cells = cells;
        self.current_depth = depth;
    }

    pub fn get_cells_in_fov<P: Projection>(&self, depth: u8) -> HashSet<HEALPixCell> {
        if let Some(pos_transformed_space) = self.pos_transformed_space {
            self.polygon_coverage::<P>(depth, &pos_transformed_space)
        } else {
            crate::healpix_cell::allsky(depth)
        }
    }

    fn polygon_coverage<P: Projection>(&self, depth: u8, vertices: &[Vector4<f32>; NUM_VERTICES]) -> HashSet<HEALPixCell> {
        /*if depth <= 1 {
            return crate::healpix_cell::allsky(depth);
        }*/
        //console::log_1(&format!("compute polygon {:?}", self.aperture_angle.0).into());

        let lon_lat_vertices = vertices.iter()
            .map(|vertex| {
                let (ra, dec) = math::xyzw_to_radec(*vertex);
                (ra as f64, dec as f64)
            })
            .collect::<Vec<_>>();

        let moc = healpix::nested::polygon_coverage(depth, &lon_lat_vertices, false);
        let cells: HashSet<HEALPixCell> = moc.flat_iter()
            .map(|idx| {
                HEALPixCell(depth, idx)
            })
            .collect();

        // Get the position of the center of the field of view
        let center_world_space: Vector3<f32> = (self.r * P::clip_to_world_space(Vector2::new(0_f32, 0_f32)).unwrap()).truncate();
        let (center_ra, center_dec) = math::xyz_to_radec(center_world_space);
        let hash_center_fov = healpix::nested::hash(depth, center_ra as f64, center_dec as f64);
        let cell_center_fov = HEALPixCell(depth, hash_center_fov);

        if !cells.contains(&cell_center_fov) {
            let allsky = crate::healpix_cell::allsky(depth);

            allsky.difference(&cells).cloned().collect()
        } else {
            cells
        }
    }

    // Returns the current set of HEALPix cells contained in the field of view
    pub fn healpix_cells(&self) -> &HashSet<HEALPixCell> {
        &self.cells
    }

    // Returns the current set of HEALPix cells contained in the field of view,
    // each associated with a flag telling whether the cell is new or not.
    pub fn new_healpix_cells(&self) -> &HashMap<HEALPixCell, bool> {
        &self.new_cells
    }

    pub fn is_new_cells(&self) -> bool {
        self.is_there_new_cells
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