use std::rc::Rc;
use std::cell::RefCell;

use wasm_bindgen::prelude::Closure;
use wasm_bindgen::JsCast;

use web_sys::{HtmlImageElement, CanvasRenderingContext2d};
use web_sys::WebGl2RenderingContext;

use crate::renderable::hips_sphere::MAX_NUMBER_TEXTURE;
use std::collections::{BinaryHeap, HashSet};

const HEIGHT_TEXTURE: i32 = 512;
const WIDTH_TEXTURE: i32 = 512;

use crate::WebGl2Context;
use web_sys::console;
use crate::RENDER_NEXT_FRAME;
/*
#[derive(Clone, Debug)]
pub struct HEALPixCellRequest {
    texture: Rc<RefCell<HtmlImageElement>>,
    idx: i32,
    depth: i32,
    time_request: f32,
    idx_in_buffer: i32,
    time_received: Option<f32>,
}

pub struct HEALPixCellGPUData {
    pub idx: i32,
    pub buf_idx: i32,
    pub time_received: f32,
}

impl From<HEALPixCellRequest> for HEALPixCellGPUData {
    fn from(cell: HEALPixCellRequest) -> Self {
        HEALPixCellGPUData {
            idx: cell.idx,
            buf_idx: cell.idx_in_buffer,
            time_received: cell.time_received.unwrap()
        }
    }
}

impl HEALPixCellRequest {
    pub fn new(depth: i32, idx: i32) -> HEALPixCellRequest {
        let time_request = utils::get_current_time();
        let time_received = None;

        let texture = Rc::new(RefCell::new(HtmlImageElement::new().unwrap()));
        let idx_in_buffer = 0;
        HEALPixCellRequest {
            texture,
            idx,
            depth,
            time_request,
            idx_in_buffer,
            time_received
        }
    }

    fn get_url(&self) -> String {
        let dir_idx = (self.idx / 10000) * 10000;

        let mut url = String::from("http://alasky.u-strasbg.fr/DSS/DSSColor/");
        url = url + "Norder" + &self.depth.to_string() + "/";
        url = url + "Dir" + &dir_idx.to_string() + "/";
        url = url + "Npix" + &self.idx.to_string() + ".jpg";

        url
    }
}

impl PartialEq for HEALPixCellRequest {
    fn eq(&self, other: &Self) -> bool {
        self.idx == other.idx && self.depth == other.depth
    }
}

impl Hash for HEALPixCellRequest {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.idx.hash(state);
        self.depth.hash(state);
    }
}

impl Eq for HEALPixCellRequest {}


impl PartialOrd for HEALPixCellRequest {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        other.time_request.partial_cmp(&self.time_request)
    }
}

impl Ord for HEALPixCellRequest {
    fn cmp(&self, other: &Self) -> Ordering {
        self.partial_cmp(&other).unwrap()
    }
}

#[derive(Clone)]
pub struct HEALPixTextureBuffer {
    gl: WebGl2Context,
    buffer: Rc<RefCell<BinaryHeap<HEALPixCellRequest>>>,
    buffer_zero_depth: Rc<RefCell<Vec<HEALPixCellRequest>>>,

    loaded_cells: Rc<RefCell<HashSet<HEALPixCellRequest>>>,

    pub webgl_texture0: Option<web_sys::WebGlTexture>,
    pub webgl_texture1: Option<web_sys::WebGlTexture>,
}

impl HEALPixTextureBuffer {
    pub fn new(gl: &WebGl2Context) -> HEALPixTextureBuffer {
        let buffer = Rc::new(RefCell::new(BinaryHeap::with_capacity(MAX_NUMBER_TEXTURE)));
        let buffer_zero_depth = Rc::new(RefCell::new(Vec::with_capacity(12)));
        let loaded_cells = Rc::new(RefCell::new(HashSet::with_capacity(MAX_NUMBER_TEXTURE)));

        // Initialize context rendering
        let webgl_texture0 = create_sampler_3d(gl, WebGl2RenderingContext::TEXTURE0, MAX_NUMBER_TEXTURE as u32);
        let webgl_texture1 = create_sampler_3d(gl, WebGl2RenderingContext::TEXTURE1, 12);

        let gl = gl.clone();
        let mut healpix_texture_buf = HEALPixTextureBuffer {
            gl,
            buffer,
            buffer_zero_depth,
            loaded_cells,
            webgl_texture0,
            webgl_texture1,
        };

        for base_cell_idx in 0..12 {
            let healpix_cell = HEALPixCellRequest::new(0, base_cell_idx);
            healpix_texture_buf.load_zero_depth_cells(healpix_cell);
        }

        healpix_texture_buf
    }

    fn load_zero_depth_cells(&mut self, healpix_cell: HEALPixCellRequest) {
        // Add it to the loaded cells hashset
        self.loaded_cells.borrow_mut().insert(healpix_cell.clone());

        let onerror = {
            let loaded_cells = self.loaded_cells.clone(); 
            let healpix_cell = healpix_cell.clone();

            Closure::wrap(Box::new(move || {
                loaded_cells.borrow_mut().remove(&healpix_cell);
            }) as Box<dyn Fn()>)
        };

        let url = healpix_cell.get_url();
        let onload = {
            let buffer = self.buffer_zero_depth.clone();
            let texture_clone = healpix_cell.texture.clone();
            let healpix_cell = healpix_cell.clone();
            let webgl_texture = self.webgl_texture1.clone();
            let gl = self.gl.clone();

            Closure::wrap(Box::new(move || {
                console::log_1(&format!("sampler3D shader").into());

                let idx = buffer.borrow().len() as i32;
                gl.active_texture(WebGl2RenderingContext::TEXTURE1);
                gl.bind_texture(WebGl2RenderingContext::TEXTURE_3D, webgl_texture.as_ref());
                gl.tex_sub_image_3d_with_html_image_element(
                    WebGl2RenderingContext::TEXTURE_3D,
                    0,
                    0,
                    0,
                    idx,
                    WIDTH_TEXTURE,
                    HEIGHT_TEXTURE,
                    1,
                    WebGl2RenderingContext::RGB,
                    WebGl2RenderingContext::UNSIGNED_BYTE,
                    &texture_clone.borrow(),
                )
                .expect("Texture 3d");
                // Tell the app to render the next frame
                // because a new base tile is downloaded
                RENDER_NEXT_FRAME.store(true, atomic::Ordering::Relaxed);

                let mut healpix_indexed = healpix_cell.clone();
                healpix_indexed.idx_in_buffer = idx as i32;
                healpix_indexed.time_received = Some(utils::get_current_time());

                buffer.borrow_mut().push(healpix_indexed);
            }) as Box<dyn Fn()>)
        };

        let image = healpix_cell.texture.borrow_mut();

        image.set_onload(Some(onload.as_ref().unchecked_ref()));
        image.set_onerror(Some(onerror.as_ref().unchecked_ref()));

        image.set_cross_origin(Some(""));
        image.set_src(&url);

        onload.forget();
    }
    
    pub fn load(&mut self, healpix_cell: HEALPixCellRequest, zoom: bool) {
        // discard base cells, they are stored in the buffer_zero_depth
        if healpix_cell.depth == 0 {
            return;
        }

        // Check if the healpix_cell is loaded or has been currently asked
        // for being loaded
        if self.loaded_cells.borrow().contains(&healpix_cell) {
            // Change its priority in the buffer (if it is present!).
            let mut tmp_buffer = self.buffer.borrow_mut().clone()
                .into_sorted_vec();

            let idx_hpx_cell = tmp_buffer.iter().position(|x| x.idx == healpix_cell.idx && x.depth == healpix_cell.depth);

            if let Some(idx_hpx_cell) = idx_hpx_cell {
                console::log_1(&format!("found healpix cell").into());
                // Found
                let mut healpix_cell_to_change_priority = tmp_buffer.remove(idx_hpx_cell);
                healpix_cell_to_change_priority.time_request = healpix_cell.time_request;
                /*if zoom {
                    healpix_cell_to_change_priority.time_received = Some(utils::get_current_time());
                }*/
                // Change time priority
                self.buffer.borrow_mut().clear();
                for hpx_cell in tmp_buffer {
                    self.buffer.borrow_mut().push(hpx_cell);
                }

                self.buffer.borrow_mut().push(healpix_cell_to_change_priority);
            }
        } else {
            // Add it to the loaded cells hashset
            self.loaded_cells.borrow_mut().insert(healpix_cell.clone());

            let onerror = {
                let loaded_cells = self.loaded_cells.clone(); 
                let healpix_cell = healpix_cell.clone();

                Closure::wrap(Box::new(move || {
                    loaded_cells.borrow_mut().remove(&healpix_cell);
                }) as Box<dyn Fn()>)
            };

            let url = healpix_cell.get_url();
            let onload = {
                let buffer = self.buffer.clone();
                let loaded_cells = self.loaded_cells.clone();
                let texture_clone = healpix_cell.texture.clone();
                let healpix_cell = healpix_cell.clone();
                let webgl_texture = self.webgl_texture0.clone();
                let gl = self.gl.clone();

                Closure::wrap(Box::new(move || {
                    console::log_1(&format!("load new tile").into());
                    let idx = if buffer.borrow().len() == MAX_NUMBER_TEXTURE {
                        // Remove the oldest tile from the buffer and from the
                        // hashset
                        let oldest_healpix_cell = buffer.borrow_mut().pop().unwrap();
                        loaded_cells.borrow_mut().remove(&oldest_healpix_cell);

                        oldest_healpix_cell.idx_in_buffer
                    } else {
                        buffer.borrow().len() as i32
                    };
                    gl.active_texture(WebGl2RenderingContext::TEXTURE0);
                    gl.bind_texture(WebGl2RenderingContext::TEXTURE_3D, webgl_texture.as_ref());
                    gl.tex_sub_image_3d_with_html_image_element(
                        WebGl2RenderingContext::TEXTURE_3D,
                        0,
                        0,
                        0,
                        idx,
                        WIDTH_TEXTURE,
                        HEIGHT_TEXTURE,
                        1,
                        WebGl2RenderingContext::RGB,
                        WebGl2RenderingContext::UNSIGNED_BYTE,
                        &texture_clone.borrow(),
                    )
                    .expect("Texture 3d");
                    // Tell the app to render the next frame
                    // because a new base tile is downloaded
                    RENDER_NEXT_FRAME.store(true, atomic::Ordering::Relaxed);

                    let mut healpix_indexed = healpix_cell.clone();
                    healpix_indexed.idx_in_buffer = idx as i32;
                    healpix_indexed.time_received = Some(utils::get_current_time());

                    buffer.borrow_mut().push(healpix_indexed);

                }) as Box<dyn Fn()>)
            };

            //let texture_clone_clone = healpix_cell.texture.clone();
            let image = healpix_cell.texture.borrow_mut();

            image.set_onload(Some(onload.as_ref().unchecked_ref()));
            image.set_onerror(Some(onerror.as_ref().unchecked_ref()));

            image.set_cross_origin(Some(""));
            image.set_src(&url);

            onload.forget();
        }
    }

    pub fn len(&self) -> usize {
        self.buffer.borrow().len()
    }

    pub fn get_tiles(&self, depth: i32) -> Vec<HEALPixCellGPUData> {
        self.buffer.borrow()
            .clone()
            .into_sorted_vec()
            .into_iter()
            .filter_map(|hpx| {
                if hpx.depth == depth {
                    Some(hpx.into())
                } else {
                    None
                }
            })
            .collect::<Vec<_>>()
    }
    
    pub fn get_zero_depth_tiles(&self) -> Vec<HEALPixCellGPUData> {
        self.buffer_zero_depth.borrow()
            .clone()
            .into_iter()
            .map(|hpx| {
                hpx.into()
            })
            .collect::<Vec<_>>()
    }
}*/

