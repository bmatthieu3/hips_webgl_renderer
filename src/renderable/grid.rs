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

use cgmath::{Rad, Deg, Vector2};

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

    (vertices, num_points_lon, num_points_lat)
}

use crate::viewport::ViewPort;
pub struct ProjetedGrid {
    lat: Vec<f32>, // The number of lines in the view
    lon: Vec<f32>, // The number of lines in the view

    pos_local_space: Vec<cgmath::Vector4<f32>>,
    pos_screen_space: Vec<f32>,
    idx_vertices: Vec<u16>,

    label_pos_local_space: Vec<cgmath::Vector4<f32>>,
    num_points_lon: usize,
    num_points_lat: usize,

    label_pos_screen_space: Vec<cgmath::Vector2<f64>>,
    label_text: Vec<String>,
    font_size: f64,

    text_canvas: web_sys::CanvasRenderingContext2d,
    color: cgmath::Vector4<f32>,
}

use cgmath::{SquareMatrix, InnerSpace};
use wasm_bindgen::JsCast;
use crate::math::radec_to_xyz;
use crate::{window_size_f32, window_size_f64, window_size_u32};

impl ProjetedGrid {
    pub fn new(step_lat: cgmath::Rad<f32>,
        step_lon: cgmath::Rad<f32>,
        lat_bound: Option<Vector2<Rad<f32>>>,
        lon_bound: Option<Vector2<Rad<f32>>>,
        projection: &ProjectionType,
        viewport: &ViewPort) -> ProjetedGrid {
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
        let pos_screen_space = vec![];
        let idx_vertices = vec![];
        // Build the label positions
        let mut label_pos_local_space = lat.iter()
            .map(|lat| {
                let lat = cgmath::Rad(lat.clone());
                let lon = cgmath::Rad(0_f32);

                radec_to_xyz(lon, lat)
            })
            .collect::<Vec<_>>();
        // Add the lat=pi/2 and lat=-pi/2
        label_pos_local_space.push(radec_to_xyz(Rad(0_f32), Deg(90_f32).into()));
        label_pos_local_space.push(radec_to_xyz(Rad(0_f32), Deg(-90_f32).into()));

        label_pos_local_space.extend(
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
                let lat = lat.0.round() as i16;
                lat.to_string() + "°"
            })
            .collect::<Vec<_>>();
        // Add the labels for the lat=pi/2 and lat=-pi/2
        label_text.push(String::from("90°"));
        label_text.push(String::from("-90°"));

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

        let (width, height) = window_size_u32();

        text_canvas.set_width(width);
        text_canvas.set_height(height);

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

            pos_local_space,
            pos_screen_space,
            idx_vertices,
            label_pos_local_space,
            num_points_lon,
            num_points_lat,

            label_pos_screen_space,
            label_text,
            font_size,

            text_canvas,
            color,
        };

        grid.update(projection, &cgmath::Matrix4::identity(), viewport);

        grid
    }

    pub fn update(&mut self, projection: &ProjectionType, model: &cgmath::Matrix4<f32>, viewport: &ViewPort) {
        let (width_screen, height_screen) = window_size_f32();

        let viewport_zoom_factor = viewport.get_zoom_factor();

        // UPDATE LABEL POSITIONS
        self.label_pos_screen_space.clear();
        for (label_text, pos_local_space) in self.label_text.iter().zip(self.label_pos_local_space.iter()) {
            let label_pos_world_space = model * pos_local_space;

            let label_pos_screen_space = projection.world_to_screen_space(label_pos_world_space).unwrap();

            let offset_pos_screen = self.text_canvas.measure_text(label_text).unwrap().width();
            
            // multiply by the zoom factor from the viewport
            let mut pos_screen_space = cgmath::Vector2::new(
                (((label_pos_screen_space.x * 0.5_f32) * viewport_zoom_factor + 0.5_f32) * width_screen) as f64,
                ((-label_pos_screen_space.y * 0.5_f32) * width_screen * viewport_zoom_factor + 0.5_f32 * height_screen) as f64
            );
            pos_screen_space += cgmath::Vector2::new(-offset_pos_screen / 2_f64, self.font_size / 2_f64);

            self.label_pos_screen_space.push(pos_screen_space);
        }

        self.pos_screen_space.clear();
        // UPDATE GRID VERTICES POSITIONS
        for pos_local_space in self.pos_local_space.iter() {
            let pos_world_space = model * pos_local_space;

            let pos_screen_space = projection.world_to_screen_space(pos_world_space.clone()).unwrap();

            self.pos_screen_space.push(pos_screen_space.x);
            self.pos_screen_space.push(pos_screen_space.y);
        }

        // UPDATE IDX VERTICES
        let num_points = self.pos_screen_space.len() >> 1;
        self.idx_vertices = vec![0; num_points * 2];

        let mut threshold_px = 2_f32 * (100_f32 / width_screen);
        threshold_px = threshold_px * threshold_px;

        let mut i = 0;
        let mut idx_start = 0;
        while idx_start < num_points {
            let num_points_step = if idx_start < self.lat.len() * self.num_points_lon {
                self.num_points_lon
            } else {
                self.num_points_lat
            };

            let idx_end = idx_start + num_points_step;
            for idx in idx_start..idx_end {
                let next_idx = (idx + 1) % num_points_step + idx_start;

                let cur_to_next_screen_pos = cgmath::Vector2::new(
                    self.pos_screen_space[2*next_idx] - self.pos_screen_space[2*idx],
                    self.pos_screen_space[2*next_idx + 1] - self.pos_screen_space[2*idx + 1]
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

    pub fn draw(&self) {
        // Clear the 2D canvas
        let (width_screen, height_screen) = window_size_f64();
        self.text_canvas.clear_rect(0_f64, 0_f64, width_screen, height_screen);
        // Fill
        for (label_text, pos_screen_space) in self.label_text.iter().zip(self.label_pos_screen_space.iter()) {
            self.text_canvas.fill_text(label_text, pos_screen_space.x as f64, pos_screen_space.y as f64).unwrap();
        }
    }
}

use crate::WebGl2Context;
impl Mesh for ProjetedGrid {
    fn create_buffers(&self, gl: &WebGl2Context, projection: &ProjectionType) -> VertexArrayObject {
        let mut vertex_array_object = VertexArrayObject::new(gl);
        vertex_array_object.bind();

        let ref vertices_data = self.pos_screen_space;
        let ref idx_data = self.idx_vertices;
        // ARRAY buffer creation
        console::log_1(&format!("vertices: {:?} {:?}", vertices_data.len(), vertices_data).into());

        let array_buffer = ArrayBuffer::new(
            gl,
            2 * std::mem::size_of::<f32>(),
            &[2],
            &[0 * std::mem::size_of::<f32>()],
            BufferData(&vertices_data),
            WebGl2RenderingContext::DYNAMIC_DRAW,
        );

        // ELEMENT ARRAY buffer creation
        console::log_1(&format!("indexes: {:?} {:?}", idx_data.len(), idx_data).into());
        //console::log_1(&format!("indexes: {:?}, len {:?}", indexes_data.0, indexes_data.0.len()).into());
        let indexes_buffer = ElementArrayBuffer::new(
            gl,
            BufferData(&idx_data),
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

    fn update_vertex_and_element_arrays<'a>(&'a self) -> (BufferData<'a, f32>, BufferData<'a, u16>) {
        (BufferData(&self.pos_screen_space), BufferData(&self.idx_vertices))
    }
}
