use web_sys::console;

use web_sys::WebGl2RenderingContext;

use crate::renderable::Mesh;
use crate::shader::Shader;

pub const NUM_VERTICES_PER_STEP: usize = 50;
pub const NUM_STEPS: usize = 20;
use crate::MAX_DEPTH;
use std::sync::atomic::Ordering;

use crate::buffer_tiles::BufferTiles;

use std::sync::Arc;
use std::sync::atomic::AtomicU8;
lazy_static! {
    pub static ref DEPTH: Arc<AtomicU8> = Arc::new(AtomicU8::new(0));
}

use crate::renderable::buffers::vertex_array_object::VertexArrayObject;
use crate::viewport::ViewPort;
use cgmath::Vector2;
use crate::WebGl2Context;

use crate::projection::Projection;
pub trait RenderingMode {
    fn new(gl: &WebGl2Context, viewport: &ViewPort) -> Self;

    fn update(&mut self, buffer: &mut BufferTiles, depth: u8, viewport: &ViewPort);

    fn draw(&self, gl: &WebGl2Context, shader: &Shader);
    fn get_shader<'a>(shaders: &'a HashMap<&'static str, Shader>) -> &'a Shader;

    fn send_to_shader(buffer: &BufferTiles, shader: &Shader);
}

use cgmath::Vector3;
#[repr(C)]
struct Vertex {
    lon: f32,
    lat: f32,

    pos: Vector3<f32>,

    uv_0: Vector2<f32>,
    uv_1: Vector2<f32>,

    alpha: f32,
}

impl Vertex {
    fn new(
        lonlat: &(f64, f64),
        uv_0: Vector2<f32>,
        uv_1: Vector2<f32>,
        alpha: f32
    ) -> Vertex {
        let (lon, lat) = (lonlat.0 as f32, lonlat.1 as f32);

        let pos = math::radec_to_xyz(Rad(lon), Rad(lat));
        Vertex {
            lon,
            lat,

            pos,

            uv_0,
            uv_1,

            alpha,
        }
    }
}

#[repr(C)]
// One tile contains 2 triangles of 3 vertices each
struct TileVertices([Vertex; 6]);

pub struct SmallFieldOfView {
    vertex_array_object: VertexArrayObject,
    num_vertices: usize,
}

use crate::renderable::buffers::buffer_data::BufferData;
use cgmath::Rad;
use crate::math;
use std::mem;

