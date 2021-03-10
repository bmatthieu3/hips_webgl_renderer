use crate::{
    camera::CameraViewPort,
    core::{VertexArrayObject, Texture2D},
    renderable::projection::Projection,
    shader::{ShaderBound, ShaderManager},
    image_fmt::FormatImageType,
    WebGl2Context,
};

pub trait RayTracingProjection {
    fn get_raytracer_vertex_array_object(raytracer: &RayTracer) -> &VertexArrayObject;
}

use crate::coo_conversion::CooSystem;
use crate::math::LonLat;
use crate::renderable::Triangulation;
fn create_vertices_array<P: Projection>(
    _gl: &WebGl2Context,
    camera: &CameraViewPort,
    _system: &CooSystem,
) -> (Vec<f32>, Vec<u16>) {
    let (vertices, idx) = Triangulation::new::<P>().into();

    let vertices = vertices
        .into_iter()
        .map(|pos_clip_space| {
            /*let pos_world_space = system.system_to_icrs_coo(
                P::clip_to_world_space(&pos_clip_space, camera.is_reversed_longitude()).unwrap()
            );*/
            let pos_world_space =
                P::clip_to_world_space(&pos_clip_space, camera.is_reversed_longitude()).unwrap();
            //let pos_world_space = system.to_icrs_j2000() * pos_world_space;
            let lonlat = pos_world_space.lonlat();

            // Cast all the double into float
            // simple precision because this buffer
            // is sent to the GPU
            vec![
                pos_clip_space.x as f32,
                pos_clip_space.y as f32,
                lonlat.lon().0 as f32,
                lonlat.lat().0 as f32,
                pos_world_space.x as f32,
                pos_world_space.y as f32,
                pos_world_space.z as f32,
            ]
        })
        .flatten()
        .collect::<Vec<_>>();

    (vertices, idx)
}

use web_sys::WebGl2RenderingContext;
use web_sys::WebGlBuffer;
use web_sys::WebGlVertexArrayObject;

pub struct RayTracer {
    gl: WebGl2Context,

    vao: WebGlVertexArrayObject,

    vbo: WebGlBuffer,
    ebo: WebGlBuffer,

    num_indices: i32,
    position_tex: Texture2D,
}

use cgmath::{Vector2, InnerSpace};
use wasm_bindgen_futures::JsFuture;
use wasm_bindgen::JsCast;
use crate::Resources;
use web_sys::{Request, RequestInit, RequestMode, Response};
use wasm_bindgen::JsValue;
use std::mem;
use crate::renderable::projection::Aitoff;
fn generate_position<P: Projection>() -> Vec<f32> {
    let (w, h) = (2048.0, 2048.0);
    let mut data = vec![];
    for y in 0..(h as u32) {
        for x in 0..(w as u32) {
            let xy = Vector2::new(x, y);
            let clip_xy = Vector2::new(2.0 * ((xy.x as f64) / (w as f64)) - 1.0, 2.0 * ((xy.y as f64) / (h as f64)) - 1.0);
            if let Some(pos) = P::clip_to_world_space(
                &clip_xy,
                true,
            ) {
                let pos = pos.truncate().normalize();

                data.push(pos.x as f32);
                data.push(pos.y as f32);
                data.push(pos.z as f32);
            } else {
                data.push(1.0);
                data.push(1.0);
                data.push(1.0);
            }
        }
    }

    data
}

