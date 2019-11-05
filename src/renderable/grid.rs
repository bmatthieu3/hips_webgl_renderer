use crate::math;
use crate::renderable::buffers::buffer_data::BufferData;

use std::convert::TryInto;


const NUM_POINTS: usize = 100;
const ISOLON_NUM_POINTS: usize = 100;
const ISOLAT_NUM_POINTS: usize = 100;

use crate::renderable::Mesh;
use std::rc::Rc;
use web_sys::WebGl2RenderingContext;

use crate::ProjectionType;

use crate::renderable::buffers::vertex_array_object::VertexArrayObject;
use crate::renderable::buffers::array_buffer::ArrayBuffer;
use crate::renderable::buffers::element_array_buffer::ElementArrayBuffer;

use crate::Shader;

use web_sys::console;

fn build_world_space_vertices_lat(lat: f32) -> Vec<cgmath::Vector4<f32>> {
    //let ang = math::angular_distance_lonlat(delta_lon.start, lat, delta_lon.end, lat);
    let theta_step = 2_f32 * std::f32::consts::PI / (ISOLAT_NUM_POINTS as f32);

    let mut pos_local_space = Vec::with_capacity(ISOLAT_NUM_POINTS);

    let delta = lat;
    let y = delta.sin();
    let xz = delta.cos();

    let theta_start = -std::f32::consts::PI;
    for i in 0..ISOLAT_NUM_POINTS {
        let theta = theta_start + (i as f32) * theta_step;

        let x = xz * theta.sin();
        let z = xz * theta.cos();

        pos_local_space.push(cgmath::Vector4::new(x, y, z, 1_f32));
    }
    pos_local_space
}
fn build_world_space_vertices_lon(lon: f32) -> Vec<cgmath::Vector4<f32>> {
    let pos_local_space = build_world_space_vertices_lat(0_f32);
    let rotation_mat = cgmath::Matrix4::<f32>::from_angle_y(cgmath::Rad(lon))
        * cgmath::Matrix4::<f32>::from_angle_z(cgmath::Deg(90_f32));

    pos_local_space.into_iter()
        .map(|p| {
            rotation_mat * p
        })
        .collect::<Vec<_>>()
}

pub struct ProjetedGrid {
    lat: Vec<f32>, // The number of lines in the view
    lon: Vec<f32>, // The number of lines in the view

    data: Vec<cgmath::Vector4<f32>>,
    label_pos_world_space: Vec<cgmath::Vector4<f32>>,
    label_pos_screen_space: Vec<cgmath::Vector2<f64>>,
    label_text: Vec<String>,
    font_size: f64,

    text_canvas: web_sys::CanvasRenderingContext2d,
    color: cgmath::Vector4<f32>,
}
fn build_grid_vertices(lat: &Vec<f32>, lon: &Vec<f32>) -> Vec<cgmath::Vector4<f32>> {
    let mut vertices = Vec::with_capacity(lat.len() * ISOLAT_NUM_POINTS + lon.len() * ISOLON_NUM_POINTS);
    for lat in lat.iter() {
        vertices.extend(build_world_space_vertices_lat(lat.clone()));
    }
    for lon in lon.iter() {
        vertices.extend(build_world_space_vertices_lon(lon.clone()));
    }

    vertices
}

use crate::viewport::get_window_size;
use cgmath::{SquareMatrix, InnerSpace};
use wasm_bindgen::JsCast;
use crate::math::radec_to_xyz;

