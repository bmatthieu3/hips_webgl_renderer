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
    fn create_vertices_array(gl: &WebGl2Context, projection: &ProjectionType) -> Vec<f32>;
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
    fn new(gl: &WebGl2Context, projection: &ProjectionType) -> SmallFieldOfViewRenderingMode {
        let vertices = Self::create_vertices_array(gl, projection);

        console::log_1(&format!("VERTICES LENGTH: {:?}", vertices.len()).into());
        //console::log_1(&format!("IDX TEXTURES LENGTH: {:?}", idx_textures.len()).into());

        let mut vertex_array_object = VertexArrayObject::new(gl);

        // VAO for the orthographic projection and small fovs on 2D projections
        vertex_array_object.bind()
            // Store the projeted and 3D vertex positions in a VBO
            .add_array_buffer(
                7 * mem::size_of::<f32>(),
                &[2, 3, 2],
                &[
                    0 * mem::size_of::<f32>(),
                    2 * mem::size_of::<f32>(),
                    5 * mem::size_of::<f32>()
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

    fn add_vertex(vertex_array: &mut Vec<f32>, lonlat: &[(f64, f64)], idx: usize, uv: Vector2<f32>) {
        let vertex = lonlat[idx];
        let (theta, delta) = (Rad(vertex.0 as f32), Rad(vertex.1 as f32));

        let pos_world_space = math::radec_to_xyz(theta, delta);
        vertex_array.extend(vec![
            theta.0,
            delta.0,
            
            pos_world_space.x,
            pos_world_space.y,
            pos_world_space.z,

            uv.x,
            uv.y
        ]);
    }
}

impl RenderingMode for SmallFieldOfViewRenderingMode {
    fn create_vertices_array(gl: &WebGl2Context, projection: &ProjectionType) -> Vec<f32> {
        let depth = 2;
        let vertices_data = (0..192)
            .map(|idx| {
                let lonlat = healpix::nested::grid(depth, idx, 1);

                let mut vertex_array = Vec::with_capacity(7 * 6);
                SmallFieldOfViewRenderingMode::add_vertex(&mut vertex_array, &lonlat, 0, Vector2::new(0_f32, 0_f32));
                SmallFieldOfViewRenderingMode::add_vertex(&mut vertex_array, &lonlat, 2, Vector2::new(0_f32, 1_f32 / 8_f32));
                SmallFieldOfViewRenderingMode::add_vertex(&mut vertex_array, &lonlat, 1, Vector2::new(1_f32 / 8_f32, 0_f32));

                SmallFieldOfViewRenderingMode::add_vertex(&mut vertex_array, &lonlat, 1, Vector2::new(1_f32 / 8_f32, 0_f32));
                SmallFieldOfViewRenderingMode::add_vertex(&mut vertex_array, &lonlat, 2, Vector2::new(0_f32, 1_f32 / 8_f32));
                SmallFieldOfViewRenderingMode::add_vertex(&mut vertex_array, &lonlat, 3, Vector2::new(1_f32 / 8_f32, 1_f32 / 8_f32));

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
            (self.vertices.len() as i32) / 7,
        );
    }
}

struct PerPixelRenderingMode {
    pub vertices: Vec<f32>,
    pub idx: Vec<u16>,

    vertex_array_object: VertexArrayObject,
}

impl PerPixelRenderingMode {
    fn new(gl: &WebGl2Context, projection: &ProjectionType) -> PerPixelRenderingMode {
        let vertices = Self::create_vertices_array(gl, projection);
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
    fn create_vertices_array(gl: &WebGl2Context, projection: &ProjectionType) -> Vec<f32> {
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

        let fov_rendering_mode = SmallFieldOfViewRenderingMode::new(gl, projection);
        let per_pixel_rendering_mode = PerPixelRenderingMode::new(gl, projection);

        let gl = gl.clone();
        let fov_mode = true;

        HiPSSphere {
            buffer,

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

        /*let ref allsky = ALLSKY.lock().unwrap();
        self.buffer_tiles.replace(BufferTiles::new(&self.gl, 64, "textures"));
        load_tiles(self.buffer_tiles.clone(), &allsky, 0, false);

        self.buffer_depth_zero_tiles.replace(BufferTiles::new(&self.gl, 12, "textures_0"));
        load_tiles(self.buffer_depth_zero_tiles.clone(), &allsky, 0, false);*/
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
use crate::viewport::LastZoomAction;
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

        load_tiles(&self.gl, self.buffer.clone(), tiles_fov, current_depth, depth_changed);

        // For Small FOV rendering mode
        if self.fov_mode {
            let num_tiles = tiles_fov.len();

            let mut vertices = Vec::with_capacity(7 * 6 * num_tiles);

            let tiles_buffer = self.buffer.borrow()
                .tiles();

            let mut tiles_rendered = BTreeSet::new();

            if viewport.last_zoom_action == LastZoomAction::Zoom {
                for tile_fov in tiles_fov.iter() {
                    // If the tile is not already processed
                    if !tiles_rendered.contains(tile_fov) {
                        let d1 = tile_fov.0;
                        let idx1 = tile_fov.1;

                        let mut tile_fov = tile_fov.clone();
                        let mut tile: TilePerPixelGPU = Tile::new(tile_fov).into();

                        while !tiles_buffer.contains(&tile) && tile_fov.0 > 0 {
                            tile_fov.0 -= 1;
                            tile_fov.1 = tile_fov.1 >> 2;

                            tile = Tile::new(tile_fov).into();
                        }

                        let texture_idx = tiles_buffer.get(&tile)
                            // The base tile are always contained in the buffer !
                            .unwrap()
                            .texture_idx as i32;
                        // tile has been found in the buffer, we will
                        // render it
                        tiles_rendered.insert(tile_fov);
                        
                        let idx_row = (texture_idx / 8) as f32; // in [0; 7]
                        let idx_col = (texture_idx % 8) as f32; // in [0; 7]

                        let d2 = tile_fov.0;
                        let idx2 = tile_fov.1 << (2*(d1 - d2));

                        assert!(idx1 >= idx2);

                        let nside = 1 << (d1 - d2);

                        let (x, y) = utils::unmortonize(idx1 - idx2);
                        assert!(x < nside);
                        assert!(y < nside);

                        let u = (idx_col + ((y as f32)/(nside as f32))) / 8_f32;
                        let v = (idx_row + ((x as f32)/(nside as f32))) / 8_f32;

                        let ds = 1_f32 / (8_f32 * (nside as f32));

                        let mut vertex_array = Vec::with_capacity(7 * 6);
                        let lonlat = healpix::nested::grid(d1, idx1, 1);

                        SmallFieldOfViewRenderingMode::add_vertex(&mut vertex_array, &lonlat, 0, Vector2::new(u, v));
                        SmallFieldOfViewRenderingMode::add_vertex(&mut vertex_array, &lonlat, 2, Vector2::new(u, v + ds));
                        SmallFieldOfViewRenderingMode::add_vertex(&mut vertex_array, &lonlat, 1, Vector2::new(u + ds, v));

                        SmallFieldOfViewRenderingMode::add_vertex(&mut vertex_array, &lonlat, 1, Vector2::new(u + ds, v));
                        SmallFieldOfViewRenderingMode::add_vertex(&mut vertex_array, &lonlat, 2, Vector2::new(u, v + ds));
                        SmallFieldOfViewRenderingMode::add_vertex(&mut vertex_array, &lonlat, 3, Vector2::new(u + ds, v + ds));

                        vertices.extend(vertex_array.iter());
                    }
                }
            } else {
                let mut tiles_process = tiles_fov.iter().cloned().collect::<VecDeque<_>>();

                while !tiles_process.is_empty() {
                    let tile = tiles_process.pop_front().unwrap();
                    if tiles_rendered.contains(&tile) {
                        continue;
                    }

                    let tile_buffer: TilePerPixelGPU = Tile::new(tile).into();

                    let (depth, idx) = (tile.0, tile.1);

                    if let Some(tile_buffer) = tiles_buffer.get(&tile_buffer) {
                        //console::log_1(&format!("found more deep TILEEEEEEEE").into());

                        let texture_idx = tile_buffer.texture_idx;
                        let idx_row = (texture_idx / 8) as f32; // in [0; 7]
                        let idx_col = (texture_idx % 8) as f32; // in [0; 7]
                        let u = idx_col / 8_f32;
                        let v = idx_row / 8_f32;

                        let ds = 1_f32 / 8_f32;

                        let mut vertex_array = Vec::with_capacity(7 * 6);
                        let lonlat = healpix::nested::grid(depth, idx, 1);
                        tiles_rendered.insert(tile);

                        SmallFieldOfViewRenderingMode::add_vertex(&mut vertex_array, &lonlat, 0, Vector2::new(u, v));
                        SmallFieldOfViewRenderingMode::add_vertex(&mut vertex_array, &lonlat, 2, Vector2::new(u, v + ds));
                        SmallFieldOfViewRenderingMode::add_vertex(&mut vertex_array, &lonlat, 1, Vector2::new(u + ds, v));

                        SmallFieldOfViewRenderingMode::add_vertex(&mut vertex_array, &lonlat, 1, Vector2::new(u + ds, v));
                        SmallFieldOfViewRenderingMode::add_vertex(&mut vertex_array, &lonlat, 2, Vector2::new(u, v + ds));
                        SmallFieldOfViewRenderingMode::add_vertex(&mut vertex_array, &lonlat, 3, Vector2::new(u + ds, v + ds));

                        vertices.extend(vertex_array.iter());
                    } else {
                        if depth > current_depth + 2 {
                            //let idx = idx >> 2*(depth - current_depth);
                            //let depth = current_depth;

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
                            tiles_rendered.insert(HEALPixCell(depth, idx));
                            
                            SmallFieldOfViewRenderingMode::add_vertex(&mut vertex_array, &lonlat, 0, Vector2::new(u, v));
                            SmallFieldOfViewRenderingMode::add_vertex(&mut vertex_array, &lonlat, 2, Vector2::new(u, v + ds));
                            SmallFieldOfViewRenderingMode::add_vertex(&mut vertex_array, &lonlat, 1, Vector2::new(u + ds, v));

                            SmallFieldOfViewRenderingMode::add_vertex(&mut vertex_array, &lonlat, 1, Vector2::new(u + ds, v));
                            SmallFieldOfViewRenderingMode::add_vertex(&mut vertex_array, &lonlat, 2, Vector2::new(u, v + ds));
                            SmallFieldOfViewRenderingMode::add_vertex(&mut vertex_array, &lonlat, 3, Vector2::new(u + ds, v + ds));

                            vertices.extend(vertex_array.iter());
                        } else {
                            tiles_process.push_back(HEALPixCell(depth + 1, idx << 2));
                            tiles_process.push_back(HEALPixCell(depth + 1, (idx << 2) + 1));
                            tiles_process.push_back(HEALPixCell(depth + 1, (idx << 2) + 2));
                            tiles_process.push_back(HEALPixCell(depth + 1, (idx << 2) + 3));
                        }
                    }
                }
            }

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