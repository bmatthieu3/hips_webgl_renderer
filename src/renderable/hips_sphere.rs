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
use crate::texture::{load_tiles, load_base_tiles};

use std::sync::Arc;
use std::sync::atomic::AtomicU8;
lazy_static! {
    pub static ref DEPTH: Arc<AtomicU8> = Arc::new(AtomicU8::new(0));
}

use crate::renderable::buffers::vertex_array_object::VertexArrayObject;
use crate::viewport::ViewPort;
use cgmath::Vector2;
use crate::WebGl2Context;

trait RenderingMode {
    fn create_vertices_array(gl: &WebGl2Context, projection: &ProjectionType, buffer: Rc<RefCell<BufferTiles>>) -> Vec<f32>;
    fn create_index_array() -> Option<Vec<u16>>;
    fn send_uniforms(gl: &WebGl2Context, shader: &Shader);
    fn draw<T: Mesh + DisableDrawing>(
        &self,
        gl: &WebGl2Context,
        renderable: &Renderable<T>,
        shaders: &HashMap<&'static str, Shader>,
        viewport: &ViewPort
    );
}

struct SmallFieldOfViewRenderingMode {
    pub vertices: Vec<f32>,
    vertex_array_object: VertexArrayObject,
}

use crate::renderable::buffers::buffer_data::BufferData;
use cgmath::Rad;
use crate::math;
use std::mem;
impl SmallFieldOfViewRenderingMode {
    fn new(gl: &WebGl2Context, projection: &ProjectionType, buffer: Rc<RefCell<BufferTiles>>) -> SmallFieldOfViewRenderingMode {
        let vertices = Self::create_vertices_array(gl, projection, buffer);

        console::log_1(&format!("VERTICES LENGTH: {:?}", vertices.len()).into());
        //console::log_1(&format!("IDX TEXTURES LENGTH: {:?}", idx_textures.len()).into());

        let mut vertex_array_object = VertexArrayObject::new(gl);

        // VAO for the orthographic projection and small fovs on 2D projections
        vertex_array_object.bind()
            // Store the projeted and 3D vertex positions in a VBO
            .add_array_buffer(
                10 * mem::size_of::<f32>(),
                &[2, 3, 2, 2, 1],
                &[
                    0 * mem::size_of::<f32>(),
                    2 * mem::size_of::<f32>(),
                    5 * mem::size_of::<f32>(),
                    7 * mem::size_of::<f32>(),
                    9 * mem::size_of::<f32>(),
                ],
                WebGl2RenderingContext::DYNAMIC_DRAW,
                BufferData::VecData(vertices.as_ref()),
            )
            // Unbind the buffer
            .unbind();

        SmallFieldOfViewRenderingMode {
            vertices,

            vertex_array_object,
        }
    }

    fn add_vertex(vertex_array: &mut Vec<f32>, lonlat: &[(f64, f64)], idx: usize, uv_start: Vector2<f32>, uv_end: Vector2<f32>, blending_factor: f32) {
        let vertex = lonlat[idx];
        let (theta, delta) = (Rad(vertex.0 as f32), Rad(vertex.1 as f32));

        let pos_world_space = math::radec_to_xyz(theta, delta);
        vertex_array.extend(vec![
            theta.0,
            delta.0,

            pos_world_space.x,
            pos_world_space.y,
            pos_world_space.z,

            uv_start.x,
            uv_start.y,

            uv_end.x,
            uv_end.y,

            blending_factor
        ]);
    }
}

impl RenderingMode for SmallFieldOfViewRenderingMode {
    fn create_vertices_array(gl: &WebGl2Context, projection: &ProjectionType, buffer: Rc<RefCell<BufferTiles>>) -> Vec<f32> {
        let depth = 2;
        let vertices_data = (0..192)
            .map(|idx| {
                let lonlat = healpix::nested::grid(depth, idx, 1);

                let blending_factor = 1_f32;
                let uv_start = [Vector2::new(0_f32, 0_f32); 4];

                let tile = Tile::new(HEALPixCell(2, idx));
                //let parent_tile_buffer = get_parent(&tile, 0, &buffer.borrow()).unwrap();
                //let uv_end = get_uv_in_parent(&tile, &parent_tile_buffer);
                let uv_end = uv_start.clone();

                let mut vertex_array = Vec::with_capacity(10 * 6);
                SmallFieldOfViewRenderingMode::add_vertex(&mut vertex_array, &lonlat, 0, uv_start[0], uv_end[0], blending_factor);
                SmallFieldOfViewRenderingMode::add_vertex(&mut vertex_array, &lonlat, 2, uv_start[2], uv_end[2], blending_factor);
                SmallFieldOfViewRenderingMode::add_vertex(&mut vertex_array, &lonlat, 1, uv_start[1], uv_end[1], blending_factor);

                SmallFieldOfViewRenderingMode::add_vertex(&mut vertex_array, &lonlat, 1, uv_start[1], uv_end[1], blending_factor);
                SmallFieldOfViewRenderingMode::add_vertex(&mut vertex_array, &lonlat, 2, uv_start[2], uv_end[2], blending_factor);
                SmallFieldOfViewRenderingMode::add_vertex(&mut vertex_array, &lonlat, 3, uv_start[3], uv_end[3], blending_factor);

                vertex_array
            })
            .flatten()
            .collect::<Vec<_>>();

        vertices_data
    }