fn add_vertices_grid(
    tiles_vertices: &mut Vec<TileVertices>,
    cell: &HEALPixCell,
    n_segments: u16,
    uv_0: &[Vector2<f32>; 4],
    uv_1: &[Vector2<f32>; 4],
    alpha: f32
) {
    let lonlat = healpix::nested::grid(cell.0, cell.1, n_segments);

    let n_vertices_per_segment = n_segments + 1;
    
    for i in 0..n_segments {
        for j in 0..n_segments {
            let id_vertex_0 = (j + i * n_vertices_per_segment) as usize;
            let id_vertex_1 = (j + 1 + i * n_vertices_per_segment) as usize;
            let id_vertex_2 = (j + (i + 1) * n_vertices_per_segment) as usize;
            let id_vertex_3 = (j + 1 + (i + 1) * n_vertices_per_segment) as usize;

            let lonlat_quad = [
                lonlat[id_vertex_0],
                lonlat[id_vertex_1],
                lonlat[id_vertex_2],
                lonlat[id_vertex_3],
            ];

            let hj0 = (j as f32) / (n_segments as f32);
            let hi0 = (i as f32) / (n_segments as f32);

            let hj1 = ((j + 1) as f32) / (n_segments as f32);
            let hi1 = (i as f32) / (n_segments as f32);

            let hj2 = (j as f32) / (n_segments as f32);
            let hi2 = ((i + 1) as f32) / (n_segments as f32);

            let hj3 = ((j + 1) as f32) / (n_segments as f32);
            let hi3 = ((i + 1) as f32) / (n_segments as f32);

            let d01s = uv_0[1].x - uv_0[0].x;
            let d02s = uv_0[2].y - uv_0[0].y;

            let uv_s_vertex_0 = Vector2::new(uv_0[0].x + hj0 * d01s, uv_0[0].y + hi0 * d02s);
            let uv_s_vertex_1 = Vector2::new(uv_0[0].x + hj1 * d01s, uv_0[0].y + hi1 * d02s);
            let uv_s_vertex_2 = Vector2::new(uv_0[0].x + hj2 * d01s, uv_0[0].y + hi2 * d02s);
            let uv_s_vertex_3 = Vector2::new(uv_0[0].x + hj3 * d01s, uv_0[0].y + hi3 * d02s);

            let uv_0_quad = [
                uv_s_vertex_0,
                uv_s_vertex_1,
                uv_s_vertex_2,
                uv_s_vertex_3,
            ];
            let d01e = uv_1[1].x - uv_1[0].x;
            let d02e = uv_1[2].y - uv_1[0].y;
            let uv_e_vertex_0 = Vector2::new(uv_1[0].x + hj0 * d01e, uv_1[0].y + hi0 * d02e);
            let uv_e_vertex_1 = Vector2::new(uv_1[0].x + hj1 * d01e, uv_1[0].y + hi1 * d02e);
            let uv_e_vertex_2 = Vector2::new(uv_1[0].x + hj2 * d01e, uv_1[0].y + hi2 * d02e);
            let uv_e_vertex_3 = Vector2::new(uv_1[0].x + hj3 * d01e, uv_1[0].y + hi3 * d02e);

            let uv_1_quad = [
                uv_e_vertex_0,
                uv_e_vertex_1,
                uv_e_vertex_2,
                uv_e_vertex_3,
            ];

            let tile_vertices = TileVertices([
                Vertex::new(&lonlat_quad[0], uv_0_quad[0], uv_1_quad[0], alpha),
                Vertex::new(&lonlat_quad[1], uv_0_quad[1], uv_1_quad[1], alpha),
                Vertex::new(&lonlat_quad[2], uv_0_quad[2], uv_1_quad[2], alpha),

                Vertex::new(&lonlat_quad[1], uv_0_quad[1], uv_1_quad[1], alpha),
                Vertex::new(&lonlat_quad[3], uv_0_quad[3], uv_1_quad[3], alpha),
                Vertex::new(&lonlat_quad[2], uv_0_quad[2], uv_1_quad[2], alpha),
            ]);
            tiles_vertices.push(tile_vertices);
        }
    }
}

impl SmallFieldOfView {
    async fn define_needed_hpx_cells(buffer: &mut BufferTiles, depth: u8, cells_fov: &Vec<HEALPixCell>) -> Vec<TileVertices> {
        let num_cells = cells_fov.len();
        let mut tiles = Vec::with_capacity(num_cells * 4);

        let num_subdivision = if depth <= 2 {
            1 << (3 - depth)
        } else {
            1
        };
        
        for cell_fov in cells_fov {
            let (uv_0, uv_1, alpha) = if let Some(tile_fov) = buffer.get(cell_fov) {
                let alpha = tile_fov.blending_factor();

                let uv_0 = if alpha < 1_f32 {
                    let parent_cell = get_nearest_parent(cell_fov, buffer);

                    get_uv_in_parent(cell_fov, &parent_cell, buffer)
                } else {
                    [Vector2::new(0_f32, 0_f32); 4]
                };
                
                let uv_1 = get_uv(cell_fov, buffer);
                (uv_0, uv_1, alpha)
            } else {
                //console::log_1(&format!("before get nearest").into());
                let parent_cell = get_nearest_parent(cell_fov, buffer);
                
                let alpha = buffer.get(&parent_cell).unwrap().blending_factor();
                let uv_0 = if alpha < 1_f32 {
                    let grand_parent_cell = get_nearest_parent(&parent_cell, buffer);

                    get_uv_in_parent(cell_fov, &grand_parent_cell, buffer)
                } else {
                    [Vector2::new(0_f32, 0_f32); 4]
                };
                let uv_1 = get_uv_in_parent(cell_fov, &parent_cell, buffer);

                (uv_0, uv_1, alpha)
            };

            add_vertices_grid(
                &mut tiles,
                cell_fov,
                num_subdivision,
                &uv_0, &uv_1,
                alpha
            );
        }

        tiles
    }
}

const NUM_F32_MAX_TO_GPU: usize = 60000;

