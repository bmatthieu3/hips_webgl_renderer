use std::rc::Rc;
use std::cell::RefCell;

use wasm_bindgen::prelude::Closure;
use wasm_bindgen::JsCast;

use web_sys::{HtmlImageElement, CanvasRenderingContext2d};
use web_sys::WebGl2RenderingContext;

use std::collections::{BinaryHeap, HashSet};

const HEIGHT_TEXTURE: i32 = 512;
const WIDTH_TEXTURE: i32 = 512;

use crate::WebGl2Context;
use web_sys::console;
use crate::RENDER_NEXT_FRAME;

static mut NUM_TEXTURE_UNIT: u32 = WebGl2RenderingContext::TEXTURE0;

#[derive(Clone)]
struct Tile {
    idx: u64,
    depth: u8,

    texture_idx: usize,

    time_request: f32,
    time_received: Option<f32>,
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

#[derive(Debug)]
pub struct TileGPU {
    pub uniq: u32,

    pub texture_idx: i32,

    pub time_request: f32,
    pub time_received: f32,
}

impl PartialEq for TileGPU {
    fn eq(&self, other: &Self) -> bool {
        self.uniq == other.uniq
    }
}
impl Eq for TileGPU {}

impl PartialOrd for TileGPU {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        // Order by UNIQ notation
        self.uniq.partial_cmp(&other.uniq)
    }
}
impl Ord for TileGPU {
    fn cmp(&self, other: &Self) -> Ordering {
        self.partial_cmp(&other).unwrap()
    }
}

impl From<Tile> for TileGPU {
    fn from(tile: Tile) -> Self {
        let idx = tile.idx;
        let depth = tile.depth;
        let uniq = (1 << (2*((depth as u64) + 1))) + idx;
        let uniq = uniq as u32;

        let texture_idx = tile.texture_idx as i32;

        let time_request = tile.time_request;
        let time_received = tile.time_received.unwrap();

        TileGPU {
            uniq,

            texture_idx,

            time_request,
            time_received,
        }
    }
}

/*use std::sync::Arc;
use std::sync::atomic::AtomicU8;
lazy_static! {
    static ref num_load_tiles: Arc<AtomicU8> = Arc::new(AtomicU8::new(0));
}*/

use web_sys::Storage;

#[derive(Clone)]
pub struct BufferTiles {
    gl: WebGl2Context,

    buffer: BinaryHeap<Tile>,
    requested_tiles: HashSet<(u8, u64)>,
    size: usize,
    idx_texture_unit: u32,

    texture: Option<web_sys::WebGlTexture>,
    texture_name: &'static str,

    num_load_tiles: u8,
    num_tiles_to_load: u8,
}