impl RayTracer {
    pub fn new<P: Projection>(
        gl: &WebGl2Context,
        camera: &CameraViewPort,
        _shaders: &mut ShaderManager,
        resources: &Resources,
        system: &CooSystem,
    ) -> RayTracer {
        let (vertices, idx) = create_vertices_array::<P>(gl, camera, system);

        let vao = gl.create_vertex_array().unwrap();
        gl.bind_vertex_array(Some(&vao));

        let vbo = gl.create_buffer().ok_or("failed to create buffer").unwrap();
        gl.bind_buffer(WebGl2RenderingContext::ARRAY_BUFFER, Some(&vbo));
        let buf_vertices = unsafe { js_sys::Float32Array::view(&vertices) };
        gl.buffer_data_with_array_buffer_view(
            WebGl2RenderingContext::ARRAY_BUFFER,
            &buf_vertices,
            WebGl2RenderingContext::STATIC_DRAW,
        );

        // layout (location = 0) in vec2 pos_clip_space;
        gl.vertex_attrib_pointer_with_i32(
            0,
            2,
            WebGl2RenderingContext::FLOAT,
            false,
            (7 * mem::size_of::<f32>()) as i32,
            (0 * mem::size_of::<f32>()) as i32,
        );
        gl.enable_vertex_attrib_array(0);

        // layout (location = 1) in vec2 lonlat;
        gl.vertex_attrib_pointer_with_i32(
            1,
            2,
            WebGl2RenderingContext::FLOAT,
            false,
            (7 * mem::size_of::<f32>()) as i32,
            (2 * mem::size_of::<f32>()) as i32,
        );
        gl.enable_vertex_attrib_array(1);

        // layout (location = 2) in vec3 pos_world_space;
        gl.vertex_attrib_pointer_with_i32(
            2,
            3,
            WebGl2RenderingContext::FLOAT,
            false,
            (7 * mem::size_of::<f32>()) as i32,
            (4 * mem::size_of::<f32>()) as i32,
        );
        gl.enable_vertex_attrib_array(2);

        let ebo = gl.create_buffer().ok_or("failed to create buffer").unwrap();
        // Bind the buffer
        gl.bind_buffer(WebGl2RenderingContext::ELEMENT_ARRAY_BUFFER, Some(&ebo));
        let num_indices = idx.len() as i32;

        let buf_indices = unsafe { js_sys::Uint16Array::view(&idx) };
        gl.buffer_data_with_array_buffer_view(
            WebGl2RenderingContext::ELEMENT_ARRAY_BUFFER,
            &buf_indices,
            WebGl2RenderingContext::STATIC_DRAW,
        );
        // create data
        let data = generate_position::<P>();

        let position_tex = Texture2D::create_empty(
            gl,
            2048,
            2048,
            &[
                (WebGl2RenderingContext::TEXTURE_MIN_FILTER, WebGl2RenderingContext::NEAREST),
                (WebGl2RenderingContext::TEXTURE_MAG_FILTER, WebGl2RenderingContext::NEAREST),

                // Prevents s-coordinate wrapping (repeating)
                (WebGl2RenderingContext::TEXTURE_WRAP_S, WebGl2RenderingContext::REPEAT),
                // Prevents t-coordinate wrapping (repeating)
                (WebGl2RenderingContext::TEXTURE_WRAP_T, WebGl2RenderingContext::REPEAT),
            ],
            FormatImageType::RGB32F
        ).unwrap();

        let buf_data = unsafe { js_sys::Float32Array::view(&data) };
        position_tex.bind()
            .tex_sub_image_2d_with_i32_and_i32_and_u32_and_type_and_opt_array_buffer_view(0, 0, 2048, 2048, Some(&buf_data));

        let gl = gl.clone();
        RayTracer {
            gl,

            vao,

            vbo,
            ebo,

            num_indices,

            position_tex
        }
    }

    pub fn bind(&self) {
        self.gl.bind_vertex_array(Some(&self.vao));
    }

    pub fn draw<'a>(&self, shader: &ShaderBound<'a>) {
        shader.attach_uniform("position_tex", &self.position_tex);
        self.gl.draw_elements_with_i32(
            //WebGl2RenderingContext::LINES,
            WebGl2RenderingContext::TRIANGLES,
            self.num_indices,
            WebGl2RenderingContext::UNSIGNED_SHORT,
            0,
        );
    }
}

impl Drop for RayTracer {
    fn drop(&mut self) {
        self.gl.delete_vertex_array(Some(&self.vao));
        self.gl.delete_buffer(Some(&self.vbo));
        self.gl.delete_buffer(Some(&self.ebo));
    }
}