impl RenderingMode for SmallFieldOfView {
    fn new(gl: &WebGl2Context, viewport: &ViewPort) -> SmallFieldOfView {
        let vertices = vec![0_f32; NUM_F32_MAX_TO_GPU];

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

        let num_vertices = NUM_F32_MAX_TO_GPU;
        SmallFieldOfView {
            vertex_array_object,
            num_vertices,
        }
    }

    fn get_shader<'a>(shaders: &'a HashMap<&'static str, Shader>) -> &'a Shader  {
        &shaders["hips_sphere_small_fov"]
    }

    fn draw(&self, gl: &WebGl2Context, shader: &Shader) {
        self.vertex_array_object.bind_ref();
        gl.draw_arrays(
            WebGl2RenderingContext::TRIANGLES,
            0,
            self.num_vertices as i32,
        );
    }

    fn update(&mut self, buffer: &mut BufferTiles, depth: u8, viewport: &ViewPort) {
        // If at least the base tiles have not been loaded
        // then we do nothing
        if !buffer.is_ready() {
            return;
        }

        let field_of_view = viewport.field_of_view();
        let cells_fov = field_of_view.healpix_cells();
        // Signals a new frame to the buffer
        buffer.signals_new_frame();
        let mut tiles = futures::executor::block_on(
            Self::define_needed_hpx_cells(
                buffer,
                depth,
                cells_fov
            )
        );

        // Assert that the number of tiles needed does not overflow the
        // GPU tile buffer
        //assert!(cells.len() <= 64);
        // Build the 4096x4096 texture containing all the
        // tiles we need
        /*if depth < 2 {
            buffer.build_texture(cells);
        }*/

        // There is 6 vertices per tiles
        let num_vertices = tiles.len() * 6;

        // Convert Vec<TileVertices> to Vec<f32>
        let len = num_vertices * 10;
        let cap = len;
        let ptr = tiles.as_mut_ptr() as *mut f32;
    
        mem::forget(tiles);
    
        let data = unsafe { Vec::from_raw_parts(ptr, len, cap) };
        self.num_vertices = num_vertices;

        // Update the buffers
        self.vertex_array_object.bind()
            .update_array(0, BufferData::VecData(&data));

        /*
        if viewport.last_zoom_action == LastZoomAction::Zoom || viewport.last_action == LastAction::Moving {
            for cell in tiles_fov.iter() {
                // If the tile is not already processed
                let (depth, idx) = (cell.0, cell.1);
                let parent_tile_buffer = get_nearest_parent(cell, buffer);

                let uv_start = get_uv_in_parent(&cell, &parent_tile_buffer);

                let mut blending_factor = 0_f32;
                let uv_end = if let Some(tile_buffer) = buffer.get(cell) {
                    let uv_end = get_uv(&tile_buffer);

                    blending_factor = tile_buffer.blending_factor();
                    uv_end
                } else {
                    let uv_end = [Vector2::new(0_f32, 0_f32); 4];

                    uv_end
                };

                let mut vertex_array = Vec::with_capacity(10 * 6);
                add_vertices_grid(
                    &mut vertex_array,
                    depth, idx,
                    1,
                    &uv_start, &uv_end,
                    blending_factor
                );

                // tile has been found in the buffer, we will
                // render it
                vertices.extend(vertex_array.iter());
            }
        } else if viewport.last_zoom_action == LastZoomAction::Unzoom {
            for tile in tiles_fov.iter() {
                if let Some(tile_buffer) = buffer.get(tile) {
                    let blending_factor = tile_buffer.blending_factor();

                    let (depth, idx) = (tile_buffer.cell.0, tile_buffer.cell.1);

                    if blending_factor == 1_f32 {
                        let uv_end = get_uv(&tile_buffer);
                        let uv_start = [Vector2::new(0_f32, 0_f32); 4];

                        let mut vertex_array = Vec::with_capacity(10 * 6);

                        add_vertices_grid(
                            &mut vertex_array,
                            depth, idx,
                            4,
                            &uv_start, &uv_end,
                            blending_factor
                        );

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
                                let uv_end = get_uv_in_parent(&child_tile, &tile_buffer);
                                let uv_start = get_uv(&child_tile_buffer);

                                let mut vertex_array = Vec::with_capacity(10 * 6);
                                add_vertices_grid(&mut vertex_array,
                                    child_depth,
                                    child_idx,
                                    1 << (depth + 2 - child_depth),
                                    &uv_start, &uv_end,
                                    blending_factor);

                                // tile has been found in the buffer, we will render it
                                vertices.extend(vertex_array.iter());
                            } else {
                                if child_depth == depth + 2 {
                                    // The grand children is not in the buffer

                                    // Find in which base cell the current tile is located
                                    let tile_buffer_parent = get_nearest_parent(&tile, buffer);

                                    let uv_start = get_uv_in_parent(&child_tile, &tile_buffer_parent);

                                    // Find in which position the child tile is located in the current fov tile
                                    let uv_end = get_uv_in_parent(&child_tile, &tile_buffer);

                                    let mut vertex_array = Vec::with_capacity(10 * 6);

                                    add_vertices_grid(&mut vertex_array,
                                        child_depth,
                                        child_idx,
                                        1,
                                        &uv_start, &uv_end,
                                        blending_factor
                                    );

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
                            let uv_start = get_uv(&child_tile_buffer);

                            let mut vertex_array = Vec::with_capacity(10 * 6);

                            add_vertices_grid(&mut vertex_array,
                                child_depth,
                                child_idx,
                                1 << (depth + 2 - child_depth),
                                &uv_start,
                                &uv_end,
                                blending_factor
                            );

                            // tile has been found in the buffer, we will render it
                            vertices.extend(vertex_array.iter());
                        } else {
                            if child_depth == depth + 2 {
                                // The grand children is not in the buffer

                                // Find in which base cell the child tile is located
                                let tile_buffer_parent = get_nearest_parent(&child_tile, buffer);

                                let uv_start = get_uv_in_parent(&child_tile, &tile_buffer_parent);
                                // Find in which position the child tile is located in the current fov tile
                                let uv_end = [Vector2::new(0_f32, 0_f32); 4];

                                let mut vertex_array = Vec::with_capacity(10 * 6);
                                add_vertices_grid(&mut vertex_array,
                                    child_depth,
                                    child_idx,
                                    1,
                                    &uv_start,
                                    &uv_end,
                                    blending_factor
                                );
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
        }

        // Update the buffers
        self.vertex_array_object.bind()
            .update_array(0, BufferData::VecData(&vertices));
        */
    }