use crate::utils;
use crate::shader::Shader;
use std::convert::TryInto;
impl BufferTiles {
    pub fn new(gl: &WebGl2Context, size: usize, texture_name: &'static str) -> BufferTiles {
        let buffer = BinaryHeap::with_capacity(size);
        let requested_tiles = HashSet::with_capacity(size);

        let (texture, idx_texture_unit) = create_sampler_3d(gl, size as u32);

        let gl = gl.clone();
        let num_load_tiles = 0;
        let num_tiles_to_load = size as u8;
        BufferTiles {
            gl,

            buffer,

            requested_tiles,
            size,
            idx_texture_unit,

            texture,
            texture_name,

            num_load_tiles,
            num_tiles_to_load,
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

        // Push it to the GPU buffer
        self.num_load_tiles += 1;
        self.push_tile(tile);

        texture_idx
    }

    pub fn prepare_for_loading(&mut self, num_tiles_to_load: u8) {
        self.num_load_tiles = 0;
        self.num_tiles_to_load = num_tiles_to_load;
    }

    fn push_tile(&mut self, tile: Tile) {
        self.buffer.push(tile);

        // if all the requested tiles are all in the buffer
        // i.e. if there is no more tiles to load
        if self.num_tiles_to_load == 0 {
            return;
        }

        if self.num_load_tiles == self.num_tiles_to_load {
            // Do not render next frame
            RENDER_NEXT_FRAME.lock().unwrap().set_for_duration_seconds(500_f32);
        }
    }

    pub fn add_to_requested_tile(&mut self, depth: u8, idx: u64) {
        let tile = (depth, idx);
        self.requested_tiles.insert(tile);
    }

    pub fn remove_from_requested_tile(&mut self, depth: u8, idx: u64) {
        let tile = (depth, idx);
        self.requested_tiles.remove(&tile);
    }

    pub fn replace_tile(&mut self, depth: u8, idx: u64, time_request: f32, reset_time_received: bool) -> bool {
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
                //console::log_1(&format!("found healpix cell").into());
                // Found
                tile.time_request = time_request;
                if reset_time_received {
                    tile.time_received = Some(utils::get_current_time());
                }
                //tile.time_received = Some(utils::get_current_time());

                // Push it to the buffer again
                let mut buffer = BinaryHeap::new();
                for t in self.buffer.iter() {
                    if t.depth != depth || t.idx != idx {
                        buffer.push(t.clone());
                    }
                }
                
                self.buffer = buffer;

                // Push it to the GPU buffer
                self.num_tiles_to_load -= 1;
                self.push_tile(tile);
            }

            true
        } else {
            false
        }
    }

    fn replace_texture_sampler_3d(&self, idx: i32, image: &HtmlImageElement) {
        self.gl.active_texture(self.idx_texture_unit);
        self.gl.bind_texture(WebGl2RenderingContext::TEXTURE_3D, self.texture.as_ref());
        self.gl.tex_sub_image_3d_with_html_image_element(
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

    fn tiles(&self) -> Vec<TileGPU> {
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

    fn send_sampler_uniform(&self, shader: &Shader) {
        let location_sampler_3d = shader.get_uniform_location(self.texture_name);
        self.gl.active_texture(self.idx_texture_unit);
        self.gl.bind_texture(WebGl2RenderingContext::TEXTURE_3D, self.texture.as_ref());

        let idx_sampler: i32 = (self.idx_texture_unit - WebGl2RenderingContext::TEXTURE0).try_into().unwrap();
        self.gl.uniform1i(location_sampler_3d, idx_sampler);
    }

    pub fn send_to_shader(&self, shader: &Shader) {
        self.send_sampler_uniform(shader);
        let tiles = self.tiles();

        for (i, tile) in tiles.iter().enumerate() {
            let mut name = String::from(self.texture_name);
            name += "_tiles";
            name += "[";
            name += &i.to_string();
            name += "].";

            let location_hpx_idx = shader.get_uniform_location(&(name.clone() + "uniq"));
            self.gl.uniform1ui(location_hpx_idx, tile.uniq);

            let location_buf_idx = shader.get_uniform_location(&(name.clone() + "texture_idx"));
            self.gl.uniform1i(location_buf_idx, tile.texture_idx);

            let location_time_received = shader.get_uniform_location(&(name.clone() + "time_received"));
            self.gl.uniform1f(location_time_received, tile.time_received);

            let location_time_request = shader.get_uniform_location(&(name + "time_request"));
            self.gl.uniform1f(location_time_request, tile.time_request);
        }
    }
}

use crate::HIPS_NAME;
pub fn load_healpix_tile(gl: &WebGl2Context, buffer: Rc<RefCell<BufferTiles>>, idx: u64, depth: u8, reset_time_received: bool) {
    let time_request = utils::get_current_time();
    
    // Check whether is already into the 24 tiles GPU buffer
    if buffer.borrow_mut().replace_tile(depth, idx, time_request, reset_time_received) {
        return;
    }

    //let uniq = idx + (1 << (2*(depth + 1)));
    //if STORAGE.lock().unwrap().get_tile_image(uniq)

    // Add it to the loaded cells hashset
    buffer.borrow_mut().add_to_requested_tile(depth, idx);

    let url = {
        let dir_idx = (idx / 10000) * 10000;

        let mut url = HIPS_NAME.lock().unwrap().clone() + "/";
        url = url + "Norder" + &depth.to_string() + "/";
        url = url + "Dir" + &dir_idx.to_string() + "/";
        url = url + "Npix" + &idx.to_string() + ".jpg";

        url
    };
    let onerror = {
        let buffer = buffer.clone();

        Closure::wrap(Box::new(move || {
            console::log_1(&format!("ERROR tile").into());
            buffer.borrow_mut().remove_from_requested_tile(depth, idx);
        }) as Box<dyn Fn()>)
    };

    let image = Rc::new(RefCell::new(HtmlImageElement::new().unwrap()));

    let onload = {
        let image = image.clone();

        Closure::wrap(Box::new(move || {
            console::log_1(&format!("load new tile").into());
            // Add the received tile to the buffer
            let idx_texture = buffer.borrow_mut().add(depth, idx, time_request);
            buffer.borrow().replace_texture_sampler_3d(idx_texture as i32, &image.borrow());

            // Tell the app to render the next frame
            // because a a new has been received
            //RENDER_NEXT_FRAME.store(true, atomic::Ordering::Relaxed);
        }) as Box<dyn Fn()>)
    };

    image.borrow_mut().set_onload(Some(onload.as_ref().unchecked_ref()));
    image.borrow_mut().set_onerror(Some(onerror.as_ref().unchecked_ref()));

    image.borrow_mut().set_cross_origin(Some(""));
    image.borrow_mut().set_src(&url);

    onload.forget();
    onerror.forget();
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

    gl.tex_parameteri(WebGl2RenderingContext::TEXTURE_3D, WebGl2RenderingContext::TEXTURE_MIN_FILTER, WebGl2RenderingContext::LINEAR as i32);
    gl.tex_parameteri(WebGl2RenderingContext::TEXTURE_3D, WebGl2RenderingContext::TEXTURE_MAG_FILTER, WebGl2RenderingContext::LINEAR_MIPMAP_LINEAR as i32);

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

use web_sys::WebGlTexture;
#[derive(Clone)]
pub struct Texture2D {
    texture: Rc<RefCell<Option<WebGlTexture>>>,
    idx_texture_unit: u32,
}

impl Texture2D {
    fn new(texture: Rc<RefCell<Option<WebGlTexture>>>, idx_texture_unit: u32) -> Texture2D {
        Texture2D {
            texture,
            idx_texture_unit
        }
    }

    pub fn send_to_shader(&self, gl: &WebGl2Context, shader: &Shader, name: &'static str) {
        let location_tex = shader.get_uniform_location(name);
        gl.active_texture(self.idx_texture_unit);
        gl.bind_texture(WebGl2RenderingContext::TEXTURE_2D, self.texture.borrow().as_ref());

        let idx_sampler: i32 = (self.idx_texture_unit - WebGl2RenderingContext::TEXTURE0).try_into().unwrap();
        gl.uniform1i(location_tex, idx_sampler);
    }
}

pub fn create_texture_2d(gl: &WebGl2Context, src: &'static str) -> Texture2D {
    let image = Rc::new(RefCell::new(HtmlImageElement::new().unwrap()));

    let webgl_texture = Rc::new(RefCell::new(gl.create_texture()));
    let idx_texture_unit = unsafe { NUM_TEXTURE_UNIT };

    unsafe {
        NUM_TEXTURE_UNIT += 1;
    }
    let onerror = {
        Closure::wrap(Box::new(move || {
            console::log_1(&format!("Cannot load texture located at: {:?}", src).into());
        }) as Box<dyn Fn()>)
    };

    let onload = {
        let image = image.clone();
        let gl = gl.clone();
        let webgl_texture = webgl_texture.clone();

        Closure::wrap(Box::new(move || {
            gl.active_texture(idx_texture_unit);
            gl.bind_texture(WebGl2RenderingContext::TEXTURE_2D, webgl_texture.borrow().as_ref());

            gl.tex_parameteri(WebGl2RenderingContext::TEXTURE_2D, WebGl2RenderingContext::TEXTURE_MIN_FILTER, WebGl2RenderingContext::NEAREST as i32);
            gl.tex_parameteri(WebGl2RenderingContext::TEXTURE_2D, WebGl2RenderingContext::TEXTURE_MAG_FILTER, WebGl2RenderingContext::NEAREST as i32);

            // Prevents s-coordinate wrapping (repeating)
            gl.tex_parameteri(WebGl2RenderingContext::TEXTURE_2D, WebGl2RenderingContext::TEXTURE_WRAP_S, WebGl2RenderingContext::CLAMP_TO_EDGE as i32);
            // Prevents t-coordinate wrapping (repeating)
            gl.tex_parameteri(WebGl2RenderingContext::TEXTURE_2D, WebGl2RenderingContext::TEXTURE_WRAP_T, WebGl2RenderingContext::CLAMP_TO_EDGE as i32);

            gl.tex_image_2d_with_u32_and_u32_and_html_image_element(
                WebGl2RenderingContext::TEXTURE_2D,
                0,
                WebGl2RenderingContext::RGBA as i32,
                WebGl2RenderingContext::RGBA,
                WebGl2RenderingContext::UNSIGNED_BYTE,
                &image.borrow()
            ).expect("Texture 2D");
            gl.generate_mipmap(WebGl2RenderingContext::TEXTURE_2D);
        }) as Box<dyn Fn()>)
    };

    image.borrow_mut().set_onload(Some(onload.as_ref().unchecked_ref()));
    image.borrow_mut().set_onerror(Some(onerror.as_ref().unchecked_ref()));

    image.borrow_mut().set_cross_origin(Some(""));
    image.borrow_mut().set_src(src);

    onload.forget();
    onerror.forget();

    Texture2D::new(webgl_texture, idx_texture_unit)
}