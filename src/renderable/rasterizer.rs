use crate::healpix_cell::SphereSubdivided;
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
    #[inline]
    fn size_of_float() -> usize {
        std::mem::size_of::<Self>() / std::mem::size_of::<f32>()
    }

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
        //assert!(off + 12 <= 30000);
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

use cgmath::Rad;
use crate::math;
use std::mem;

use crate::renderable::uv::{TileUVW, TileCorner};
use crate::projection::Projection;
use crate::healpix_cell::HEALPixCell;
use crate::viewport::ViewPort;
fn add_cell_vertices<P: Projection, E: UpdateTextureBufferEvent>(
    sphere_sub: &SphereSubdivided,
    vertices: &mut [f32],
    idx_vertices: &mut [u16],
    num_vertices: &mut usize,
    num_idx: &mut u16,
    cell: &HEALPixCell,
    uv_0: &TileUVW,
    uv_1: &TileUVW,
    alpha: f32,
    viewport: &ViewPort,
) {
    let num_subdivision = E::num_subdivision::<P>(cell, sphere_sub, viewport);
    add_vertices_grid(
        vertices,
        idx_vertices,
        num_vertices,
        num_idx,
        cell,
        num_subdivision,
        uv_0,
        uv_1,
        alpha,
    );
}

fn add_vertices_grid(
    vertices: &mut [f32],
    idx_vertices: &mut [u16],
    num_vertices: &mut usize,
    num_idx: &mut u16,
    cell: &HEALPixCell,
    num_subdivision: u8,
    uv_0: &TileUVW,
    uv_1: &TileUVW,
    alpha: f32
) {
    let n_segments: u16 = 1_u16 << num_subdivision;
    let lonlat = healpix::nested::grid(cell.0, cell.1, n_segments);

    let n_vertices_per_segment = n_segments + 1;
    let n_vertices_per_patch = (n_vertices_per_segment as usize) * (n_vertices_per_segment as usize);
    if vertices.len() < 12*((*num_vertices) + n_vertices_per_patch*12) {
        return;
    }

    let off_idx_vertices = *num_vertices as u16;
    for i in 0..n_vertices_per_segment {
        for j in 0..n_vertices_per_segment {
            let id_vertex_0 = (j + i * n_vertices_per_segment) as usize;

            let hj0 = (j as f32) / (n_segments as f32);
            let hi0 = (i as f32) / (n_segments as f32);

            let d01s = uv_0[TileCorner::BottomRight].x - uv_0[TileCorner::BottomLeft].x;
            let d02s = uv_0[TileCorner::TopLeft].y - uv_0[TileCorner::BottomLeft].y;

            let uv_s_vertex_0 = Vector3::new(
                uv_0[TileCorner::BottomLeft].x + hj0 * d01s,
                uv_0[TileCorner::BottomLeft].y + hi0 * d02s,
                uv_0[TileCorner::BottomLeft].z
            );

            let d01e = uv_1[TileCorner::BottomRight].x - uv_1[TileCorner::BottomLeft].x;
            let d02e = uv_1[TileCorner::TopLeft].y - uv_1[TileCorner::BottomLeft].y;
            let uv_e_vertex_0 = Vector3::new(
                uv_1[TileCorner::BottomLeft].x + hj0 * d01e,
                uv_1[TileCorner::BottomLeft].y + hi0 * d02e,
                uv_1[TileCorner::BottomLeft].z
            );

            Vertex::new(&lonlat[id_vertex_0], uv_s_vertex_0, uv_e_vertex_0, alpha)
                .add_to_vertices(vertices, 12 * (*num_vertices));
            *num_vertices += 1;
        }
    }

    let mut k = *num_idx as usize;
    for i in 0..n_segments {
        for j in 0..n_segments {
            let idx_0 = (j + i * n_vertices_per_segment) as u16;
            let idx_1 = (j + 1 + i * n_vertices_per_segment) as u16;
            let idx_2 = (j + (i + 1) * n_vertices_per_segment) as u16;
            let idx_3 = (j + 1 + (i + 1) * n_vertices_per_segment) as u16;

            idx_vertices[k] = off_idx_vertices + idx_0;
            idx_vertices[k + 1] = off_idx_vertices + idx_1;
            idx_vertices[k + 2] = off_idx_vertices + idx_2;

            idx_vertices[k + 3] = off_idx_vertices + idx_1;
            idx_vertices[k + 4] = off_idx_vertices + idx_3;
            idx_vertices[k + 5] = off_idx_vertices + idx_2;
            k += 6;
        }
    }
    *num_idx = k as u16;
}