static mut NUM_TEXTURE_UNIT: u32 = WebGl2RenderingContext::TEXTURE0;

#[derive(Clone)]
struct Tile {
    idx: u64,
    depth: u8,

    texture_idx: usize,

    time_request: f32,
    time_received: Option<f32>,

    //image: Rc<HtmlImageElement>,
}

// Equality impl allowing to store
// tiles in a set
impl PartialEq for Tile {
    fn eq(&self, other: &Self) -> bool {
        self.idx == other.idx && self.depth == other.depth
    }
}
impl Eq for Tile {}

use std::hash::{Hash, Hasher};
impl Hash for Tile {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.idx.hash(state);
        self.depth.hash(state);
    }
}

use std::cmp::Ordering;
// Requested for storing tiles in a BinaryHeap
impl PartialOrd for Tile {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        other.time_request.partial_cmp(&self.time_request)
    }
}
impl Ord for Tile {
    fn cmp(&self, other: &Self) -> Ordering {
        self.partial_cmp(&other).unwrap()
    }
}

pub struct TileGPU {
    pub idx: i32,
    pub depth: i32,

    pub texture_idx: i32,

    pub time_request: f32,
    pub time_received: f32,
}

impl PartialEq for TileGPU {
    fn eq(&self, other: &Self) -> bool {
        self.idx == other.idx && self.depth == other.depth
    }
}
impl Eq for TileGPU {}

