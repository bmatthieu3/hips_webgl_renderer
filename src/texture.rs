use std::rc::Rc;
use std::cell::RefCell;

use wasm_bindgen::prelude::Closure;
use wasm_bindgen::JsCast;

use web_sys::HtmlImageElement;
use web_sys::WebGl2RenderingContext;

use std::collections::{BinaryHeap, HashSet};

const HEIGHT_TEXTURE: i32 = 512;
const WIDTH_TEXTURE: i32 = 512;

use crate::WebGl2Context;
use web_sys::console;
use crate::RENDER_FRAME;

static mut NUM_TEXTURE_UNIT: u32 = WebGl2RenderingContext::TEXTURE0;

#[derive(Clone)]
pub struct Tile {
    pub cell: HEALPixCell,

    pub texture_idx: u8,

    time_request: f32,
    time_received: Option<f32>,
}

impl Tile {
    pub fn new(cell: HEALPixCell) -> Tile {
        let texture_idx = 0;

        let time_request = 0_f32;
        let time_received = None;

        Tile {
            cell,

            texture_idx,

            time_request,
            time_received
        }
    }
}

// Equality impl allowing to store
// tiles in a set
impl PartialEq for Tile {
    fn eq(&self, other: &Self) -> bool {
        self.cell == other.cell
    }
}
impl Eq for Tile {}

use std::hash::{Hash, Hasher};
impl Hash for Tile {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.cell.hash(state);
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
pub struct TilePerPixelGPU {
    pub uniq: u32,

    pub texture_idx: i32,

    pub time_request: f32,
    pub time_received: f32,
}

impl PartialEq for TilePerPixelGPU {
    fn eq(&self, other: &Self) -> bool {
        self.uniq == other.uniq
    }
}
impl Eq for TilePerPixelGPU {}

impl PartialOrd for TilePerPixelGPU {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        // Order by UNIQ notation
        self.uniq.partial_cmp(&other.uniq)
    }
}
impl Ord for TilePerPixelGPU {
    fn cmp(&self, other: &Self) -> Ordering {
        self.partial_cmp(&other).unwrap()
    }
}

impl From<Tile> for TilePerPixelGPU {
    fn from(tile: Tile) -> Self {
        let depth = tile.cell.0;
        let idx = tile.cell.1;

        let uniq = (1 << (2*((depth as u64) + 1))) + idx;
        let uniq = uniq as u32;

        let texture_idx = tile.texture_idx as i32;

        let time_request = tile.time_request;
        let time_received = tile.time_received.unwrap();

        TilePerPixelGPU {
            uniq,

            texture_idx,

            time_request,
            time_received,
        }
    }
}

struct TileSmallFOVGPU {
    cell: HEALPixCell,

    pub texture_idx: i32,

    pub time_request: f32,
    pub time_received: f32,
}

impl PartialEq for TileSmallFOVGPU {
    fn eq(&self, other: &Self) -> bool {
        self.cell == other.cell
    }
}

impl Eq for TileSmallFOVGPU {}

impl PartialOrd for TileSmallFOVGPU {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        // Order by UNIQ notation
        self.cell.partial_cmp(&other.cell)
    }
}

impl Ord for TileSmallFOVGPU {
    fn cmp(&self, other: &Self) -> Ordering {
        self.partial_cmp(&other).unwrap()
    }
}

impl From<Tile> for TileSmallFOVGPU {
    fn from(tile: Tile) -> Self {
        let cell = tile.cell;

        let texture_idx = tile.texture_idx as i32;

        let time_request = tile.time_request;
        let time_received = tile.time_received.unwrap();

        TileSmallFOVGPU {
            cell,

            texture_idx,

            time_request,
            time_received,
        }
    }
}

#[derive(Clone)]
#[derive(Debug)]
pub struct TileRequest {
    pub cell: HEALPixCell,
    
    time_request: f32,

    image: Rc<RefCell<HtmlImageElement>>,
}

