use crate::math;
use crate::renderable::buffers::buffer_data::BufferData;

use std::convert::TryInto;


const NUM_POINTS: usize = 100;
const ISOLON_NUM_POINTS: usize = 100;
const ISOLAT_NUM_POINTS: usize = 100;

pub struct Grid;
impl Grid {
    pub fn new() -> Grid {
        Grid {}
    }

    fn create_vertices_array() -> BufferData<f32> {
        let lon_step = 2_f32 * std::f32::consts::PI / (ISOLON_NUM_POINTS as f32);
        let lat_step = std::f32::consts::PI / (ISOLAT_NUM_POINTS as f32);

        let mut pos_world_space = Vec::with_capacity((ISOLAT_NUM_POINTS + 1) * (ISOLON_NUM_POINTS + 1) * 3);

        for i in 0..(ISOLAT_NUM_POINTS + 1) {
            let delta = (std::f32::consts::PI/2_f32) - (i as f32)*lat_step;

            let y = delta.sin();
            let xz = delta.cos();

            for j in 0..(ISOLON_NUM_POINTS + 1) {
                let theta = -(j as f32) * lon_step;
                let x = xz * theta.sin();
                let z = xz * theta.cos();

                pos_world_space.push(x);
                pos_world_space.push(y);
                pos_world_space.push(z);
            }
        }

        pos_world_space.into()
    }

    fn create_index_array() -> BufferData<u16> {
        let mut indices = Vec::with_capacity(NUM_POINTS * 2);

        for i in 0..ISOLAT_NUM_POINTS {
            let mut k1 = (i * (ISOLON_NUM_POINTS + 1)) as u16;
            let mut k2 = (k1 + (ISOLON_NUM_POINTS as u16) + 1) as u16;

            for j in 0..ISOLON_NUM_POINTS {
                // k1 => k2 => k1+1
                if i != 0 {
                    indices.push(k1);
                    indices.push(k2);
                    indices.push(k1 + 1);
                }

                // k1+1 => k2 => k2+1
                if i != ISOLAT_NUM_POINTS - 1 {
                    indices.push(k1 + 1);
                    indices.push(k2);
                    indices.push(k2 + 1);
                }

                k1 += 1;
                k2 += 1;
            }
        }

        BufferData(indices)
    }
}

use crate::renderable::Mesh;
use std::rc::Rc;
use web_sys::WebGl2RenderingContext;

use crate::ProjectionType;

use crate::renderable::buffers::vertex_array_object::VertexArrayObject;
use crate::renderable::buffers::array_buffer::ArrayBuffer;
use crate::renderable::buffers::element_array_buffer::ElementArrayBuffer;

use crate::Shader;

use web_sys::console;