impl PartialOrd for TileGPU {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        // Order by UNIQ notation
        let u1: i32 = 1 << (2*(self.depth + 1)) + self.idx;
        let u2: i32 = 1 << (2*(other.depth + 1)) + other.idx;

        u1.partial_cmp(&u2)
    }
}
impl Ord for TileGPU {
    fn cmp(&self, other: &Self) -> Ordering {
        self.partial_cmp(&other).unwrap()
    }
}


impl From<Tile> for TileGPU {
    fn from(tile: Tile) -> Self {
        let idx = tile.idx as i32;
        let depth = tile.depth as i32;

        let texture_idx = tile.texture_idx as i32;

        let time_request = tile.time_request;
        let time_received = tile.time_received.unwrap();

        TileGPU {
            idx,
            depth,

            texture_idx,

            time_request,
            time_received,
        }
    }
}

#[derive(Clone)]
pub struct BufferTiles {
    gl: WebGl2Context,

    buffer: BinaryHeap<Tile>,
    requested_tiles: HashSet<(u8, u64)>,
    size: usize,
    idx_texture_unit: u32,

    texture: Option<web_sys::WebGlTexture>,
}

use crate::utils;
use std::sync::atomic;
use crate::shader::Shader;
use std::convert::TryInto;
impl BufferTiles {
    pub fn new(gl: &WebGl2Context, size: usize) -> BufferTiles {
        let buffer = BinaryHeap::with_capacity(size);
        let requested_tiles = HashSet::with_capacity(size);

        let (texture, idx_texture_unit) = create_sampler_3d(gl, size as u32);

        let gl = gl.clone();
        BufferTiles {
            gl,
            buffer,
            requested_tiles,
            size,
            idx_texture_unit,
            texture,
        }
    }
    
