use web_sys::console;

use web_sys::WebGl2RenderingContext;

use crate::renderable::Mesh;
use crate::shader::Shader;

pub const NUM_VERTICES_PER_STEP: usize = 50;
pub const NUM_STEPS: usize = 20;

use crate::buffer_tiles::BufferTiles;

use std::sync::Arc;
use std::sync::atomic::AtomicU8;
lazy_static! {
    pub static ref DEPTH: Arc<AtomicU8> = Arc::new(AtomicU8::new(0));
}

use crate::core::{
 BufferData,
 VertexArrayObject
};

use crate::viewport::ViewPort;
use cgmath::Vector2;
use crate::WebGl2Context;

use crate::projection::Projection;
use crate::event_manager::EventManager;
pub trait RenderingMode {
    fn new(gl: &WebGl2Context, viewport: &ViewPort) -> Self;

    fn update(
        &mut self,
        buffer: &mut BufferTiles,
        viewport: &ViewPort,
        events: &EventManager
    );

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

    uv_0: Vector3<f32>,
    uv_1: Vector3<f32>,

    time_received: f32,
}

impl Vertex {
    fn new(
        lonlat: &(f64, f64),
        uv_0: Vector3<f32>,
        uv_1: Vector3<f32>,
        time_received: f32
    ) -> Vertex {
        let (lon, lat) = (lonlat.0 as f32, lonlat.1 as f32);

        let pos = math::radec_to_xyz(Rad(lon), Rad(lat));
        Vertex {
            lon,
            lat,

            pos,

            uv_0,
            uv_1,

            time_received,
        }
    }

    fn add_to_vertices(&self, vertices: &mut [f32], off: usize) {
        assert!(off + 12 <= 60000);
        vertices[off] = self.lon;
        vertices[off+1] = self.lat;

        vertices[off+2] = self.pos.x;
        vertices[off+3] = self.pos.y;
        vertices[off+4] = self.pos.z;

        vertices[off+5] = self.uv_0.x;
        vertices[off+6] = self.uv_0.y;
        vertices[off+7] = self.uv_0.z;

        vertices[off+8] = self.uv_1.x;
        vertices[off+9] = self.uv_1.y;
        vertices[off+10] = self.uv_1.z;

        vertices[off+11] = self.time_received;
    }
}

#[repr(C)]
// One tile contains 2 triangles of 3 vertices each
struct TileVertices([Vertex; 6]);

pub struct SmallFieldOfView {
    vertices: [f32; 60000],

    num_vertices: usize,
    //num_tiles: usize,

    vertex_array_object: VertexArrayObject,
}

use cgmath::Rad;
use crate::math;
use std::mem;

