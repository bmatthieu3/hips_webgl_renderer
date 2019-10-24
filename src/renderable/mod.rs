use web_sys::WebGl2RenderingContext;
use web_sys::WebGlVertexArrayObject;
use crate::shader::Shader;

use crate::viewport::ViewPort;
use crate::renderable::projection::ProjectionType;

use std::rc::Rc;
use std::borrow::Borrow;

pub mod buffers;
pub mod hips_sphere;
pub mod projection;
pub mod grid;

trait VertexBufferObject {
    fn bind(&self);
    fn unbind(&self);
}

use buffers::vertex_array_object::VertexArrayObject;
pub trait Mesh {
    fn create_buffers(gl: Rc<WebGl2RenderingContext>, projection: &ProjectionType) -> VertexArrayObject;
    fn send_uniforms(&self, gl: &WebGl2RenderingContext, shader: &Shader);
}

use std::cell::RefCell;

pub struct Renderable<T>
where T: Mesh {
    shader: Rc<Shader>,
    model_mat: cgmath::Matrix4::<f32>,

    scale_mat: cgmath::Matrix4::<f32>,
    rotation_mat: cgmath::Matrix4::<f32>,
    translation_mat: cgmath::Matrix4::<f32>,

    // VAO index
    vertex_array_object: VertexArrayObject,

    mesh: Rc<RefCell<T>>,

    gl: Rc<WebGl2RenderingContext>
}

use cgmath;
use cgmath::SquareMatrix;

impl<T> Renderable<T>
where T: Mesh {
    pub fn new(gl: Rc<WebGl2RenderingContext>, shader: Rc<Shader>, projection: Rc<RefCell<ProjectionType>>, mesh: Rc<RefCell<T>>) -> Renderable<T> {
        shader.bind(&gl);

        let vertex_array_object = T::create_buffers(gl.clone(), &projection.as_ref().borrow());

        let model_mat = cgmath::Matrix4::identity();

        let scale_mat = cgmath::Matrix4::identity();
        let rotation_mat = cgmath::Matrix4::identity();
        let translation_mat = cgmath::Matrix4::identity();

        Renderable {
            // The shader to bind when drawing the renderable
            shader,
            // The model matrix of the Renderable
            model_mat,
            // And its submatrices
            scale_mat,
            rotation_mat,
            translation_mat,
            // Vertex-Array Object index
            vertex_array_object,
            mesh,
            gl,
        }
    }

    fn recompute_model_matrix(&mut self) {
        self.model_mat = self.translation_mat * self.rotation_mat * self.scale_mat;
    }

    pub fn rotate(&mut self, axis: cgmath::Vector3<f32>, angle: cgmath::Rad<f32>) {
        self.rotation_mat = cgmath::Matrix4::<f32>::from_axis_angle(axis, angle);
        self.recompute_model_matrix();
    }

    pub fn apply_rotation(&mut self, axis: cgmath::Vector3<f32>, angle: cgmath::Rad<f32>) {
        self.rotation_mat = cgmath::Matrix4::<f32>::from_axis_angle(axis, angle) * self.rotation_mat;
        self.recompute_model_matrix();
    }

    pub fn scale(&mut self, factor: f32) {
        self.scale_mat = cgmath::Matrix4::<f32>::from_scale(factor);
        self.recompute_model_matrix();
    }

    pub fn get_model_mat(&self) -> cgmath::Matrix4<f32> {
        return self.model_mat.clone(); 
    }

    pub fn draw(&self, mode: u32, viewport: &ViewPort) {
        self.shader.bind(&self.gl);

        self.vertex_array_object.bind();

        // Send Uniforms
        viewport.send_to_vertex_shader(&self.gl, self.shader.borrow());
        self.mesh.as_ref().borrow().send_uniforms(&self.gl, &self.shader);

        // Get the attribute location of the model matrix from the Vertex shader
        let model_mat_location = self.shader.get_uniform_location(&self.gl, "model");
        let model_mat_f32_slice: &[f32; 16] = self.model_mat.as_ref();
        self.gl.uniform_matrix4fv_with_f32_array(model_mat_location.as_ref(), false, model_mat_f32_slice);

        self.gl.draw_elements_with_i32(
            mode,
            self.vertex_array_object.num_vertices() as i32,
            WebGl2RenderingContext::UNSIGNED_INT,
            0,
        );

        self.vertex_array_object.unbind();
    }
}
