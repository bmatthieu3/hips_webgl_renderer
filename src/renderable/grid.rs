use crate::core::{
 BufferData,
 VertexArrayObject
};

const NUM_POINTS: usize = 40;

use web_sys::WebGl2RenderingContext;

use crate::color::Color;

use crate::Shader;

use web_sys::console;

use cgmath::{Rad, Vector2};

fn build_world_space_vertices_lat(lat: f32, lon_start: f32, lon_end: f32, num_points: usize) -> Vec<cgmath::Vector4<f32>> {
    //let ang = math::angular_distance_lonlat(delta_lon.start, lat, delta_lon.end, lat);
    let theta_step = (lon_end - lon_start) / ((num_points - 1) as f32);

    let mut pos_local_space = Vec::with_capacity(num_points);

    let delta = lat;
    let y = delta.sin();
    let xz = delta.cos();

    let theta_start = lon_start;
    for i in 0..num_points {
        let theta = theta_start + (i as f32) * theta_step;

        let x = xz * theta.sin();
        let z = xz * theta.cos();

        pos_local_space.push(cgmath::Vector4::new(x, y, z, 1_f32));
    }
    pos_local_space
}
fn build_world_space_vertices_lon(lon: f32, lat_start: f32, lat_end: f32, num_points: usize) -> Vec<cgmath::Vector4<f32>> {
    let pos_local_space = build_world_space_vertices_lat(0_f32, lat_start, lat_end, num_points);
    let rotation_mat = cgmath::Matrix4::<f32>::from_angle_y(cgmath::Rad(lon))
        * cgmath::Matrix4::<f32>::from_angle_z(cgmath::Deg(90_f32));

    pos_local_space.into_iter()
        .map(|p| {
            rotation_mat * p
        })
        .collect::<Vec<_>>()
}

fn build_grid_vertices(lat: &Vec<f32>, lon: &Vec<f32>, lat_start: f32, lat_end: f32, lon_start: f32, lon_end: f32) -> (Vec<cgmath::Vector4<f32>>, usize, usize) {
    let line_length_lon = lon_end - lon_start;
    let linear_vertices_density_lon = line_length_lon / (2_f32 * std::f32::consts::PI);
    let num_points_lon = (linear_vertices_density_lon * (NUM_POINTS as f32)) as usize;

    let line_length_lat = lat_end - lat_start;
    let linear_vertices_density_lat = line_length_lat / std::f32::consts::PI;
    let num_points_lat = (linear_vertices_density_lat * (NUM_POINTS as f32)) as usize;

    let mut vertices = Vec::with_capacity(lat.len() * num_points_lon + lon.len() * num_points_lat);
    for lat in lat.iter() {
        vertices.extend(build_world_space_vertices_lat(lat.clone(), lon_start, lon_end, num_points_lon));
    }
    for lon in lon.iter() {
        vertices.extend(build_world_space_vertices_lon(lon.clone(), lat_start, lat_end, num_points_lat));
    }
    console::log_1(&format!("num per isolon {:?}, num per isolat {:?}", num_points_lat, num_points_lon).into());
    console::log_1(&format!("num isolon {:?}, num isolat {:?}", lon.len(), lat.len()).into());
    //vertices.extend(build_world_space_vertices_lon(lon[0], lat_start, lat_end, num_points_lat));

    (vertices, num_points_lon, num_points_lat)
}

use crate::viewport::ViewPort;
pub struct ProjetedGrid {
    lat: Vec<f32>, // The number of lines in the view
    lon: Vec<f32>, // The number of lines in the view

    pos_local_space: Vec<cgmath::Vector4<f32>>,
    pos_clip_space: Vec<f32>,
    idx_vertices: Vec<u16>,

    label_pos_local_space: Vec<cgmath::Vector4<f32>>,
    num_points_lon: usize,
    num_points_lat: usize,

    label_pos_screen_space: Vec<cgmath::Vector2<f32>>,
    label_text: Vec<String>,
    font_size: f32,

    text_canvas: web_sys::CanvasRenderingContext2d,
    color: Color,

    vertex_array_object: VertexArrayObject,
}

use cgmath::InnerSpace;
use wasm_bindgen::JsCast;
use crate::math::radec_to_xyzw;
use crate::projection::Projection;

