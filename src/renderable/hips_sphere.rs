use web_sys::console;

use web_sys::WebGl2RenderingContext;

use crate::renderable::projection::ProjectionType;

use std::rc::Rc;
use std::cell::RefCell;

use crate::renderable::Mesh;
use crate::shader::Shader;

const NUM_VERTICES_PER_STEP: usize = 50;
const NUM_STEPS: usize = 20;
use crate::MAX_DEPTH;
use std::sync::atomic::Ordering;

use crate::texture::BufferTiles;
use crate::texture::load_tiles;

use crate::texture::Texture2D;
use crate::texture::create_texture_2d;

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
use crate::field_of_view::FieldOfView;

impl<'a> HiPSSphere {
    pub fn new(gl: &WebGl2Context, projection: &ProjectionType) -> HiPSSphere {
        let buffer_tiles = Rc::new(RefCell::new(BufferTiles::new(gl, 20, "textures")));
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
}

use crate::renderable::VertexArrayObject;
use crate::renderable::buffers::array_buffer::ArrayBuffer;
use crate::renderable::buffers::buffer_data::BufferData;
use crate::renderable::buffers::element_array_buffer::ElementArrayBuffer;

use cgmath::Matrix4;

impl Mesh for HiPSSphere {
    fn create_buffers(&self, gl: &WebGl2Context) -> VertexArrayObject {
        let mut vertex_array_object = VertexArrayObject::new(gl);
        vertex_array_object.bind();

        // ARRAY buffer creation
        let array_buffer = ArrayBuffer::new(
            gl,
            5 * std::mem::size_of::<f32>(),
            &[2, 3],
            &[0 * std::mem::size_of::<f32>(), 2 * std::mem::size_of::<f32>()],
            BufferData(self.vertices.as_ref()),
            WebGl2RenderingContext::STATIC_DRAW,
        );

        // ELEMENT ARRAY buffer creation
        let indexes_buffer = ElementArrayBuffer::new(
            gl,
            BufferData(self.idx_vertices.as_ref()),
            WebGl2RenderingContext::STATIC_DRAW,
        );

        vertex_array_object.set_array_buffer(array_buffer);
        vertex_array_object.set_element_array_buffer(indexes_buffer);

        vertex_array_object.unbind();
        // Unbind the buffer
        //gl.bind_buffer(WebGl2RenderingContext::ELEMENT_ARRAY_BUFFER, None);
        vertex_array_object
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

    fn get_vertices<'a>(&'a self) -> (BufferData<'a, f32>, BufferData<'a, u16>) {
        unreachable!();
    }

    fn update(&mut self, projection: &ProjectionType, local_to_world_mat: &Matrix4<f32>, viewport: &ViewPort) {
        let field_of_view = viewport.field_of_view();
        let (depth, hpx_idx) = field_of_view.get_healpix_cells(local_to_world_mat);

        let current_depth = DEPTH.load(Ordering::Relaxed);
        let reset_time_received = if current_depth != depth {
            true
        } else {
            false
        };
        
        //console::log_1(&format!("{:?}", self.buffer_tiles.borrow().requested_tiles).into());
        DEPTH.store(depth, Ordering::Relaxed);

        // TODO: wrap that into a method load_healpix_tiles of BufferTiles
        load_tiles(self.buffer_tiles.clone(), &hpx_idx, depth, reset_time_received);
        //console::log_1(&format!("{:?}", self.buffer_tiles.borrow().requested_tiles).into());
    }
}

use crate::renderable::DisableDrawing;
impl DisableDrawing for HiPSSphere {
    fn disable(&mut self) {
    }
}