impl ProjetedGrid {
    pub fn new(step_lat: cgmath::Rad<f32>, step_lon: cgmath::Rad<f32>, projection: &ProjectionType) -> ProjetedGrid {
        let mut num_lat = (std::f32::consts::PI / step_lat.0) as usize;
        let lat_start = (-std::f32::consts::PI / 2_f32) + step_lat.0;

        let num_lon = (2_f32 * std::f32::consts::PI / step_lon.0) as usize;
        let lon_start = (-std::f32::consts::PI) + step_lon.0;

        let lat = (0..num_lat)
            .into_iter()
            .map(|idx_lat| {
                let lat = lat_start + (idx_lat as f32) * step_lat.0;
                //let lon = lon_start + (idx_lon as f32) * step_lon.0;
                lat
            })
            .collect::<Vec<_>>();
        let lon = (0..num_lon)
            .map(|idx_lon| {
                let lon = lon_start + (idx_lon as f32) * step_lon.0;
                //let lon = lon_start + (idx_lon as f32) * step_lon.0;
                lon
            })
            .collect::<Vec<_>>();

        // Build the line vertices
        let num_iso_lon = lon.len() >> 1;
        let data = build_grid_vertices(&lat, &lon[0..num_iso_lon].to_vec());
        // Build the label positions
        let mut label_pos_world_space = lat.iter()
            .map(|lat| {
                let lat = cgmath::Rad(lat.clone());
                let lon = cgmath::Rad(0_f32);

                radec_to_xyz(lon, lat)
            })
            .collect::<Vec<_>>();
        label_pos_world_space.extend(
            lon.iter()
            .map(|lon| {
                let lat = cgmath::Rad(0_f32);
                let lon = cgmath::Rad(lon.clone());

                radec_to_xyz(lon, lat)
            })
            .collect::<Vec<_>>()
        );
        // Build the label text
        let mut label_text = lat.iter()
            .map(|lat| {
                let lat: cgmath::Deg<f32> = cgmath::Rad(lat.clone()).into();
                (lat.0.round() as i16).to_string() + "°"
            })
            .collect::<Vec<_>>();
        label_text.extend(
            lon.iter()
            .map(|lon| {
                let lon: cgmath::Deg<f32> = cgmath::Rad(lon.clone()).into();
                (lon.0.round() as i16).to_string() + "°"
            })
            .collect::<Vec<_>>()
        );

        // Get a reference to the text canvas
        let text_canvas = web_sys::window().unwrap()
            .document().unwrap()
            .get_element_by_id("labels_grid").unwrap()
            .dyn_into::<web_sys::HtmlCanvasElement>().unwrap();

        let (width_screen, height_screen) = get_window_size(&web_sys::window().unwrap());
        text_canvas.set_width(width_screen as u32);
        text_canvas.set_height(height_screen as u32);

        let text_canvas = text_canvas.get_context("2d")
            .unwrap()
            .unwrap()
            .dyn_into::<web_sys::CanvasRenderingContext2d>().unwrap();

        let color = cgmath::Vector4::new(0_f32, 1_f32, 0_f32, 0.2_f32);
        let mut text_color = String::from("rgb(");
        text_color += &((color.x * 255_f32) as u8).to_string();
        text_color += &", ";
        text_color += &((color.y * 255_f32) as u8).to_string();
        text_color += &", ";
        text_color += &((color.z * 255_f32) as u8).to_string();
        text_color += &")";

        text_canvas.set_fill_style(&text_color.into());
        text_canvas.set_global_alpha(0.7_f64);
        //console::log_1(&format!("font: {:?}", text_canvas.font()).into());
        
        let font_size = 12_f64;
        let font = (font_size as u8).to_string() + "px sans-serif";
        text_canvas.set_font(&font);

        let label_pos_screen_space = Vec::new();
        let mut grid = ProjetedGrid {
            lat,
            lon,

            data,
            label_pos_world_space,
            label_pos_screen_space,
            label_text,
            font_size,

            text_canvas,
            color,
        };

        grid.update(projection, &cgmath::Matrix4::identity());

        grid
    }

    pub fn update(&mut self, projection: &ProjectionType, model: &cgmath::Matrix4<f32>) {
        let (width_screen, height_screen) = get_window_size(&web_sys::window().unwrap());

        self.label_pos_screen_space.clear();
        for (label_text, pos_local_space) in self.label_text.iter().zip(self.label_pos_world_space.iter()) {
            let pos_world_space = model * pos_local_space;
            let pos_screen_space = projection.world_to_screen_space(pos_world_space.clone()).unwrap();

            let offset_pos_screen = self.text_canvas.measure_text(label_text).unwrap().width();

            let mut pos_screen_space = cgmath::Vector2::new(
                (((pos_screen_space.x * 0.5_f32) + 0.5_f32) * width_screen) as f64,
                ((-pos_screen_space.y * 0.5_f32) * width_screen + 0.5_f32 * height_screen) as f64
            );
            pos_screen_space += cgmath::Vector2::new(-offset_pos_screen / 2_f64, self.font_size / 2_f64);

            self.label_pos_screen_space.push(pos_screen_space);
        }
    }