    fn create_index_array() -> Option<Vec<u16>> {
        None
    }

    fn send_uniforms(gl: &WebGl2Context, shader: &Shader) {
    }

    fn draw<T: Mesh + DisableDrawing>(
        &self,
        gl: &WebGl2Context,
        renderable: &Renderable<T>,
        shaders: &HashMap<&'static str, Shader>,
        viewport: &ViewPort
    ) {
        self.vertex_array_object.bind_ref();
        gl.draw_arrays(
            WebGl2RenderingContext::TRIANGLES,
            0,
            (self.vertices.len() as i32) / 10,
        );
    }
}

struct PerPixelRenderingMode {
    pub vertices: Vec<f32>,
    pub idx: Vec<u16>,

    vertex_array_object: VertexArrayObject,
}

impl PerPixelRenderingMode {
    fn new(gl: &WebGl2Context, projection: &ProjectionType, buffer: Rc<RefCell<BufferTiles>>) -> PerPixelRenderingMode {
        let vertices = Self::create_vertices_array(gl, projection, buffer);
        let idx = Self::create_index_array().unwrap();

        let mut vertex_array_object = VertexArrayObject::new(gl);
        // VAO for per-pixel computation mode (only in case of large fovs and 2D projections)
        vertex_array_object.bind()
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

        PerPixelRenderingMode {
            vertices,
            idx,

            vertex_array_object
        }
    }
}

impl RenderingMode for PerPixelRenderingMode {
    fn create_vertices_array(gl: &WebGl2Context, projection: &ProjectionType, _buffer: Rc<RefCell<BufferTiles>>) -> Vec<f32> {
        let vertex_screen_space_positions = projection.build_screen_map();

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

        vertices_data
    }

    fn create_index_array() -> Option<Vec<u16>> {
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

        Some(indices)
    }

    fn send_uniforms(gl: &WebGl2Context, shader: &Shader) {
        // Send current depth
        let location_current_depth = shader.get_uniform_location("current_depth");
        gl.uniform1i(location_current_depth, DEPTH.load(Ordering::Relaxed) as i32);
        // Send max depth of the current HiPS
        let location_max_depth = shader.get_uniform_location("max_depth");
        gl.uniform1i(location_max_depth, MAX_DEPTH.load(Ordering::Relaxed) as i32);
    }

    fn draw<T: Mesh + DisableDrawing>(
        &self,
        gl: &WebGl2Context,
        renderable: &Renderable<T>,
        shaders: &HashMap<&'static str, Shader>,
        viewport: &ViewPort
    ) {
        self.vertex_array_object.bind_ref();
        gl.draw_elements_with_i32(
            WebGl2RenderingContext::TRIANGLES,
            self.vertex_array_object.num_elements() as i32,
            WebGl2RenderingContext::UNSIGNED_SHORT,
            0,
        );
    }
}

pub struct HiPSSphere {
    buffer: Rc<RefCell<BufferTiles>>,

    fov_rendering_mode: SmallFieldOfViewRenderingMode,
    per_pixel_rendering_mode: PerPixelRenderingMode,

    fov_mode: bool,

    gl: WebGl2Context,
}