use crate::renderable::uv::{TileUVW, TileCorner};
fn add_vertices_grid(
    vertices: &mut [f32],
    num_vertices: &mut usize,
    cell: &HEALPixCell,
    n_segments: u16,
    uv_0: &TileUVW,
    uv_1: &TileUVW,
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

            let d01s = uv_0[TileCorner::BottomRight].x - uv_0[TileCorner::BottomLeft].x;
            let d02s = uv_0[TileCorner::TopLeft].y - uv_0[TileCorner::BottomLeft].y;

            let uv_s_vertex_0 = Vector3::new(
                uv_0[TileCorner::BottomLeft].x + hj0 * d01s,
                uv_0[TileCorner::BottomLeft].y + hi0 * d02s,
                uv_0[TileCorner::BottomLeft].z
            );
            let uv_s_vertex_1 = Vector3::new(
                uv_0[TileCorner::BottomLeft].x + hj1 * d01s,
                uv_0[TileCorner::BottomLeft].y + hi1 * d02s,
                uv_0[TileCorner::BottomLeft].z
            );
            let uv_s_vertex_2 = Vector3::new(
                uv_0[TileCorner::BottomLeft].x + hj2 * d01s,
                uv_0[TileCorner::BottomLeft].y + hi2 * d02s,
                uv_0[TileCorner::BottomLeft].z
            );
            let uv_s_vertex_3 = Vector3::new(
                uv_0[TileCorner::BottomLeft].x + hj3 * d01s,
                uv_0[TileCorner::BottomLeft].y + hi3 * d02s,
                uv_0[TileCorner::BottomLeft].z
            );

            let d01e = uv_1[TileCorner::BottomRight].x - uv_1[TileCorner::BottomLeft].x;
            let d02e = uv_1[TileCorner::TopLeft].y - uv_1[TileCorner::BottomLeft].y;
            let uv_e_vertex_0 = Vector3::new(
                uv_1[TileCorner::BottomLeft].x + hj0 * d01e,
                uv_1[TileCorner::BottomLeft].y + hi0 * d02e,
                uv_1[TileCorner::BottomLeft].z
            );
            let uv_e_vertex_1 = Vector3::new(
                uv_1[TileCorner::BottomLeft].x + hj1 * d01e,
                uv_1[TileCorner::BottomLeft].y + hi1 * d02e,
                uv_1[TileCorner::BottomLeft].z
            );
            let uv_e_vertex_2 = Vector3::new(
                uv_1[TileCorner::BottomLeft].x + hj2 * d01e,
                uv_1[TileCorner::BottomLeft].y + hi2 * d02e,
                uv_1[TileCorner::BottomLeft].z
            );
            let uv_e_vertex_3 = Vector3::new(
                uv_1[TileCorner::BottomLeft].x + hj3 * d01e,
                uv_1[TileCorner::BottomLeft].y + hi3 * d02e,
                uv_1[TileCorner::BottomLeft].z
            );
            
            Vertex::new(&lonlat[id_vertex_0], uv_s_vertex_0, uv_e_vertex_0, alpha)
                .add_to_vertices(vertices, 12 * (*num_vertices));
            *num_vertices += 1;

            Vertex::new(&lonlat[id_vertex_1], uv_s_vertex_1, uv_e_vertex_1, alpha)
                .add_to_vertices(vertices, 12 * (*num_vertices));
            *num_vertices += 1;

            Vertex::new(&lonlat[id_vertex_2], uv_s_vertex_2, uv_e_vertex_2, alpha)
                .add_to_vertices(vertices, 12 * (*num_vertices));
            *num_vertices += 1;

            Vertex::new(&lonlat[id_vertex_1], uv_s_vertex_1, uv_e_vertex_1, alpha)
                .add_to_vertices(vertices, 12 * (*num_vertices));
            *num_vertices += 1;

            Vertex::new(&lonlat[id_vertex_3], uv_s_vertex_3, uv_e_vertex_3, alpha)
                .add_to_vertices(vertices, 12 * (*num_vertices));
            *num_vertices += 1;

            Vertex::new(&lonlat[id_vertex_2], uv_s_vertex_2, uv_e_vertex_2, alpha)
                .add_to_vertices(vertices, 12 * (*num_vertices));
            *num_vertices += 1;
        }
    }
}

use crate::event_manager::Event;
trait UpdateTextureBufferEvent: Event {
    // Returns:
    // * The UV of the starting tile in the global 4096x4096 texture
    // * The UV of the ending tile in the global 4096x4096 texture
    // * the blending factor between the two tiles in the texture
    fn update_texture_buffer(
        // The VBO data to fill
        vertices: &mut [f32],
        num_vertices: &mut usize,
        // The buffer that will be modified due to the need of specific tile textures by the GPU
        buffer: &mut BufferTiles,
        // The HEALPix cells located in the FOV
        viewport: &ViewPort,
    );
}

use crate::event_manager::{
 MouseMove,
 MouseWheelDown,
 MouseWheelUp
};