use crate::ShaderManager;
use crate::shaders;
use cgmath::SquareMatrix;
impl ProjetedGrid {
    pub fn new<P: Projection>(
        gl: &WebGl2Context,
        step_lat: cgmath::Rad<f32>,
        step_lon: cgmath::Rad<f32>,
        lat_bound: Option<Vector2<Rad<f32>>>,
        lon_bound: Option<Vector2<Rad<f32>>>,
        viewport: &ViewPort,
        shaders: &ShaderManager
    ) -> ProjetedGrid {
        let (lat_min, lat_max) = if let Some(lat_bound) = lat_bound {
            (lat_bound.x.0, lat_bound.y.0)
        } else {
            (-std::f32::consts::PI / 2_f32, std::f32::consts::PI / 2_f32)
        };
        let (lon_min, lon_max) = if let Some(lon_bound) = lon_bound {
            (lon_bound.x.0, lon_bound.y.0)
        } else {
            (-std::f32::consts::PI, std::f32::consts::PI)
        };

        let num_lat = ((lat_max - lat_min) / step_lat.0) as usize + 1;
        let lat_start = lat_min;
        let lat_end = lat_max;

        let num_lon = ((lon_max - lon_min) / step_lon.0) as usize + 1;
        let lon_start = lon_min;
        let lon_end = lon_max;

        let lat = (0..num_lat)
            .into_iter()
            .map(|idx_lat| {
                let lat = lat_start + (idx_lat as f32) * step_lat.0;
                lat
            })
            .collect::<Vec<_>>();
        let lon = (0..num_lon)
            .map(|idx_lon| {
                let lon = lon_start + (idx_lon as f32) * step_lon.0;
                lon
            })
            .collect::<Vec<_>>();

        // Build the line vertices
        let num_iso_lon = lon.len() >> 1;
        let (pos_local_space, num_points_lon, num_points_lat) = build_grid_vertices(&lat, &lon, lat_start, lat_end, lon_start, lon_end);
        let pos_clip_space = vec![];
        let idx_vertices = vec![];
        // Build the label positions
        let mut label_pos_local_space = lat.iter()
            .map(|lat| {
                let lat = cgmath::Rad(lat.clone());
                let lon = cgmath::Rad(0_f32);

                radec_to_xyzw(lon, lat)
            })
            .collect::<Vec<_>>();

        label_pos_local_space.extend(
            lon.iter()
            .map(|lon| {
                let lat = cgmath::Rad(0_f32);
                let lon = cgmath::Rad(lon.clone());

                radec_to_xyzw(lon, lat)
            })
            .collect::<Vec<_>>()
        );
        // Build the label text
        let mut label_text = lat.iter()
            .map(|lat| {
                let lat: cgmath::Deg<f32> = cgmath::Rad(lat.clone()).into();
                let lat = lat.0.round() as i16;
                lat.to_string() + "°"
            })
            .collect::<Vec<_>>();

        label_text.extend(
            lon.iter()
            .map(|lon| {
                let lon: cgmath::Deg<f32> = cgmath::Rad(lon.clone()).into();
                let mut lon = lon.0.round() as i16;
                if lon < 0 {
                    lon += 360;
                }
                lon.to_string() + "°"
            })
            .collect::<Vec<_>>()
        );

        // Get a reference to the text canvas
        let text_canvas = web_sys::window().unwrap()
            .document().unwrap()
            .get_element_by_id("labels_grid").unwrap()
            .dyn_into::<web_sys::HtmlCanvasElement>().unwrap();

        let window_size = viewport.get_window_size();
        text_canvas.set_width(window_size.x as u32);
        text_canvas.set_height(window_size.y as u32);

        let text_canvas = text_canvas.get_context("2d")
            .unwrap()
            .unwrap()
            .dyn_into::<web_sys::CanvasRenderingContext2d>().unwrap();

        let color = Color::new(0_f32, 1_f32, 0_f32, 0.2_f32);
        let text_color: String = (&color).into();

        text_canvas.set_fill_style(&text_color.into());
        text_canvas.set_global_alpha(0.7_f64);
        //console::log_1(&format!("font: {:?}", text_canvas.font()).into());
        
        let font_size = 12_f32;
        let font = (font_size as u8).to_string() + "px sans-serif";
        text_canvas.set_font(&font);

        let label_pos_screen_space = Vec::new();

        // Define the Vertex Array Object
        let mut vertex_array_object = VertexArrayObject::new(&gl);

        let shader = shaders.get::<shaders::Grid>().unwrap();
        shader.bind(gl)
            .bind_vertex_array_object(&mut vertex_array_object)
                // Store the vertex positions
                .add_array_buffer(
                    2 * std::mem::size_of::<f32>(),
                    &[2],
                    &[0 * std::mem::size_of::<f32>()],
                    WebGl2RenderingContext::DYNAMIC_DRAW,
                    BufferData::VecData(&pos_clip_space),
                )
                // Set the element buffer
                .add_element_buffer(
                    WebGl2RenderingContext::DYNAMIC_DRAW,
                    BufferData::VecData(&idx_vertices),
                )
                // Unbind the buffer
                .unbind();

        let mut grid = ProjetedGrid {
            lat,
            lon,

            pos_local_space,
            pos_clip_space,
            idx_vertices,
            label_pos_local_space,
            num_points_lon,
            num_points_lat,

            label_pos_screen_space,
            label_text,
            font_size,

            text_canvas,
            color,

            vertex_array_object
        };

        grid.update_grid_positions::<P>(&cgmath::Matrix4::identity(), viewport);
        grid.update_label_positions::<P>(&cgmath::Matrix4::identity(), viewport);

        grid
    }