    fn send_to_shader(buffer: &BufferTiles, shader: &Shader) {
        buffer.send_texture_to_shader(shader);
    }
}

pub struct PerPixel<P> where P: Projection {
    vertex_array_object: VertexArrayObject,

    projection: std::marker::PhantomData<P>
}

impl<P> PerPixel<P> where P: Projection {
    fn create_vertices_array(gl: &WebGl2Context, viewport: &ViewPort) -> (Vec<f32>, Vec<u16>) {
        let (vertex_screen_space_positions, indices) = <P>::build_screen_map(viewport);

        let vertices_data = vertex_screen_space_positions
            .into_iter()
            .map(|pos_screen_space| {
                let pos_clip_space = crate::projection::screen_to_clip_space(pos_screen_space, viewport);
                let pos_world_space = P::clip_to_world_space(pos_clip_space).unwrap();

                vec![pos_clip_space.x, pos_clip_space.y, pos_world_space.x, pos_world_space.y, pos_world_space.z]
            })
            .flatten()
            .collect::<Vec<_>>();
        console::log_1(&format!("End Generation per pixel mode vertices").into());

        (vertices_data, indices)
    }
}

impl<P> RenderingMode for PerPixel<P> where P: Projection {
    fn new(gl: &WebGl2Context, viewport: &ViewPort) -> PerPixel<P> {
        let (vertices, idx) = Self::create_vertices_array(gl, viewport);
        //let idx = Self::create_index_array().unwrap();

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

        PerPixel::<P> {
            vertex_array_object,

            projection: std::marker::PhantomData
        }
    }

    fn draw(
        &self,
        gl: &WebGl2Context,
        shader: &Shader,
    ) {
        self.vertex_array_object.bind_ref();

        gl.draw_elements_with_i32(
            WebGl2RenderingContext::TRIANGLES,
            self.vertex_array_object.num_elements() as i32,
            WebGl2RenderingContext::UNSIGNED_SHORT,
            0,
        );
    }