    pub fn draw(&self) {
        // Clear the 2D canvas
        let (width_screen, height_screen) = get_window_size(&web_sys::window().unwrap());
        self.text_canvas.clear_rect(0_f64, 0_f64, width_screen as f64, height_screen as f64);
        // Fill
        for (label_text, pos_screen_space) in self.label_text.iter().zip(self.label_pos_screen_space.iter()) {
            self.text_canvas.fill_text(label_text, pos_screen_space.x as f64, pos_screen_space.y as f64).unwrap();
        }
    }

    fn update_arrays(&self, model: &cgmath::Matrix4::<f32>, projection: &ProjectionType) -> (BufferData<f32>, BufferData<u16>) {
        let pos_screen_space = self.data.iter()
            .map(|pos_local_space| {
                let pos_world_space = model * pos_local_space;
                let pos_screen_space = projection.world_to_screen_space(pos_world_space).unwrap();

                vec![pos_screen_space.x, pos_screen_space.y]
            })
            .flatten()
            .collect::<Vec<_>>();

        let num_points = pos_screen_space.len() >> 1;
        let mut indices = vec![0; num_points * 2];

        let (width, _) = get_window_size(&web_sys::window().unwrap());
        let mut threshold_px = 2_f32 * (100_f32 / (width as f32));
        threshold_px = threshold_px * threshold_px;

        let mut i = 0;
        let mut idx_start = 0;
        while idx_start < num_points {
            let idx_end = idx_start + NUM_POINTS;
            for idx in idx_start..idx_end {
                let next_idx = (idx + 1) % NUM_POINTS + idx_start;

                let cur_to_next_screen_pos = cgmath::Vector2::new(
                    pos_screen_space[2*next_idx] - pos_screen_space[2*idx],
                    pos_screen_space[2*next_idx + 1] - pos_screen_space[2*idx + 1]
                );

                if cur_to_next_screen_pos.magnitude2() < threshold_px {
                    indices[i] = idx as u16;
                    indices[i + 1] = next_idx as u16;
                    i += 2;
                }
            }
            idx_start += NUM_POINTS;
        }

        (pos_screen_space.into(), indices.into())
    }
}

impl Mesh for ProjetedGrid {
    fn create_buffers(&self, gl: Rc<WebGl2RenderingContext>, projection: &ProjectionType) -> VertexArrayObject {
        let mut vertex_array_object = VertexArrayObject::new(gl.clone());
        vertex_array_object.bind();

        // ARRAY buffer creation
        let (vertices_data, indexes_data) = self.update_arrays(&cgmath::Matrix4::identity(), projection);
        console::log_1(&format!("vertices: {:?}", vertices_data.0).into());

        let array_buffer = ArrayBuffer::new(
            gl.clone(),
            2 * std::mem::size_of::<f32>(),
            &[2],
            &[0 * std::mem::size_of::<f32>()],
            vertices_data,
            WebGl2RenderingContext::DYNAMIC_DRAW,
        );

        // ELEMENT ARRAY buffer creation
        console::log_1(&format!("indexes: {:?}, len {:?}", indexes_data.0, indexes_data.0.len()).into());
        let indexes_buffer = ElementArrayBuffer::new(
            gl,
            indexes_data,
            WebGl2RenderingContext::DYNAMIC_DRAW,
        );

        vertex_array_object.set_array_buffer(array_buffer);
        vertex_array_object.set_element_array_buffer(indexes_buffer);
        console::log_1(&format!("grid init").into());
        vertex_array_object.unbind();
        // Unbind the buffer
        //gl.bind_buffer(WebGl2RenderingContext::ELEMENT_ARRAY_BUFFER, None);
        vertex_array_object
    }

    fn send_uniforms(&self, gl: &WebGl2RenderingContext, shader: &Shader) {
        let location_color = shader.get_uniform_location(gl, "location_color");
        gl.uniform4f(location_color.as_ref(), self.color.x, self.color.y, self.color.z, self.color.w);
    }

    fn update_vertex_and_element_arrays(&self, model: &cgmath::Matrix4::<f32>, projection: &ProjectionType) -> (BufferData<f32>, BufferData<u16>) {
        self.update_arrays(model, projection)
    }
}
