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
                BufferData::new(
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
                BufferData::new(indices.as_ref()),
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
                BufferData::new(self.vertices.as_ref()),
            )
            // Store the cartesian position of the center of the source in the a instanced VBO
            .add_instanced_array_buffer(
                3 * std::mem::size_of::<f32>(),
                3,
                WebGl2RenderingContext::STATIC_DRAW,
                BufferData::new(self.center_xyz.as_ref()),
            )
            // Store the spherical position of the center of the source in the a instanced VBO
            .add_instanced_array_buffer(
                2 * std::mem::size_of::<f32>(),
                2,
                WebGl2RenderingContext::STATIC_DRAW,
                BufferData::new(self.center_lonlat.as_ref()),
            )
            // Set the element buffer
            .add_element_buffer(
                WebGl2RenderingContext::STATIC_DRAW,
                BufferData::new(self.indices.as_ref()),
            )
            // Unbind the buffer
            .unbind();

        //gl.bind_buffer(WebGl2RenderingContext::ELEMENT_ARRAY_BUFFER, None);
        vertex_array_object
    }

    fn update<T: Mesh + DisableDrawing>(
        &mut self,
        local_to_world: &Matrix4<f32>,
        world_to_local: &Matrix4<f32>,
        projection: &ProjectionType,
        viewport: &ViewPort
    ) {
        
    }

    fn update_vao(&self, vertex_array_object: &mut VertexArrayObject) {
        // Update the VAO
        vertex_array_object.bind()
            .update_instanced_array(0, BufferData::new(&self.center_xyz))
            .update_instanced_array(1, BufferData::new(&self.center_lonlat));
    }

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
}

use crate::renderable::DisableDrawing;
impl DisableDrawing for Catalog {
    fn disable(&mut self) {
    }
}

const MAX_SOURCES: usize = 100;

struct Node<'a> {
    children: [Option<Box<Node<'a>>>; 4],

    idx: u64,
    depth: u8,

    sources: Vec<&'a Source>,
    num_sources: usize,
}

use healpix;
use cgmath::Rad;
impl<'a> Node<'a> {
    fn new(idx: u64, depth: u8) -> Node<'a> {
        let children = [None, None, None, None];
        let sources = Vec::with_capacity(MAX_SOURCES);
        let num_sources = 0;

        Node {
            children,
            idx,
            depth,
            sources,
            num_sources
        }
    }

    fn add_to_child(&mut self, idx_child: usize, source: &'a Source) {
        if let Some(ref mut child) = &mut self.children[idx_child] {
            child.add(source);
        } else {
            // Create the child node
            let depth = self.depth + 1;
            let idx = (self.idx << 2) + (idx_child as u64);
            let mut child = Box::new(Node::new(idx, depth));

            // Add the source
            child.add(source);

            // Loop over all the sources of the current node
            // and add them if they are located in the child node
            for s in self.sources.iter() {
                let lon: Rad<f32> = s.ra.into();
                let lat: Rad<f32> = s.dec.into();
                let child_idx = healpix::nested::hash(depth, lon.0 as f64, lat.0 as f64);

                if child_idx == idx {
                    child.add(s);
                }
            }

            // Add it to the tree
            self.children[idx_child] = Some(child);
        }
    } 

    pub fn add(&mut self, source: &'a Source) {
        if self.num_sources < MAX_SOURCES || self.depth == 29 {
            self.sources.push(source);
            self.num_sources += 1;
        } else {
            // Determine the children concerned by the source
            let lon: Rad<f32> = source.ra.into();
            let lat: Rad<f32> = source.dec.into();
            let depth = self.depth + 1;
            let idx = healpix::nested::hash(depth, lon.0 as f64, lat.0 as f64);

            let offset = self.idx << 2;
            if idx == offset {
                self.add_to_child(0, source);
            } else if idx == offset + 1 {
                self.add_to_child(1, source);
            } else if idx == offset + 2 {
                self.add_to_child(2, source);
            } else {
                self.add_to_child(3, source);
            }
        }
    }

    fn num_sources(&self) -> usize {
        self.num_sources
    }

    pub fn get_sources(&self, depth: u8, idx: u64) -> Option<&Vec<&'a Source>> {
        if self.depth == depth && self.idx == idx {
            return Some(&self.sources);
        }

        if self.depth < depth {
            for child in self.children.iter() {
                if let Some(child) = child {
                    let sources = child.get_sources(depth, idx);
                    if sources.is_some() {
                        return sources;
                    }
                }
            }
        }

        None
    }
}

struct QuadTree<'a> {
    nodes: [Option<Box<Node<'a>>>; 12],
}

impl<'a> QuadTree<'a> {
    pub fn new(sources: &'a Vec<Source>) -> QuadTree<'a> {
        let mut nodes: [Option<Box<Node<'a>>>; 12] = [
            None, None, None, None,
            None, None, None, None,
            None, None, None, None
        ];

        // Loop over all the sources of the current node
        // and them if they are located in the child node
        for s in sources.iter() {
            let lon: Rad<f32> = s.ra.into();
            let lat: Rad<f32> = s.dec.into();
            let child_idx = healpix::nested::hash(0, lon.0 as f64, lat.0 as f64);

            if let Some(ref mut child) = &mut nodes[child_idx as usize] {
                child.add(s);
            } else {
                let mut node = Box::new(Node::new(child_idx, 0));
                node.add(s);

                nodes[child_idx as usize] = Some(node);
            }
        }

        QuadTree {
            nodes,
        }
    }

    pub fn get(&self, depth: u8, idx: u64) -> Option<&Vec<&'a Source>> {
        for node in self.nodes.iter() {
            if let Some(node) = node {
                let sources = node.get_sources(depth, idx);
                if sources.is_some() {
                    return sources;
                }
            }
        }
        None
    }
}

