use web_sys::console;

use web_sys::WebGl2RenderingContext;

use crate::renderable::projection::ProjectionType;

use std::rc::Rc;
use std::cell::RefCell;

use crate::renderable::Mesh;
use crate::shader::Shader;

pub const NUM_VERTICES_PER_STEP: usize = 50;
pub const NUM_STEPS: usize = 20;
use crate::MAX_DEPTH;
use std::sync::atomic::Ordering;

use crate::texture::BufferTiles;
use crate::texture::load_tiles;

use std::sync::Arc;
use std::sync::atomic::AtomicU8;
lazy_static! {
    pub static ref DEPTH: Arc<AtomicU8> = Arc::new(AtomicU8::new(0));
}

#[derive(Clone)]
pub struct HiPSSphere {
    buffer_tiles: Rc<RefCell<BufferTiles>>,
    buffer_depth_zero_tiles: Rc<RefCell<BufferTiles>>,
    //pub current_depth: u8,

    vertices: Vec<f32>,
    idx_vertices: Vec<u16>,

    size_in_pixels: cgmath::Vector2<f32>,

    // ang2pix textures
    //ang2pix_textures: [Texture2D; 1],

    gl: WebGl2Context,
}

use crate::viewport::ViewPort;

use cgmath::Vector2;

use crate::WebGl2Context;

impl<'a> HiPSSphere {
    pub fn new(gl: &WebGl2Context, projection: &ProjectionType) -> HiPSSphere {
        let buffer_tiles = Rc::new(RefCell::new(BufferTiles::new(gl, 64, "textures")));
        let base_tiles = (0..12).collect::<Vec<u64>>();
        load_tiles(buffer_tiles.clone(), &base_tiles, 0, false);

        let buffer_depth_zero_tiles = Rc::new(RefCell::new(BufferTiles::new(gl, 12, "textures_0")));
        load_tiles(buffer_depth_zero_tiles.clone(), &base_tiles, 0, false);

        let (vertices, size_in_pixels) = HiPSSphere::create_vertices_array(gl, projection);
        let idx_vertices = HiPSSphere::create_index_array();

        // Load the ang2pix values
        /*let ang2pix_textures = [
            //create_texture_2d(gl, "./textures/ang2pix_depth0.png"),
            //create_texture_2d(gl, "./textures/ang2pix_depth1.jpg"),
            //create_texture_2d(gl, "./textures/ang2pix_depth2.jpg"),
        ];*/

        let gl = gl.clone();
        HiPSSphere {
            buffer_tiles,
            buffer_depth_zero_tiles,

            vertices,
            idx_vertices,

            size_in_pixels,
            //ang2pix_textures,

            gl,
        }
    }

    /// Called when the HiPS has been changed
    pub fn refresh_buffer_tiles(&mut self) {
        console::log_1(&format!("refresh buffers").into());
        let base_tiles = (0..12).collect::<Vec<u64>>();

        self.buffer_tiles.replace(BufferTiles::new(&self.gl, 64, "textures"));
        load_tiles(self.buffer_tiles.clone(), &base_tiles, 0, false);

        self.buffer_depth_zero_tiles.replace(BufferTiles::new(&self.gl, 12, "textures_0"));
        load_tiles(self.buffer_depth_zero_tiles.clone(), &base_tiles, 0, false);
    }

    pub fn get_default_pixel_size(&self) -> &Vector2<f32> {
        &self.size_in_pixels
    }

    fn create_vertices_array(gl: &WebGl2Context, projection: &ProjectionType) -> (Vec<f32>, Vector2<f32>) {
        let (vertex_screen_space_positions, size_px) = projection.build_screen_map();

        let vertices_data = vertex_screen_space_positions
            .into_iter()
            .map(|pos_screen_space| {
                // Perform the inverse projection that converts
                // screen position to the 3D space position
                let pos_world_space = projection.screen_to_world_space(&pos_screen_space).unwrap();

                vec![pos_screen_space.x, pos_screen_space.y, pos_world_space.x, pos_world_space.y, pos_world_space.z]
            })
            .flatten()
            .collect::<Vec<_>>();

        (vertices_data, size_px)
    }

    fn create_index_array() -> Vec<u16> {
        let mut indices = Vec::with_capacity(3 * NUM_VERTICES_PER_STEP * NUM_STEPS);

        for j in 0..NUM_STEPS {
            if j == 0 {
                for i in 1..NUM_VERTICES_PER_STEP {
                    indices.push(0 as u16);
                    indices.push(i as u16);
                    indices.push((i + 1) as u16);
                }
                
                indices.push(0 as u16);
                indices.push(NUM_VERTICES_PER_STEP as u16);
                indices.push(1 as u16);
            } else {
                for i in 0..NUM_VERTICES_PER_STEP {
                    let start_p_idx = (j - 1) * NUM_VERTICES_PER_STEP + i + 1;
                    let next_p_idx = if i + 1 == NUM_VERTICES_PER_STEP {
                        (j - 1) * NUM_VERTICES_PER_STEP + 1
                    } else {
                        (j - 1) * NUM_VERTICES_PER_STEP + i + 2
                    };

                    let start_c_idx = j * NUM_VERTICES_PER_STEP + i + 1;
                    let next_c_idx = if i + 1 == NUM_VERTICES_PER_STEP {
                        j * NUM_VERTICES_PER_STEP + 1
                    } else {
                        j * NUM_VERTICES_PER_STEP + i + 2
                    };

                    // Triangle touching the prec circle
                    indices.push(start_p_idx as u16);
                    indices.push(next_p_idx as u16);
                    indices.push(start_c_idx as u16);
                    // Triangle touching the next circle
                    indices.push(start_c_idx as u16);
                    indices.push(next_p_idx as u16);
                    indices.push(next_c_idx as u16);
                }
            }
        }

        indices
    }

    fn send_uniforms(&self, gl: &WebGl2Context, shader: &Shader) {
        // TEXTURES DEPTH 0 TILES BUFFER
        self.buffer_depth_zero_tiles.borrow().send_to_shader(shader);
        // TEXTURES TILES BUFFER
        self.buffer_tiles.borrow().send_to_shader(shader);

        // ANG2PIX TEXTURES
        //self.ang2pix_textures[0].send_to_shader(gl, shader, "ang2pix_0_texture");

        // Send current depth
        let location_current_depth = shader.get_uniform_location("current_depth");
        gl.uniform1i(location_current_depth, DEPTH.load(Ordering::Relaxed) as i32);
        // Send max depth of the current HiPS
        let location_max_depth = shader.get_uniform_location("max_depth");
        gl.uniform1i(location_max_depth, MAX_DEPTH.load(Ordering::Relaxed) as i32);
    }
}