impl<'a> HiPSSphere {
    pub fn new(gl: &WebGl2Context, projection: &ProjectionType) -> HiPSSphere {
        let buffer = Rc::new(RefCell::new(BufferTiles::new(gl)));
        load_base_tiles(gl, buffer.clone());

        let fov_rendering_mode = SmallFieldOfViewRenderingMode::new(gl, projection, buffer.clone());
        let per_pixel_rendering_mode = PerPixelRenderingMode::new(gl, projection, buffer.clone());

        let gl = gl.clone();
        //let fov_mode = false;
        let fov_mode = true;

        HiPSSphere {
            buffer: buffer,

            // Two modes:
            // - One for large FOVs on 2D projections
            per_pixel_rendering_mode,
            // - The other for the orthographic projection and small FOVs on 2D projections
            fov_rendering_mode,

            fov_mode,

            gl,
        }
    }

    fn send_uniforms<T: Mesh + DisableDrawing>(&self, gl: &WebGl2Context, shader: &Shader, viewport: &ViewPort, renderable: &Renderable<T>) {
        // TEXTURES TILES BUFFER
        self.buffer.borrow().send_to_shader(shader);

        // Send viewport uniforms
        viewport.send_to_vertex_shader(gl, shader);
        
        // Send model matrix
        let model_mat_location = shader.get_uniform_location("model");
        let model_mat_f32_slice: &[f32; 16] = renderable.model_mat.as_ref();
        gl.uniform_matrix4fv_with_f32_array(model_mat_location, false, model_mat_f32_slice);

        // Send current time
        let location_time = shader.get_uniform_location("current_time");
        gl.uniform1f(location_time, utils::get_current_time());
    }
    
    /// Called when the HiPS has been changed
    pub fn refresh_buffer_tiles(&mut self) {
        console::log_1(&format!("refresh buffers").into());
        load_base_tiles(&self.gl, self.buffer.clone());

        self.buffer.borrow_mut()
            .clear();
    }
}

use std::collections::HashMap;
use crate::renderable::Renderable;
use cgmath::Matrix4;

use crate::utils;
use crate::texture::{Tile, TilePerPixelGPU};
use std::collections::BTreeSet;
use std::collections::VecDeque;

use crate::field_of_view::HEALPixCell;
use crate::viewport::{LastZoomAction, LastAction};

fn get_uv_in_parent(tile: &Tile, parent_tile: &Tile) -> [Vector2<f32>; 4] {
    let texture_parent_idx = parent_tile.texture_idx as i32;
        
    let parent_idx_row = (texture_parent_idx / 8) as f32; // in [0; 7]
    let parent_idx_col = (texture_parent_idx % 8) as f32; // in [0; 7]

    let (depth, idx) = (tile.cell.0, tile.cell.1);
    let (parent_depth, parent_idx) = (parent_tile.cell.0, parent_tile.cell.1);

    let idx_off = parent_idx << (2*(depth - parent_depth));

    assert!(idx >= idx_off);
    assert!(depth >= parent_depth);
    let nside = 1 << (depth - parent_depth);

    let (x, y) = utils::unmortonize(idx - idx_off);
    assert!(x < nside);
    assert!(y < nside);

    let u = (parent_idx_col + ((y as f32)/(nside as f32))) / 8_f32;
    let v = (parent_idx_row + ((x as f32)/(nside as f32))) / 8_f32;

    let ds = 1_f32 / (8_f32 * (nside as f32));

    [
        Vector2::new(u, v),
        Vector2::new(u + ds, v),
        Vector2::new(u, v + ds),
        Vector2::new(u + ds, v + ds)
    ]
}
fn get_uv(tile: &Tile) -> [Vector2<f32>; 4] {
    let texture_idx = tile.texture_idx as i32;
        
    let idx_row = (texture_idx / 8) as f32; // in [0; 7]
    let idx_col = (texture_idx % 8) as f32; // in [0; 7]

    let (depth, idx) = (tile.cell.0, tile.cell.1);

    let u = idx_col / 8_f32;
    let v = idx_row / 8_f32;

    let ds = 1_f32 / 8_f32;

    [
        Vector2::new(u, v),
        Vector2::new(u + ds, v),
        Vector2::new(u, v + ds),
        Vector2::new(u + ds, v + ds)
    ]
}

fn get_parent<'a>(tile: &Tile, parent_depth: u8, buffer: &'a BufferTiles) -> Option<&'a Tile> {
    let (depth, idx) = (tile.cell.0, tile.cell.1);
    assert!(parent_depth <= depth);

    let parent_idx = idx >> (2*(depth - parent_depth));
    //console::log_1(&format!("unzoom11").into());

    buffer.get(&HEALPixCell(parent_depth, parent_idx))
}

impl Mesh for HiPSSphere {
    fn create_buffers(&mut self, gl: &WebGl2Context) {}

