use crate::math;
use crate::renderable::buffers::buffer_data::BufferData;

use std::convert::TryInto;

const NUM_POINTS: usize = 50;

const ISOLON_NUM_POINTS: usize = 50;
const ISOLAT_NUM_POINTS: usize = 50;

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

    fn create_index_array() -> BufferData<u32> {
        let mut indices = Vec::with_capacity(NUM_POINTS * 2);

        for i in 0..ISOLAT_NUM_POINTS {
            let mut k1 = (i * (ISOLON_NUM_POINTS + 1)) as u32;
            let mut k2 = (k1 + (ISOLON_NUM_POINTS as u32) + 1) as u32;

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

    fn update_vertex_and_element_arrays(&self, model: &cgmath::Matrix4::<f32>, projection: &ProjectionType) -> (BufferData<f32>, BufferData<u32>) {
        unreachable!();
    }
}

use std::ops::Range;
pub struct IsoLatitudeLine {
    lat: f32,
    delta_lon: Range<f32>,

    data: Vec<cgmath::Vector4<f32>>,
}
fn build_world_space_vertices(lat: f32, delta_lon: Range<f32>) -> Vec<cgmath::Vector4<f32>> {
    //let ang = math::angular_distance_lonlat(delta_lon.start, lat, delta_lon.end, lat);
    let theta_step = 2_f32 * std::f32::consts::PI / ((ISOLAT_NUM_POINTS - 1) as f32);

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

use crate::viewport::get_window_size;
use cgmath::{SquareMatrix, InnerSpace};
impl IsoLatitudeLine {
    pub fn new(lat: f32, delta_lon: Range<f32>) -> IsoLatitudeLine {
        let data = build_world_space_vertices(lat, delta_lon.clone());

        IsoLatitudeLine {
            lat,
            delta_lon,
            data,
        }
    }

    fn update_arrays(&self, model: &cgmath::Matrix4::<f32>, projection: &ProjectionType) -> (BufferData<f32>, BufferData<u32>) {
        let pos_screen_space = self.data.iter()
            .map(|pos_local_space| {
                let pos_world_space = model * pos_local_space;
                let pos_screen_space = projection.world_to_screen_space(pos_world_space).unwrap();

                //vec![pos_screen_space.x, pos_screen_space.y]
                pos_screen_space
            })
            .collect::<Vec<_>>();
        
        let mut indices = Vec::with_capacity(ISOLAT_NUM_POINTS * 2);

        let (width, _) = get_window_size(&web_sys::window().unwrap());
        let threshold_px = 2_f32 * (100_f32 / (width as f32));

        for idx in 0..ISOLAT_NUM_POINTS {
            let next_idx = (idx + 1) % ISOLAT_NUM_POINTS;

            let cur_to_next_screen_pos = pos_screen_space[next_idx] - pos_screen_space[idx];

            if cur_to_next_screen_pos.magnitude() < threshold_px {
                indices.push(idx as u32);
                indices.push(next_idx as u32);
            }
        }

        //console::log_1(&format!("update line").into());

        /*let mut idx = 0;
        while idx < 2*ISOLAT_NUM_POINTS {
            let next_idx = (idx + 2) % (2*ISOLAT_NUM_POINTS);

            let cur_to_next_screen_pos = cgmath::Vector2::new(
                pos_screen_space[next_idx] - pos_screen_space[idx],
                pos_screen_space[next_idx + 1] - pos_screen_space[idx + 1],
            );

            if cur_to_next_screen_pos.magnitude() < threshold_px {
                indices.push((idx/2) as u32);
                indices.push((next_idx/2) as u32);
            }

            idx += 2;
        }*/

        let pos_screen_space = pos_screen_space.into_iter()
            .map(|p| {
                vec![p.x, p.y]
            })
            .flatten()
            .collect::<Vec<_>>();

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
        console::log_1(&format!("indexes: {:?}", indexes_data.0).into());
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
}