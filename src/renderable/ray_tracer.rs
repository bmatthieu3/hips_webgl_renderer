use crate::core::VertexArrayObject;
use crate::projection::{Aitoff, Mollweide, AzimutalEquidistant, Mercator, Orthographic};

pub trait RayTracingProjection {
    fn get_raytracer_vertex_array_object(raytracer: &RayTracer) -> &VertexArrayObject;
}

impl RayTracingProjection for Aitoff {
    fn get_raytracer_vertex_array_object(raytracer: &RayTracer) -> &VertexArrayObject {
        &raytracer.vertex_array_objects[ProjectionType::Aitoff]
    }
}
impl RayTracingProjection for Mollweide {
    fn get_raytracer_vertex_array_object(raytracer: &RayTracer) -> &VertexArrayObject {
        &raytracer.vertex_array_objects[ProjectionType::Mollweide]
    }
}
impl RayTracingProjection for AzimutalEquidistant {
    fn get_raytracer_vertex_array_object(raytracer: &RayTracer) -> &VertexArrayObject {
        &raytracer.vertex_array_objects[ProjectionType::Arc]
    }
}
impl RayTracingProjection for Mercator {
    fn get_raytracer_vertex_array_object(raytracer: &RayTracer) -> &VertexArrayObject {
        &raytracer.vertex_array_objects[ProjectionType::Mercator]
    }
}
impl RayTracingProjection for Orthographic {
    fn get_raytracer_vertex_array_object(raytracer: &RayTracer) -> &VertexArrayObject {
        &raytracer.vertex_array_objects[ProjectionType::Orthographic]
    }
}

fn create_vertices_array<P: Projection>(gl: &WebGl2Context, viewport: &ViewPort) -> (Vec<f32>, Vec<u16>) {
    let (vertex_screen_space_positions, indices) = <P>::build_screen_map(viewport);

    let vertices_data = vertex_screen_space_positions
        .into_iter()
        .map(|pos_screen_space| {
            let pos_clip_space = crate::projection::screen_to_clip_space(&pos_screen_space, viewport);
            let pos_model_space = P::clip_to_model_space(&pos_clip_space).unwrap();

            vec![
                pos_clip_space.x,
                pos_clip_space.y,
                
                pos_model_space.x,
                pos_model_space.y,
                pos_model_space.z
            ]
        })
        .flatten()
        .collect::<Vec<_>>();
    console::log_1(&format!("End Generation per pixel mode vertices").into());

    (vertices_data, indices)
}

use web_sys::WebGl2RenderingContext;
use crate::core::BufferData;
use crate::shaders;
fn create_vertex_array_object<P: Projection>(
    gl: &WebGl2Context,
    viewport: &ViewPort,
    shaders: &ShaderManager
) -> VertexArrayObject {
    let (vertices, idx) = create_vertices_array::<P>(gl, viewport);
    
    let mut vertex_array_object = VertexArrayObject::new(gl);

    let shader = shaders.get::<shaders::Raytracing>().unwrap();
    shader.bind(gl)
        // VAO for per-pixel computation mode (only in case of large fovs and 2D projections)
        .bind_vertex_array_object(&mut vertex_array_object)
            // Store the projeted and 3D vertex positions in a VBO
            .add_array_buffer(
                5 * std::mem::size_of::<f32>(),
                &[2, 3],
                &[0 * std::mem::size_of::<f32>(), 2 * std::mem::size_of::<f32>()],
                WebGl2RenderingContext::STATIC_DRAW,
                BufferData::VecData(vertices.as_ref()),
            )
            // Set the element buffer
            .add_element_buffer(
                WebGl2RenderingContext::STATIC_DRAW,
                BufferData::VecData(idx.as_ref()),
            )
            // Unbind the buffer
            .unbind();

    vertex_array_object
}

struct RayTracerVAOs([VertexArrayObject; 5]);

impl RayTracerVAOs {
    fn new(
        gl: &WebGl2Context,
        viewport: &ViewPort,
        shaders: &ShaderManager
    ) -> RayTracerVAOs {
        // Build all the VAOs at the beginning of the program execution
        // The VAO are labeled as STATIC_DRAW, they won't change 
        RayTracerVAOs([
            create_vertex_array_object::<Aitoff>(gl, viewport, shaders),
            create_vertex_array_object::<Mollweide>(gl, viewport, shaders),
            create_vertex_array_object::<AzimutalEquidistant>(gl, viewport, shaders),
            create_vertex_array_object::<Mercator>(gl, viewport, shaders),
            create_vertex_array_object::<Orthographic>(gl, viewport, shaders),
        ])
    }
}

enum ProjectionType {
    Aitoff,
    Mollweide,
    Arc,
    Mercator,
    Orthographic,
}
use std::ops::Index;
impl Index<ProjectionType> for RayTracerVAOs {
    type Output = VertexArrayObject;

    fn index(&self, p: ProjectionType) -> &Self::Output {
        match p {
            ProjectionType::Aitoff => &self.0[0],
            ProjectionType::Mollweide => &self.0[1],
            ProjectionType::Arc => &self.0[2],
            ProjectionType::Mercator => &self.0[3],
            ProjectionType::Orthographic => &self.0[4],
        }
    }
}

pub struct RayTracer {
    vertex_array_objects: RayTracerVAOs,
}

use crate::projection::Projection;
use crate::WebGl2Context;
use crate::viewport::ViewPort;

use web_sys::console;

use crate::renderable::RenderingMode;
use crate::buffer::BufferTextures;
use crate::event_manager::EventManager;

use crate::shader::ShaderManager;
use crate::shader::ShaderBound;
impl RenderingMode for RayTracer {
    fn new(gl: &WebGl2Context, viewport: &ViewPort, shaders: &mut ShaderManager) -> RayTracer {
        let vertex_array_objects = RayTracerVAOs::new(gl, viewport, shaders);

        RayTracer {
            vertex_array_objects
        }
    }

    fn draw<P: Projection>(
        &self,
        gl: &WebGl2Context,
        shader: &ShaderBound,
    ) {
        let vertex_array_object = P::get_raytracer_vertex_array_object(&self);
        shader.bind_vertex_array_object_ref(&vertex_array_object)
            .draw_elements_with_i32(
                WebGl2RenderingContext::TRIANGLES,
                None
            );
    }

    //fn update<P: Projection>(&mut self, buffer: &mut BufferTiles, viewport: &ViewPort, events: &EventManager) {}

    //fn send_to_shader(buffer: &BufferTiles, shader: &Shader) {}
}