impl UpdateTextureBufferEvent for MouseMove  {
    // Returns:
    // * The UV of the starting tile in the global 4096x4096 texture
    // * The UV of the ending tile in the global 4096x4096 texture
    // * the blending factor between the two tiles in the texture
    fn update_texture_buffer(
        // The VBO data to fill
        vertices: &mut [f32],
        num_vertices: &mut usize,
        // The buffer that will be modified due to the need of specific tile textures by the GPU
        buffer: &mut BufferTiles,
        // The HEALPix cells located in the FOV
        viewport: &ViewPort
    ) {
        let cells_fov = viewport.field_of_view()
            .healpix_cells();
        let depth = viewport.field_of_view()
            .current_depth();

        let num_subdivision = if depth <= 2 {
            1 << (3 - depth)
        } else {
            1
        };
        for cell in cells_fov {
            let (uv_0, uv_1, time_received) = if buffer.contains(cell) {
                let parent_cell = buffer.get_nearest_parent(cell);

                let cell_in_tex = buffer.get_cell_in_texture(cell);
                let parent_cell_in_tex = buffer.get_cell_in_texture(&parent_cell);

                // look_in_parent can lead to a sub_tex_2d call which
                // is quite costly!
                // This checks whether the end of the blending animation is reached
                // and if so, we can forget about moving the parent tile texture to the
                // 4096x4096 texture!
                /*let uv_0 = if utils::get_current_time() - cell_in_tex.tile.time_received > 500_f32 {
                    TileUVW::empty()
                } else {
                    buffer.get_uvw_in_parent(cell, &parent_cell)
                };*/

                let uv_0 = TileUVW::new(&cell, &parent_cell_in_tex);
                let uv_1 = TileUVW::new(cell, &cell_in_tex);

                let time_received = cell_in_tex.time_received;
                (uv_0, uv_1, time_received)
            } else {
                let parent_cell = buffer.get_nearest_parent(cell);
                let grand_parent_cell = buffer.get_nearest_parent(&parent_cell);

                let parent_cell_in_tex = buffer.get_cell_in_texture(&parent_cell);
                let grand_parent_cell_in_tex = buffer.get_cell_in_texture(&grand_parent_cell);

                /*let uv_0 = if utils::get_current_time() - time_received > 500_f32 {
                    TileUVW::empty()
                } else {
                    buffer.get_uvw_in_parent(cell, &grand_parent_cell)
                };*/
                let uv_0 = TileUVW::new(&cell, &grand_parent_cell_in_tex);
                let uv_1 = TileUVW::new(&cell, &parent_cell_in_tex);

                let time_received = parent_cell_in_tex.time_received;
                (uv_0, uv_1, time_received)
            };

            add_vertices_grid(
                vertices,
                num_vertices,
                cell,
                num_subdivision,
                &uv_0, &uv_1,
                time_received
            );
        }
    }
}