    // Add a new tile to the buffer
    pub fn add(&mut self, depth: u8, idx: u64, time_request: f32) -> usize {
        let texture_idx = if self.buffer.len() == self.size {
            // Remove the oldest tile from the buffer and from the
            // hashset
            let oldest_requested_tile = self.buffer.pop().unwrap();

            let tile = (oldest_requested_tile.depth, oldest_requested_tile.idx);
            self.requested_tiles.remove(&tile);

            oldest_requested_tile.texture_idx
        } else {
            self.buffer.len()
        };

        let time_received = Some(utils::get_current_time());
        let tile = Tile {
            idx,
            depth,

            texture_idx,

            time_request,
            time_received,
        };

        self.buffer.push(tile);

        texture_idx
    }

    pub fn add_to_requested_tile(&mut self, depth: u8, idx: u64) {
        let tile = (depth, idx);
        self.requested_tiles.insert(tile);
    }

    pub fn remove_from_requested_tile(&mut self, depth: u8, idx: u64) {
        let tile = (depth, idx);
        self.requested_tiles.remove(&tile);
    }

    pub fn replace_tile(&mut self, depth: u8, idx: u64, time_request: f32) -> bool {
        let tile_request = (depth, idx);
        if self.requested_tiles.contains(&tile_request) {
            // Change its priority in the buffer (if it is present!).
            let tile = self.buffer
                .clone()
                .into_iter()
                .find(|tile| {
                    tile.depth == depth && tile.idx == idx
                });

            if let Some(mut tile) = tile {
                console::log_1(&format!("found healpix cell").into());
                // Found 
                tile.time_request = time_request;

                // Push it to the buffer again
                let mut buffer = BinaryHeap::new();
                for t in self.buffer.iter() {
                    if t.depth != depth || t.idx != idx {
                        buffer.push(t.clone());
                    }
                }
                
                buffer.push(tile);

                self.buffer = buffer;
            }

            true
        } else {
            false
        }
    }