use crate::event_manager::Event;
use crate::buffer_tiles::BufferTiles;
trait UpdateTextureBufferEvent: Event {
    // Returns:
    // * The UV of the starting tile in the global 4096x4096 texture
    // * The UV of the ending tile in the global 4096x4096 texture
    // * the blending factor between the two tiles in the texture
    fn update_texture_buffer<P: Projection>(
        sphere_sub: &SphereSubdivided,
        // The VBO data to fill
        vertices: &mut [f32],
        idx_vertices: &mut [u16],
        num_vertices: &mut usize,
        num_idx: &mut u16,
        // The buffer that will be modified due to the need of specific tile textures by the GPU
        buffer: &mut BufferTiles,
        // The HEALPix cells located in the FOV
        viewport: &ViewPort,
    );

    fn num_subdivision<P: Projection>(cell: &HEALPixCell, sphere_sub: &SphereSubdivided, viewport: &ViewPort) -> u8;
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
    fn update_texture_buffer<P: Projection>(
        sphere_sub: &SphereSubdivided,
        // The VBO data to fill
        vertices: &mut [f32],
        idx_vertices: &mut [u16],
        num_vertices: &mut usize,
        num_idx: &mut u16,
        // The buffer that will be modified due to the need of specific tile textures by the GPU
        buffer: &mut BufferTiles,
        // The HEALPix cells located in the FOV
        viewport: &ViewPort
    ) {
        let cells_fov = viewport.field_of_view()
            .healpix_cells();
        let depth = viewport.field_of_view()
            .current_depth();

        for cell in cells_fov {
            let (uv_0, uv_1, time_received) = if buffer.contains(cell) {
                let parent_cell = buffer.get_nearest_parent(cell);

                let cell_in_tex = buffer.get_cell_in_texture(cell);
                let parent_cell_in_tex = buffer.get_cell_in_texture(&parent_cell);

                let uv_0 = TileUVW::new(&cell, &parent_cell_in_tex);
                let uv_1 = TileUVW::new(cell, &cell_in_tex);

                let time_received = cell_in_tex.time_received;
                (uv_0, uv_1, time_received)
            } else {
                let parent_cell = buffer.get_nearest_parent(cell);
                let grand_parent_cell = buffer.get_nearest_parent(&parent_cell);

                let parent_cell_in_tex = buffer.get_cell_in_texture(&parent_cell);
                let grand_parent_cell_in_tex = buffer.get_cell_in_texture(&grand_parent_cell);

                let uv_0 = TileUVW::new(&cell, &grand_parent_cell_in_tex);
                let uv_1 = TileUVW::new(&cell, &parent_cell_in_tex);

                let time_received = parent_cell_in_tex.time_received;
                (uv_0, uv_1, time_received)
            };

            add_cell_vertices::<P, Self>(
                sphere_sub,
                vertices,
                idx_vertices,
                num_vertices,
                num_idx,
                &cell,
                &uv_0, &uv_1,
                time_received,
                viewport,
            );
        }
    }
    fn num_subdivision<P: Projection>(cell: &HEALPixCell, sphere_sub: &SphereSubdivided, viewport: &ViewPort) -> u8 {
        sphere_sub.get_num_subdivide::<P>(cell, viewport, cell.depth())
    }
}

impl UpdateTextureBufferEvent for MouseWheelUp {
    // Returns:
    // * The UV of the starting tile in the global 4096x4096 texture
    // * The UV of the ending tile in the global 4096x4096 texture
    // * the blending factor between the two tiles in the texture
    fn update_texture_buffer<P: Projection>(
        sphere_sub: &SphereSubdivided,
        // The VBO data to fill
        vertices: &mut [f32],
        idx_vertices: &mut [u16],
        num_vertices: &mut usize,
        num_idx: &mut u16,
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

                let uv_0 = TileUVW::new(&cell, &parent_cell_in_tex);
                let uv_1 = TileUVW::new(cell, &cell_in_tex);

                let time_received = cell_in_tex.time_received;
                (uv_0, uv_1, time_received)
            } else {
                let parent_cell = buffer.get_nearest_parent(cell);
                let grand_parent_cell = buffer.get_nearest_parent(&parent_cell);

                let parent_cell_in_tex = buffer.get_cell_in_texture(&parent_cell);
                let grand_parent_cell_in_tex = buffer.get_cell_in_texture(&grand_parent_cell);

                let uv_0 = TileUVW::new(&cell, &grand_parent_cell_in_tex);
                let uv_1 = TileUVW::new(&cell, &parent_cell_in_tex);

                let time_received = parent_cell_in_tex.time_received;
                (uv_0, uv_1, time_received)
            };

