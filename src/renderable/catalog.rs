use crate::texture::Texture2D;

use web_sys::WebGlFramebuffer;
pub struct Catalog {
    center_lonlat: Vec<f32>, // Spherical position of the observations
    center_xyz: Vec<f32>, // Cartesian position of the observations
    num_instances: usize,

    vertices: Vec<f32>, // Offsets and UVs
    indices: Vec<u16>,
    size: f32,

    alpha: f32,
    strength: f32,

    kernel_texture: Texture2D,

    fbo: Option<WebGlFramebuffer>,
    fbo_texture: Texture2D,
    colormap_texture: Texture2D,

    fbo_texture_width: i32,
    fbo_texture_height: i32,

    // VAO for the screen
    vao_screen: VertexArrayObject,  
}

use cgmath::Deg;
#[derive(Debug)]
pub struct Source {
    ra: Deg<f32>,
    dec: Deg<f32>,
}

impl Source {
    pub fn new(ra: Deg<f32>, dec: Deg<f32>) -> Source {
        Source {
            ra,
            dec,
        }
    }
}

impl<'a> From<&[f32]> for Source {
    fn from(data: &[f32]) -> Self {
        Source::new(Deg(data[0]), Deg(data[1]))
    }
}

use crate::math;
impl Catalog {
    pub fn new(gl: &WebGl2Context, sources: Vec<Source>) -> Catalog {
        let mut center_lonlat = Vec::with_capacity(sources.len());
        let mut center_xyz = Vec::with_capacity(sources.len());

        let num_instances = sources.len();
        for source in sources.into_iter() {
            let vertex = math::radec_to_xyz(source.ra.into(), source.dec.into());

            center_xyz.push(vertex.x);
            center_xyz.push(vertex.y);
            center_xyz.push(vertex.z);

            center_lonlat.push(source.ra.0);
            center_lonlat.push(source.dec.0);
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
        let kernel_texture = Texture2D::create(gl, "./textures/kernel.png");
        let colormap_texture = Texture2D::create(gl, "./textures/magma_colormap.png");

        // Initialize texture for framebuffer
        let fbo_texture_width = 768;
        let fbo_texture_height = 768;
        let fbo_texture = Texture2D::create_empty(gl, fbo_texture_width, fbo_texture_height);
        // Create and bind the framebuffer
        let fbo = gl.create_framebuffer();
        gl.bind_framebuffer(WebGl2RenderingContext::FRAMEBUFFER, fbo.as_ref());
        // attach the texture as the first color attachment
        fbo_texture.attach_to_framebuffer(&gl);
        // Unbind the framebuffer
        gl.bind_framebuffer(WebGl2RenderingContext::FRAMEBUFFER, None);

        // Create the VAO for the screen
        let mut vao_screen = VertexArrayObject::new(gl);
        vao_screen.bind()
            // Store the screen and uv of the billboard in a VBO
            .add_array_buffer(
                4 * std::mem::size_of::<f32>(),
                &[2, 2],
                &[0 * std::mem::size_of::<f32>(), 2 * std::mem::size_of::<f32>()],
                WebGl2RenderingContext::STATIC_DRAW,
                BufferData(
                    &vec![
                        -1.0, -1.0, 0.0, 0.0,
                        1.0, -1.0, 1.0, 0.0,
                        1.0, 1.0, 1.0, 1.0,
                        -1.0, 1.0, 0.0, 1.0,
                    ]
                ),
            )
            // Set the element buffer
            .add_element_buffer(
                WebGl2RenderingContext::STATIC_DRAW,
                BufferData(indices.as_ref()),
            )
            // Unbind the buffer
            .unbind();

        let alpha = 0.2_f32;
        let strength = 1_f32;
        Catalog {
            center_lonlat,
            center_xyz,

            num_instances, 

            vertices,
            indices,

            size,
            alpha,
            strength,

            kernel_texture,

            fbo,
            fbo_texture,
            colormap_texture,

            fbo_texture_width,
            fbo_texture_height,

            vao_screen
        }
    }

    pub fn set_alpha(&mut self, alpha: f32) {
        self.alpha = alpha;
    }

    pub fn set_kernel_strength(&mut self, strength: f32) {
        self.strength = strength;
    }
}

use crate::renderable::Mesh;
use crate::shader::Shader;

use crate::renderable::VertexArrayObject;
use crate::renderable::buffers::buffer_data::BufferData;

use cgmath::Matrix4;

use web_sys::WebGl2RenderingContext;
use crate::WebGl2Context;

use crate::projection::ProjectionType;
use crate::viewport::ViewPort;

use crate::renderable::Renderable;
use crate::utils;
use std::collections::HashMap;

use crate::window_size_u32;

impl Mesh for Catalog {
    fn create_buffers(&self, gl: &WebGl2Context) -> VertexArrayObject {
        let mut vertex_array_object = VertexArrayObject::new(gl);
        vertex_array_object.bind()
            // Store the UV and the offsets of the billboard in a VBO
            .add_array_buffer(
                4 * std::mem::size_of::<f32>(),
                &[2, 2],
                &[0 * std::mem::size_of::<f32>(), 2 * std::mem::size_of::<f32>()],
                WebGl2RenderingContext::STATIC_DRAW,
                BufferData(self.vertices.as_ref()),
            )
            // Store the cartesian position of the center of the source in the a instanced VBO
            .add_instanced_array_buffer(
                3 * std::mem::size_of::<f32>(),
                3,
                WebGl2RenderingContext::STATIC_DRAW,
                BufferData(self.center_xyz.as_ref()),
            )
            // Store the spherical position of the center of the source in the a instanced VBO
            .add_instanced_array_buffer(
                2 * std::mem::size_of::<f32>(),
                2,
                WebGl2RenderingContext::STATIC_DRAW,
                BufferData(self.center_lonlat.as_ref()),
            )
            // Set the element buffer
            .add_element_buffer(
                WebGl2RenderingContext::STATIC_DRAW,
                BufferData(self.indices.as_ref()),
            )
            // Unbind the buffer
            .unbind();

        //gl.bind_buffer(WebGl2RenderingContext::ELEMENT_ARRAY_BUFFER, None);
        vertex_array_object
    }