impl UpdateTextureBufferEvent for MouseWheelUp {
    // Returns:
    // * The UV of the starting tile in the global 4096x4096 texture
    // * The UV of the ending tile in the global 4096x4096 texture
    // * the blending factor between the two tiles in the texture
    fn update_texture_buffer(
        // The VBO data to fill
        vertices: &mut [f32],
        num_vertices: &mut usize,
        // The buffer that will be modified due to the need of specific tile textures by the GPU
        buffer: &mut BufferTiles,
        // The HEALPix cells located in the FOV
        viewport: &ViewPort,
    ) {
        let cells_fov = viewport.field_of_view()
            .healpix_cells();
        let depth = viewport.field_of_view()
            .current_depth();

        let num_subdivision = if depth <= 2 {
            1 << (3 - depth)
        } else {
            1
        };
        for cell in cells_fov {
            let (uv_0, uv_1, time_received) = if buffer.contains(cell) {
                let parent_cell = buffer.get_nearest_parent(cell);

                let cell_in_tex = buffer.get_cell_in_texture(cell);
                let parent_cell_in_tex = buffer.get_cell_in_texture(&parent_cell);

                // look_in_parent can lead to a sub_tex_2d call which
                // is quite costly!
                // This checks whether the end of the blending animation is reached
                // and if so, we can forget about moving the parent tile texture to the
                // 4096x4096 texture!
                /*let uv_0 = if utils::get_current_time() - cell_in_tex.tile.time_received > 500_f32 {
                    TileUVW::empty()
                } else {
                    buffer.get_uvw_in_parent(cell, &parent_cell)
                };*/

                let uv_0 = TileUVW::new(&cell, &parent_cell_in_tex);
                let uv_1 = TileUVW::new(cell, &cell_in_tex);

                let time_received = cell_in_tex.time_received;
                (uv_0, uv_1, time_received)
            } else {
                let parent_cell = buffer.get_nearest_parent(cell);
                let grand_parent_cell = buffer.get_nearest_parent(&parent_cell);

                let parent_cell_in_tex = buffer.get_cell_in_texture(&parent_cell);
                let grand_parent_cell_in_tex = buffer.get_cell_in_texture(&grand_parent_cell);

                /*let uv_0 = if utils::get_current_time() - time_received > 500_f32 {
                    TileUVW::empty()
                } else {
                    buffer.get_uvw_in_parent(cell, &grand_parent_cell)
                };*/
                let uv_0 = TileUVW::new(&cell, &grand_parent_cell_in_tex);
                let uv_1 = TileUVW::new(&cell, &parent_cell_in_tex);

                let time_received = parent_cell_in_tex.time_received;
                (uv_0, uv_1, time_received)
            };

            add_vertices_grid(
                vertices,
                num_vertices,
                cell,
                num_subdivision,
                &uv_0, &uv_1,
                time_received
            );
        }
    }
}

impl UpdateTextureBufferEvent for MouseWheelDown {
    // Returns:
    // * The UV of the starting tile in the global 4096x4096 texture
    // * The UV of the ending tile in the global 4096x4096 texture
    // * the blending factor between the two tiles in the texture
    fn update_texture_buffer(
        // The VBO data to fill
        vertices: &mut [f32],
        num_vertices: &mut usize,
        // The buffer that will be modified due to the need of specific tile textures by the GPU
        buffer: &mut BufferTiles,
        // The HEALPix cells located in the FOV
        viewport: &ViewPort,
    ) {
        let depth_plus_two = viewport.field_of_view()
            .current_depth() + 2;

        let cells_fov = viewport.field_of_view()
            .get_cells_in_fov(depth_plus_two);

        let num_subdivision = if depth_plus_two <= 2 {
            1 << (3 - depth_plus_two)
        } else {
            1
        };

        for cell in cells_fov {
            let parent_cell = cell.parent();
            let grand_parent_cell = parent_cell.parent();

            if buffer.contains(&grand_parent_cell) {
                //let uv_1 = buffer.get_uvw_in_parent(&cell, &grand_parent_cell);
                //let time_received = buffer.get_time_received(&grand_parent_cell).unwrap();

                let starting_cell = if buffer.contains(&cell) {
                    cell
                } else {
                    buffer.get_nearest_parent(&cell)
                };
                let starting_cell_in_tex = buffer.get_cell_in_texture(&starting_cell);
                let uv_0 = TileUVW::new(&cell, &starting_cell_in_tex);

                /*let uv_0 = if buffer.contains(&parent_cell) {
                    let parent_cell_in_tex = buffer.get_cell_in_texture(&parent_cell);
                    TileUVW::new(&cell, &parent_cell_in_tex)
                } else if buffer.contains(&cell) {
                    let cell_in_tex = buffer.get_cell_in_texture(&cell);
                    TileUVW::new(&cell, &cell_in_tex)
                } else {
                    let starting_cell = buffer.get_nearest_parent(&grand_parent_cell);

                    let starting_cell_in_tex = buffer.get_cell_in_texture(&starting_cell);
                    TileUVW::new(&cell, &starting_cell_in_tex)
                };*/

                let grand_parent_cell_in_tex = buffer.get_cell_in_texture(&grand_parent_cell);
                let uv_1 = TileUVW::new(&cell, &grand_parent_cell_in_tex);

                let time_received = grand_parent_cell_in_tex.time_received;

                add_vertices_grid(
                    vertices,
                    num_vertices,
                    &cell,
                    num_subdivision,
                    &uv_0, &uv_1,
                    time_received
                );
            } else {
                /*let nearest_parent = buffer.get_nearest_parent(&grand_parent_cell);
                let ending_cell = if buffer.contains(&parent_cell) {
                    parent_cell
                } else if buffer.contains(&cell) {
                    let d1 = cell.0 - grand_parent_cell.0;
                    let d2 = grand_parent_cell.0 - nearest_parent.0;

                    //if d1 < d2 {
                        cell
                    //} else {
                    //    nearest_parent
                    //}
                } else {
                    nearest_parent
                };*/
                let ending_cell = if buffer.contains(&cell) {
                    cell
                } else {
                    buffer.get_nearest_parent(&cell)
                };

                /*let starting_cell = if ending_cell.0 == depth_plus_two - 1 && buffer.contains(&cell) {
                    cell
                } else {
                    buffer.get_nearest_parent(&ending_cell)
                };*/
                let starting_cell = buffer.get_nearest_parent(&ending_cell);

                let starting_cell_in_tex = buffer.get_cell_in_texture(&starting_cell);
                let ending_cell_in_tex = buffer.get_cell_in_texture(&ending_cell);

                let time_received = ending_cell_in_tex.time_received;

                let uv_0 = TileUVW::new(&cell, &starting_cell_in_tex);
                let uv_1 = TileUVW::new(&cell, &ending_cell_in_tex);

                add_vertices_grid(
                    vertices,
                    num_vertices,
                    &cell,
                    num_subdivision,
                    &uv_0, &uv_1,
                    time_received
                );
            }
        }
    }
}

