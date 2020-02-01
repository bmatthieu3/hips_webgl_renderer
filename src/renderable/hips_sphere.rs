use web_sys::console;

use web_sys::WebGl2RenderingContext;

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

use crate::projection::Projection;
pub trait RenderingMode {
    fn new(gl: &WebGl2Context, viewport: &ViewPort) -> Self;

    fn update(&mut self, buffer: &BufferTiles, current_depth: u8, tiles_fov: &BTreeSet<HEALPixCell>, viewport: &ViewPort);

    fn draw(&self, gl: &WebGl2Context, shader: &Shader);
    fn get_shader<'a>(shaders: &'a HashMap<&'static str, Shader>) -> &'a Shader;
}

pub struct SmallFieldOfView {
    pub vertices: Vec<f32>,
    vertex_array_object: VertexArrayObject,
}

use crate::renderable::buffers::buffer_data::BufferData;
use cgmath::Rad;
use crate::math;
use std::mem;
impl SmallFieldOfView {
    fn add_vertex(
        vertex_array: &mut Vec<f32>,
        lonlat: &[(f64, f64)],
        idx: usize,
        uv_start: Vector2<f32>,
        uv_end: Vector2<f32>,
        blending_factor: f32,
        idx_texture_start: f32,
        idx_texture_end: f32,
    ) {
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

            blending_factor,

            idx_texture_start,
            idx_texture_end,
        ]);
    }
}

fn add_vertices_grid(
    vertex_array: &mut Vec<f32>,
    depth: u8, idx: u64, n_segments: u16,
    uv_start: &[Vector2<f32>; 4],
    uv_end: &[Vector2<f32>; 4],
    idx_texture_start: u8,
    idx_texture_end: u8,
    blending_factor: f32
) {
    let lonlat = healpix::nested::grid(depth, idx, n_segments);

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
            //console::log_1(&format!("id_vertex: {:?} {:?} {:?} {:?}", id_vertex_0, id_vertex_1, id_vertex_2, id_vertex_3).into());

            let hj0 = (j as f32) / (n_segments as f32);
            let hi0 = (i as f32) / (n_segments as f32);

            let hj1 = ((j + 1) as f32) / (n_segments as f32);
            let hi1 = (i as f32) / (n_segments as f32);

            let hj2 = (j as f32) / (n_segments as f32);
            let hi2 = ((i + 1) as f32) / (n_segments as f32);

            let hj3 = ((j + 1) as f32) / (n_segments as f32);
            let hi3 = ((i + 1) as f32) / (n_segments as f32);

            let d01s = uv_start[1].x - uv_start[0].x;
            let d02s = uv_start[2].y - uv_start[0].y;

            let uv_s_vertex_0 = Vector2::new(uv_start[0].x + hj0 * d01s, uv_start[0].y + hi0 * d02s);
            let uv_s_vertex_1 = Vector2::new(uv_start[0].x + hj1 * d01s, uv_start[0].y + hi1 * d02s);
            let uv_s_vertex_2 = Vector2::new(uv_start[0].x + hj2 * d01s, uv_start[0].y + hi2 * d02s);
            let uv_s_vertex_3 = Vector2::new(uv_start[0].x + hj3 * d01s, uv_start[0].y + hi3 * d02s);

            let uv_start_quad = [
                uv_s_vertex_0,
                uv_s_vertex_1,
                uv_s_vertex_2,
                uv_s_vertex_3,
            ];
            let d01e = uv_end[1].x - uv_end[0].x;
            let d02e = uv_end[2].y - uv_end[0].y;
            let uv_e_vertex_0 = Vector2::new(uv_end[0].x + hj0 * d01e, uv_end[0].y + hi0 * d02e);
            let uv_e_vertex_1 = Vector2::new(uv_end[0].x + hj1 * d01e, uv_end[0].y + hi1 * d02e);
            let uv_e_vertex_2 = Vector2::new(uv_end[0].x + hj2 * d01e, uv_end[0].y + hi2 * d02e);
            let uv_e_vertex_3 = Vector2::new(uv_end[0].x + hj3 * d01e, uv_end[0].y + hi3 * d02e);

            let uv_end_quad = [
                uv_e_vertex_0,
                uv_e_vertex_1,
                uv_e_vertex_2,
                uv_e_vertex_3,
            ];

            let idx_texture_start = (idx_texture_start / 64) as f32;
            let idx_texture_end = (idx_texture_end / 64) as f32;

            SmallFieldOfView::add_vertex(vertex_array, &lonlat_quad, 0, uv_start_quad[0], uv_end_quad[0], blending_factor, idx_texture_start, idx_texture_end);
            SmallFieldOfView::add_vertex(vertex_array, &lonlat_quad, 1, uv_start_quad[1], uv_end_quad[1], blending_factor, idx_texture_start, idx_texture_end);
            SmallFieldOfView::add_vertex(vertex_array, &lonlat_quad, 2, uv_start_quad[2], uv_end_quad[2], blending_factor, idx_texture_start, idx_texture_end);

            SmallFieldOfView::add_vertex(vertex_array, &lonlat_quad, 1, uv_start_quad[1], uv_end_quad[1], blending_factor, idx_texture_start, idx_texture_end);
            SmallFieldOfView::add_vertex(vertex_array, &lonlat_quad, 3, uv_start_quad[3], uv_end_quad[3], blending_factor, idx_texture_start, idx_texture_end);
            SmallFieldOfView::add_vertex(vertex_array, &lonlat_quad, 2, uv_start_quad[2], uv_end_quad[2], blending_factor, idx_texture_start, idx_texture_end);
        }
    }
}