    fn get_shader<'a>(shaders: &'a HashMap<&'static str, Shader>) -> &'a Shader  {
        &shaders["hips_sphere"]
    }

    fn update(&mut self, buffer: &mut BufferTiles, depth: u8, viewport: &ViewPort) {
    }

    fn send_to_shader(buffer: &BufferTiles, shader: &Shader) {
    }
}

use crate::projection::*;
pub struct HiPSSphere {
    buffer: BufferTiles,

    ortho: SmallFieldOfView,
    aitoff_perpixel: PerPixel<Aitoff>,
    moll_perpixel: PerPixel<MollWeide>,
    arc_perpixel: PerPixel<AzimutalEquidistant>,
    mercator_perpixel: PerPixel<Mercator>,

    gl: WebGl2Context,

    depth: u8,
}

use cgmath::Deg;

impl HiPSSphere {
    pub fn new(gl: &WebGl2Context, viewport: &ViewPort) -> HiPSSphere {
        let buffer = BufferTiles::new(gl);

        let gl = gl.clone();

        let ortho = SmallFieldOfView::new(&gl, &viewport);
        let aitoff_perpixel = PerPixel::<Aitoff>::new(&gl, &viewport);
        let moll_perpixel = PerPixel::<MollWeide>::new(&gl, &viewport);
        let arc_perpixel = PerPixel::<AzimutalEquidistant>::new(&gl, &viewport);
        let mercator_perpixel = PerPixel::<Mercator>::new(&gl, &viewport);

        let depth = 0;

        HiPSSphere {
            buffer,

            ortho,
            aitoff_perpixel,
            moll_perpixel,
            arc_perpixel,
            mercator_perpixel,

            gl,

            depth,
        }
    }

    fn send_global_uniforms<T: Mesh + DisableDrawing>(&self, gl: &WebGl2Context, shader: &Shader, viewport: &ViewPort, renderable: &Renderable<T>) {
        // TEXTURES TILES BUFFER
        SmallFieldOfView::send_to_shader(&self.buffer, shader);
        
        // Send viewport uniforms
        viewport.send_to_vertex_shader(gl, shader);
        //console::log_1(&format!("ADFSD").into());
        // Send model matrix
        let model_mat_location = shader.get_uniform_location("model");
        let model_mat_f32_slice: &[f32; 16] = renderable.model_mat.as_ref();
        gl.uniform_matrix4fv_with_f32_array(model_mat_location, false, model_mat_f32_slice);

        // Send current time
        let location_time = shader.get_uniform_location("current_time");
        gl.uniform1f(location_time, utils::get_current_time());

        // Send current depth
        let location_current_depth = shader.get_uniform_location("current_depth");
        gl.uniform1i(location_current_depth, self.depth as i32);
        // Send max depth of the current HiPS
        let location_max_depth = shader.get_uniform_location("max_depth");
        gl.uniform1i(location_max_depth, MAX_DEPTH.load(Ordering::Relaxed) as i32);
    }
    
    /// Called when the HiPS has been changed
    pub fn refresh_buffer_tiles(&mut self) {
        self.buffer.clear();
    }

    /*pub fn get_buffer(&self) -> Rc<RefCell<BufferTiles>> {
        self.buffer.clone()
    }*/

    pub fn request_tiles(&mut self, viewport: &ViewPort) {
        let field_of_view = viewport.field_of_view();
        let tiles_fov = field_of_view.healpix_cells();
        
        let depth = field_of_view.current_depth();
        self.depth = depth;

        let depth_changed = depth != self.depth;

        self.buffer.request_tiles(tiles_fov, depth_changed);
    }

    pub fn update<P: Projection>(&mut self, viewport: &ViewPort) {
        match P::name() {
            "Orthographic" => {
                // Ortho mode
                self.ortho.update(
                    &mut self.buffer,
                    self.depth,
                    viewport,
                );
            },
            _ => (),
        }
    }