impl TileRequest {
    fn new(cell: HEALPixCell, time_request: f32, image: Rc<RefCell<HtmlImageElement>>) -> TileRequest {
        TileRequest {
            cell,
            time_request,
            image
        }
    }
}

impl PartialEq for TileRequest {
    fn eq(&self, other: &Self) -> bool {
        self.cell == other.cell
    }
}
impl Eq for TileRequest {}
// Requested for storing tiles in a BinaryHeap
use crate::renderable::hips_sphere::DEPTH;
use std::sync::atomic;
impl PartialOrd for TileRequest {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        let depth = DEPTH.load(atomic::Ordering::Relaxed) as i8;

        let delta_depth = (depth - (self.cell.0 as i8)).abs();
        let delta_depth_other = (depth - (other.cell.1 as i8)).abs();

        delta_depth.partial_cmp(&delta_depth_other)
    }
}
impl Ord for TileRequest {
    fn cmp(&self, other: &Self) -> Ordering {
        self.partial_cmp(&other).unwrap()
    }
}

#[derive(Clone)]
pub struct BufferTiles {
    gl: WebGl2Context,

    buffer: BinaryHeap<Tile>,
    loaded_tiles: HashSet<HEALPixCell>,
    // This binary heap contains the requests that must be canceled
    // This must be rebuilt whenever we change of order
    pub requested_tiles: BinaryHeap<TileRequest>,

    size: usize,
    idx_texture_unit: u32,

    texture: Option<web_sys::WebGlTexture>,
    texture_name: &'static str,
}

use crate::utils;
use crate::shader::Shader;
use std::convert::TryInto;
impl BufferTiles {
    pub fn new(gl: &WebGl2Context, size: usize, texture_name: &'static str) -> BufferTiles {
        let buffer = BinaryHeap::with_capacity(size);
        let loaded_tiles = HashSet::with_capacity(size);
        let requested_tiles = BinaryHeap::with_capacity(size);

        let (texture, idx_texture_unit) = create_texture_tile_buffer(gl);

        let gl = gl.clone();
        BufferTiles {
            gl,

            buffer,
            loaded_tiles,
            requested_tiles,

            size,

            idx_texture_unit,

            texture,
            texture_name,
        }
    }

    // Add a new tile to the buffer
    pub fn add(&mut self, cell: HEALPixCell, time_request: f32) -> u8 {
        let texture_idx = if self.buffer.len() == self.size {
            // Remove the oldest tile from the buffer and from the
            // hashset
            let oldest_requested_tile = self.buffer.pop().unwrap();

            let vacant_texture_idx = oldest_requested_tile.texture_idx;

            let tile = oldest_requested_tile.into();
            self.remove_from_loaded_tiles(&tile);

            vacant_texture_idx
        } else {
            self.buffer.len() as u8
        };

        let time_received = Some(utils::get_current_time());
        let tile = Tile {
            cell,

            texture_idx,

            time_request,
            time_received,
        };

        // Push it to the GPU buffer
        self.push_tile(tile);

        texture_idx
    }

    pub fn refresh_requested_tiles_heap(&mut self) {
        self.requested_tiles = self.requested_tiles
            .clone()
            .into_vec()
            .into_iter()
            .collect::<BinaryHeap<_>>();
    }

    fn push_tile(&mut self, tile: Tile) {
        self.buffer.push(tile);
        RENDER_FRAME.lock().unwrap().set_for_duration_seconds(500_f32);
    }

    pub fn add_to_loaded_tiles(&mut self, tile: HEALPixCell) {
        self.loaded_tiles.insert(tile);
    }

    pub fn remove_from_loaded_tiles(&mut self, tile: &HEALPixCell) {
        self.loaded_tiles.remove(&tile);
    }

    pub fn add_to_requested_tiles(&mut self, tile_request: TileRequest) {
        self.requested_tiles.push(tile_request);
    }