impl RenderingMode for SmallFieldOfView {
    fn new(gl: &WebGl2Context, viewport: &ViewPort) -> SmallFieldOfView {
        let vertices = vec![0_f32; 60000];

        let mut vertex_array_object = VertexArrayObject::new(gl);

        // VAO for the orthographic projection and small fovs on 2D projections
        vertex_array_object.bind()
            // Store the projeted and 3D vertex positions in a VBO
            .add_array_buffer(
                12 * mem::size_of::<f32>(),
                &[2, 3, 2, 2, 1, 2],    
                &[
                    0 * mem::size_of::<f32>(),
                    2 * mem::size_of::<f32>(),
                    5 * mem::size_of::<f32>(),
                    7 * mem::size_of::<f32>(),
                    9 * mem::size_of::<f32>(),
                    10 * mem::size_of::<f32>(),
                ],
                WebGl2RenderingContext::DYNAMIC_DRAW,
                BufferData::VecData(vertices.as_ref()),
            )
            // Unbind the buffer
            .unbind();

        SmallFieldOfView {
            vertices,

            vertex_array_object,
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
            (self.vertices.len() as i32) / 12,
        );
    }

    fn update(&mut self, buffer: &BufferTiles, current_depth: u8, tiles_fov: &BTreeSet<HEALPixCell>, viewport: &ViewPort) {
        let num_tiles = tiles_fov.len();
        let mut vertices = Vec::with_capacity(10 * 6 * num_tiles);

        if current_depth <= 2 {
            for idx in 0..192 {
                let current_tile = Tile::new(HEALPixCell(2, idx));

                let current_idx = idx >> (2*(2 - current_depth));
                let tile = HEALPixCell(current_depth, current_idx);
                if let Some(tile_buffer) = buffer.get(&tile) {
                    let blending_factor = tile_buffer.blending_factor();
                    let idx_texture_end = tile_buffer.texture_idx;

                    let (uv_start, idx_texture_start) = if blending_factor == 1_f32 {
                        let uv_start = [Vector2::new(0_f32, 0_f32); 4];
                        let idx_texture_start = 0;

                        (uv_start, idx_texture_start)
                    } else {
                        let parent_tile_buffer = get_nearest_parent(&tile, &buffer);

                        let uv_start = get_uv_in_parent(&current_tile, &parent_tile_buffer);
                        let idx_texture_start = parent_tile_buffer.texture_idx;

                        (uv_start, idx_texture_start)
                    };

                    let uv_end = if blending_factor == 0_f32 {
                        [Vector2::new(0_f32, 0_f32); 4]
                    } else {
                        get_uv_in_parent(&current_tile, tile_buffer)
                    };

                    let mut vertex_array = Vec::with_capacity(10 * 6);
                    add_vertices_grid(&mut vertex_array,
                        2,
                        idx,
                        1,
                        &uv_start, &uv_end,
                        idx_texture_start, idx_texture_end,
                        blending_factor
                    );

                    vertices.extend(vertex_array.iter());
                } else {
                    let blending_factor = 0_f32;
                    let uv_end = [Vector2::new(0_f32, 0_f32); 4];
                    let idx_texture_end = 0;

                    let parent_tile_buffer = get_nearest_parent(&tile, &buffer);

                    let uv_start = get_uv_in_parent(&current_tile, &parent_tile_buffer);
                    let idx_texture_start = parent_tile_buffer.texture_idx;
                    
                    let mut vertex_array = Vec::with_capacity(10 * 6);
                    add_vertices_grid(&mut vertex_array,
                        2, idx, 1,
                        &uv_start, &uv_end,
                        idx_texture_start, idx_texture_end,
                        blending_factor
                    );

                    vertices.extend(vertex_array.iter());
                }
            }
            // Update the buffers
            self.vertex_array_object.bind()
                .update_array(0, BufferData::VecData(&vertices));

            self.vertices = vertices;
            return;
        }

        if viewport.last_zoom_action == LastZoomAction::Zoom || viewport.last_action == LastAction::Moving {
            for tile in tiles_fov.iter() {
                // If the tile is not already processed
                let (depth, idx) = (tile.0, tile.1);
                let parent_tile_buffer = get_nearest_parent(tile, &buffer);

                let uv_start = get_uv_in_parent(&Tile::new(*tile), &parent_tile_buffer);
                let idx_texture_start = parent_tile_buffer.texture_idx;

                let mut blending_factor = 0_f32;
                let mut idx_texture_end = 0;
                let uv_end = if let Some(tile_buffer) = buffer.get(tile) {
                    let uv_end = get_uv(tile_buffer);

                    idx_texture_end = tile_buffer.texture_idx;
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
                    idx_texture_start, idx_texture_end,
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
                        let uv_end = get_uv(tile_buffer);
                        let uv_start = [Vector2::new(0_f32, 0_f32); 4];

                        let mut vertex_array = Vec::with_capacity(10 * 6);

                        let idx_texture_start = 0;
                        let idx_texture_end = tile_buffer.texture_idx;
                        add_vertices_grid(
                            &mut vertex_array,
                            depth, idx,
                            4,
                            &uv_start, &uv_end,
                            idx_texture_start, idx_texture_end,
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
                                let uv_end = get_uv_in_parent(child_tile_buffer, tile_buffer);
                                let idx_texture_end = tile_buffer.texture_idx;

                                let uv_start = get_uv(child_tile_buffer);
                                let idx_texture_start = child_tile_buffer.texture_idx;

                                let mut vertex_array = Vec::with_capacity(10 * 6);
                                add_vertices_grid(&mut vertex_array,
                                    child_depth,
                                    child_idx,
                                    1 << (depth + 2 - child_depth),
                                    &uv_start, &uv_end,
                                    idx_texture_start, idx_texture_end,
                                    blending_factor);

                                // tile has been found in the buffer, we will render it
                                vertices.extend(vertex_array.iter());
                            } else {
                                if child_depth == depth + 2 {
                                    // The grand children is not in the buffer

                                    let tile_child = Tile::new(HEALPixCell(child_depth, child_idx));

                                    // Find in which base cell the current tile is located
                                    let tile_buffer_parent = get_nearest_parent(&tile, &buffer);

                                    let uv_start = get_uv_in_parent(&tile_child, &tile_buffer_parent);
                                    let idx_texture_start = tile_buffer_parent.texture_idx;

                                    // Find in which position the child tile is located in the current fov tile
                                    let uv_end = get_uv_in_parent(&tile_child, tile_buffer);
                                    let idx_texture_end = tile_buffer.texture_idx;

                                    let mut vertex_array = Vec::with_capacity(10 * 6);

                                    add_vertices_grid(&mut vertex_array,
                                        child_depth,
                                        child_idx,
                                        1,
                                        &uv_start, &uv_end,
                                        idx_texture_start, idx_texture_end,
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
                            let idx_texture_end = 0;

                            let uv_start = get_uv(child_tile_buffer);
                            let idx_texture_start = child_tile_buffer.texture_idx;

                            let mut vertex_array = Vec::with_capacity(10 * 6);

                            add_vertices_grid(&mut vertex_array,
                                child_depth,
                                child_idx,
                                1 << (depth + 2 - child_depth),
                                &uv_start,
                                &uv_end,
                                idx_texture_start,
                                idx_texture_end,
                                blending_factor
                            );

                            // tile has been found in the buffer, we will render it
                            vertices.extend(vertex_array.iter());
                        } else {
                            if child_depth == depth + 2 {
                                // The grand children is not in the buffer
                                let tile_child = Tile::new(HEALPixCell(child_depth, child_idx));

                                // Find in which base cell the child tile is located
                                let tile_buffer_parent = get_nearest_parent(&tile, &buffer);

                                let uv_start = get_uv_in_parent(&tile_child, &tile_buffer_parent);
                                let idx_texture_start = tile_buffer_parent.texture_idx;

                                // Find in which position the child tile is located in the current fov tile
                                let uv_end = [Vector2::new(0_f32, 0_f32); 4];
                                let idx_texture_end = 0;

                                let mut vertex_array = Vec::with_capacity(10 * 6);
                                add_vertices_grid(&mut vertex_array,
                                    child_depth,
                                    child_idx,
                                    1,
                                    &uv_start,
                                    &uv_end,
                                    idx_texture_start,
                                    idx_texture_end,
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

        //console::log_1(&format!("num vertices {:?}", vertices.len()).into());
        // Update the buffers
        self.vertex_array_object.bind()
            .update_array(0, BufferData::VecData(&vertices));

        self.vertices = vertices;
    }
}

pub struct PerPixel<P> where P: Projection {
    pub vertices: Vec<f32>,
    pub idx: Vec<u16>,

    vertex_array_object: VertexArrayObject,

    projection: std::marker::PhantomData<P>
}

impl<P> PerPixel<P> where P: Projection {
    fn create_vertices_array(gl: &WebGl2Context, viewport: &ViewPort) -> Vec<f32> {
        let vertex_screen_space_positions = <P>::build_screen_map(viewport);

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

        vertices_data
    }

    fn create_index_array() -> Option<Vec<u16>> {
        let mut indices = Vec::with_capacity(3 * NUM_VERTICES_PER_STEP * NUM_STEPS);

        for j in 0..NUM_STEPS {
            if j == 0 {
                for i in 1..NUM_VERTICES_PER_STEP {
                    indices.push(0 as u16);
                    indices.push((i + 1) as u16);
                    indices.push(i as u16);
                }
                
                indices.push(0 as u16);
                indices.push(1 as u16);
                indices.push(NUM_VERTICES_PER_STEP as u16);
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
}

impl<P> RenderingMode for PerPixel<P> where P: Projection {
    fn new(gl: &WebGl2Context, viewport: &ViewPort) -> PerPixel<P> {
        let vertices = Self::create_vertices_array(gl, viewport);
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

        PerPixel::<P> {
            vertices,
            idx,

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

        // Send current depth
        let location_current_depth = shader.get_uniform_location("current_depth");
        gl.uniform1i(location_current_depth, DEPTH.load(Ordering::Relaxed) as i32);
        // Send max depth of the current HiPS
        let location_max_depth = shader.get_uniform_location("max_depth");
        gl.uniform1i(location_max_depth, MAX_DEPTH.load(Ordering::Relaxed) as i32);

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

    fn update(&mut self, buffer: &BufferTiles, current_depth: u8, tiles_fov: &BTreeSet<HEALPixCell>, viewport: &ViewPort) {}
}

use crate::projection::*;
pub struct HiPSSphere {
    buffer: Rc<RefCell<BufferTiles>>,

    ortho: SmallFieldOfView,
    aitoff_perpixel: PerPixel<Aitoff>,
    moll_perpixel: PerPixel<MollWeide>,

    gl: WebGl2Context,
}

use cgmath::Deg;

impl HiPSSphere {
    pub fn new(gl: &WebGl2Context, viewport: &ViewPort) -> HiPSSphere {
        let buffer = Rc::new(RefCell::new(BufferTiles::new(gl)));
        load_base_tiles(gl, buffer.clone());

        //let fov_rendering_mode = SmallFieldOfViewRenderingMode::new::<P>(gl, viewport, buffer.clone());

        let gl = gl.clone();
        //let fov_mode = false;
        //let fov_mode = true;

        let ortho = SmallFieldOfView::new(&gl, &viewport);
        let aitoff_perpixel = PerPixel::<Aitoff>::new(&gl, &viewport);
        let moll_perpixel = PerPixel::<MollWeide>::new(&gl, &viewport);

        HiPSSphere {
            buffer: buffer,

            ortho,
            aitoff_perpixel,
            moll_perpixel,

            gl,
        }
    }

    fn send_global_uniforms<T: Mesh + DisableDrawing>(&self, gl: &WebGl2Context, shader: &Shader, viewport: &ViewPort, renderable: &Renderable<T>) {
        // TEXTURES TILES BUFFER
        self.buffer.borrow().send_to_shader(shader);
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
    }
    
    /// Called when the HiPS has been changed
    pub fn refresh_buffer_tiles(&mut self) {
        console::log_1(&format!("refresh buffers").into());
        load_base_tiles(&self.gl, self.buffer.clone());

        self.buffer.borrow_mut()
            .clear();
    }

    pub fn get_buffer(&self) -> Rc<RefCell<BufferTiles>> {
        self.buffer.clone()
    }

    pub fn update<P: Projection>(&mut self, viewport: &ViewPort) {
        let field_of_view = viewport.field_of_view();

        let tiles_fov = field_of_view.healpix_cells();
        let current_depth = field_of_view.current_depth();

        let prev_depth = DEPTH.load(Ordering::Relaxed);
        DEPTH.store(current_depth, Ordering::Relaxed);

        let depth_changed = current_depth != prev_depth;
        load_tiles(&self.gl, self.buffer.clone(), tiles_fov, current_depth, depth_changed);

        match P::name() {
            "Orthographic" => {
                // Ortho mode
                self.ortho.update(&self.buffer.borrow(),
                    current_depth,
                    tiles_fov,
                    viewport
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
        //let field_of_view = viewport.field_of_view();

        // Big field of view, check the projections
        match P::name() {
            "Aitoff" => {
                let shader = PerPixel::<Aitoff>::get_shader(shaders); // TODO: The same shader for all projection
                shader.bind(gl);

                self.send_global_uniforms(gl, shader, viewport, renderable);
                self.aitoff_perpixel.draw(gl, shader)
            },
            "MollWeide" => {
                let shader = PerPixel::<MollWeide>::get_shader(shaders); // TODO: The same shader for all projection
                shader.bind(gl);

                self.send_global_uniforms(gl, shader, viewport, renderable);
                self.moll_perpixel.draw(gl, shader)
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
use cgmath::Matrix4;

use crate::utils;
use crate::texture::{Tile, TilePerPixelGPU};
use std::collections::BTreeSet;
use std::collections::VecDeque;

use crate::field_of_view::HEALPixCell;
use crate::viewport::{LastZoomAction, LastAction};

fn get_uv_in_parent(tile: &Tile, parent_tile: &Tile) -> [Vector2<f32>; 4] {
    let texture_parent_idx = (parent_tile.texture_idx as i32) % 64;

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
    let texture_idx = (tile.texture_idx as i32) % 64;
        
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

    buffer.get(&HEALPixCell(parent_depth, parent_idx))
}

fn get_root_parent(tile: &HEALPixCell) -> Tile {
    let (depth, idx) = (tile.0, tile.1);
    assert!(depth >= 0);

    let parent_idx = idx >> (2*depth);

    Tile::new(HEALPixCell(0, parent_idx))
}

// Get the nearest parent tile found in the buffer of `tile`
fn get_nearest_parent<'a>(tile: &HEALPixCell, buffer: &'a BufferTiles) -> Tile {
    let (mut depth, mut idx) = ((tile.0 as i8) - 1, tile.1 >> 2);

    while depth > 0 {
        let current_tile = &HEALPixCell(depth as u8, idx);
        if let Some(parent_tile) = buffer.get(current_tile) {
            return parent_tile.clone();
        }

        depth -= 1;
        idx = idx >> 2;
    }

    // Depth equals to 0, it is thus a base tile and we are sure it is located in the buffer
    get_root_parent(tile)
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