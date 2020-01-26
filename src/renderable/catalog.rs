use crate::texture::Texture2D;

use web_sys::WebGlFramebuffer;
use std::collections::{HashSet, BTreeSet};
use crate::field_of_view::HEALPixCell;

use crate::renderable::buffers::vertex_array_object::VertexArrayObject;

pub struct Catalog {
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

    data: Storage,
    cells: HashSet<(u8, u32)>,

    prev_field_of_view: BTreeSet<HEALPixCell>,
    sources: BinaryHeap<Source>,

    vertex_array_object: VertexArrayObject,
}

use cgmath::Rad;
#[derive(Debug)]
#[derive(Clone, PartialEq)]
#[repr(C)]
pub struct Source {
    x: f32,
    y: f32,
    z: f32,

    lon: f32,
    lat: f32,

    mag: f32,
}

impl Eq for Source {}

impl Source {
    pub fn new(lon: Rad<f32>, lat: Rad<f32>, mag: f32) -> Source {
        let world_pos = math::radec_to_xyz(lon, lat);

        let x = world_pos.x;
        let y = world_pos.y;
        let z = world_pos.z;

        let lon = lon.0;
        let lat = lat.0;

        Source {
            x,
            y,
            z,

            lon,
            lat,

            mag
        }
    }
}

use std::cmp::Ordering;
use std::cmp::PartialOrd;
use std::cmp::Ord;
impl PartialOrd for Source {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.mag.partial_cmp(&other.mag)
    }
}
impl Ord for Source {
    fn cmp(&self, other: &Self) -> Ordering {
        self.partial_cmp(&other).unwrap()
    }
}

use cgmath::Deg;
impl From<&[f32]> for Source {
    fn from(data: &[f32]) -> Source {
        let lon = Deg(data[0]).into();
        let lat = Deg(data[1]).into();
        let score = data[2];

        Source::new(lon, lat, score)
    }
}