    fn send_uniforms(&self, gl: &WebGl2Context, shader: &Shader) {
        unreachable!();
    }

    fn get_vertices<'a>(&'a self) -> (BufferData<'a, f32>, BufferData<'a, u16>) {
        unreachable!();
    }

    fn update(&mut self, projection: &ProjectionType, local_to_world_mat: &Matrix4<f32>, viewport: &ViewPort) {}

    fn draw<T: Mesh + DisableDrawing>(
        &self,
        gl: &WebGl2Context,
        renderable: &Renderable<T>,
        shaders: &HashMap<&'static str, Shader>,
        viewport: &ViewPort
    ) {
        // Render to the FRAMEBUFFER
        {
            // bind the FBO
            gl.bind_framebuffer(WebGl2RenderingContext::FRAMEBUFFER, self.fbo.as_ref());

            // Set the viewport
            gl.viewport(0, 0, self.fbo_texture_width, self.fbo_texture_height);
            gl.scissor(0, 0, self.fbo_texture_width, self.fbo_texture_height);

            gl.clear_color(0.0, 0.0, 0.0, 1.0);
            gl.clear(WebGl2RenderingContext::COLOR_BUFFER_BIT);

            let shader = &shaders["catalog"];
            shader.bind(gl);

            renderable.vertex_array_object.bind_ref();

            // Send uniforms
            // Send the viewport uniforms
            viewport.send_to_vertex_shader(gl, shader);
            // Send the gaussian kernel texture
            self.kernel_texture.send_to_shader(&gl, shader, "kernel_texture");
            // Send the max strength of one kernel
            let location_strength = shader.get_uniform_location("strength");
            gl.uniform1f(location_strength, self.strength);

            // Send model matrix
            let model_mat_location = shader.get_uniform_location("model");
            let model_mat_f32_slice: &[f32; 16] = renderable.model_mat.as_ref();
            gl.uniform_matrix4fv_with_f32_array(model_mat_location, false, model_mat_f32_slice);

            // Send current time
            let location_time = shader.get_uniform_location("current_time");
            gl.uniform1f(location_time, utils::get_current_time());

            gl.draw_elements_instanced_with_i32(
                WebGl2RenderingContext::TRIANGLES,
                renderable.vertex_array_object.num_elements() as i32,
                WebGl2RenderingContext::UNSIGNED_SHORT,
                0,
                renderable.vertex_array_object.num_instances() as i32,
            );

            // Unbind the FBO
            gl.bind_framebuffer(WebGl2RenderingContext::FRAMEBUFFER, None);
        }

        // Render to the heatmap to the screen
        {
            // Set the viewport
            let (width_screen, height_screen) = window_size_u32();
            gl.viewport(0, 0, width_screen as i32, height_screen as i32);
            viewport.update_scissor();

            let shader = &shaders["heatmap"];
            shader.bind(gl);

            self.vao_screen.bind_ref();

            self.colormap_texture.send_to_shader(gl, shader, "colormap");
            self.fbo_texture.send_to_shader(gl, shader, "texture_fbo");

            // Send alpha
            let location_alpha = shader.get_uniform_location("alpha");
            gl.uniform1f(location_alpha, self.alpha);

            gl.draw_elements_with_i32(
                WebGl2RenderingContext::TRIANGLES,
                self.vao_screen.num_elements() as i32,
                WebGl2RenderingContext::UNSIGNED_SHORT,
                0,
            );
        }
    }
    /*
    fn draw(&self, gl: &WebGl2Context, vao: &VertexArrayObject) {

        //let (width_screen, height_screen) = window_size_u32();
        // Set the viewport
        //gl.viewport(0, 0, width_screen as i32, height_screen as i32);

        {
            // Render to the canvas
            gl.bind_framebuffer(WebGl2RenderingContext::FRAMEBUFFER, None);
        
            // Set the viewport
            gl.viewport(0, 0, self.fbo_texture_width, self.fbo_texture_height);

            // Bind shader



            // Send the texture we just rendered to
            self.fbo_texture.send_to_shader(&gl, shader, "texture");
        
            // Tell WebGL how to convert from clip space to pixels
            gl.viewport(0, 0, gl.canvas.width, gl.canvas.height);
        
            // Clear the canvas AND the depth buffer.
            gl.clearColor(1, 1, 1, 1);   // clear to white
            gl.clear(gl.COLOR_BUFFER_BIT | gl.DEPTH_BUFFER_BIT);
        
            const aspect = gl.canvas.clientWidth / gl.canvas.clientHeight;
            drawCube(aspect)
        }
    }*/
}

use crate::renderable::DisableDrawing;
impl DisableDrawing for Catalog {
    fn disable(&mut self) {
    }
}

const MAX_SOURCES: usize = 100;

struct Node<'a, 'b: 'a> {
    children: Option<[&'a Node<'a, 'b>; 4]>,

    idx: u64,
    depth: u8,

    sources: Vec<&'b Source>,
    num_sources: usize,
}

use healpix;
use cgmath::Rad;
impl<'a, 'b> Node<'a, 'b> {
    fn add(&mut self, source: &'b Source) {
        // Check whether the source belongs to this cell
        // if not, we will exit the method here
        let lon: Rad<f32> = source.ra.into();
        let lat: Rad<f32> = source.dec.into();

        let idx = healpix::nested::hash(self.depth, lon.0 as f64, lat.0 as f64);
        if idx == self.idx {
            if self.num_sources < MAX_SOURCES || self.depth == 29 {
                self.sources.push(source);
                self.num_sources += 1;
            } else {
                // Otherwise, we add it to the children
                

            }
        }
    }

    fn num_sources(&self) -> usize {
        self.num_sources
    }
}