    fn update<T: Mesh + DisableDrawing>(
        &mut self,
        _local_to_world: &Matrix4<f32>,
        _projection: &ProjectionType,
        viewport: &ViewPort
    ) {
        let field_of_view = viewport.field_of_view();

        let tiles_fov = field_of_view.cells();
        let current_depth = field_of_view.get_current_depth();

        let prev_depth = DEPTH.load(Ordering::Relaxed);
        DEPTH.store(current_depth, Ordering::Relaxed);

        let depth_changed = current_depth != prev_depth;

        if depth_changed {
            //self.fov_mode = current_depth >= 1;
        }

        load_tiles(&self.gl, self.buffer.clone(), tiles_fov, current_depth, depth_changed);

        // For Small FOV rendering mode
        if self.fov_mode {
            //let mut tiles_rendered = BTreeSet::new();
            let buffer = self.buffer.borrow();
            let vertices = if current_depth <= 2 {
                (0..192).map(|idx| {
                    let lonlat = healpix::nested::grid(2, idx, 1);

                    let blending_factor = 1_f32;
                    let uv_start = [Vector2::new(0_f32, 0_f32); 4];

                    let tile = Tile::new(HEALPixCell(2, idx));
                    let parent_tile_buffer = get_parent(&tile, 0, &buffer).unwrap();
                    let uv_end = get_uv_in_parent(&tile, &parent_tile_buffer);

                    let mut vertex_array = Vec::with_capacity(10 * 6);
                    let blending_factor = 1_f32;
                    SmallFieldOfViewRenderingMode::add_vertex(&mut vertex_array, &lonlat, 0, uv_start[0], uv_end[0], blending_factor);
                    SmallFieldOfViewRenderingMode::add_vertex(&mut vertex_array, &lonlat, 1, uv_start[1], uv_end[1], blending_factor);
                    SmallFieldOfViewRenderingMode::add_vertex(&mut vertex_array, &lonlat, 2, uv_start[2], uv_end[2], blending_factor);

                    SmallFieldOfViewRenderingMode::add_vertex(&mut vertex_array, &lonlat, 1, uv_start[1], uv_end[1], blending_factor);
                    SmallFieldOfViewRenderingMode::add_vertex(&mut vertex_array, &lonlat, 3, uv_start[3], uv_end[3], blending_factor);
                    SmallFieldOfViewRenderingMode::add_vertex(&mut vertex_array, &lonlat, 2, uv_start[2], uv_end[2], blending_factor);

                    vertex_array
                })
                .flatten()
                .collect::<Vec<_>>()
            } else {
                let num_tiles = tiles_fov.len();
                let mut vertices = Vec::with_capacity(10 * 6 * num_tiles);

                if viewport.last_zoom_action == LastZoomAction::Zoom {
                    for tile in tiles_fov.iter() {
                        // If the tile is not already processed
                        let (depth, idx) = (tile.0, tile.1);
                        let (mut parent_depth, mut parent_idx) = (depth - 1, idx >> 2);

                        let mut parent_tile = HEALPixCell(parent_depth, parent_idx);

                        while parent_depth > 0 {
                            if let Some(parent_tile) = buffer.get(&parent_tile) {
                                break;
                            }

                            parent_depth -= 1;
                            parent_idx = parent_idx >> 2;

                            parent_tile = Tile::new(HEALPixCell(parent_depth, parent_idx)).into();
                        }

                        let parent_tile_buffer = buffer.get(&parent_tile).unwrap();

                        let uv_start = get_uv_in_parent(&Tile::new(*tile), parent_tile_buffer);

                        let (uv_end, blending_factor) = if let Some(tile_buffer) = buffer.get(tile) {
                            let uv_end = get_uv(tile_buffer);
                            let blending_factor = tile_buffer.blending_factor();
                            //console::log_1(&format!("blending {:?}", blending_factor).into());

                            (uv_end, blending_factor)
                        } else {
                            let uv_end = [Vector2::new(0_f32, 0_f32); 4];
                            let blending_factor = 0_f32;

                            (uv_end, blending_factor)
                        };

                        let mut vertex_array = Vec::with_capacity(10 * 6);
                        let lonlat = healpix::nested::grid(depth, idx, 1);

                        SmallFieldOfViewRenderingMode::add_vertex(&mut vertex_array, &lonlat, 0, uv_start[0], uv_end[0], blending_factor);
                        SmallFieldOfViewRenderingMode::add_vertex(&mut vertex_array, &lonlat, 1, uv_start[1], uv_end[1], blending_factor);
                        SmallFieldOfViewRenderingMode::add_vertex(&mut vertex_array, &lonlat, 2, uv_start[2], uv_end[2], blending_factor);

                        SmallFieldOfViewRenderingMode::add_vertex(&mut vertex_array, &lonlat, 1, uv_start[1], uv_end[1], blending_factor);
                        SmallFieldOfViewRenderingMode::add_vertex(&mut vertex_array, &lonlat, 3, uv_start[3], uv_end[3], blending_factor);
                        SmallFieldOfViewRenderingMode::add_vertex(&mut vertex_array, &lonlat, 2, uv_start[2], uv_end[2], blending_factor);

                        // tile has been found in the buffer, we will
                        // render it
                        //tiles_rendered.insert(tile);
                        vertices.extend(vertex_array.iter());
                    }
                } else if viewport.last_action == LastAction::Moving || viewport.last_zoom_action == LastZoomAction::Unzoom {
                    let buffer = self.buffer.borrow();

                    for tile in tiles_fov.iter() {
                        if let Some(tile_buffer) = buffer.get(tile) {
                            
                            //console::log_1(&format!("unzoom1").into());
                            let blending_factor = tile_buffer.blending_factor();
                            //console::log_1(&format!("unzoom2").into());

                            let (depth, idx) = (tile_buffer.cell.0, tile_buffer.cell.1);
                            let texture_idx = tile_buffer.texture_idx;
                            let idx_row = (texture_idx / 8) as f32; // in [0; 7]
                            let idx_col = (texture_idx % 8) as f32; // in [0; 7]

                            if blending_factor == 1_f32 {
                                let uv_end = get_uv(tile_buffer);
                                let uv_start = [Vector2::new(0_f32, 0_f32); 4];

                                let mut vertex_array = Vec::with_capacity(10 * 6);
                                let lonlat = healpix::nested::grid(depth, idx, 1);
                                SmallFieldOfViewRenderingMode::add_vertex(&mut vertex_array, &lonlat, 0, uv_start[0], uv_end[0], blending_factor);
                                SmallFieldOfViewRenderingMode::add_vertex(&mut vertex_array, &lonlat, 1, uv_start[1], uv_end[1], blending_factor);
                                SmallFieldOfViewRenderingMode::add_vertex(&mut vertex_array, &lonlat, 2, uv_start[2], uv_end[2], blending_factor);

                                SmallFieldOfViewRenderingMode::add_vertex(&mut vertex_array, &lonlat, 1, uv_start[1], uv_end[1], blending_factor);
                                SmallFieldOfViewRenderingMode::add_vertex(&mut vertex_array, &lonlat, 3, uv_start[3], uv_end[3], blending_factor);
                                SmallFieldOfViewRenderingMode::add_vertex(&mut vertex_array, &lonlat, 2, uv_start[2], uv_end[2], blending_factor);

                                // tile has been found in the buffer, we will render it
                                vertices.extend(vertex_array.iter());
                            } else {
                                // We need to check the children tiles first to get the uv_start!
                                // Let is see at current_depth + 1 first
                                let mut children_tiles = VecDeque::with_capacity(4);
                                children_tiles.push_back(HEALPixCell(depth + 1, idx << 2));
                                children_tiles.push_back(HEALPixCell(depth + 1, (idx << 2) + 1));
                                children_tiles.push_back(HEALPixCell(depth + 1, (idx << 2) + 2));
                                children_tiles.push_back(HEALPixCell(depth + 1, (idx << 2) + 3));

                                while !children_tiles.is_empty() {
                                    let child_tile = children_tiles.pop_front().unwrap();
                                    let (child_depth, child_idx) = (child_tile.0, child_tile.1);

                                    if let Some(child_tile_buffer) = buffer.get(&child_tile) {
                                        // Find in which position the child tile is in the
                                        // parent to get the uv_end
                                        let uv_end = get_uv_in_parent(child_tile_buffer, tile_buffer);
                                        let uv_start = get_uv(child_tile_buffer);

                                        let mut vertex_array = Vec::with_capacity(10 * 6);
                                        let lonlat = healpix::nested::grid(child_depth, child_idx, 1);
                                        SmallFieldOfViewRenderingMode::add_vertex(&mut vertex_array, &lonlat, 0, uv_start[0], uv_end[0], blending_factor);
                                        SmallFieldOfViewRenderingMode::add_vertex(&mut vertex_array, &lonlat, 1, uv_start[1], uv_end[1], blending_factor);
                                        SmallFieldOfViewRenderingMode::add_vertex(&mut vertex_array, &lonlat, 2, uv_start[2], uv_end[2], blending_factor);

                                        SmallFieldOfViewRenderingMode::add_vertex(&mut vertex_array, &lonlat, 1, uv_start[1], uv_end[1], blending_factor);
                                        SmallFieldOfViewRenderingMode::add_vertex(&mut vertex_array, &lonlat, 3, uv_start[3], uv_end[3], blending_factor);
                                        SmallFieldOfViewRenderingMode::add_vertex(&mut vertex_array, &lonlat, 2, uv_start[2], uv_end[2], blending_factor);

                                        // tile has been found in the buffer, we will render it
                                        vertices.extend(vertex_array.iter());
                                    } else {
                                        if child_depth == depth + 2 {
                                            // The grand children is not in the buffer
        
                                            let tile_child = Tile::new(HEALPixCell(child_depth, child_idx));

                                            // Find in which base cell the child tile is located
                                            let tile_buffer_base = get_parent(&tile_child, 0, &buffer)
                                                .unwrap();
        
                                            let uv_start = get_uv_in_parent(&tile_child, tile_buffer_base);

                                            // Find in which position the child tile is located in the current fov tile
                                            let uv_end = get_uv_in_parent(&tile_child, tile_buffer);

                                            let mut vertex_array = Vec::with_capacity(10 * 6);
                                            let lonlat = healpix::nested::grid(child_depth, child_idx, 1);
                                            SmallFieldOfViewRenderingMode::add_vertex(&mut vertex_array, &lonlat, 0, uv_start[0], uv_end[0], blending_factor);
                                            SmallFieldOfViewRenderingMode::add_vertex(&mut vertex_array, &lonlat, 1, uv_start[1], uv_end[1], blending_factor);
                                            SmallFieldOfViewRenderingMode::add_vertex(&mut vertex_array, &lonlat, 2, uv_start[2], uv_end[2], blending_factor);

                                            SmallFieldOfViewRenderingMode::add_vertex(&mut vertex_array, &lonlat, 1, uv_start[1], uv_end[1], blending_factor);
                                            SmallFieldOfViewRenderingMode::add_vertex(&mut vertex_array, &lonlat, 3, uv_start[3], uv_end[3], blending_factor);
                                            SmallFieldOfViewRenderingMode::add_vertex(&mut vertex_array, &lonlat, 2, uv_start[2], uv_end[2], blending_factor);
                                            // tile has been found in the buffer, we will render it
                                            vertices.extend(vertex_array.iter());
                                        } else {
                                            // Split the child cell in its 4 children (e.g. grand children of the tile 
                                            // in the FOV)
                                            children_tiles.push_back(HEALPixCell(child_depth + 1, child_idx << 2));
                                            children_tiles.push_back(HEALPixCell(child_depth + 1, (child_idx << 2) + 1));
                                            children_tiles.push_back(HEALPixCell(child_depth + 1, (child_idx << 2) + 2));
                                            children_tiles.push_back(HEALPixCell(child_depth + 1, (child_idx << 2) + 3));
                                        }
                                    }
                                }
                            }
                        } else {
                            let (depth, idx) = (tile.0, tile.1);
                            // We need to check the children tiles first to get the uv_start!
                            // Let is see at current_depth + 1 first
                            let mut children_tiles = VecDeque::with_capacity(4);
                            children_tiles.push_back(HEALPixCell(depth + 1, idx << 2));
                            children_tiles.push_back(HEALPixCell(depth + 1, (idx << 2) + 1));
                            children_tiles.push_back(HEALPixCell(depth + 1, (idx << 2) + 2));
                            children_tiles.push_back(HEALPixCell(depth + 1, (idx << 2) + 3));

                            let blending_factor = 0_f32;
                            while !children_tiles.is_empty() {
                                let child_tile = children_tiles.pop_front().unwrap();
                                let (child_depth, child_idx) = (child_tile.0, child_tile.1);

                                if let Some(child_tile_buffer) = buffer.get(&child_tile) {
                                    // Find in which position the child tile is in the
                                    // parent to get the uv_end
                                    let uv_end = [Vector2::new(0_f32, 0_f32); 4];
                                    let uv_start = get_uv(child_tile_buffer);

                                    let mut vertex_array = Vec::with_capacity(10 * 6);
                                    let lonlat = healpix::nested::grid(child_depth, child_idx, 1);
                                    SmallFieldOfViewRenderingMode::add_vertex(&mut vertex_array, &lonlat, 0, uv_start[0], uv_end[0], blending_factor);
                                    SmallFieldOfViewRenderingMode::add_vertex(&mut vertex_array, &lonlat, 1, uv_start[1], uv_end[1], blending_factor);
                                    SmallFieldOfViewRenderingMode::add_vertex(&mut vertex_array, &lonlat, 2, uv_start[2], uv_end[2], blending_factor);

                                    SmallFieldOfViewRenderingMode::add_vertex(&mut vertex_array, &lonlat, 1, uv_start[1], uv_end[1], blending_factor);
                                    SmallFieldOfViewRenderingMode::add_vertex(&mut vertex_array, &lonlat, 3, uv_start[3], uv_end[3], blending_factor);
                                    SmallFieldOfViewRenderingMode::add_vertex(&mut vertex_array, &lonlat, 2, uv_start[2], uv_end[2], blending_factor);

                                    // tile has been found in the buffer, we will render it
                                    vertices.extend(vertex_array.iter());
                                } else {
                                    if child_depth == depth + 2 {
                                        // The grand children is not in the buffer
                                        let tile_child = Tile::new(HEALPixCell(child_depth, child_idx));

                                        // Find in which base cell the child tile is located
                                        let tile_buffer_base = get_parent(&tile_child, 0, &buffer)
                                            .unwrap();

                                        let uv_start = get_uv_in_parent(&tile_child, tile_buffer_base);

                                        // Find in which position the child tile is located in the current fov tile
                                        let uv_end = [Vector2::new(0_f32, 0_f32); 4];

                                        let mut vertex_array = Vec::with_capacity(10 * 6);
                                        let lonlat = healpix::nested::grid(child_depth, child_idx, 1);
                                        SmallFieldOfViewRenderingMode::add_vertex(&mut vertex_array, &lonlat, 0, uv_start[0], uv_end[0], blending_factor);
                                        SmallFieldOfViewRenderingMode::add_vertex(&mut vertex_array, &lonlat, 1, uv_start[1], uv_end[1], blending_factor);
                                        SmallFieldOfViewRenderingMode::add_vertex(&mut vertex_array, &lonlat, 2, uv_start[2], uv_end[2], blending_factor);

                                        SmallFieldOfViewRenderingMode::add_vertex(&mut vertex_array, &lonlat, 1, uv_start[1], uv_end[1], blending_factor);
                                        SmallFieldOfViewRenderingMode::add_vertex(&mut vertex_array, &lonlat, 3, uv_start[3], uv_end[3], blending_factor);
                                        SmallFieldOfViewRenderingMode::add_vertex(&mut vertex_array, &lonlat, 2, uv_start[2], uv_end[2], blending_factor);

                                        // tile has been found in the buffer, we will render it
                                        vertices.extend(vertex_array.iter());
                                    } else {
                                        // Split the child cell in its 4 children (e.g. grand children of the tile 
                                        // in the FOV)
                                        children_tiles.push_back(HEALPixCell(child_depth + 1, child_idx << 2));
                                        children_tiles.push_back(HEALPixCell(child_depth + 1, (child_idx << 2) + 1));
                                        children_tiles.push_back(HEALPixCell(child_depth + 1, (child_idx << 2) + 2));
                                        children_tiles.push_back(HEALPixCell(child_depth + 1, (child_idx << 2) + 3));
                                    }
                                }
                            }
                        }
                    }


                    /*while !tiles_process.is_empty() {
                        let tile = tiles_process.pop_front().unwrap();
                        if tiles_rendered.contains(&tile) {
                            continue;
                        }

                        let tile_buffer: TilePerPixelGPU = Tile::new(tile).into();

                        let (depth, idx) = (tile.0, tile.1);

                        if let Some(tile_buffer) = tiles_buffer.get(&tile_buffer) {
                            let texture_idx = tile_buffer.texture_idx;
                            let idx_row = (texture_idx / 8) as f32; // in [0; 7]
                            let idx_col = (texture_idx % 8) as f32; // in [0; 7]
                            let u = idx_col / 8_f32;
                            let v = idx_row / 8_f32;

                            let ds = 1_f32 / 8_f32;

                            let mut vertex_array = Vec::with_capacity(7 * 6);
                            let lonlat = healpix::nested::grid(depth, idx, 1);

                            // Check whether the tile is at the current depth
                            if depth == current_depth {

                            }

                            SmallFieldOfViewRenderingMode::add_vertex(&mut vertex_array, &lonlat, 0, Vector2::new(u, v));
                            SmallFieldOfViewRenderingMode::add_vertex(&mut vertex_array, &lonlat, 2, Vector2::new(u, v + ds));
                            SmallFieldOfViewRenderingMode::add_vertex(&mut vertex_array, &lonlat, 1, Vector2::new(u + ds, v));

                            SmallFieldOfViewRenderingMode::add_vertex(&mut vertex_array, &lonlat, 1, Vector2::new(u + ds, v));
                            SmallFieldOfViewRenderingMode::add_vertex(&mut vertex_array, &lonlat, 2, Vector2::new(u, v + ds));
                            SmallFieldOfViewRenderingMode::add_vertex(&mut vertex_array, &lonlat, 3, Vector2::new(u + ds, v + ds));

                            //vertices.extend(vertex_array.iter());
                            //tiles_rendered.insert(tile);

                        } else {
                            if depth == current_depth + 2 {
                                // Find the zero depth tile
                                let idx_depth_0 = idx >> (2*depth);
                                let idx_off = idx_depth_0 << (2*depth);

                                // Find the position of the cell in its base cell
                                assert!(idx >= idx_off);
                                let nside = 1 << depth;

                                let (x, y) = utils::unmortonize(idx - idx_off);
                                assert!(x < nside);
                                assert!(y < nside);

                                /*console::log_1(&format!("unzoom: {:?}", idx_depth_0).into());
                                unreachable!();*/
                                let texture_idx = tiles_buffer.get(&Tile::new(HEALPixCell(0, idx_depth_0)).into())
                                    // The base tile are always contained in the buffer !
                                    .unwrap()
                                    .texture_idx as i32;
                                let idx_row = (texture_idx / 8) as f32; // in [0; 7]
                                let idx_col = (texture_idx % 8) as f32; // in [0; 7]
                                
                                let u = (idx_col + ((y as f32)/(nside as f32))) / 8_f32;
                                let v = (idx_row + ((x as f32)/(nside as f32))) / 8_f32;

                                let ds = 1_f32 / (8_f32 * (nside as f32));

                                let mut vertex_array = Vec::with_capacity(7 * 6);
                                let lonlat = healpix::nested::grid(depth, idx, 1);
                                
                                SmallFieldOfViewRenderingMode::add_vertex(&mut vertex_array, &lonlat, 0, Vector2::new(u, v));
                                SmallFieldOfViewRenderingMode::add_vertex(&mut vertex_array, &lonlat, 2, Vector2::new(u, v + ds));
                                SmallFieldOfViewRenderingMode::add_vertex(&mut vertex_array, &lonlat, 1, Vector2::new(u + ds, v));

                                SmallFieldOfViewRenderingMode::add_vertex(&mut vertex_array, &lonlat, 1, Vector2::new(u + ds, v));
                                SmallFieldOfViewRenderingMode::add_vertex(&mut vertex_array, &lonlat, 2, Vector2::new(u, v + ds));
                                SmallFieldOfViewRenderingMode::add_vertex(&mut vertex_array, &lonlat, 3, Vector2::new(u + ds, v + ds));

                                vertices.extend(vertex_array.iter());
                                tiles_rendered.insert(HEALPixCell(depth, idx));
                            } else {
                                tiles_process.push_back(HEALPixCell(depth + 1, idx << 2));
                                tiles_process.push_back(HEALPixCell(depth + 1, (idx << 2) + 1));
                                tiles_process.push_back(HEALPixCell(depth + 1, (idx << 2) + 2));
                                tiles_process.push_back(HEALPixCell(depth + 1, (idx << 2) + 3));
                            }
                        }
                    }*/
                }
                vertices
            };

            // Update the buffers
            self.fov_rendering_mode.vertex_array_object.bind()
                .update_array(0, BufferData::VecData(&vertices));

            self.fov_rendering_mode.vertices = vertices;
        }
    }

    fn draw<T: Mesh + DisableDrawing>(
        &self,
        gl: &WebGl2Context,
        renderable: &Renderable<T>,
        shaders: &HashMap<&'static str, Shader>,
        viewport: &ViewPort
    ) {
        if self.fov_mode {
            let shader = &shaders["hips_sphere_small_fov"];
            shader.bind(gl);

            self.send_uniforms(gl, shader, viewport, renderable);

            SmallFieldOfViewRenderingMode::send_uniforms(gl, shader);
            self.fov_rendering_mode.draw(gl, renderable, shaders, viewport);
        } else {
            let shader = &shaders["hips_sphere"];
            shader.bind(gl);

            self.send_uniforms(gl, shader, viewport, renderable);

            PerPixelRenderingMode::send_uniforms(gl, shader);
            self.per_pixel_rendering_mode.draw(gl, renderable, shaders, viewport);
        }
    }
}

use crate::renderable::DisableDrawing;
impl DisableDrawing for HiPSSphere {
    fn disable(&mut self) {
    }
}