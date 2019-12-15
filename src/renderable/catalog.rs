use crate::texture::Texture2D;

use web_sys::WebGlFramebuffer;
use std::collections::HashSet;
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
    // Quadtree referring to the sources
    //quadtree: QuadTree
}

use cgmath::Rad;
#[derive(Debug)]
#[derive(Clone, PartialEq)]
#[repr(C)]
pub struct Source {
    x: f32,
    y: f32,
    z: f32,

    lon: Rad<f32>,
    lat: Rad<f32>,

    score: f32,
}

impl Eq for Source {}

impl Source {
    pub fn new(lon: Rad<f32>, lat: Rad<f32>, score: f32) -> Source {
        let world_pos = math::radec_to_xyz(lon, lat);

        let x = world_pos.x;
        let y = world_pos.y;
        let z = world_pos.z;

        Source {
            x,
            y,
            z,

            lon,
            lat,

            score
        }
    }
}

use std::cmp::Ordering;
use std::cmp::PartialOrd;
use std::cmp::Ord;
impl PartialOrd for Source {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.score.partial_cmp(&other.score)
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

        let cells = HashSet::new();
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

            //quadtree,
            data,
            cells,
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

use crate::math;
use std::mem;
use crate::renderable::buffers::buffer_data::BufferDataSlice;
impl Mesh for Catalog {
    fn create_buffers(&mut self, gl: &WebGl2Context) -> VertexArrayObject {
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
                std::mem::size_of::<Source>(),
                &[3, 2, 1],
                &[0 * mem::size_of::<f32>(), 3 * mem::size_of::<f32>(), 5 * mem::size_of::<f32>()],
                WebGl2RenderingContext::DYNAMIC_DRAW,
                BufferData::new(self.data.sources.as_ref()),
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
        vertex_array_object: &mut VertexArrayObject,
        _local_to_world: &Matrix4<f32>,
        _projection: &ProjectionType,
        viewport: &ViewPort
    ) {
        let field_of_view = viewport.field_of_view();
        let (depth, hpx_idx) = field_of_view.cells();
        console::log_1(&format!("depth: {:?}", depth).into());

        let mut sources = vec![];

        if *depth > 7 {
            self.cells.clear();
            for idx in hpx_idx.iter() {
                let idx_7 = *idx >> (2*(*depth - 7));
                let cell = (7 as u8, idx_7 as u32);

                if !self.cells.contains(&cell) {
                    let result = self.data.get_sources(*depth, *idx as u32);
                    sources.push(result);

                    self.cells.insert(cell);
                }
            }
        } else {
            for idx in hpx_idx.iter() {
                let result = self.data.get_sources(*depth, *idx as u32);
                sources.push(result);
            }
        }

        let sources = &sources[..].concat();

        self.num_instances = mem::size_of_val(&sources[..]) / mem::size_of::<Source>();
        console::log_1(&format!("num sources: {:?}", self.num_instances).into());

        /*for idx in hpx_idx.iter() {
            let tile = self.quadtree.get(*depth, *idx);
            if let Some(tile) = tile {
                //tiles.push(tile);
                for &source_idx in tile.borrow().sources_idx.iter() {
                    sources.push(&self.sources[source_idx]);
                }
                //console::log_1(&format!("num aa: {:?}", tile.borrow().sources.len()).into());
                //sources_idx.extend(tile.borrow().sources_idx.iter().cloned());
            } else {
                console::log_1(&format!("depth, idx: {:?}, {:?}", depth, idx).into());
            }
        }*/

        // Update the VAO
        vertex_array_object.bind()
            .update_instanced_array(0, BufferData::new(&sources));
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
                self.num_instances as i32,
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
impl<'a> DisableDrawing for Catalog {
    fn disable(&mut self) {
    }
}

const MAX_SOURCES: usize = 1000;
/*
use std::rc::Rc;
use std::cell::RefCell;
struct Node {
    children: [Option<Rc<RefCell<Node>>>; 4],

    idx: u64,
    depth: u8,

    sources_idx: Vec<usize>,
    num_sources: usize,
}

use healpix;
use cgmath::Rad;
impl Node {
    fn new(idx: u64, depth: u8) -> Node {
        let children = [None, None, None, None];
        let sources_idx = Vec::with_capacity(MAX_SOURCES);
        let num_sources = 0;

        Node {
            children,
            idx,
            depth,
            sources_idx,
            num_sources
        }
    }

    fn add_to_child(&mut self, source_idx: usize, sources: &Vec<Source>, tiles: &mut HashMap<u64, Rc<RefCell<Node>>>) {
        let source = &sources[source_idx];
        let lon: Rad<f32> = source.ra.into();
        let lat: Rad<f32> = source.dec.into();

        let depth = self.depth + 1;
        let idx = healpix::nested::hash(depth, lon.0 as f64, lat.0 as f64);

        let offset = self.idx << 2;
        let idx_child = (idx - offset) as usize;

        if idx_child < 0 || idx_child > 3 {
            unreachable!();
        }

        if let Some(child) = self.children[idx_child].clone() {
            child.borrow_mut().add(source_idx, sources, tiles);
        } else {
            // Create the child node
            let child = Rc::new(RefCell::new(Node::new(idx, depth)));
            // Add the source
            child.borrow_mut().add(source_idx, sources, tiles);

            // Add the node to the hashmap
            let uniq = (1 << (2*((depth as u64) + 1))) + idx;
            tiles.insert(uniq, child.clone());

            // Loop over all the sources of the current node
            // and add them if they are located in the child node
            for &parent_source_idx in self.sources_idx.iter() {
                let parent_source = &sources[parent_source_idx];
                let lon: Rad<f32> = parent_source.ra.into();
                let lat: Rad<f32> = parent_source.dec.into();
                let child_idx = healpix::nested::hash(depth, lon.0 as f64, lat.0 as f64);

                if child_idx == child.borrow().idx {
                    child.borrow_mut().add(parent_source_idx, sources, tiles);
                }
            }

            // Add it to the tree
            self.children[idx_child] = Some(child);
        }
    }

    pub fn add(&mut self, source_idx: usize, sources: &Vec<Source>, tiles: &mut HashMap<u64, Rc<RefCell<Node>>>) {
        if self.num_sources < MAX_SOURCES || self.depth == 29 {
            self.sources_idx.push(source_idx);
            self.num_sources += 1;
        } else {
            // Determine the children concerned by the source
            self.add_to_child(source_idx, sources, tiles);
        }
    }
}

// The sources have longer lifetimes than
// the nodes
struct QuadTree {
    nodes: [Option<Rc<RefCell<Node>>>; 12],
    tiles: HashMap<u64, Rc<RefCell<Node>>>,
}

impl QuadTree {
    pub fn new(sources: &Vec<Source>) -> QuadTree {
        let mut nodes: [Option<Rc<RefCell<Node>>>; 12] = [
            None, None, None, None,
            None, None, None, None,
            None, None, None, None
        ];
        let mut tiles = HashMap::new();
        // Loop over all the sources of the current node
        // and them if they are located in the child node
        for (source_idx, source) in sources.iter().enumerate() {
            let lon: Rad<f32> = source.ra.into();
            let lat: Rad<f32> = source.dec.into();
            let child_idx = healpix::nested::hash(0, lon.0 as f64, lat.0 as f64);

            if let Some(child) = nodes[child_idx as usize].clone() {
                child.borrow_mut().add(source_idx, sources, &mut tiles);
            } else {
                let node = Rc::new(RefCell::new(Node::new(child_idx, 0)));
                node.borrow_mut().add(source_idx, sources, &mut tiles);

                // Add it to the hashmap
                let uniq = 4 + child_idx;
                tiles.insert(uniq, node.clone());

                nodes[child_idx as usize] = Some(node);
            }
        }

        QuadTree {
            nodes,
            tiles,
        }
    }

    pub fn get(&self, mut depth: u8, mut idx: u64) -> Option<Rc<RefCell<Node>>> {
        // Add the node to the hashmap
        let mut uniq = (1 << (2*((depth as u64) + 1))) + idx;

        if let Some(tile) = self.tiles.get(&uniq) {
            Some(tile.clone())
        } else {
            while self.tiles.get(&uniq).is_none() {
                if depth == 0 {
                    return None;
                }
                depth -= 1;
                idx = idx >> 2;

                uniq = (1 << (2*((depth as u64) + 1))) + idx;
            }

            if let Some(tile) = self.tiles.get(&uniq) {
                if tile.borrow().num_sources < MAX_SOURCES {
                    // It is a leaf
                    Some(tile.clone())
                } else {
                    None
                }
            } else {
                None
            }
        }
        //Some(self.tiles.get(&uniq).unwrap().clone())
    }
}
*/
use std::ops::Range;
struct Storage {
    // Sources data
    sources: Vec<f32>,