impl Mesh for Grid {
    fn create_buffers(&self, gl: Rc<WebGl2RenderingContext>, projection: &ProjectionType) -> VertexArrayObject {
        let mut vertex_array_object = VertexArrayObject::new(gl.clone());
        vertex_array_object.bind();

        // ARRAY buffer creation
        let vertices_data = Self::create_vertices_array();
        console::log_1(&format!("vertices: {:?}", vertices_data.0).into());

        let array_buffer = ArrayBuffer::new(
            gl.clone(),
            3 * std::mem::size_of::<f32>(),
            &[3],
            &[0 * std::mem::size_of::<f32>()],
            vertices_data,
            WebGl2RenderingContext::STATIC_DRAW,
        );

        // ELEMENT ARRAY buffer creation
        let indexes_data = Self::create_index_array();
        console::log_1(&format!("indexes: {:?}", indexes_data.0).into());
        let indexes_buffer = ElementArrayBuffer::new(
            gl,
            indexes_data,
            WebGl2RenderingContext::STATIC_DRAW,
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
        gl.uniform4f(location_color.as_ref(), 1_f32, 1_f32, 1_f32, 0.2_f32);
    }

    fn update_vertex_and_element_arrays(&self, model: &cgmath::Matrix4::<f32>, projection: &ProjectionType) -> (BufferData<f32>, BufferData<u16>) {
        unreachable!();
    }
}

pub struct IsoLatitudeLine {
    lat: f32,
    data: Vec<cgmath::Vector4<f32>>,
}
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

/*
fn build_world_space_vertices_lon(lon: f32) -> Vec<cgmath::Vector4<f32>> {
    //let ang = math::angular_distance_lonlat(delta_lon.start, lat, delta_lon.end, lat);
    let phi_step = std::f32::consts::PI / ((ISOLON_NUM_POINTS - 1) as f32);

    let mut pos_local_space = Vec::with_capacity(ISOLON_NUM_POINTS - 1);

    let delta = lat;
    let y = delta.sin();
    let xz = delta.cos();

    let theta_start = -std::f32::consts::PI;
    for i in 0..(ISOLAT_NUM_POINTS - 1) {
        let theta = theta_start + (i as f32) * theta_step;

        let x = xz * theta.sin();
        let z = xz * theta.cos();

        pos_local_space.push(cgmath::Vector4::new(x, y, z, 1_f32));
    }
    pos_local_space
}*/


/*impl IsoLatitudeLine {
    pub fn new(lat: f32) -> IsoLatitudeLine {
        let data = build_world_space_vertices_lat(lat);

        IsoLatitudeLine {
            lat,
            data,
        }
    }

    fn update_arrays(&self, model: &cgmath::Matrix4::<f32>, projection: &ProjectionType) -> (BufferData<f32>, BufferData<u32>) {
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
        let mut threshold_px = 2_f32 * (200_f32 / (width as f32));
        threshold_px = threshold_px * threshold_px;

        let mut i = 0;
        for idx in 0..num_points {
            let next_idx = (idx + 1) % num_points;

            let cur_to_next_screen_pos = cgmath::Vector2::new(
                pos_screen_space[2*next_idx] - pos_screen_space[2*idx],
                pos_screen_space[2*next_idx + 1] - pos_screen_space[2*idx + 1]
            );

            if cur_to_next_screen_pos.magnitude2() < threshold_px {
                indices[i] = idx as u32;
                indices[i + 1] = next_idx as u32;
                i += 2;
            }
        }

        (pos_screen_space.into(), indices.into())
    }
}

impl Mesh for IsoLatitudeLine {
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
        gl.uniform4f(location_color.as_ref(), 0_f32, 1_f32, 0_f32, 0.2_f32);
    }

    fn update_vertex_and_element_arrays(&self, model: &cgmath::Matrix4::<f32>, projection: &ProjectionType) -> (BufferData<f32>, BufferData<u32>) {
        self.update_arrays(model, projection)
    }
}*/

pub struct ProjetedGrid {
    step_lat: cgmath::Rad<f32>, // The number of lines in the view
    step_lon: cgmath::Rad<f32>, // The number of lines in the view
    data: Vec<cgmath::Vector4<f32>>,
}
fn build_grid_vertices(step_lat: &cgmath::Rad<f32>, step_lon: &cgmath::Rad<f32>) -> Vec<cgmath::Vector4<f32>> {
    let mut num_lat = (std::f32::consts::PI / step_lat.0) as usize;
    num_lat = num_lat - 1;
    let lat_start = (-std::f32::consts::PI / 2_f32) + step_lat.0;

    let mut num_lon = (std::f32::consts::PI / step_lon.0) as usize;
    //num_lon = num_lon;
    let lon_start = (-std::f32::consts::PI) + step_lon.0;

    let mut vertices = Vec::with_capacity(num_lat * ISOLAT_NUM_POINTS + num_lon * ISOLON_NUM_POINTS);
    for i in 0..num_lat {
        let lat = lat_start + (i as f32) * step_lat.0;
        vertices.extend(build_world_space_vertices_lat(lat));
    }

    for i in 0..num_lon {
        let lon = lon_start + (i as f32) * step_lon.0;
        vertices.extend(build_world_space_vertices_lon(lon));
    }

    vertices
}

use crate::viewport::get_window_size;
use cgmath::{SquareMatrix, InnerSpace};
impl ProjetedGrid {
    pub fn new(step_lat: cgmath::Rad<f32>, step_lon: cgmath::Rad<f32>) -> ProjetedGrid {
        let data = build_grid_vertices(&step_lat, &step_lon);

        ProjetedGrid {
            step_lat,
            step_lon,
            data,
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
        /*let mut i = 0;
        for idx in 0..num_points {
            let next_idx = (idx + 1) % num_points;

            let cur_to_next_screen_pos = cgmath::Vector2::new(
                pos_screen_space[2*next_idx] - pos_screen_space[2*idx],
                pos_screen_space[2*next_idx + 1] - pos_screen_space[2*idx + 1]
            );

            if cur_to_next_screen_pos.magnitude2() < threshold_px {
                indices[i] = idx as u32;
                indices[i + 1] = next_idx as u32;
                i += 2;
            }
        }*/

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
        gl.uniform4f(location_color.as_ref(), 0_f32, 0_f32, 1_f32, 0.2_f32);
    }

    fn update_vertex_and_element_arrays(&self, model: &cgmath::Matrix4::<f32>, projection: &ProjectionType) -> (BufferData<f32>, BufferData<u16>) {
        self.update_arrays(model, projection)
    }
}
