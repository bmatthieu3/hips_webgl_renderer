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

#[derive(Clone)]
pub struct Tile {
    pub cell: HEALPixCell,

    pub texture_idx: u8,

    time_request: f32,
    time_received: Option<f32>,

    image: Rc<RefCell<HtmlImageElement>>
}

pub const BLENDING_DURATION_MS: f32 = 500_f32;
impl Tile {
    fn new(cell: HEALPixCell) -> Tile {
        let time_request = utils::get_current_time();
        let time_received = None;

        let texture_idx = 0;

        let image = Rc::new(RefCell::new(HtmlImageElement::new().unwrap()));

        Tile {
            cell,

            texture_idx,

            time_request,
            time_received,

            image,
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

        let uniq = ((1 << ((depth + 1) << 1)) + idx) as u32;

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

        let uniq = ((1 << ((depth + 1) << 1)) + idx) as u32;

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

use std::collections::HashMap;
pub struct BufferTiles {
    gl: WebGl2Context,

    // Tiles to send to the GPU at each frames
    base_tiles: [Tile; 12],
    newest_tiles: BinaryHeap<Tile>,

    // Remaining tiles that we keep in the browser cache
    oldest_tiles: BinaryHeap<Tile>,

    // All the tiles that are loaded i.e. either in the tiles, either in the buffer
    loaded_tiles: HashMap<HEALPixCell, Tile>,
    // This binary heap contains the requests that must be canceled
    // This must be rebuilt whenever we change of depth
    pub requested_tiles: BinaryHeap<TileRequest>,
    requested_tiles_set: HashSet<HEALPixCell>,

    max_size_newest_tiles: usize,
    max_size_oldest_tiles: usize,

    // 4094x4096 texture
    texture: Texture2D,
}

use crate::utils;
use crate::shader::Shader;
use std::convert::TryInto;
impl BufferTiles {
    pub fn new(gl: &WebGl2Context) -> BufferTiles {
        let max_size_newest_tiles = 64 - 12;
        let max_size_oldest_tiles = 1024 - (max_size_newest_tiles + 12);

        // Base tile slice
        let base_tiles = [
            Tile::new(HEALPixCell(0, 0)),
            Tile::new(HEALPixCell(0, 1)),
            Tile::new(HEALPixCell(0, 2)),
            Tile::new(HEALPixCell(0, 3)),
            Tile::new(HEALPixCell(0, 4)),
            Tile::new(HEALPixCell(0, 5)),
            Tile::new(HEALPixCell(0, 6)),
            Tile::new(HEALPixCell(0, 7)),
            Tile::new(HEALPixCell(0, 8)),
            Tile::new(HEALPixCell(0, 9)),
            Tile::new(HEALPixCell(0, 10)),
            Tile::new(HEALPixCell(0, 11)),
        ];
        // The newest requested tiles. Those tiles will be send to the GPU
        let newest_tiles = BinaryHeap::with_capacity(max_size_newest_tiles);
        // Tiles that have been requested but not yet received. This has to fit in the
        // newest tiles buffer.
        let requested_tiles = BinaryHeap::with_capacity(max_size_newest_tiles);
        let requested_tiles_set = HashSet::with_capacity(max_size_newest_tiles);

        // Oldest requested tiles. This is a cache of tiles
        let oldest_tiles = BinaryHeap::with_capacity(max_size_oldest_tiles);

        // A set containing all the tiles in the buffer
        // i.e. the newest and oldest tiles (512 tiles in all)
        let loaded_tiles = HashMap::with_capacity(max_size_oldest_tiles + max_size_newest_tiles);

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

        let gl = gl.clone();
        BufferTiles {
            gl,

            base_tiles,
            newest_tiles,
            oldest_tiles,

            loaded_tiles,

            requested_tiles,
            requested_tiles_set,

            max_size_newest_tiles,
            max_size_oldest_tiles,

            texture,
        }
    }

    // Add a new tile to the buffer
    pub fn add(
        &mut self,

        cell: HEALPixCell,

        time_request: f32,
        time_received: f32,
        image: Rc<RefCell<HtmlImageElement>>
    ) -> u8 {
        let texture_idx = if self.newest_tiles.len() == self.max_size_newest_tiles {
            // Remove the oldest tile from the newest tiles buffer
            let oldest_tile = self.newest_tiles.pop().unwrap();
            // Get its texture idx and assign it to the new added tile
            let texture_idx = oldest_tile.texture_idx;

            // Remove the oldest tile from the oldest buffer
            // if there is one
            if self.oldest_tiles.len() == self.max_size_oldest_tiles {
                // This tile is lost
                let lost_tile = self.oldest_tiles.pop().unwrap();

                // Remove it from the loaded tile set
                self.remove_from_loaded_tiles(&lost_tile.cell);
            }
            // Add this tile to the oldest tiles buffer
            self.oldest_tiles.push(oldest_tile);
            // The number of tiles in the oldest buffer is garanted to be the same

            texture_idx
        } else {
            (self.newest_tiles.len() as u8) + 12
        };

        let tile = Tile {
            cell,

            texture_idx,

            time_request,
            time_received: Some(time_received),

            image
        };

        // Push it to the newest tiles buffer
        self.push_new_tile(tile.clone());
        // Add it to the loaded tiles set
        self.add_to_loaded_tiles(&cell, tile);

        // Remove from requested tiles buffer
        self.remove_from_requested_tiles(&cell);

        texture_idx
    }

    pub fn refresh_requested_tiles_heap(&mut self) {
        self.requested_tiles = self.requested_tiles
            .clone()
            .into_vec()
            .into_iter()
            .collect::<BinaryHeap<_>>();
    }

    fn push_new_tile(&mut self, tile: Tile) {
        self.newest_tiles.push(tile);
        *LATEST_TIME_TILE_RECEIVED.lock().unwrap() = utils::get_current_time();
    }

    pub fn add_to_loaded_tiles(&mut self, cell: &HEALPixCell, tile: Tile) {
        self.loaded_tiles.insert(*cell, tile);
    }

    pub fn remove_from_loaded_tiles(&mut self, cell: &HEALPixCell) {
        self.loaded_tiles.remove(&cell);
    }

    pub fn add_to_requested_tiles(&mut self, tile_request: TileRequest) {
        self.requested_tiles.push(tile_request.clone());
        self.requested_tiles_set.insert(tile_request.cell);
    }

    pub fn remove_from_requested_tiles(&mut self, cell: &HEALPixCell) {
        self.requested_tiles = self.requested_tiles
            .iter()
            .filter(|tile| {
                tile.cell != *cell
            })
            .cloned()
            .collect::<BinaryHeap<_>>();
        
        // Remove from the set as well
        self.requested_tiles_set.remove(cell);
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

        // Get the set of the remaining requests
        self.requested_tiles_set = self.requested_tiles.iter()
            .map(|tile| tile.cell)
            .collect();
    }

    pub fn replace_tile(&mut self,
        cell: HEALPixCell,
        time_request: f32,
        
        depth_changed: bool,
    ) -> bool {
        // Check whether the buffer already contains the requested tile
        let already_load_tile = self.loaded_tiles.get(&cell).cloned();
        if let Some(mut tile) = already_load_tile {
            // If tile is already in the buffer
            // Then we do change its priority by
            // updating the reception time.
            tile.time_request = time_request;
            if depth_changed {
                // If the depth has changed we do want to see the
                // blending effect between different level tiles
                // So I reset the time when the tile has been received
                tile.time_received = Some(utils::get_current_time());
            }

            // Find whether the tile is in the newest or oldest binary heap
            // and remove it
            let mut newest_tile_found = false;
            let mut newest_tiles = self.newest_tiles.iter()
                .cloned()
                .collect::<HashSet<_>>();
            if let Some(mut t) = newest_tiles.take(&tile) {
                t.time_request = tile.time_request;
                t.time_received = tile.time_received;

                // Now that the time have been updated
                // we can push it again in the newest tiles
                newest_tiles.insert(t);
                newest_tile_found = true;
            }
            // Update the newest tiles binary heap
            self.newest_tiles = newest_tiles.into_iter()
                .collect::<BinaryHeap<_>>();
            
            // If the tile is not among the newest then it has to be
            // among the oldest ones
            if !newest_tile_found {
                let mut oldest_tiles = self.oldest_tiles.iter()
                    .collect::<HashSet<_>>();
                let t = oldest_tiles.take(&tile);
                if t.is_none() {
                    panic!("A tile that is currently loaded
                        but not among the newest tiles
                        has to be among the oldest ones.");
                }
                self.oldest_tiles = oldest_tiles
                    .into_iter()
                    .cloned()
                    .collect::<BinaryHeap<_>>();

                // Now we can add it to the buffer.
                // The oldest from the newest tile set
                // will be moved to the oldest tile set
                let texture_idx = self.add(
                    tile.cell,
                    
                    tile.time_request,
                    tile.time_received.unwrap(),
                    tile.image.clone()
                );
                replace_texture_sampler_2d(&self.gl, &self.texture, texture_idx as i32, &tile.image.borrow());
            }

            return true;
        }
        
        // The tile has been requested so we do launch a new
        // async request to alasky
        if self.requested_tiles_set.contains(&cell) {
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

    pub fn get(&mut self, cell: &HEALPixCell) -> Option<&Tile> {
        let tile = self.newest_tiles.iter()
            .find(|&x| *cell == x.cell);
        
        if tile.is_some() {
            tile
        } else {
            let tile = self.base_tiles.iter()
                .find(|&x| *cell == x.cell);
            
            if tile.is_some() {
                tile
            } else {
                if let Some(t) = self.loaded_tiles.get(&cell) {
                    self.replace_tile(t.cell, t.time_request, false);

                    Some(t)
                } else {
                    None
                }
                /*// Check in the oldest loaded tiles
                let tile = self.oldest_tiles.iter()
                    .find(|&x| *cell == x.cell)
                    .cloned();
                if let Some(t) = tile {
                    self.oldest_tiles = self.oldest_tiles.iter()
                        .filter(|t| t.cell != *cell)
                        .cloned()
                        .collect::<BinaryHeap<_>>();

                    // Now we can add it to the buffer.
                    // The oldest from the newest tile set
                    // will be moved to the oldest tile set
                    let texture_idx = self.add(
                        t.cell,

                        t.time_request,
                        t.time_received.unwrap(),
                        t.image.clone()
                    );
                    replace_texture_sampler_2d(&self.gl, &self.texture, texture_idx as i32, &t.image.borrow());

                    Some(t)
                } else {
                    None
                }*/
            }
        }
    }

    fn uniq_ordered_tiles(&self) -> [TilePerPixelGPU; 64] {
        let mut tiles = [TilePerPixelGPU::new(); 64];

        for (j, tile) in self.base_tiles.iter().enumerate() {
            tiles[j] = tile.into();
        }

        let off = self.base_tiles.len();
        for (i, tile) in self.newest_tiles.iter().enumerate() {
            tiles[i + off] = tile.into();
        }

        tiles.sort();

        /*for tile in tiles.iter() {
            console::log_1(&format!("idx tile {:?}", tile.texture_idx).into());
        }*/

        tiles
    }

    /*fn send_texture(&self, shader: &Shader) {
        let texture = &self.texture;
        let location_tex_name = String::from("tex");
        let location_texture = shader.get_uniform_location(&location_tex_name);

        let (texture_unit, webgl_texture) = (texture.idx_texture_unit, texture.texture.clone());

        self.gl.active_texture(texture_unit);
        self.gl.bind_texture(WebGl2RenderingContext::TEXTURE_2D, webgl_texture.borrow().as_ref());

        let idx_texture = (texture_unit - WebGl2RenderingContext::TEXTURE0).try_into().unwrap();
        self.gl.uniform1i(location_texture, idx_texture);
    }*/

    pub fn send_to_shader(&self, shader: &Shader) {
        self.texture.send_to_shader(&self.gl, shader, "tex");
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

        let location_size_buffer = shader.get_uniform_location("num_tiles");
        self.gl.uniform1i(location_size_buffer, (self.newest_tiles.len() + 12) as i32);
    }

    /*pub fn len(&self) -> usize {
        self.size_binary_heap + 12
    }

    pub fn len_variable_tiles(&self) -> usize {
        self.size_binary_heap
    }*/

    pub fn clear(&mut self) {
        // Clear the texture
        self.texture.clear(&self.gl);
        
        // TODO: Clear the texture
        self.newest_tiles.clear();
        self.oldest_tiles.clear();

        self.loaded_tiles.clear();
        
        self.requested_tiles.clear();
        self.requested_tiles_set.clear();
    }
}

use crate::HIPS_NAME;
use crate::field_of_view::HEALPixCell;

pub fn load_base_tiles(gl: &WebGl2Context, buffer: Rc<RefCell<BufferTiles>>) {
    for idx in 0..12 {
        let image = Rc::new(RefCell::new(HtmlImageElement::new().unwrap()));
        let time_request = utils::get_current_time();

        let url = {            
            let mut url = HIPS_NAME.lock().unwrap().clone() + "/";
            url = url + "Norder0/";
            url = url + "Dir0/";
            url = url + "Npix" + &idx.to_string() + ".jpg";

            url
        };
        let onerror = {
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

                    image: image.clone()
                };

                replace_texture_sampler_2d(&gl, &buffer.borrow().texture, texture_idx as i32, &image.borrow());
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
    // Do not load base tiles, they are already in the buffer
    if depth == 0 {
        return;
    }

    /*{
        let mut buffer = buffer.borrow_mut();
        // If the depth has just changed, we must rebuild the
        // requested tiles binary heap
        if depth_changed {
            buffer.refresh_requested_tiles_heap();
        }
        // And cancel the oldest async tile requests i.e. of depth
        // > current_depth + 1 and < current_depth - 1
        buffer.cancel_obsolete_tile_requests(depth);
    }*/
    for tile in tiles {
        load_healpix_tile(gl, buffer.clone(), tile.clone(), depth_changed);
    }
}
fn load_healpix_tile(gl: &WebGl2Context, buffer: Rc<RefCell<BufferTiles>>, cell: HEALPixCell, depth_changed: bool) {
    let time_request = utils::get_current_time();

    let image = Rc::new(RefCell::new(HtmlImageElement::new().unwrap()));
    // Check whether the tile is already in the buffer or requested
    let tile_found = buffer.borrow_mut()
        .replace_tile(
            cell,
            time_request,
            depth_changed
        );
    if tile_found {
        return;
    }

    // Here we know we have to launch a new async request
    let tile_request = TileRequest::new(
        cell,
        time_request,
        image.clone()
    );
    buffer.borrow_mut()
        .add_to_requested_tiles(tile_request);

    let url = {
        let depth = cell.0;
        let idx = cell.1;
        
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
            // Remove from the currently requested tiles
            buffer.borrow_mut()
                .remove_from_requested_tiles(&cell);
        }) as Box<dyn Fn()>)
    };

    let onload = {
        let image = image.clone();
        let gl = gl.clone();

        Closure::wrap(Box::new(move || {
            let time_received = utils::get_current_time();
            let mut buffer = buffer.borrow_mut();
            // Add the received tile to the buffer
            let texture_idx = buffer.add(cell, time_request, time_received, image.clone());
            //console::log_1(&format!("load new tile {:?}", texture_idx).into());
            //buffer.borrow().replace_texture_sampler_3d(idx_texture as i32, &image.borrow());
            replace_texture_sampler_2d(&gl, &buffer.texture, texture_idx as i32, &image.borrow());

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

fn replace_texture_sampler_2d(gl: &WebGl2Context, texture: &Texture2D, idx: i32, image: &HtmlImageElement) {
    let texture_unit = texture.idx_texture_unit;
    let webgl_texture = texture.texture.borrow();

    gl.active_texture(texture_unit);
    gl.bind_texture(WebGl2RenderingContext::TEXTURE_2D, webgl_texture.as_ref());

    let idx_row = idx / 8;
    let idx_col = idx % 8;

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

enum TextureType {
    // The image containing the width and height
    // of the texture
    ImageElement(Rc<RefCell<HtmlImageElement>>),
    // Width and Height
    Bytes(u32, u32)
}

impl TextureType {
    fn get_width(&self) -> u32 {
        match self {
            TextureType::ImageElement(image) => image.borrow().width() as u32,
            TextureType::Bytes(width, _) => *width
        }
    }

    fn get_height(&self) -> u32 {
        match self {
            TextureType::ImageElement(image) => image.borrow().height() as u32,
            TextureType::Bytes(_, height) => *height
        }
    }
}

use web_sys::WebGlTexture;
pub struct Texture2D {
    texture: Rc<RefCell<Option<WebGlTexture>>>,
    idx_texture_unit: u32,

    gl: WebGl2Context,
    tex_params: &'static [(u32, u32)],

    data: TextureType,
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
                gl.generate_mipmap(WebGl2RenderingContext::TEXTURE_2D);
            }) as Box<dyn Fn()>)
        };

        image.borrow_mut().set_onload(Some(onload.as_ref().unchecked_ref()));
        image.borrow_mut().set_onerror(Some(onerror.as_ref().unchecked_ref()));

        image.borrow_mut().set_cross_origin(Some(""));
        image.borrow_mut().set_src(src);

        onload.forget();
        onerror.forget();
        
        let data = TextureType::ImageElement(image);
        let gl = gl.clone();
        Texture2D {
            texture,
            idx_texture_unit,

            gl,
            tex_params,

            data
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
        gl.generate_mipmap(WebGl2RenderingContext::TEXTURE_2D);

        let gl = gl.clone();
        let data = TextureType::Bytes(width as u32, height as u32);

        Texture2D {
            texture,
            idx_texture_unit,

            gl,
            tex_params,

            data
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

    pub fn clear(&self, gl: &WebGl2Context) {
        let texture_unit = self.idx_texture_unit;
        let webgl_texture = self.texture.borrow();

        self.gl.active_texture(texture_unit);
        self.gl.bind_texture(WebGl2RenderingContext::TEXTURE_2D, webgl_texture.as_ref());

        let (width, height) = (self.data.get_width(), self.data.get_height());

        let data = vec![0 as u8; 3 * (height as usize) * (width as usize)];
        gl.tex_sub_image_2d_with_i32_and_i32_and_u32_and_type_and_opt_u8_array(
            WebGl2RenderingContext::TEXTURE_2D,
            0,
            0,
            0,

            width as i32,
            height as i32,

            WebGl2RenderingContext::RGB,
            WebGl2RenderingContext::UNSIGNED_BYTE,
            Some(&data),
        )
        .expect("Sub texture 2d");
    }
}