    pub fn update_grid_positions<P: Projection>(&mut self, local_to_world_mat: &Matrix4<f32>, viewport: &ViewPort) {
        let window_size = viewport.get_window_size();

        self.pos_clip_space.clear();
        // UPDATE GRID VERTICES POSITIONS
        for pos_local_space in self.pos_local_space.iter() {
            let pos_clip_space = P::model_to_clip_space(&(local_to_world_mat * pos_local_space));

            self.pos_clip_space.push(pos_clip_space.x);
            self.pos_clip_space.push(pos_clip_space.y);
        }

        // TODO: Remove that this is inacceptable :))!
        /*let threshold: Rad<f32> = Deg(150_f32).into(); 
        if viewport.field_of_view().get_aperture() < threshold {
            return;
        }*/

        // UPDATE IDX VERTICES
        let num_vertices = (self.lat.len() * (self.num_points_lon - 1) + self.lon.len() * (self.num_points_lat - 1)) * 2;
        //let num_vertices = self.lat.len() * (self.num_points_lon - 1) * 2;
        self.idx_vertices = vec![0; num_vertices];

        let mut threshold_px = 2_f32 * (150_f32 / window_size.x);
        threshold_px = threshold_px * threshold_px;

        let mut i = 0;
        let mut idx_start = 0;
        while idx_start < self.lat.len() * self.num_points_lon {
            let num_points_step = self.num_points_lon;

            for j in 0..(num_points_step - 1) {
                let idx = idx_start + j;
                let next_idx = idx + 1;

                let cur_to_next_screen_pos = cgmath::Vector2::new(
                    self.pos_clip_space[2*next_idx] - self.pos_clip_space[2*idx],
                    self.pos_clip_space[2*next_idx + 1] - self.pos_clip_space[2*idx + 1]
                );

                if cur_to_next_screen_pos.magnitude2() < threshold_px {
                    self.idx_vertices[i] = idx as u16;
                    self.idx_vertices[i + 1] = next_idx as u16;
                    i += 2;
                }
            }
            idx_start += num_points_step;
        }

        while idx_start < (self.lat.len() * self.num_points_lon + self.lon.len() * self.num_points_lat) {
            let num_points_step = self.num_points_lat;

            for j in 0..(num_points_step - 1) {
                let idx = idx_start + j;
                let next_idx = idx + 1;

                let cur_to_next_screen_pos = cgmath::Vector2::new(
                    self.pos_clip_space[2*next_idx] - self.pos_clip_space[2*idx],
                    self.pos_clip_space[2*next_idx + 1] - self.pos_clip_space[2*idx + 1]
                );

                if cur_to_next_screen_pos.magnitude2() < threshold_px {
                    self.idx_vertices[i] = idx as u16;
                    self.idx_vertices[i + 1] = next_idx as u16;
                    i += 2;
                }
            }
            idx_start += num_points_step;
        }
    }

