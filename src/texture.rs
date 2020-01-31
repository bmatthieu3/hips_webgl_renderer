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
use crate::LATEST_TIME_TILE_RECEIVED;

static mut NUM_TEXTURE_UNIT: u32 = WebGl2RenderingContext::TEXTURE0;

#[derive(Clone, Copy)]
pub struct Tile {
    pub cell: HEALPixCell,

    pub texture_idx: u8,

    time_request: f32,
    time_received: Option<f32>,
}

pub const BLENDING_DURATION_MS: f32 = 500_f32;
impl Tile {
    pub fn new(cell: HEALPixCell) -> Tile {
        let texture_idx = if cell.0 == 0 {
            cell.1 as u8
        } else {
            0
        };

        let time_request = 0_f32;
        let time_received = None;

        Tile {
            cell,

            texture_idx,

            time_request,
            time_received
        }
    }

    pub fn blending_factor(&self) -> f32 {
        if let Some(time_received) = self.time_received {
            let mut t = (utils::get_current_time() - time_received) / BLENDING_DURATION_MS;

            if t > 1_f32 {
                t = 1_f32;
            } else if t < 0_f32 {
                t = 0_f32;
            }

            t
        } else {
            0_f32
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
#[derive(Clone, Copy)]
pub struct TilePerPixelGPU {
    pub uniq: u32,

    pub texture_idx: i32,

    pub time_request: f32,
    pub time_received: f32,
}

impl TilePerPixelGPU {
    fn new() -> TilePerPixelGPU {
        let uniq = std::u32::MAX;
        let texture_idx = 0;
        let time_request = std::f32::MIN;
        let time_received = std::f32::MIN;
        TilePerPixelGPU {
            uniq,
            texture_idx,
            time_request,
            time_received,
        }
    }
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
        let time_received = if let Some(time_received) = tile.time_received {
            time_received
        } else {
            time_request
        };

        TilePerPixelGPU {
            uniq,

            texture_idx,

            time_request,
            time_received,
        }
    }
}

impl From<&Tile> for TilePerPixelGPU {
    fn from(tile: &Tile) -> Self {
        let depth = tile.cell.0;
        let idx = tile.cell.1;

        let uniq = (1 << (2*((depth as u64) + 1))) + idx;
        let uniq = uniq as u32;

        let texture_idx = tile.texture_idx as i32;

        let time_request = tile.time_request;
        let time_received = if let Some(time_received) = tile.time_received {
            time_received
        } else {
            time_request
        };

        TilePerPixelGPU {
            uniq,

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

pub struct BufferTiles {
    gl: WebGl2Context,

    base_tiles: [Tile; 12],

    buffer: BinaryHeap<Tile>,
    loaded_tiles: HashSet<HEALPixCell>,
    // This binary heap contains the requests that must be canceled
    // This must be rebuilt whenever we change of order
    pub requested_tiles: BinaryHeap<TileRequest>,

    size_binary_heap: usize,

    // Lots of machines are limited to a MAX_TEXTURE_SIZE of
    // 4096x4096. We thus create more textures to size up the tile buffer.
    // A further improve would be to take into account the MAX_TEXTURE_SIZE constant
    // to make it work on more architectures.
    texture_stack: [Texture2D; 2],
    //texture: Texture2D,
}

use crate::utils;
use crate::shader::Shader;
use std::convert::TryInto;
impl BufferTiles {
    pub fn new(gl: &WebGl2Context) -> BufferTiles {
        // The buffer will always contain the 12 base cells of depth 0
        // Therefore there is size - 12 tiles to handle in the binary heap.
        let size = 128;
        let size_binary_heap = size - 12;

        let buffer = BinaryHeap::with_capacity(size_binary_heap);
        let loaded_tiles = HashSet::with_capacity(size_binary_heap);
        let requested_tiles = BinaryHeap::with_capacity(size_binary_heap);

        let texture = Texture2D::create_empty(
            gl,
            WIDTH_TEXTURE * 8,
            HEIGHT_TEXTURE * 8,
            &[
                // The HiPS tiles sampling is NEAREST
                (WebGl2RenderingContext::TEXTURE_MIN_FILTER, WebGl2RenderingContext::NEAREST),
                (WebGl2RenderingContext::TEXTURE_MAG_FILTER, WebGl2RenderingContext::NEAREST),
                
                // Prevents s-coordinate wrapping (repeating)
                (WebGl2RenderingContext::TEXTURE_WRAP_S, WebGl2RenderingContext::CLAMP_TO_EDGE),
                // Prevents t-coordinate wrapping (repeating)
                (WebGl2RenderingContext::TEXTURE_WRAP_T, WebGl2RenderingContext::CLAMP_TO_EDGE),
            ]
        );
        let texture_cloned = texture.clone();

        let texture_stack = [
            texture,
            texture_cloned,
        ];

        // Load base tiles
        let base_tiles = [Tile::new(HEALPixCell(0, 0)); 12];

        let gl = gl.clone();
        let mut buffer_tiles = BufferTiles {
            gl,

            base_tiles,
            buffer,

            loaded_tiles,
            requested_tiles,

            size_binary_heap,

            texture_stack,
        };

        buffer_tiles
    }

    // Add a new tile to the buffer
    pub fn add(&mut self, cell: HEALPixCell, time_request: f32) -> u8 {
        let texture_idx = if self.buffer.len() == self.size_binary_heap {
            // Remove the oldest tile from the buffer and from the
            // hashset
            let oldest_requested_tile = self.buffer.pop().unwrap();

            let vacant_texture_idx = oldest_requested_tile.texture_idx;

            let tile = oldest_requested_tile.into();
            self.remove_from_loaded_tiles(&tile);

            vacant_texture_idx
        } else {
            (self.buffer.len() as u8) + 12
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
        *LATEST_TIME_TILE_RECEIVED.lock().unwrap() = utils::get_current_time();
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

    pub fn replace_tile(&mut self, tile_loaded: HEALPixCell, time_request: f32, tile_request: &TileRequest, depth_changed: bool) -> bool {
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
                if depth_changed {
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

    /*pub fn tiles(&self) -> BTreeSet<TilePerPixelGPU> {
        let refreshed_tiles: BTreeSet<TilePerPixelGPU> = self.buffer
            .iter()
            .map(|tile| {
                tile.into()
            })
            .collect::<BTreeSet<_>>();

        let base_tiles: BTreeSet<TilePerPixelGPU> = self.base_tiles
            .iter()
            .map(|tile| {
                tile.into()
            })
            .collect::<BTreeSet<_>>();

        refreshed_tiles.union(&base_tiles)
            .cloned()
            .collect::<BTreeSet<_>>()
    }*/

    pub fn get(&self, tile: &HEALPixCell) -> Option<&Tile> {
        let tile_buffer = Tile::new(*tile);

        let tile = self.buffer
            .iter()
            .find(|&&x| x == tile_buffer);
        
        if tile.is_some() {
            tile
        } else {
            self.base_tiles
                .iter()
                .find(|&&x| x == tile_buffer)
        }
    }

    fn uniq_ordered_tiles(&self) -> [TilePerPixelGPU; 128] {
        let mut tiles = [TilePerPixelGPU::new(); 128];

        for (i, tile) in self.buffer.iter().enumerate() {
            tiles[i] = tile.into();
        }

        let off = self.buffer.len();
        for (j, tile) in self.base_tiles.iter().enumerate() {
            tiles[off + j] = tile.into();
        }

        /*let mut tiles = self.buffer
            .iter()
            .map(|tile| {
                tile.into()
            })
            .collect::<Vec<_>>();

        let base_tiles = self.base_tiles
            .iter()
            .map(|tile| {
                tile.into()
            }).collect::<Vec<_>>();
        
        tiles.extend(base_tiles);*/

        tiles.sort();
        console::log_1(&format!("tiles LENGTH {:?}", tiles.len()).into());

        tiles
    }

    fn send_texture(&self, shader: &Shader) {
        //console::log_1(&format!("get uniform textures").into());

        for (i, texture) in self.texture_stack.iter().enumerate() {
            let mut location_tex_name = String::from("textures");
            location_tex_name += "[";
            location_tex_name += &i.to_string();
            location_tex_name += "]";
            let location_texture = shader.get_uniform_location(&location_tex_name);

            let (texture_unit, webgl_texture) = (texture.idx_texture_unit, texture.texture.clone());

            self.gl.active_texture(texture_unit);
            self.gl.bind_texture(WebGl2RenderingContext::TEXTURE_2D, webgl_texture.borrow().as_ref());
    
            let idx_texture = (texture_unit - WebGl2RenderingContext::TEXTURE0).try_into().unwrap();
            self.gl.uniform1i(location_texture, idx_texture);
        }
    }

    pub fn send_to_shader(&self, shader: &Shader) {
        self.send_texture(shader);
        let tiles = self.uniq_ordered_tiles();
        for (i, tile) in tiles.iter().enumerate() {
            let mut name = String::from("textures");
            name += "_tiles";
            name += "[";
            name += &i.to_string();
            name += "].";
            let location_hpx_idx = shader.get_uniform_location(&(name.clone() + "uniq"));
            self.gl.uniform1i(location_hpx_idx, tile.uniq as i32);

            let location_buf_idx = shader.get_uniform_location(&(name.clone() + "texture_idx"));
            self.gl.uniform1i(location_buf_idx, tile.texture_idx);

            let location_time_received = shader.get_uniform_location(&(name.clone() + "time_received"));
            self.gl.uniform1f(location_time_received, tile.time_received);

            let location_time_request = shader.get_uniform_location(&(name + "time_request"));
            self.gl.uniform1f(location_time_request, tile.time_request);
        }
    }

    pub fn len(&self) -> usize {
        self.size_binary_heap + 12
    }

    pub fn len_variable_tiles(&self) -> usize {
        self.size_binary_heap
    }

    pub fn clear(&mut self) {
        // TODO: Clear the texture
        self.buffer.clear();
        self.loaded_tiles.clear();
        self.requested_tiles.clear();
    }
}

use crate::HIPS_NAME;
use crate::field_of_view::HEALPixCell;

pub fn load_base_tiles(gl: &WebGl2Context, buffer: Rc<RefCell<BufferTiles>>) {
    for idx in 0..12 {
        let image = Rc::new(RefCell::new(HtmlImageElement::new().unwrap()));
        let time_request = utils::get_current_time();

        let url = {
            let depth = 0;
            
            let dir_idx = (idx / 10000) * 10000;

            let mut url = HIPS_NAME.lock().unwrap().clone() + "/";
            url = url + "Norder0/";
            url = url + "Dir0/";
            url = url + "Npix" + &idx.to_string() + ".jpg";

            url
        };
        let onerror = {
            let image = image.clone();

            Closure::wrap(Box::new(move || {
                console::log_1(&format!("ERROR base tile").into());
            }) as Box<dyn Fn()>)
        };

        let onload = {
            let buffer = buffer.clone();
            let image = image.clone();
            let gl = gl.clone();

            Closure::wrap(Box::new(move || {
                //console::log_1(&format!("load new tile").into());
                // Add it to the loaded cells hashset
                // Add the received tile to the buffer
                let texture_idx = idx as u8;
                let cell = HEALPixCell(0, idx as u64);
                let time_received = Some(utils::get_current_time());

                buffer.borrow_mut().base_tiles[idx] = Tile {
                    cell,

                    texture_idx,

                    time_request,
                    time_received,
                };

                replace_texture_sampler_2d(&gl, &buffer.borrow().texture_stack, texture_idx as i32, &image.borrow());
            }) as Box<dyn Fn()>)
        };

        image.borrow_mut().set_onload(Some(onload.as_ref().unchecked_ref()));
        image.borrow_mut().set_onerror(Some(onerror.as_ref().unchecked_ref()));

        image.borrow_mut().set_cross_origin(Some(""));
        image.borrow_mut().set_src(&url);

        onload.forget();
        onerror.forget();
    }
}

use std::collections::BTreeSet;
pub fn load_tiles(
    gl: &WebGl2Context,
    buffer: Rc<RefCell<BufferTiles>>,
    tiles: &BTreeSet<HEALPixCell>,
    depth: u8,
    depth_changed: bool
) {
    {
        let mut buffer = buffer.borrow_mut();
        // If the depth has just changed, we must rebuild the
        // requested tiles binary heap
        if depth_changed {
            buffer.refresh_requested_tiles_heap();
        }
        // And cancel the oldest async tile requests i.e. of depth
        // > current_depth + 1 and < current_depth - 1
        buffer.cancel_obsolete_tile_requests(depth);
    }
    for tile in tiles {
        load_healpix_tile(gl, buffer.clone(), tile.clone(), depth_changed);
    }
}
fn load_healpix_tile(gl: &WebGl2Context, buffer: Rc<RefCell<BufferTiles>>, tile: HEALPixCell, depth_changed: bool) {
    let time_request = utils::get_current_time();

    let image = Rc::new(RefCell::new(HtmlImageElement::new().unwrap()));
    let tile_request = TileRequest::new(tile, time_request, image.clone());
    // Check whether the tile is already in the buffer or requested
    if buffer.borrow_mut().replace_tile(tile, time_request, &tile_request, depth_changed) {
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
            buffer.borrow_mut()
                .remove_from_requested_tiles(tile_request);
        }) as Box<dyn Fn()>)
    };

    let onload = {
        let image = image.clone();
        let gl = gl.clone();

        Closure::wrap(Box::new(move || {
            console::log_1(&format!("load new tile").into());
            let mut buffer = buffer.borrow_mut();
            // Remove from the currently requested tiles
            let tile_request = TileRequest::new(tile, time_request, image.clone());
            buffer.remove_from_requested_tiles(tile_request);

            // Add it to the loaded cells hashset
            buffer.add_to_loaded_tiles(tile);
            // Add the received tile to the buffer
            let texture_idx = buffer.add(tile, time_request);
            //buffer.borrow().replace_texture_sampler_3d(idx_texture as i32, &image.borrow());
            replace_texture_sampler_2d(&gl, &buffer.texture_stack, texture_idx as i32, &image.borrow());

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

fn replace_texture_sampler_2d(gl: &WebGl2Context, texture_slack: &[Texture2D; 2], idx: i32, image: &HtmlImageElement) {
    // Get the slice to act on
    let idx_texture = idx / 64;

    let texture = &texture_slack[idx_texture as usize];
    let texture_unit = texture.idx_texture_unit;
    let webgl_texture = texture.texture.borrow();

    gl.active_texture(texture_unit);
    gl.bind_texture(WebGl2RenderingContext::TEXTURE_2D, webgl_texture.as_ref());

    let idx_row = (idx - idx_texture * 64) / 8;
    let idx_col = (idx - idx_texture * 64) % 8;

    let xoffset = idx_col * WIDTH_TEXTURE;
    let yoffset = idx_row * HEIGHT_TEXTURE;

    gl.tex_sub_image_2d_with_u32_and_u32_and_html_image_element(
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

use web_sys::WebGlTexture;
pub struct Texture2D {
    texture: Rc<RefCell<Option<WebGlTexture>>>,
    idx_texture_unit: u32,

    gl: WebGl2Context,
    tex_params: &'static [(u32, u32)],

    origin: Origin,
}

enum Origin {
    FromFile(&'static str),
    Empty(i32, i32),
}

impl Clone for Texture2D {
    fn clone(&self) -> Self {
        match self.origin {
            Origin::Empty(width, height) => {
                Texture2D::create_empty(&self.gl, width, height, self.tex_params)
            },
            Origin::FromFile(src) => {
                Texture2D::create(&self.gl, src, self.tex_params)
            }
        }
    }
}

impl Texture2D {
    pub fn create(gl: &WebGl2Context, src: &'static str, tex_params: &'static [(u32, u32)]) -> Texture2D {
        let image = Rc::new(RefCell::new(HtmlImageElement::new().unwrap()));

        let texture = Rc::new(RefCell::new(gl.create_texture()));
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
            let texture = texture.clone();

            Closure::wrap(Box::new(move || {
                gl.active_texture(idx_texture_unit);
                gl.bind_texture(WebGl2RenderingContext::TEXTURE_2D, texture.borrow().as_ref());

                for (pname, param) in tex_params.iter() {
                    gl.tex_parameteri(WebGl2RenderingContext::TEXTURE_2D, *pname, *param as i32);
                }

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
        
        let origin = Origin::FromFile(src);
        let gl = gl.clone();
        Texture2D {
            texture,
            idx_texture_unit,

            gl,
            tex_params,

            origin
        }
    }

    pub fn create_empty(gl: &WebGl2Context, width: i32, height: i32, tex_params: &'static [(u32, u32)]) -> Texture2D {
        let texture = Rc::new(RefCell::new(gl.create_texture()));
        let idx_texture_unit = unsafe { NUM_TEXTURE_UNIT };
        unsafe {
            NUM_TEXTURE_UNIT += 1;
        }
        gl.active_texture(idx_texture_unit);

        gl.bind_texture(WebGl2RenderingContext::TEXTURE_2D, texture.borrow().as_ref());

        for (pname, param) in tex_params.iter() {
            gl.tex_parameteri(WebGl2RenderingContext::TEXTURE_2D, *pname, *param as i32);
        }

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
        //gl.generate_mipmap(WebGl2RenderingContext::TEXTURE_2D);

        let origin = Origin::Empty(width, height);
        let gl = gl.clone();
        Texture2D {
            texture,
            idx_texture_unit,

            gl,
            tex_params,

            origin
        }
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