use crate::renderable::VertexArrayObject;
use crate::renderable::buffers::buffer_data::BufferData;

use std::collections::HashMap;
use crate::renderable::Renderable;
use cgmath::Matrix4;

use crate::utils;
impl Mesh for HiPSSphere {
    fn create_buffers(&mut self, gl: &WebGl2Context) -> VertexArrayObject {
        let mut vertex_array_object = VertexArrayObject::new(gl);

        vertex_array_object.bind()
            // Store the projeted and 3D vertex positions in a VBO
            .add_array_buffer(
                5 * std::mem::size_of::<f32>(),
                &[2, 3],
                &[0 * std::mem::size_of::<f32>(), 2 * std::mem::size_of::<f32>()],
                WebGl2RenderingContext::STATIC_DRAW,
                BufferData::new(self.vertices.as_ref()),
            )
            // Set the element buffer
            .add_element_buffer(
                WebGl2RenderingContext::STATIC_DRAW,
                BufferData::new(self.idx_vertices.as_ref()),
            )
            // Unbind the buffer
            .unbind();

        // Unbind the buffer
        //gl.bind_buffer(WebGl2RenderingContext::ELEMENT_ARRAY_BUFFER, None);
        vertex_array_object
    }

    fn update<T: Mesh + DisableDrawing>(
        &mut self,
        _vertex_array_object: &mut VertexArrayObject,
        _local_to_world: &Matrix4<f32>,
        _projection: &ProjectionType,
        viewport: &ViewPort
    ) {
        let field_of_view = viewport.field_of_view();
        let (depth, hpx_idx) = field_of_view.cells();

        let current_depth = DEPTH.load(Ordering::Relaxed);
        let reset_time_received = if current_depth != *depth {
            true
        } else {
            false
        };

        DEPTH.store(*depth, Ordering::Relaxed);

        // TODO: wrap that into a method load_healpix_tiles of BufferTiles
        load_tiles(self.buffer_tiles.clone(), &hpx_idx, *depth, reset_time_received);
    }

    fn draw<T: Mesh + DisableDrawing>(
        &self,
        gl: &WebGl2Context,
        renderable: &Renderable<T>,
        shaders: &HashMap<&'static str, Shader>,
        viewport: &ViewPort
    ) {
        let shader = &shaders["hips_sphere"];
        shader.bind(gl);

        renderable.vertex_array_object.bind_ref();

        // Send Uniforms
        viewport.send_to_vertex_shader(gl, shader);
        self.send_uniforms(gl, shader);

        // Send model matrix
        let model_mat_location = shader.get_uniform_location("model");
        let model_mat_f32_slice: &[f32; 16] = renderable.model_mat.as_ref();
        gl.uniform_matrix4fv_with_f32_array(model_mat_location, false, model_mat_f32_slice);

        // Send current time
        let location_time = shader.get_uniform_location("current_time");
        gl.uniform1f(location_time, utils::get_current_time());

        gl.draw_elements_with_i32(
            WebGl2RenderingContext::TRIANGLES,
            renderable.vertex_array_object.num_elements() as i32,
            WebGl2RenderingContext::UNSIGNED_SHORT,
            0,
        );
    }
}

use crate::renderable::DisableDrawing;
impl DisableDrawing for HiPSSphere {
    fn disable(&mut self) {
    }
}