use web_sys::console;
impl Catalog {
    pub fn new(gl: &WebGl2Context, sources: Vec<Source>) -> Catalog {
        // Build the quadtree from the list of sources
        //console::log_1(&format!("BEGIN quadtree build").into());
        //let quadtree = QuadTree::new(sources);
        //console::log_1(&format!("Quadtree BUILT").into());
        let num_instances = sources.len();
        let data = Storage::new(sources);

        /*let num_instances_max = MAX_SOURCES * 64;
        let mut center_lonlat = vec![0_f32; num_instances_max * 2];
        let mut center_xyz = vec![0_f32; num_instances_max * 3];*/

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
        let kernel_texture = Texture2D::create(gl, "./textures/kernel.png", &[
            (WebGl2RenderingContext::TEXTURE_MIN_FILTER, WebGl2RenderingContext::LINEAR),
            (WebGl2RenderingContext::TEXTURE_MAG_FILTER, WebGl2RenderingContext::LINEAR),
            
            // Prevents s-coordinate wrapping (repeating)
            (WebGl2RenderingContext::TEXTURE_WRAP_S, WebGl2RenderingContext::CLAMP_TO_EDGE),
            // Prevents t-coordinate wrapping (repeating)
            (WebGl2RenderingContext::TEXTURE_WRAP_T, WebGl2RenderingContext::CLAMP_TO_EDGE),
        ]);
        let colormap_texture = Texture2D::create(gl, "./textures/magma_colormap.png", &[
            (WebGl2RenderingContext::TEXTURE_MIN_FILTER, WebGl2RenderingContext::LINEAR),
            (WebGl2RenderingContext::TEXTURE_MAG_FILTER, WebGl2RenderingContext::LINEAR),
            
            // Prevents s-coordinate wrapping (repeating)
            (WebGl2RenderingContext::TEXTURE_WRAP_S, WebGl2RenderingContext::CLAMP_TO_EDGE),
            // Prevents t-coordinate wrapping (repeating)
            (WebGl2RenderingContext::TEXTURE_WRAP_T, WebGl2RenderingContext::CLAMP_TO_EDGE),
        ]);

        // Initialize texture for framebuffer
        let fbo_texture_width = 768;
        let fbo_texture_height = 768;
        let fbo_texture = Texture2D::create_empty(
            gl,
            fbo_texture_width,
            fbo_texture_height,
            &[
                (WebGl2RenderingContext::TEXTURE_MIN_FILTER, WebGl2RenderingContext::LINEAR),
                (WebGl2RenderingContext::TEXTURE_MAG_FILTER, WebGl2RenderingContext::LINEAR),
                
                // Prevents s-coordinate wrapping (repeating)
                (WebGl2RenderingContext::TEXTURE_WRAP_S, WebGl2RenderingContext::CLAMP_TO_EDGE),
                // Prevents t-coordinate wrapping (repeating)
                (WebGl2RenderingContext::TEXTURE_WRAP_T, WebGl2RenderingContext::CLAMP_TO_EDGE),
            ]
        );
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
                BufferData::VecData(
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
                BufferData::VecData(indices.as_ref()),
            )
            // Unbind the buffer
            .unbind();

        let alpha = 0.7_f32;
        let strength = 1_f32;

        let cells = HashSet::new();
        let prev_field_of_view = BTreeSet::new();

        let sources = BinaryHeap::with_capacity(MAX_SOURCES);
        let vertex_array_object = VertexArrayObject::new(gl);

        Catalog {
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

            vao_screen,

            data,
            cells,
            prev_field_of_view,

            sources,

            vertex_array_object
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

use crate::renderable::buffers::buffer_data::BufferData;

use cgmath::Matrix4;

use web_sys::WebGl2RenderingContext;
use crate::WebGl2Context;

use crate::viewport::ViewPort;

use crate::renderable::Renderable;
use crate::utils;
use std::collections::HashMap;
use std::collections::BinaryHeap;

use crate::math;
use crate::projection::Projection; 

impl Mesh for Catalog {
    fn create_buffers(&mut self, gl: &WebGl2Context) {
        self.vertex_array_object.bind()
            // Store the UV and the offsets of the billboard in a VBO
            .add_array_buffer(
                4 * std::mem::size_of::<f32>(),
                &[2, 2],
                &[0 * std::mem::size_of::<f32>(), 2 * std::mem::size_of::<f32>()],
                WebGl2RenderingContext::STATIC_DRAW,
                BufferData::VecData(self.vertices.as_ref()),
            )
            // Store the cartesian position of the center of the source in the a instanced VBO
            .add_instanced_array_buffer(
                std::mem::size_of::<Source>(),
                &[3, 2, 1],
                &[0 * mem::size_of::<f32>(), 3 * mem::size_of::<f32>(), 5 * mem::size_of::<f32>()],
                WebGl2RenderingContext::DYNAMIC_DRAW,
                BufferData::VecData(self.data.sources.as_ref()),
            )
            // Set the element buffer
            .add_element_buffer(
                WebGl2RenderingContext::STATIC_DRAW,
                BufferData::VecData(self.indices.as_ref()),
            )
            // Unbind the buffer
            .unbind();
    }

    fn update<P: Projection>(
        &mut self,
        _local_to_world: &Matrix4<f32>,
        viewport: &ViewPort
    ) {
        let field_of_view = viewport.field_of_view();

        let mut current_field_of_view = field_of_view.healpix_cells();
        let current_depth = field_of_view.current_depth();

        let mut rebuild_binary_heap = true;

        if self.prev_field_of_view.symmetric_difference(&current_field_of_view).count() == 0 {
            // The field of view has not changed
            // We do not refresh the sources
            return;
        }

        let removed_cells = self.prev_field_of_view.difference(&current_field_of_view).collect::<Vec<_>>();
        let healpix_cells = if removed_cells.is_empty() {
            // Only new cells must be added. We just can add the corresponding sources
            // to the binary heap without clearing it!

            rebuild_binary_heap = false;
            // There is only new cells
            current_field_of_view.difference(&self.prev_field_of_view).cloned().collect::<BTreeSet<_>>()
        } else {
            // Some cells has been removed so their sources must be removed
            // from the binary heap too. We must clear it.
            current_field_of_view.clone()
        };
        self.prev_field_of_view = current_field_of_view.clone();

        let mut sources = vec![];

        if current_depth > 7 {
            if rebuild_binary_heap {
                self.cells.clear();
            }
            for tile in healpix_cells.iter() {
                let depth = tile.0;
                let idx = tile.1;

                let idx_7 = idx >> (2*(depth - 7));
                let cell = (7 as u8, idx_7 as u32);

                if !self.cells.contains(&cell) {
                    let result = self.data.get_sources(depth, idx as u32);
                    sources.push(result);

                    self.cells.insert(cell);
                }
            }
        } else {
            for tile in healpix_cells.iter() {
                let depth = tile.0;
                let idx = tile.1;

                let result = self.data.get_sources(depth, idx as u32);
                sources.push(result);
            }
        }

        let area_tile = area_clip_zoomed_space_healpix_tile::<P>(viewport, 7);
        let max_num_sources = self.data.get_max_number_sources();
        let max_source_density = (max_num_sources as f32) / area_tile;
        //console::log_1(&format!("max_source_density: {:?}", max_source_density).into());

        let scaling_cst = 20_f32; 
        self.strength = scaling_cst / max_source_density.sqrt();
        // Clamp the strength to 1_f32
        if self.strength > 1_f32 {
            self.strength = 1_f32;
        }

        let mut sources = sources[..].concat();

        // Get a vector of sources from a f32 vector
        let sources = { 
            let ptr = sources.as_mut_ptr();
            let len = sources.len() / 6;
            let cap = sources.capacity() / 6;

            mem::forget(sources);

            unsafe { Vec::from_raw_parts(ptr as *mut Source, len, cap) }
        };

        if rebuild_binary_heap {
            self.sources.clear();
        }

        for source in sources.into_iter() {
            if self.sources.len() < MAX_SOURCES {
                self.sources.push(source);
            } else {
                if self.sources.peek().unwrap().mag > source.mag {
                    // We insert it into the heap and remove the faintest source
                    self.sources.pop();
                    self.sources.push(source);
                }
            }
        }

        let mut sources = self.sources.clone().into_vec();
        self.num_instances = sources.len();

        // Get back a vector of f32 from the vec of Sources for drawing purposes
        let sources = { 
            let ptr = sources.as_mut_ptr();
            let len = sources.len() * 6;
            let cap = sources.capacity() * 6;

            mem::forget(sources);

            unsafe { Vec::from_raw_parts(ptr as *mut f32, len, cap) }
        };
        
        // Update the VAO
        self.vertex_array_object.bind()
            .update_instanced_array(0, BufferData::VecData(&sources));

        console::log_1(&format!("num sources: {:?}", self.num_instances).into());
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

            self.vertex_array_object.bind_ref();

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
                self.vertex_array_object.num_elements() as i32,
                WebGl2RenderingContext::UNSIGNED_SHORT,
                0,
                self.num_instances as i32,
            );

            // Unbind the FBO
            gl.bind_framebuffer(WebGl2RenderingContext::FRAMEBUFFER, None);
        }

        // Render to the heatmap to the screen
        {
            // Set the viewport
            let window_size = viewport.get_window_size();
            gl.viewport(0, 0, window_size.x as i32, window_size.y as i32);
            //viewport.update_scissor();

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
impl<'a> DisableDrawing for Catalog {
    fn disable(&mut self) {
    }
}

use cgmath::Vector2;
fn area_clip_zoomed_space_healpix_tile<P: Projection>(viewport: &ViewPort, depth: u8) -> f32 {
    let sphere_area = 4_f32 * std::f32::consts::PI;

    let num_hpx_cells = 12_f32 * 4_f32.powf(depth as f32);
    let hpx_cell_ang = Rad((sphere_area / num_hpx_cells).sqrt());

    let half_hpx_ang = hpx_cell_ang * 2_f32.sqrt() / 2_f32;

    // Vertex in the WCS of the FOV
    let v0 = math::radec_to_xyz(half_hpx_ang, Rad(0_f32));
    let v1 = math::radec_to_xyz(Rad(0_f32), half_hpx_ang);

    // Project this vertex into the screen
    let clip_zoom_factor = viewport.get_clip_zoom_factor();
    let p0 = P::world_to_clip_space(v0) / clip_zoom_factor;
    let p1 = P::world_to_clip_space(v1) / clip_zoom_factor;

    let area_ndc_hpx_tile = p0.x * p0.x + p1.y * p1.y;

    area_ndc_hpx_tile
}

const MAX_SOURCES: usize = 100000;

use std::ops::Range;
struct Storage {
    // Sources data
    sources: Vec<f32>,

    //healpix_idx: [Option<IdxSourceRange<u32>>; 196608], // depth 7
    healpix_idx: Box<[Option<Range<u32>>]>, // depth 7

    max_num_sources: usize,
}
/*
#[derive(Clone, Copy)]
struct IdxSourceRange<T: Clone + Copy> {
    start: T,
    end: T,
}

impl<T> IdxSourceRange<T>
where T: Clone + Copy {
    fn new(start: T, end: T) -> IdxSourceRange<T> {
        IdxSourceRange {
            start,
            end
        }
    }
}*/

use std::mem;
impl Storage {
    fn new(mut sources: Vec<Source>) -> Storage {
        // Sort the sources by HEALPix indices at depth 7
        sources.sort_unstable_by(|s1, s2| {
            let idx1 = healpix::nested::hash(7, s1.lon as f64, s1.lat as f64);
            let idx2 = healpix::nested::hash(7, s2.lon as f64, s2.lat as f64);

            idx1.partial_cmp(&idx2).unwrap()
        });

        let mut healpix_idx: Vec<Option<Range<u32>>> = vec![None; 196608];

        for (idx_source, s) in sources.iter().enumerate() {
            let idx = healpix::nested::hash(7, s.lon as f64, s.lat as f64) as usize;

            if let Some(ref mut healpix_idx) = &mut healpix_idx[idx] {
                healpix_idx.end += 1;
            } else {
                healpix_idx[idx] = Some((idx_source as u32)..((idx_source + 1) as u32));
            }
        }

        let mut idx_source = 0;
        let mut max_num_sources = 0;
        for i in 0..healpix_idx.len() {
            if let Some(ref healpix_idx) = healpix_idx[i] {
                let num_sources = healpix_idx.end - healpix_idx.start;
                max_num_sources = std::cmp::max(max_num_sources, num_sources);

                idx_source = healpix_idx.end;
            } else {
                healpix_idx[i] = Some(idx_source..idx_source);
            }
        }

        let sources = {            
            let ptr = sources.as_mut_ptr();
            let len = sources.len() * mem::size_of::<Source>();
            let cap = sources.capacity() * mem::size_of::<Source>();
            
            mem::forget(sources);

            unsafe { Vec::from_raw_parts(ptr as *mut f32, len, cap) }
        };        

        let healpix_idx = healpix_idx.into_boxed_slice();
        let max_num_sources = max_num_sources as usize;
        Storage {
            sources,

            healpix_idx,

            max_num_sources,
        }
    }

    fn get_sources(&self, depth: u8, idx: u32) -> &[f32] {
        let idx = idx as usize;
        let depth = depth as usize;

        let mut sources_idx_range = if depth <= 7 {
            let off = 2*(7 - depth);

            let healpix_idx_start = idx << off;
            let healpix_idx_end = (idx + 1) << off;

            let idx_start_sources = self.healpix_idx[healpix_idx_start]
                .as_ref()
                .unwrap().start as usize;
            let idx_end_sources = self.healpix_idx[healpix_idx_end - 1]
                .as_ref()
                .unwrap().end as usize;

            idx_start_sources..idx_end_sources
        } else {
            // depth > 7
            // Get the sources that are contained in parent cell of depth 7
            let off = 2*(depth - 7);
            let idx_start = idx >> off;

            let idx_start_sources = self.healpix_idx[idx_start]
                .as_ref()
                .unwrap().start as usize;
            let idx_end_sources = self.healpix_idx[idx_start]
                .as_ref()
                .unwrap().end as usize;

            idx_start_sources..idx_end_sources
        };
        let size = std::mem::size_of::<Source>() / std::mem::size_of::<f32>();

        sources_idx_range.start = sources_idx_range.start * size;
        sources_idx_range.end = sources_idx_range.end * size;
        &self.sources[sources_idx_range.start..sources_idx_range.end]
    }

    fn get_max_number_sources(&self) -> usize {
        self.max_num_sources
    }
}