impl SmallFieldOfView {
    fn define_needed_hpx_cells<T: UpdateTextureBufferEvent>(
        &mut self,
        // The buffer that will be modified due to the need of specific tile textures by the GPU
        buffer: &mut BufferTiles,
        // The HEALPix cells at the depth
        viewport: &ViewPort) {
        // Refill the vertices slice
        // Set its current index to 0
        self.num_vertices = 0;
        T::update_texture_buffer(&mut self.vertices, &mut self.num_vertices, buffer, viewport);
    }
}

impl RenderingMode for SmallFieldOfView {
    fn new(gl: &WebGl2Context, viewport: &ViewPort) -> SmallFieldOfView {
        // Initialise the buffer of 
        let vertices = [0_f32; 60000];
        let mut vertex_array_object = VertexArrayObject::new(gl);

        // VAO for the orthographic projection and small fovs on 2D projections
        vertex_array_object.bind()
            // Store the projeted and 3D vertex positions in a VBO
            .add_array_buffer(
                12 * mem::size_of::<f32>(),
                &[2, 3, 3, 3, 1],    
                &[
                    0 * mem::size_of::<f32>(),
                    2 * mem::size_of::<f32>(),
                    5 * mem::size_of::<f32>(),
                    8 * mem::size_of::<f32>(),
                    11 * mem::size_of::<f32>(),
                ],
                WebGl2RenderingContext::DYNAMIC_DRAW,
                BufferData::SliceData(&vertices),
            )
            // Unbind the buffer
            .unbind();

        let num_vertices = 5000;
        //let num_tiles = num_vertices / 6;
        /*let tiles = unsafe { 
            mem::transmute::<[f32; 60000], [TileVertices; 1000]>(data)
        };*/
        SmallFieldOfView {
            vertices,

            num_vertices,
            //num_tiles,

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
            self.num_vertices as i32,
        );
    }