    pub fn update_label_positions<P: Projection>(&mut self, local_to_world_mat: &Matrix4<f32>, viewport: &ViewPort) {
        let window_size = viewport.get_window_size();
        //let viewport_zoom_factor = viewport.get_scaling_screen_factor();

        // UPDATE LABEL POSITIONS
        self.label_pos_screen_space.clear();
        for (label_text, pos_local_space) in self.label_text.iter().zip(self.label_pos_local_space.iter()) {
            let offset_pos_screen = self.text_canvas.measure_text(label_text).unwrap().width() as f32;

            let mut pos_screen_space = P::model_to_screen_space(&(local_to_world_mat * pos_local_space), viewport);
            //pos_screen_space += cgmath::Vector2::new(pos_screen_space.x - offset_pos_screen / 2_f32, pos_screen_space.y + self.font_size / 2_f32);

            self.label_pos_screen_space.push(pos_screen_space);
        }
    }

    pub fn draw_labels(&self, viewport: &ViewPort) {
        // Fill
        for (label_text, pos_screen_space) in self.label_text.iter().zip(self.label_pos_screen_space.iter()) {
            self.text_canvas.fill_text(label_text, pos_screen_space.x as f64, pos_screen_space.y as f64).unwrap();
        }
    }

    pub fn clear_canvas(&self, viewport: &ViewPort) {
        let window_size = viewport.get_window_size();
        self.text_canvas.clear_rect(0_f64, 0_f64, window_size.x as f64, window_size.y as f64);
    }

    pub fn set_color_rgb(&mut self, red: f32, green: f32, blue: f32) {
        self.color.red = red;
        self.color.green = green;
        self.color.blue = blue;

        // Change the text label color
        let text_color: String = (&self.color).into();
        self.text_canvas.set_fill_style(&text_color.into());
    }

    pub fn set_alpha(&mut self, alpha: f32) {
        self.color.alpha = alpha;
    }

    pub fn reproject<P: Projection>(&mut self, viewport: &ViewPort) {
        //if P::name() != "Orthographic" {
            self.update_grid_positions::<P>(
                viewport.get_inverted_model_mat(),
                viewport,
            );
            self.update_label_positions::<P>(
                viewport.get_inverted_model_mat(),
                viewport
            );

            // Update the VAO
            self.vertex_array_object.bind_for_update()
                .update_array(
                    0, 
                    WebGl2RenderingContext::DYNAMIC_DRAW,
                    BufferData::VecData(&self.pos_clip_space)
                )
                .update_element_array(
                    WebGl2RenderingContext::DYNAMIC_DRAW,
                    BufferData::VecData(&self.idx_vertices)
                );
        //}
    }

    pub fn draw(
        &self,
        gl: &WebGl2Context,
        shaders: &ShaderManager,
        viewport: &ViewPort
    ) {
        let shader = shaders.get::<shaders::Grid>().unwrap();
        shader.bind(gl)
            // Attach all the uniforms from the viewport
            .attach_uniforms_from(viewport)
            // Attach grid specialized uniforms
            .attach_uniform("grid_color", &self.color)
            .attach_uniform("model", viewport.get_inverted_model_mat())
            .attach_uniform("current_time", &utils::get_current_time())
            // Bind the Vertex Array Object for drawing
            .bind_vertex_array_object_ref(&self.vertex_array_object)
                .draw_elements_with_i32(
                    // Mode of render
                    WebGl2RenderingContext::LINES,
                    // Number of elements, by default None
                    None
                );
    }
}

use crate::WebGl2Context;
use cgmath::Matrix4;

use crate::utils;

use crate::renderable::DisableDrawing;
impl DisableDrawing for ProjetedGrid {
    fn disable(&mut self, viewport: &ViewPort) {
        // Clear the 2D canvas
        let window_size = viewport.get_window_size();
        self.text_canvas.clear_rect(0_f64, 0_f64, window_size.x as f64, window_size.y as f64);
    }
}