    fn replace_texture_sampler_3d(&self, gl: &WebGl2Context, idx: i32, image: &HtmlImageElement) {
        gl.active_texture(self.idx_texture_unit);
        gl.bind_texture(WebGl2RenderingContext::TEXTURE_3D, self.texture.as_ref());
        gl.tex_sub_image_3d_with_html_image_element(
            WebGl2RenderingContext::TEXTURE_3D,
            0,
            0,
            0,
            idx,
            WIDTH_TEXTURE,
            HEIGHT_TEXTURE,
            1,
            WebGl2RenderingContext::RGB,
            WebGl2RenderingContext::UNSIGNED_BYTE,
            image,
        )
        .expect("Texture 3d");
    }

    pub fn tiles(&self) -> Vec<TileGPU> {
        let mut tiles: Vec<TileGPU> = self.buffer
            .clone()
            .into_iter()
            .map(|tile| {
                tile.into()
            })
            .collect::<Vec<_>>();

        tiles.sort_unstable();

        tiles
    }

    pub fn send_sampler_uniform(&self, shader: &Shader) {
        let mut name = String::from("textures_");
        let idx_sampler: i32 = (self.idx_texture_unit - WebGl2RenderingContext::TEXTURE0).try_into().unwrap();

        name += &idx_sampler.to_string();

        let location_sampler_3d = shader.get_uniform_location(&self.gl, &name);
        self.gl.active_texture(self.idx_texture_unit);
        self.gl.bind_texture(WebGl2RenderingContext::TEXTURE_3D, self.texture.as_ref());

        self.gl.uniform1i(location_sampler_3d.as_ref(), idx_sampler);
    }
}

pub fn load_healpix_tile(gl: &WebGl2Context, buffer: Rc<RefCell<BufferTiles>>, idx: u64, depth: u8) {
    let time_request = utils::get_current_time();
    
    if buffer.borrow_mut().replace_tile(depth, idx, time_request) {
        return;
    }

    // Add it to the loaded cells hashset
    buffer.borrow_mut().add_to_requested_tile(depth, idx);

    let url = {
        let dir_idx = (idx / 10000) * 10000;

        let mut url = String::from("http://alasky.u-strasbg.fr/DSS/DSSColor/");
        url = url + "Norder" + &depth.to_string() + "/";
        url = url + "Dir" + &dir_idx.to_string() + "/";
        url = url + "Npix" + &idx.to_string() + ".jpg";

        url
    };
    
    let onerror = {
        let buffer = buffer.clone();

        Closure::wrap(Box::new(move || {
            buffer.borrow_mut().remove_from_requested_tile(depth, idx);
        }) as Box<dyn Fn()>)
    };

    let image = Rc::new(RefCell::new(HtmlImageElement::new().unwrap()));

    let onload = {
        let image = image.clone();
        let gl = gl.clone();

        Closure::wrap(Box::new(move || {
            console::log_1(&format!("load new tile").into());

            // Add the received tile to the buffer
            let idx_texture = buffer.borrow_mut().add(depth, idx, time_request);
            buffer.borrow().replace_texture_sampler_3d(&gl, idx_texture as i32, &image.borrow());

            // Tell the app to render the next frame
            // because a a new has been received
            RENDER_NEXT_FRAME.store(true, atomic::Ordering::Relaxed);
        }) as Box<dyn Fn()>)
    };

    image.borrow_mut().set_onload(Some(onload.as_ref().unchecked_ref()));
    image.borrow_mut().set_onerror(Some(onerror.as_ref().unchecked_ref()));

    image.borrow_mut().set_cross_origin(Some(""));
    image.borrow_mut().set_src(&url);

    onload.forget();
}