            add_cell_vertices::<P, Self>(
                sphere_sub,
                vertices,
                idx_vertices,
                num_vertices,
                num_idx,
                &cell,
                &uv_0, &uv_1,
                time_received,
                viewport,
            );
        }
    }

    fn num_subdivision<P: Projection>(cell: &HEALPixCell, sphere_sub: &SphereSubdivided, viewport: &ViewPort) -> u8 {
        sphere_sub.get_num_subdivide::<P>(cell, viewport, cell.depth())
    }
}

impl UpdateTextureBufferEvent for MouseWheelDown {
    // Returns:
    // * The UV of the starting tile in the global 4096x4096 texture
    // * The UV of the ending tile in the global 4096x4096 texture
    // * the blending factor between the two tiles in the texture
    fn update_texture_buffer<P: Projection>(
        sphere_sub: &SphereSubdivided,
        // The VBO data to fill
        vertices: &mut [f32],
        idx_vertices: &mut [u16],
        num_vertices: &mut usize,
        num_idx: &mut u16,
        // The buffer that will be modified due to the need of specific tile textures by the GPU
        buffer: &mut BufferTiles,
        // The HEALPix cells located in the FOV
        viewport: &ViewPort,
    ) {
        let depth_plus_two = viewport.field_of_view()
            .current_depth() + 2;

        let cells_fov = viewport.field_of_view()
            .get_cells_in_fov(depth_plus_two);

        for cell in cells_fov {
            let parent_cell = cell.parent();
            let grand_parent_cell = parent_cell.parent();

            let (uv_0, uv_1, time_received) = if buffer.contains(&grand_parent_cell) {
                let starting_cell = if buffer.contains(&cell) {
                    cell
                } else {
                    buffer.get_nearest_parent(&cell)
                };
                let starting_cell_in_tex = buffer.get_cell_in_texture(&starting_cell);
                let uv_0 = TileUVW::new(&cell, &starting_cell_in_tex);

                let grand_parent_cell_in_tex = buffer.get_cell_in_texture(&grand_parent_cell);
                let uv_1 = TileUVW::new(&cell, &grand_parent_cell_in_tex);

                let time_received = grand_parent_cell_in_tex.time_received;
                (uv_0, uv_1, time_received)
            } else {

                let ending_cell = if buffer.contains(&cell) {
                    cell
                } else {
                    buffer.get_nearest_parent(&cell)
                };

                let starting_cell = buffer.get_nearest_parent(&ending_cell);

                let starting_cell_in_tex = buffer.get_cell_in_texture(&starting_cell);
                let ending_cell_in_tex = buffer.get_cell_in_texture(&ending_cell);

                let time_received = ending_cell_in_tex.time_received;

                let uv_0 = TileUVW::new(&cell, &starting_cell_in_tex);
                let uv_1 = TileUVW::new(&cell, &ending_cell_in_tex);
                (uv_0, uv_1, time_received)
            };

            add_cell_vertices::<P, Self>(
                sphere_sub,
                vertices,
                idx_vertices,
                num_vertices,
                num_idx,
                &cell,
                &uv_0, &uv_1,
                time_received,
                viewport
            );
        }
    }

    fn num_subdivision<P: Projection>(cell: &HEALPixCell, sphere_sub: &SphereSubdivided, viewport: &ViewPort) -> u8 {
        let num_subdivision = sphere_sub.get_num_subdivide::<P>(cell, viewport, cell.depth());
        if num_subdivision <= 2 {
            0
        } else {
            num_subdivision - 2
        }
    }
}

use crate::projection::*;
pub trait RasterizerProjection {
    fn get_rasterize_shader(shaders: &ShaderManager) -> &Shader;
}

impl RasterizerProjection for Aitoff {
    fn get_rasterize_shader(shaders: &ShaderManager) -> &Shader {
        shaders.get::<shaders::Rasterize_Ortho>().unwrap()
    }
}
impl RasterizerProjection for Mollweide {
    fn get_rasterize_shader(shaders: &ShaderManager) -> &Shader {
        shaders.get::<shaders::Rasterize_Ortho>().unwrap()
    }
}
impl RasterizerProjection for AzimutalEquidistant {
    fn get_rasterize_shader(shaders: &ShaderManager) -> &Shader {
        shaders.get::<shaders::Rasterize_Ortho>().unwrap()
    }
}
impl RasterizerProjection for Mercator {
    fn get_rasterize_shader(shaders: &ShaderManager) -> &Shader {
        shaders.get::<shaders::Rasterize_Ortho>().unwrap()
    }
}
impl RasterizerProjection for Orthographic {
    fn get_rasterize_shader(shaders: &ShaderManager) -> &Shader {
        shaders.get::<shaders::Rasterize_Ortho>().unwrap()
    }
}

