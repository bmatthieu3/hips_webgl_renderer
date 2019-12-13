use crate::texture::Texture2D;

use web_sys::WebGlFramebuffer;
pub struct Catalog {
    center_lonlat: Vec<f32>, // Spherical position of the observations
    center_xyz: Vec<f32>, // Cartesian position of the observations
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

    // Quadtree referring to the sources
    //quadtree: QuadTree
}

use cgmath::Deg;
#[derive(Debug)]
#[derive(Clone, PartialEq)]
pub struct Source {
    ra: Deg<f32>,
    dec: Deg<f32>,
    mag: f32,
}

impl Eq for Source {}

impl Source {
    pub fn new(ra: Deg<f32>, dec: Deg<f32>, mag: f32) -> Source {
        Source {
            ra,
            dec,
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

impl<'a> From<&[f32]> for Source {
    fn from(data: &[f32]) -> Self {
        Source::new(Deg(data[0]), Deg(data[1]), data[2])
    }
}

use web_sys::console;
impl Catalog {
    pub fn new(gl: &WebGl2Context, sources: Vec<Source>) -> Catalog {
        // Build the quadtree from the list of sources
        //console::log_1(&format!("BEGIN quadtree build").into());
        //let quadtree = QuadTree::new(sources);
        //console::log_1(&format!("Quadtree BUILT").into());

        let num_instances_max = sources.len();
        let mut center_xyz = vec![];
        let mut center_lonlat = vec![];
        for source in sources.into_iter() {
            let vertex = math::radec_to_xyz(source.ra.into(), source.dec.into());

            center_xyz.push(vertex.x);
            center_xyz.push(vertex.y);
            center_xyz.push(vertex.z);

            center_lonlat.push(source.ra.0);
            center_lonlat.push(source.dec.0);
        }
        /*let num_instances_max = MAX_SOURCES * 64;
        let mut center_lonlat = vec![0_f32; num_instances_max * 2];
        let mut center_xyz = vec![0_f32; num_instances_max * 3];*/
        let num_instances = num_instances_max;

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
        Catalog {
            center_lonlat,
            center_xyz,

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
impl Mesh for Catalog {
    fn create_buffers(&self, gl: &WebGl2Context) -> VertexArrayObject {
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
                3 * std::mem::size_of::<f32>(),
                3,
                WebGl2RenderingContext::DYNAMIC_DRAW,
                BufferData::new(self.center_xyz.as_ref()),
            )
            // Store the spherical position of the center of the source in the a instanced VBO
            .add_instanced_array_buffer(
                2 * std::mem::size_of::<f32>(),
                2,
                WebGl2RenderingContext::DYNAMIC_DRAW,
                BufferData::new(self.center_lonlat.as_ref()),
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
        /*let field_of_view = viewport.field_of_view();
        let (depth, hpx_idx) = field_of_view.cells();

        let mut sources = Vec::with_capacity(MAX_SOURCES * hpx_idx.len());

        for idx in hpx_idx.iter() {
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
        }

        console::log_1(&format!("num sources to sort: {:?}", sources.len()).into());

        sources.sort_unstable_by(|a, b| {
            a.partial_cmp(b).unwrap()
        });

        self.num_instances = std::cmp::min(sources.len(), MAX_SOURCES);
        console::log_1(&format!("num instances: {:?}", self.num_instances).into());

        self.center_lonlat.clear();
        self.center_xyz.clear();

        for &source in &sources[..self.num_instances] {
            let vertex = math::radec_to_xyz(source.ra.into(), source.dec.into());

            self.center_xyz.push(vertex.x);
            self.center_xyz.push(vertex.y);
            self.center_xyz.push(vertex.z);

            self.center_lonlat.push(source.ra.0);
            self.center_lonlat.push(source.dec.0);
        }

        // Update the VAO
        vertex_array_object.bind()
            .update_instanced_array(0, BufferData::new(&self.center_xyz))
            .update_instanced_array(1, BufferData::new(&self.center_lonlat));*/
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

use std::ops::Range;
struct Storage {
    sources: Vec<Source>,
    cells: Vec<Option<Range<u32>>>, // depth 7
}

impl Storage {
    fn new(mut sources: Vec<Source>) -> Storage {
        // Sort by HEALPix indices at depth 7
        sources.sort_unstable_by(|a, b| {
            let lon_a: Rad<f32> = a.ra.into();
            let lat_a: Rad<f32> = a.dec.into();
            let idx_a = healpix::nested::hash(7, lon_a.0 as f64, lat_a.0 as f64);

            let lon_b: Rad<f32> = b.ra.into();
            let lat_b: Rad<f32> = b.dec.into();
            let idx_b = healpix::nested::hash(7, lon_b.0 as f64, lat_b.0 as f64);

            idx_a.partial_cmp(&idx_b).unwrap()
        });

        let mut cells: Vec<Option<Range<u32>>> = vec![None; 196608];

        for s in sources.iter() {
            let lon: Rad<f32> = s.ra.into();
            let lat: Rad<f32> = s.dec.into();
            let idx = healpix::nested::hash(7, lon.0 as f64, lat.0 as f64) as usize;

            if let Some(cell) = &mut cells[idx] {
                cell.end += 1;
            } else {
                cells[idx] = Some((idx as u32)..((idx + 1) as u32));
            }
        }

        Storage {
            sources,
            cells
        }
    }

    /*fn get_sources(&self, depth: u8, idx: u32) -> Vec<&Source> {
        /*if depth <= 7 {
            let range = 12 * 
        }*/
    }*/
}

