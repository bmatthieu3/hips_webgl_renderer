use crate::texture::create_texture_2d;
use crate::texture::Texture2D;

pub struct Catalog {
    center: Vec<f32>, // Position of the observations
    num_instances: usize,

    vertices: Vec<f32>, // Offsets and UVs
    indices: Vec<u16>,
    size: f32,

    kernel_texture: Texture2D,
}

use js_sys::Math;
use cgmath::Vector3;
use cgmath::InnerSpace;

impl Catalog {
    pub fn new(gl: &WebGl2Context) -> Catalog {
        let mut center = vec![];

        let num_instances = 10000;
        for _ in 0..num_instances {
            let x = (Math::random() as f32) * 2_f32 - 1_f32;
            let y = (Math::random() as f32) * 2_f32 - 1_f32;
            let z = (Math::random() as f32) * 2_f32 - 1_f32;

            let mut vertex = Vector3::new(x, y, z);
            vertex = vertex.normalize();

            center.push(vertex.x);
            center.push(vertex.y);
            center.push(vertex.z);
        }

        // Store the vertices position and UV
        let vertices = vec![
            -0.5, -0.5, 0.0, 0.0,
            0.5, -0.5, 1.0, 0.0,
            0.5, 0.5, 1.0, 1.0,
            -0.5, 0.5, 0.0, 1.0, 
        ];

        let indices = vec![
            0, 1, 2,
            0, 2, 3,
        ];

        let size = 0.01_f32;

        // Load the texture of the gaussian kernel
        let kernel_texture = create_texture_2d(gl, "./textures/kernel.png");

        Catalog {
            center,
            num_instances, 

            vertices,
            indices,

            size,

            kernel_texture
        }
    }
}

use crate::renderable::Mesh;
use crate::shader::Shader;

use crate::renderable::VertexArrayObject;
use crate::renderable::buffers::array_buffer::ArrayBuffer;
use crate::renderable::buffers::array_buffer_instanced::ArrayBufferInstanced;
use crate::renderable::buffers::buffer_data::BufferData;
use crate::renderable::buffers::element_array_buffer::ElementArrayBuffer;

use cgmath::Matrix4;

use web_sys::WebGl2RenderingContext;
use crate::WebGl2Context;

use crate::projection::ProjectionType;
use crate::viewport::ViewPort;

impl Mesh for Catalog {
    fn create_buffers(&self, gl: &WebGl2Context) -> VertexArrayObject {
        let mut vertex_array_object = VertexArrayObject::new(gl);
        vertex_array_object.bind();

        // ARRAY buffer creation
        let array_buffer = ArrayBuffer::new(
            gl,
            4 * std::mem::size_of::<f32>(),
            &[2, 2],
            &[0 * std::mem::size_of::<f32>(), 2 * std::mem::size_of::<f32>()],
            BufferData(self.vertices.as_ref()),
            WebGl2RenderingContext::STATIC_DRAW,
        );
        let array_buffer_instanced = ArrayBufferInstanced::new(
            gl,
            2,
            3 * std::mem::size_of::<f32>(),
            3,
            BufferData(self.center.as_ref()),
            WebGl2RenderingContext::STATIC_DRAW,
        );

        // ELEMENT ARRAY buffer creation
        let indexes_buffer = ElementArrayBuffer::new(
            gl,
            BufferData(self.indices.as_ref()),
            WebGl2RenderingContext::STATIC_DRAW,
        );

        vertex_array_object.set_array_buffer(array_buffer);
        vertex_array_object.set_array_buffer_instanced(array_buffer_instanced);

        vertex_array_object.set_element_array_buffer(indexes_buffer);

        vertex_array_object.unbind();
        // Unbind the buffer
        //gl.bind_buffer(WebGl2RenderingContext::ELEMENT_ARRAY_BUFFER, None);
        vertex_array_object
    }

    fn send_uniforms(&self, gl: &WebGl2Context, shader: &Shader) {
        // Send current depth
        //let location_current_depth = shader.get_uniform_location("current_depth");
        //gl.uniform1i(location_current_depth, 0);
        // Send the gaussian kernel texture
        self.kernel_texture.send_to_shader(&gl, shader, "kernel_texture");
    }

    fn get_vertices<'a>(&'a self) -> (BufferData<'a, f32>, BufferData<'a, u16>) {
        unreachable!();
    }

    fn update(&mut self, projection: &ProjectionType, local_to_world_mat: &Matrix4<f32>, viewport: &ViewPort) {}

    fn draw(&self, gl: &WebGl2Context, vao: &VertexArrayObject) {
        gl.draw_elements_instanced_with_i32(
            WebGl2RenderingContext::TRIANGLES,
            vao.num_elements() as i32,
            WebGl2RenderingContext::UNSIGNED_SHORT,
            0,
            vao.num_instances() as i32,
        );
    }
}

use crate::renderable::DisableDrawing;
impl DisableDrawing for Catalog {
    fn disable(&mut self) {
    }
}