    pub fn draw<T: Mesh + DisableDrawing, P: Projection>(
        &self,
        gl: &WebGl2Context,
        renderable: &Renderable<T>,
        shaders: &HashMap<&'static str, Shader>,
        viewport: &ViewPort,
    ) {
        // Big field of view, check the projections
        match P::name() {
            "Aitoff" => {
                let shader = PerPixel::<P>::get_shader(shaders); // TODO: The same shader for all projection
                shader.bind(gl);

                self.send_global_uniforms(gl, shader, viewport, renderable);
                self.aitoff_perpixel.draw(gl, shader)
            },
            "MollWeide" => {
                let shader = PerPixel::<P>::get_shader(shaders); // TODO: The same shader for all projection
                shader.bind(gl);

                self.send_global_uniforms(gl, shader, viewport, renderable);
                self.moll_perpixel.draw(gl, shader)
            },
            "Arc" => {
                let shader = PerPixel::<P>::get_shader(shaders); // TODO: The same shader for all projection
                shader.bind(gl);

                self.send_global_uniforms(gl, shader, viewport, renderable);
                self.arc_perpixel.draw(gl, shader)
            },
            "Mercator" => {
                let shader = PerPixel::<P>::get_shader(shaders); // TODO: The same shader for all projection
                shader.bind(gl);

                self.send_global_uniforms(gl, shader, viewport, renderable);
                self.mercator_perpixel.draw(gl, shader)
            },
            // By construction, we are in orthographic projection when we have zoomed or the ortho projection selected
            "Orthographic" => {
                let shader = SmallFieldOfView::get_shader(shaders); // TODO: The same shader for all projection
                shader.bind(gl);

                self.send_global_uniforms(gl, shader, viewport, renderable);
                self.ortho.draw(gl, shader)
            },
            _ => panic!("Not all projection are handled!"),
        }
    }
}

use std::collections::HashMap;
use crate::renderable::Renderable;

use crate::utils;

use crate::healpix_cell::HEALPixCell;
use crate::viewport::{LastZoomAction, LastAction};

fn get_uv_in_parent(
    cell: &HEALPixCell,
    parent_cell: &HEALPixCell,
    buffer: &mut BufferTiles
) -> [Vector2<f32>; 4] {
    let (depth, idx) = (cell.0, cell.1);
    let (parent_depth, parent_idx) = (parent_cell.0, parent_cell.1);

    let idx_off = parent_idx << (2*(depth - parent_depth));

    assert!(idx >= idx_off);
    assert!(depth >= parent_depth);
    let nside = 1 << (depth - parent_depth);

    let (x, y) = utils::unmortonize(idx - idx_off);
    assert!(x < nside);
    assert!(y < nside);

    let parent_idx_tile = buffer.get_idx_texture(parent_cell);
    let parent_idx_row = (parent_idx_tile / 8) as f32; // in [0; 7]
    let parent_idx_col = (parent_idx_tile % 8) as f32; // in [0; 7]
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
fn get_uv(cell: &HEALPixCell, buffer: &mut BufferTiles) -> [Vector2<f32>; 4] {
    let idx_tile = buffer.get_idx_texture(cell);
    let idx_row = (idx_tile / 8) as f32; // in [0; 7]
    let idx_col = (idx_tile % 8) as f32; // in [0; 7]

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

// Get the nearest parent tile found in the buffer of `tile`
fn get_nearest_parent(cell: &HEALPixCell, buffer: &BufferTiles) -> HEALPixCell {
    let depth_start = cell.0 as i8;
    let idx_start = cell.1;
    if depth_start == 0 {
        // Base cells are in the buffer by construction
        return *cell;
    }

    let (mut depth, mut idx) = (depth_start - 1, idx_start >> 2);

    while depth > 0 {
        let parent_cell = HEALPixCell(depth as u8, idx);
        if buffer.contains(&parent_cell) {
            return parent_cell;
        }

        depth -= 1;
        idx = idx >> 2;
    }

    // Base cells are in the buffer by construction
    HEALPixCell(depth as u8, idx)
}

impl Mesh for HiPSSphere {
    fn get_shader<'a>(&self, shaders: &'a HashMap<&'static str, Shader>) -> &'a Shader {
        &shaders["hips_sphere"]
    }

    fn create_buffers(&mut self, gl: &WebGl2Context) {}
}

use crate::renderable::DisableDrawing;
impl DisableDrawing for HiPSSphere {
    fn disable(&mut self, _: &ViewPort) {
    }
}