fn create_sampler_3d(gl: &WebGl2Context, size_buffer: u32) -> (Option<web_sys::WebGlTexture>, u32) {
    let window = web_sys::window().unwrap();
    let document = window.document().unwrap();
    let canvas = document.create_element("canvas").unwrap();
    let canvas: web_sys::HtmlCanvasElement = canvas.dyn_into::<web_sys::HtmlCanvasElement>().unwrap();
    canvas.set_width(WIDTH_TEXTURE as u32);
    canvas.set_height((HEIGHT_TEXTURE as u32) * size_buffer);
    
    let ctx = Rc::new(
        RefCell::new(
            canvas.get_context("2d")
                .unwrap()
                .unwrap()
                .dyn_into::<CanvasRenderingContext2d>().unwrap()
        )
    );
    let webgl_texture = gl.create_texture();

    let idx_texture_unit = unsafe { NUM_TEXTURE_UNIT };
    unsafe {
        NUM_TEXTURE_UNIT += 1;
    }
    
    gl.active_texture(idx_texture_unit);
    gl.bind_texture(WebGl2RenderingContext::TEXTURE_3D, webgl_texture.as_ref());

    gl.tex_parameteri(WebGl2RenderingContext::TEXTURE_3D, WebGl2RenderingContext::TEXTURE_MIN_FILTER, WebGl2RenderingContext::NEAREST as i32);
    gl.tex_parameteri(WebGl2RenderingContext::TEXTURE_3D, WebGl2RenderingContext::TEXTURE_MAG_FILTER, WebGl2RenderingContext::NEAREST as i32);

    // Prevents s-coordinate wrapping (repeating)
    gl.tex_parameteri(WebGl2RenderingContext::TEXTURE_3D, WebGl2RenderingContext::TEXTURE_WRAP_S, WebGl2RenderingContext::CLAMP_TO_EDGE as i32);
    // Prevents t-coordinate wrapping (repeating)
    gl.tex_parameteri(WebGl2RenderingContext::TEXTURE_3D, WebGl2RenderingContext::TEXTURE_WRAP_T, WebGl2RenderingContext::CLAMP_TO_EDGE as i32);
    // Prevents r-coordinate wrapping (repeating)
    gl.tex_parameteri(WebGl2RenderingContext::TEXTURE_3D, WebGl2RenderingContext::TEXTURE_WRAP_R, WebGl2RenderingContext::CLAMP_TO_EDGE as i32);

    gl.tex_image_3d_with_html_canvas_element(
        WebGl2RenderingContext::TEXTURE_3D,
        0,
        WebGl2RenderingContext::RGB as i32,
        WIDTH_TEXTURE,
        HEIGHT_TEXTURE,
        size_buffer as i32,
        0,
        WebGl2RenderingContext::RGB,
        WebGl2RenderingContext::UNSIGNED_BYTE,
        &ctx.borrow().canvas().unwrap(),
    )
    .expect("Texture 3d");
    gl.generate_mipmap(WebGl2RenderingContext::TEXTURE_3D);

    (webgl_texture, idx_texture_unit)
}