use crate::core::VertexArrayObject;
pub struct Rasterizer {
    vertices: [f32; 60000],
    idx_vertices: [u16; 20000],
    //num_vertices: usize,
    num_idx: u16,

    sphere_sub: SphereSubdivided,

    vertex_array_object: VertexArrayObject,
}
impl Rasterizer {
    fn define_needed_hpx_cells<T: UpdateTextureBufferEvent, P: Projection>(
        &mut self,
        // The buffer that will be modified due to the need of specific tile textures by the GPU
        buffer: &mut BufferTiles,
        // The HEALPix cells at the depth
        viewport: &ViewPort) {
        // Refill the vertices slice
        // Set its current index to 0
        let mut num_vertices = 0;
        self.num_idx = 0;
        T::update_texture_buffer::<P>(&self.sphere_sub, &mut self.vertices, &mut self.idx_vertices, &mut num_vertices, &mut self.num_idx, buffer, viewport);
    }

    // The rasterizer has several shaders, one for each projection
    pub fn get_shader<P: Projection>(shaders: &ShaderManager) -> &Shader {
        P::get_rasterize_shader(shaders)
    }
}

use crate::shader::Shader;
use crate::WebGl2Context;
use crate::renderable::RenderingMode;
use web_sys::{WebGl2RenderingContext, console};
use crate::core::BufferData;
use crate::viewport::LastAction;
use crate::event_manager::EventManager;

use crate::shader::ShaderManager;
use crate::shaders;
impl RenderingMode for Rasterizer {
    fn new(gl: &WebGl2Context, viewport: &ViewPort, shaders: &mut ShaderManager) -> Rasterizer {
        // Define rasterization new shaders reponsible for rendering the HiPS
        let uniforms_raster = vec![
            // General uniforms
            "current_time",
            "model",
            // Viewport uniforms
            "ndc_to_clip",
            "clip_zoom_factor",
            "aspect",
            "last_zoom_action",
            // HiPS Ortho specific uniforms
            "current_depth",
            "max_depth",
            // Textures
            "tex",
            "num_tiles",
        ];
        shaders.insert::<shaders::Rasterize_Ortho>(gl, &uniforms_raster[..]);

        // Define the Vertex Array Object where vertices data will be put
        // Memory reserved from the stack
        let vertices = [0_f32; 60000];
        let idx_vertices = [0_u16; 20000];
        let mut vertex_array_object = VertexArrayObject::new(gl);

        let shader = shaders.get::<shaders::Rasterize_Ortho>().unwrap();
        shader.bind(gl);

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
            // Set the element buffer
            .add_element_buffer(
                WebGl2RenderingContext::DYNAMIC_DRAW,
                BufferData::SliceData(&idx_vertices),
            )
            // Unbind the buffer
            .unbind();

        let sphere_sub = SphereSubdivided::new();
        Rasterizer {
            vertices,
            idx_vertices,

            num_idx: idx_vertices.len() as u16,
            sphere_sub,

            vertex_array_object,
        }
    }

    fn draw<P: Projection>(&self, gl: &WebGl2Context, shader: &Shader) {
        self.vertex_array_object.bind_ref();
        gl.draw_elements_with_i32(
            //WebGl2RenderingContext::LINES,
            WebGl2RenderingContext::TRIANGLES,
            //self.vertex_array_object.num_elements() as i32,
            self.num_idx as i32,
            WebGl2RenderingContext::UNSIGNED_SHORT,
            0,
        );
    }

    fn update<P: Projection>(&mut self, buffer: &mut BufferTiles, viewport: &ViewPort, events: &EventManager) {
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
                LastAction::Unzooming => self.define_needed_hpx_cells::<MouseWheelDown, P>(buffer, viewport),
                LastAction::Zooming => self.define_needed_hpx_cells::<MouseWheelUp, P>(buffer, viewport),
                LastAction::Moving => self.define_needed_hpx_cells::<MouseMove, P>(buffer, viewport)
            };
            buffer.signals_end_frame();

            // Update the VAO
            self.vertex_array_object.bind()
                .update_array(
                    0, 
                    WebGl2RenderingContext::DYNAMIC_DRAW,
                    BufferData::SliceData(&self.vertices)
                )
                .update_element_array(
                    WebGl2RenderingContext::DYNAMIC_DRAW,
                    BufferData::SliceData(&self.idx_vertices)
                );
        }
        //console::log_1(&format!("poll").into());
        buffer.poll_textures();
    }

    fn send_to_shader(buffer: &BufferTiles, shader: &Shader) {
        buffer.send_texture_to_shader(shader);
    }
}