    fn update(&mut self, buffer: &mut BufferTiles, viewport: &ViewPort, events: &EventManager) {
        // If at least the base tiles have not been loaded
        // then we do nothing
        if !buffer.is_ready() {
            return;
        }
        // A tile has been received
        if buffer.is_sphere_vbo_rebuild_necessary() {
            console::log_1(&format!("update vbo").into());
            // Signals a new frame to the buffer
            buffer.signals_new_frame();
            let last_user_action = viewport.get_last_action();
            match last_user_action {
                LastAction::Unzooming => self.define_needed_hpx_cells::<MouseWheelDown>(buffer, viewport),
                LastAction::Zooming => self.define_needed_hpx_cells::<MouseWheelUp>(buffer, viewport),
                LastAction::Moving => self.define_needed_hpx_cells::<MouseMove>(buffer, viewport)
            };
            buffer.signals_end_frame();

            // Update the buffers
            self.vertex_array_object.bind()
                .update_array(0, BufferData::SliceData(&self.vertices));
        }
        //console::log_1(&format!("poll").into());
        buffer.poll_textures();
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

    fn update(&mut self, buffer: &mut BufferTiles, viewport: &ViewPort, events: &EventManager) {}

    fn send_to_shader(buffer: &BufferTiles, shader: &Shader) {
    }
}

use crate::projection::*;
pub struct HiPSSphere {
    // Some information about the HiPS
    pub config: HiPSConfig,
    
    // The buffer responsible for: 
    // * Performing the async request of tiles
    // * Storing the most recently asked texture tiles
    // * Sending them to the GPU
    buffer: BufferTiles,

    ortho: SmallFieldOfView,
    aitoff_perpixel: PerPixel<Aitoff>,
    moll_perpixel: PerPixel<MollWeide>,
    arc_perpixel: PerPixel<AzimutalEquidistant>,
    mercator_perpixel: PerPixel<Mercator>,

    gl: WebGl2Context,

    depth: u8,
}

use crate::buffer_tiles::HiPSConfig;
impl HiPSSphere {
    pub fn new(gl: &WebGl2Context, viewport: &ViewPort, config: HiPSConfig) -> HiPSSphere {
        let buffer = BufferTiles::new(gl, &config);

        let gl = gl.clone();

        let ortho = SmallFieldOfView::new(&gl, &viewport);
        let aitoff_perpixel = PerPixel::<Aitoff>::new(&gl, &viewport);
        let moll_perpixel = PerPixel::<MollWeide>::new(&gl, &viewport);
        let arc_perpixel = PerPixel::<AzimutalEquidistant>::new(&gl, &viewport);
        let mercator_perpixel = PerPixel::<Mercator>::new(&gl, &viewport);

        let depth = 0;

        HiPSSphere {
            config,
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

    pub fn set_hips_config<P: Projection>(&mut self, config: HiPSConfig, viewport: &mut ViewPort, events: &EventManager) {        
        // Tell the viewport the config has changed
        viewport.set_max_depth(&config);
        // Clear the buffer
        self.buffer.reset(&config);
        // Erase the old config with the new one
        self.config = config;
    
        self.request_tiles(viewport);
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

        // Send HiPS config
        self.config.send_to_shader(gl, shader);
    }

    pub fn request_tiles(&mut self, viewport: &ViewPort) {
        let field_of_view = viewport.field_of_view();
        let cells = field_of_view.new_healpix_cells();
        
        let depth = field_of_view.current_depth();
        self.depth = depth;

        let depth_changed = depth != self.depth;

        self.buffer.request_tiles(cells, &self.config);
    }

    pub fn update<P: Projection>(&mut self, viewport: &ViewPort, events: &EventManager) {
        match P::name() {
            "Orthographic" => {
                // Ortho mode
                self.ortho.update(
                    &mut self.buffer,
                    viewport,
                    events
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


use crate::buffer_tiles::{NUM_TILES_BY_TEXTURE, NUM_CELLS_BY_TEXTURE_SIDE};

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