    pub fn remove_from_requested_tiles(&mut self, tile_request: TileRequest) {
        self.requested_tiles = self.requested_tiles
            .iter()
            .filter(|&tile| {
                *tile != tile_request
            })
            .cloned()
            .collect::<BinaryHeap<_>>();
    }

    pub fn cancel_obsolete_tile_requests(&mut self, depth: u8) {
        if !self.requested_tiles.is_empty() {
            let mut peek = self.requested_tiles.peek().unwrap();

            while (peek.cell.0 as i8 - (depth as i8)).abs() > 1 {
                peek.image.borrow_mut().set_src("");

                self.requested_tiles.pop();

                if self.requested_tiles.is_empty() {
                    break;
                }
                peek = self.requested_tiles.peek().unwrap();
            }
        }
    }

    pub fn replace_tile(&mut self, tile_loaded: HEALPixCell, time_request: f32, tile_request: &TileRequest, reset_time_received: bool) -> bool {
        // Check whether the buffer already contains the requested tile
        if self.loaded_tiles.contains(&tile_loaded) {
            // Change its priority in the buffer (if it is present!).
            let tile = self.buffer
                .iter()
                .find(|&tile| {
                    tile.cell == tile_loaded
                });

            if let Some(tile) = tile {
                //console::log_1(&format!("found healpix cell").into());
                // Found
                let mut tile = tile.clone();
                tile.time_request = time_request;
                if reset_time_received {
                    tile.time_received = Some(utils::get_current_time());
                }

                // Push it to the buffer again
                self.buffer = self.buffer.iter()
                    .filter(|&tile| {
                        tile.cell != tile_loaded
                    })
                    .cloned()
                    .collect();

                // Push it to the GPU buffer
                self.push_tile(tile);
            } else {
                unreachable!();
            }

            return true;
        }
        
        // The tile has been requested so we do launch a new
        // async request to alasky
        let is_requested = self.requested_tiles
            .iter()
            .find(|&tile| *tile == *tile_request);

        if let Some(_) = is_requested {
            return true;
        }

        // A new async must be launched
        false
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

    fn replace_texture_sampler_2d(&self, idx: i32, image: &HtmlImageElement) {
        self.gl.active_texture(self.idx_texture_unit);
        self.gl.bind_texture(WebGl2RenderingContext::TEXTURE_2D, self.texture.as_ref());

        let idx_row = idx / 8;
        let idx_col = idx % 8;

        let xoffset = idx_col * WIDTH_TEXTURE;
        let yoffset = idx_row * HEIGHT_TEXTURE;

        self.gl.tex_sub_image_2d_with_u32_and_u32_and_html_image_element(
            WebGl2RenderingContext::TEXTURE_2D,
            0,
            xoffset,
            yoffset,
            WebGl2RenderingContext::RGB,
            WebGl2RenderingContext::UNSIGNED_BYTE,
            image,
        )
        .expect("Sub texture 2d");
    }

    pub fn tiles(&self) -> BTreeSet<&Tile> {
        self.buffer
            .iter()
            .collect::<BTreeSet<_>>()
    }

    fn uniq_ordered_tiles(&self) -> Vec<TilePerPixelGPU> {
        let mut tiles = self.buffer
            .clone()
            .into_iter()
            .map(|tile| {
                tile.into()
            })
            .collect::<Vec<_>>();

        tiles.sort_unstable();

        tiles
    }

    fn send_texture(&self, shader: &Shader) {
        let location_sampler_3d = shader.get_uniform_location(self.texture_name);
        self.gl.active_texture(self.idx_texture_unit);
        //self.gl.bind_texture(WebGl2RenderingContext::TEXTURE_3D, self.texture.as_ref());
        self.gl.bind_texture(WebGl2RenderingContext::TEXTURE_2D, self.texture.as_ref());

        let idx_sampler: i32 = (self.idx_texture_unit - WebGl2RenderingContext::TEXTURE0).try_into().unwrap();
        self.gl.uniform1i(location_sampler_3d, idx_sampler);
    }

    pub fn send_to_shader(&self, shader: &Shader) {
        self.send_texture(shader);
        let tiles = self.uniq_ordered_tiles();

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

    pub fn len(&self) -> usize {
        self.size
    }
}
/*
pub struct BufferTilesSmallFOV<'a> {
    tiles: &'a BTreeSet<TileSmallFOVGPU>
}

impl<'a> BufferTilesSmallFOV<'a> {
    fn new(tiles: &'a BTreeSet<TileSmallFOVGPU>) -> BufferTilesSmallFOV<'a> {
        BufferTilesSmallFOV {
            tiles
        }
    }

    fn get(&self, cell: HEALPixCell) -> Option<&TileSmallFOVGPU> {
        let time_request = 0_f32;
        let time_received = 0_f32;

        let texture_idx = 0;
        let tile = TileSmallFOVGPU {
            cell,

            texture_idx,

            time_request,
            time_received,
        };
        self.tiles.get(&tile)
    }
}
*/
use crate::HIPS_NAME;
use crate::field_of_view::HEALPixCell;
use std::collections::BTreeSet;
pub fn load_tiles(
    buffer_tiles: Rc<RefCell<BufferTiles>>,
    tiles: &BTreeSet<HEALPixCell>,
    depth: u8,
    reset_time_received: bool
) {
    //buffer_tiles.borrow_mut().prepare_for_loading(tiles_idx.len() as u8);
    // If the depth has just changed, we must rebuild the
    // requested tiles binary heap
    if reset_time_received {
        buffer_tiles.borrow_mut().refresh_requested_tiles_heap();
    }
    // And cancel the oldest async tile requests i.e. of depth
    // > current_depth + 1 and < current_depth - 1
    buffer_tiles.borrow_mut().cancel_obsolete_tile_requests(depth);
    for tile in tiles {
        load_healpix_tile(buffer_tiles.clone(), tile.clone(), reset_time_received);
    }
}
fn load_healpix_tile(buffer: Rc<RefCell<BufferTiles>>, tile: HEALPixCell, reset_time_received: bool) {
    let time_request = utils::get_current_time();

    let image = Rc::new(RefCell::new(HtmlImageElement::new().unwrap()));
    let tile_request = TileRequest::new(tile, time_request, image.clone());
    // Check whether the tile is already in the buffer or requested
    if buffer.borrow_mut().replace_tile(tile, time_request, &tile_request, reset_time_received) {
        return;
    }

    // Here we know we have to launch a new async request
    buffer.borrow_mut().add_to_requested_tiles(tile_request);

    let url = {
        let depth = tile.0;
        let idx = tile.1;
        
        let dir_idx = (idx / 10000) * 10000;

        let mut url = HIPS_NAME.lock().unwrap().clone() + "/";
        url = url + "Norder" + &depth.to_string() + "/";
        url = url + "Dir" + &dir_idx.to_string() + "/";
        url = url + "Npix" + &idx.to_string() + ".jpg";

        url
    };
    let onerror = {
        let buffer = buffer.clone();
        let image = image.clone();

        Closure::wrap(Box::new(move || {
            console::log_1(&format!("ERROR tile").into());
            //buffer.borrow_mut().remove_from_loaded_tiles(depth, idx);
            // Remove from the currently requested tiles
            let tile_request = TileRequest::new(tile, time_request, image.clone());
            buffer.borrow_mut().remove_from_requested_tiles(tile_request);
        }) as Box<dyn Fn()>)
    };

    let onload = {
        let image = image.clone();

        Closure::wrap(Box::new(move || {
            console::log_1(&format!("load new tile").into());
            // Remove from the currently requested tiles
            let tile_request = TileRequest::new(tile, time_request, image.clone());
            buffer.borrow_mut().remove_from_requested_tiles(tile_request);

            // Add it to the loaded cells hashset
            buffer.borrow_mut().add_to_loaded_tiles(tile);
            // Add the received tile to the buffer
            let idx_texture = buffer.borrow_mut().add(tile, time_request);
            //buffer.borrow().replace_texture_sampler_3d(idx_texture as i32, &image.borrow());
            buffer.borrow().replace_texture_sampler_2d(idx_texture as i32, &image.borrow());

            // Tell the app to render the next frame
            // because a a new has been received
            //RENDER_FRAME.store(true, atomic::Ordering::Relaxed);
        }) as Box<dyn Fn()>)
    };

    image.borrow_mut().set_onload(Some(onload.as_ref().unchecked_ref()));
    image.borrow_mut().set_onerror(Some(onerror.as_ref().unchecked_ref()));

    image.borrow_mut().set_cross_origin(Some(""));
    image.borrow_mut().set_src(&url);

    onload.forget();
    onerror.forget();
}

// Create a 4096x4096 texture that contains 8*8 tiles
fn create_texture_tile_buffer(gl: &WebGl2Context) -> (Option<web_sys::WebGlTexture>, u32) {
    let window = web_sys::window().unwrap();
    let document = window.document().unwrap();
    let canvas = document.create_element("canvas").unwrap();
    let canvas: web_sys::HtmlCanvasElement = canvas.dyn_into::<web_sys::HtmlCanvasElement>().unwrap();

    canvas.set_width((WIDTH_TEXTURE as u32) * 8);
    canvas.set_height((HEIGHT_TEXTURE as u32) * 8);

    let idx_texture_unit = unsafe { NUM_TEXTURE_UNIT };
    unsafe {
        NUM_TEXTURE_UNIT += 1;
    }
    let webgl_texture = gl.create_texture();
    gl.active_texture(idx_texture_unit);

    gl.bind_texture(WebGl2RenderingContext::TEXTURE_2D, webgl_texture.as_ref());

    gl.tex_parameteri(WebGl2RenderingContext::TEXTURE_2D, WebGl2RenderingContext::TEXTURE_MAG_FILTER, WebGl2RenderingContext::NEAREST as i32);
    gl.tex_parameteri(WebGl2RenderingContext::TEXTURE_2D, WebGl2RenderingContext::TEXTURE_MIN_FILTER, WebGl2RenderingContext::NEAREST as i32);

    // Prevents s-coordinate wrapping (repeating)
    gl.tex_parameteri(WebGl2RenderingContext::TEXTURE_2D, WebGl2RenderingContext::TEXTURE_WRAP_S, WebGl2RenderingContext::CLAMP_TO_EDGE as i32);
    // Prevents t-coordinate wrapping (repeating)
    gl.tex_parameteri(WebGl2RenderingContext::TEXTURE_2D, WebGl2RenderingContext::TEXTURE_WRAP_T, WebGl2RenderingContext::CLAMP_TO_EDGE as i32);

    gl.tex_image_2d_with_u32_and_u32_and_html_canvas_element(
        WebGl2RenderingContext::TEXTURE_2D,
        0,
        WebGl2RenderingContext::RGB as i32,
        WebGl2RenderingContext::RGB,
        WebGl2RenderingContext::UNSIGNED_BYTE,
        &canvas,
    )
    .expect("Texture 2d");
    //gl.generate_mipmap(WebGl2RenderingContext::TEXTURE_2D);

    (webgl_texture, idx_texture_unit)
}

fn create_sampler_3d(gl: &WebGl2Context, size_buffer: u32) -> (Option<web_sys::WebGlTexture>, u32) {
    let window = web_sys::window().unwrap();
    let document = window.document().unwrap();
    let canvas = document.create_element("canvas").unwrap();
    let canvas: web_sys::HtmlCanvasElement = canvas.dyn_into::<web_sys::HtmlCanvasElement>().unwrap();
    canvas.set_width(WIDTH_TEXTURE as u32);
    canvas.set_height((HEIGHT_TEXTURE as u32) * size_buffer);

    let idx_texture_unit = unsafe { NUM_TEXTURE_UNIT };
    unsafe {
        NUM_TEXTURE_UNIT += 1;
    }
    let webgl_texture = gl.create_texture();
    gl.active_texture(idx_texture_unit);

    gl.bind_texture(WebGl2RenderingContext::TEXTURE_3D, webgl_texture.as_ref());

    gl.tex_parameteri(WebGl2RenderingContext::TEXTURE_3D, WebGl2RenderingContext::TEXTURE_MIN_FILTER, WebGl2RenderingContext::LINEAR as i32);
    gl.tex_parameteri(WebGl2RenderingContext::TEXTURE_3D, WebGl2RenderingContext::TEXTURE_MAG_FILTER, WebGl2RenderingContext::LINEAR as i32);

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
        &canvas,
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

    pub fn create(gl: &WebGl2Context, src: &'static str) -> Texture2D {
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

                gl.tex_parameteri(WebGl2RenderingContext::TEXTURE_2D, WebGl2RenderingContext::TEXTURE_MIN_FILTER, WebGl2RenderingContext::LINEAR as i32);

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
                //gl.generate_mipmap(WebGl2RenderingContext::TEXTURE_2D);
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

    pub fn create_empty(gl: &WebGl2Context, width: i32, height: i32) -> Texture2D {
        let webgl_texture = gl.create_texture();
        let idx_texture_unit = unsafe { NUM_TEXTURE_UNIT };
        unsafe {
            NUM_TEXTURE_UNIT += 1;
        }
        gl.active_texture(idx_texture_unit);

        gl.bind_texture(WebGl2RenderingContext::TEXTURE_2D, webgl_texture.as_ref());

        gl.tex_parameteri(WebGl2RenderingContext::TEXTURE_2D, WebGl2RenderingContext::TEXTURE_MIN_FILTER, WebGl2RenderingContext::LINEAR as i32);

        // Prevents s-coordinate wrapping (repeating)
        gl.tex_parameteri(WebGl2RenderingContext::TEXTURE_2D, WebGl2RenderingContext::TEXTURE_WRAP_S, WebGl2RenderingContext::CLAMP_TO_EDGE as i32);
        // Prevents t-coordinate wrapping (repeating)
        gl.tex_parameteri(WebGl2RenderingContext::TEXTURE_2D, WebGl2RenderingContext::TEXTURE_WRAP_T, WebGl2RenderingContext::CLAMP_TO_EDGE as i32);

        gl.tex_image_2d_with_i32_and_i32_and_i32_and_format_and_type_and_opt_u8_array(
            WebGl2RenderingContext::TEXTURE_2D,
            0,
            WebGl2RenderingContext::RGB as i32,
            width,
            height,
            0,
            WebGl2RenderingContext::RGB,
            WebGl2RenderingContext::UNSIGNED_BYTE,
            None
        ).expect("Texture 2D");

        Texture2D::new(Rc::new(RefCell::new(webgl_texture)), idx_texture_unit)
    }

    pub fn attach_to_framebuffer(&self, gl: &WebGl2Context) {
        gl.framebuffer_texture_2d(
            WebGl2RenderingContext::FRAMEBUFFER,
            WebGl2RenderingContext::COLOR_ATTACHMENT0,
            WebGl2RenderingContext::TEXTURE_2D,
            self.texture.borrow().as_ref(),
            0
        );
    }

    pub fn send_to_shader(&self, gl: &WebGl2Context, shader: &Shader, name: &'static str) {
        let location_tex = shader.get_uniform_location(name);
        gl.active_texture(self.idx_texture_unit);
        gl.bind_texture(WebGl2RenderingContext::TEXTURE_2D, self.texture.borrow().as_ref());

        let idx_sampler: i32 = (self.idx_texture_unit - WebGl2RenderingContext::TEXTURE0).try_into().unwrap();
        gl.uniform1i(location_tex, idx_sampler);
    }
}