    healpix_idx: Box<[Option<Range<u32>>]>, // depth 7
}

impl Storage {
    fn new(mut sources: Vec<Source>) -> Storage {
        // Sort the sources by HEALPix indices at depth 7
        sources.sort_unstable_by(|s1, s2| {
            let idx1 = healpix::nested::hash(7, s1.lon.0 as f64, s1.lat.0 as f64);
            let idx2 = healpix::nested::hash(7, s2.lon.0 as f64, s2.lat.0 as f64);

            idx1.partial_cmp(&idx2).unwrap()
        });

        let mut healpix_idx: Vec<Option<Range<u32>>> = vec![None; 196608];
        for (idx_source, s) in sources.iter().enumerate() {
            let idx = healpix::nested::hash(7, s.lon.0 as f64, s.lat.0 as f64) as usize;

            if let Some(ref mut healpix_idx) = &mut healpix_idx[idx] {
                healpix_idx.end += 1;
            } else {
                healpix_idx[idx] = Some((idx_source as u32)..((idx_source + 1) as u32));
            }
        }

        let mut idx_source = 0;
        for i in 0..healpix_idx.len() {
            if let Some(ref healpix_idx) = healpix_idx[i] {
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
        console::log_1(&format!("sources ranges: {:?}", healpix_idx).into());

        let mut healpix_idx = healpix_idx.into_boxed_slice();
        Storage {
            sources,

            healpix_idx
        }
    }

    fn get_sources(&self, depth: u8, idx: u32) -> &[f32] {
        let idx = idx as usize;
        let depth = depth as usize;

        let size = std::mem::size_of::<Source>() / std::mem::size_of::<f32>();

        let sources = if depth <= 7 {
            let off = 2*(7 - depth);

            let healpix_idx_start = idx << off;
            let healpix_idx_end = (idx + 1) << off;

            let idx_start_sources = self.healpix_idx[healpix_idx_start]
                .as_ref()
                .unwrap().start as usize;
            let idx_end_sources = self.healpix_idx[healpix_idx_end - 1]
                .as_ref()
                .unwrap().end as usize;

            &self.sources[(size*idx_start_sources)..(size*idx_end_sources)]
        } else {
            // depth > 7
            // Get the sources that are contained in parent cell of depth 7
            /*let shift = 2*(depth - 7);
            let off = (1 << shift) - 1;
            let mask = !off;

            let add = off;

            let idx_start = idx & mask;*/

            let off = 2*(depth - 7);
            let idx_start = idx >> off;

            let idx_start_sources = self.healpix_idx[idx_start]
                .as_ref()
                .unwrap().start as usize;
            let idx_end_sources = self.healpix_idx[idx_start]
                .as_ref()
                .unwrap().end as usize;

            &self.sources[(size*idx_start_sources)..(size*idx_end_sources)]

            /*let candidates = &self.sources[6*idx_start_sources..6*idx_end_sources];
            let mut result = Vec::<&f32>::new();
            let d = depth as u8;
            let idx = idx as u64;*/
            /*for source in candidates {
                let candidate_idx = healpix::nested::hash(d, source.lon.0 as f64, source.lat.0 as f64);

                if idx == candidate_idx {
                    result.push(source);
                }
            }
            result
            */
        };

        sources
    }
}


