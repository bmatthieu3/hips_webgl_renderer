use std::rc::Rc;
use std::cell::RefCell;

use wasm_bindgen::prelude::Closure;
use wasm_bindgen::JsCast;

use web_sys::HtmlImageElement;
use web_sys::WebGl2RenderingContext;

use std::collections::HashSet;

/*const HEIGHT_TEXTURE: i32 = 512;
const WIDTH_TEXTURE: i32 = 512;*/

use crate::WebGl2Context;

use crate::binary_heap_tiles::BinaryHeapTiles;
use crate::healpix_cell::HEALPixCell;

use crate::core::Texture2DArray;

use std::collections::VecDeque;
use std::collections::HashMap;
pub struct HEALPixTexture {
    gl: WebGl2Context,
    // A FIFO storing the cells that are needed
    // to be plot to the screen.
    // The size is fixed to 64 tiles
    cells: VecDeque<HEALPixCell>,
    // A HashMap storing the indices of the cells
    // in the 8x8 tile texture.
    idx_texture: HashMap<HEALPixCell, usize>,
    
    // The texture storing the 64 tiles
    textures: Texture2DArray,

    // The width of a tile texture
    // Some HiPS are configured to have 32x32, 64x64 tile images
    // instead of the more traditional 512x512 size
    width_tile: i32,
    height_tile: i32,
}

pub static NUM_CELLS_BY_TEXTURE_SIDE: usize = 8;
pub static NUM_TILES_BY_TEXTURE: usize = NUM_CELLS_BY_TEXTURE_SIDE * NUM_CELLS_BY_TEXTURE_SIDE;
pub static NUM_TEXTURES: usize = 2;
pub static NUM_TILES: usize = NUM_TILES_BY_TEXTURE * NUM_TEXTURES;

impl HEALPixTexture {
    fn new(gl: &WebGl2Context, config: &HiPSConfig) -> HEALPixTexture {
        let cells = VecDeque::new();
        let idx_texture = HashMap::new();

        let width_tile = config.width_tile;
        let height_tile = config.height_tile;
        let textures = Texture2DArray::create_empty(
            gl,
            width_tile * NUM_CELLS_BY_TEXTURE_SIDE as i32,
            height_tile * NUM_CELLS_BY_TEXTURE_SIDE as i32,
            NUM_TEXTURES as i32,
            &[
                // The HiPS tiles sampling is NEAREST
                (WebGl2RenderingContext::TEXTURE_MIN_FILTER, WebGl2RenderingContext::NEAREST),
                (WebGl2RenderingContext::TEXTURE_MAG_FILTER, WebGl2RenderingContext::NEAREST),
                
                // Prevents s-coordinate wrapping (repeating)
                (WebGl2RenderingContext::TEXTURE_WRAP_S, WebGl2RenderingContext::CLAMP_TO_EDGE),
                // Prevents t-coordinate wrapping (repeating)
                (WebGl2RenderingContext::TEXTURE_WRAP_T, WebGl2RenderingContext::CLAMP_TO_EDGE),
            ],
            config.format as u32,
        );

        let gl = gl.clone();
        HEALPixTexture {
            gl,

            cells,
            idx_texture,

            textures,

            width_tile,
            height_tile
        }
    }

    fn tex_sub_image_2d(&self, image: Rc<RefCell<HtmlImageElement>>, idx: usize) {
        let idx_texture = idx / NUM_TILES_BY_TEXTURE;
        let idx = idx % NUM_TILES_BY_TEXTURE;

        let idx_row = idx / NUM_CELLS_BY_TEXTURE_SIDE;
        let idx_col = idx % NUM_CELLS_BY_TEXTURE_SIDE;
    
        let dx = (idx_col as i32) * self.width_tile;
        let dy = (idx_row as i32) * self.height_tile;
    
        self.textures.bind()
            .tex_sub_image_3d_with_html_image_element(
                dx,
                dy,
                idx_texture as i32,
                self.width_tile,
                self.height_tile,
                &image.borrow(),
            );
    }

    fn insert(&mut self, cell: HEALPixCell, image: Rc<RefCell<HtmlImageElement>>) -> usize {
        let idx = if !self.idx_texture.contains_key(&cell) {
            let idx = if self.cells.len() == NUM_TILES {
                let lost_cell = self.cells.pop_front().unwrap();
                self.idx_texture.remove(&lost_cell).unwrap()
            } else {
                self.cells.len()
            };

            self.cells.push_back(cell.clone());
            self.idx_texture.insert(cell, idx);

            self.tex_sub_image_2d(image, idx);

            idx
        } else {
            // Seek the position of the cell in the vecqueue
            let idx_cell = self.cells.iter().position(|c| *c == cell).unwrap();
            // Remove it
            self.cells.remove(idx_cell);
            // Push it back again
            self.cells.push_back(cell);
            // We do not touch the idx_texture hash map

            self.idx_texture.get(&cell)
                .cloned()
                .unwrap()
        };

        idx
    }

    fn clear(&mut self) {
        self.textures.bind()
            .clear();

        self.cells.clear();
        self.idx_texture.clear();
    }

    fn get_texture(&self) -> &Texture2DArray {
        &self.textures
    }
}

use std::cell::Cell;

pub struct HiPSConfig {
    // The size of the tile texture images
    pub width_tile: i32,
    pub height_tile: i32,

    // HEALPix depth of the more precise tiles
    pub max_depth: u8,

    pub name: String,

    // Format of the texture images e.g.:
    // * WebGl2RenderingContext::RGB for jpg images
    // * WebGl2RenderingContext::RGBA for png images
    // * WebGl2RenderingContext::R for one channel images such as FITS images
    format: i32,
    format_str: &'static str
}

pub enum TileImageFormat {
    JPG, // RGB
    PNG, // RGBA
    FITS, // One channel (grayscale)
}

impl HiPSConfig {
    pub fn new(name: String, width_tile: i32, height_tile: i32, max_depth: u8, format: TileImageFormat) -> HiPSConfig {
        let (format, format_str) = match &format {
            TileImageFormat::JPG => (WebGl2RenderingContext::RGB as i32, "jpg"),
            TileImageFormat::PNG => (WebGl2RenderingContext::RGBA as i32, "png"),
            TileImageFormat::FITS => (WebGl2RenderingContext::DEPTH_COMPONENT32F as i32, "fits"),
        };
        HiPSConfig {
            width_tile,
            height_tile,

            max_depth,

            name,

            format,
            format_str
        }
    }

    pub fn send_to_shader(&self, gl: &WebGl2Context, shader: &Shader) {
        // Send max depth of the current HiPS
        let location_max_depth = shader.get_uniform_location("max_depth");
        gl.uniform1i(location_max_depth, self.max_depth as i32);
    }
}

pub struct BufferTiles {
    gl: WebGl2Context,

    // The cells that are currently in the buffer.
    // The buffer is composed of two parts:
    // * A fixed part that will never change. The 12 base tiles are always
    //   stored
    // * A binary heap storing the most recent requested cells.
    heap: Rc<RefCell<BinaryHeapTiles>>,
    // A set of the cells that have been requested but
    // not yet received
    requested_tiles: Rc<RefCell<HashSet<HEALPixCell>>>,

    // A data-structure storing the cells texture indexes
    hpx_texture: Rc<RefCell<HEALPixTexture>>,

    // A boolean ensuring at least the base
    // tiles have been loaded
    ready: Rc<Cell<bool>>,

    // A boolean that is reinitialized at the beginning
    // of each frame. This tells the sphere object if he has
    // to recompute its vbo. This is due to the fact the vbo
    // is built from the tiles found in the buffer at a specific time.
    update_sphere_vbo: Rc<Cell<bool>>,

    // The number of tiles needed in the current frame
    num_tiles_per_frame: usize,
}

use crate::utils;
use crate::shader::Shader;

use crate::binary_heap_tiles::Tile;

macro_rules! console_error {
    ( $( $t:tt )* ) => {
        web_sys::console::error(
            &js_sys::Array::from(
                &format!( $( $t )* ).into()
            )
        );
    }
}

impl BufferTiles {
    pub fn new(gl: &WebGl2Context, config: &HiPSConfig) -> BufferTiles {
        let heap = Rc::new(RefCell::new(BinaryHeapTiles::new(512)));
        let requested_tiles = Rc::new(RefCell::new(HashSet::with_capacity(NUM_TILES)));

        let hpx_texture = Rc::new(RefCell::new(HEALPixTexture::new(gl, config)));

        let gl = gl.clone();
        let ready = Rc::new(Cell::new(false));
        let update_sphere_vbo = Rc::new(Cell::new(true));
        let num_tiles_per_frame = 0;
        let mut buffer = BufferTiles {
            gl,

            heap,
            requested_tiles,

            hpx_texture,

            ready,
            update_sphere_vbo,

            num_tiles_per_frame
        };

        buffer.request_tiles(
            // HEALPix cells to load
            &vec![
                HEALPixCell(0, 0),
                HEALPixCell(0, 1),
                HEALPixCell(0, 2),
                HEALPixCell(0, 3),
                HEALPixCell(0, 4),
                HEALPixCell(0, 5),
                HEALPixCell(0, 6),
                HEALPixCell(0, 7),
                HEALPixCell(0, 8),
                HEALPixCell(0, 9),
                HEALPixCell(0, 10),
                HEALPixCell(0, 11),
            ],
            // HiPS config infos
            config,
            // Depth change
            true,
        );

        buffer
    }

    pub fn reset(&mut self, config: &HiPSConfig) {
        self.heap.borrow_mut()
            .clear();
        self.requested_tiles.borrow_mut()
            .clear();
        self.hpx_texture.borrow_mut()
            .clear();

        self.ready.set(false);

        self.request_tiles(
            // HEALPix cells to load
            &vec![
                HEALPixCell(0, 0),
                HEALPixCell(0, 1),
                HEALPixCell(0, 2),
                HEALPixCell(0, 3),
                HEALPixCell(0, 4),
                HEALPixCell(0, 5),
                HEALPixCell(0, 6),
                HEALPixCell(0, 7),
                HEALPixCell(0, 8),
                HEALPixCell(0, 9),
                HEALPixCell(0, 10),
                HEALPixCell(0, 11),
            ],
            // HiPS config infos
            config,
            // Depth change
            true,
        );
    }

    pub fn signals_new_frame(&mut self) {
        self.num_tiles_per_frame = 0;
    }
    pub fn signals_end_frame(&mut self) {
        // Ensure the current frame does not need more than
        // 64 different tiles. Otherwise it will exceed the texture
        // size capacity!
        if self.num_tiles_per_frame > NUM_TILES {
            console_error!(
                "The number of tiles needed to be plot ({:?}) exceeds the GPU texture buffer size ({:?})",
                self.num_tiles_per_frame, NUM_TILES
            );
        }

        // After the vbo has been recomputed
        // we can put this flag to false.
        // When a new cell will be received it will
        // be set to true again.
        self.update_sphere_vbo.set(false);
    }

    pub fn get_idx_texture(&mut self, cell: &HEALPixCell) -> usize {
        let tile_texture = self.heap.borrow()
            .get(cell)
            .unwrap()
            .texture.clone();

        // The number of HEALPix cells needed for this frame
        self.num_tiles_per_frame += 1;

        let idx_texture = self.hpx_texture.borrow_mut()
            .insert(
                // The HEALPix cell that we need for drawing purposes
                *cell,
                // Its texture stored in a HTMLElementImage
                tile_texture,

            );

        idx_texture
    }

    pub fn request_tiles(&mut self, cells: &Vec<HEALPixCell>, hips: &HiPSConfig, depth_changed: bool) {
        // The viewport has changed (moving, zooming, resizing)
        // We will tell the sphere to recompute its vbo.
        self.update_sphere_vbo.set(true);

        for cell in cells.iter() {
            self.load_tile(cell, hips, depth_changed);
        }
    }

    fn load_tile(&mut self, cell: &HEALPixCell, hips: &HiPSConfig, depth_changed: bool) {
        let already_loaded = self.heap.borrow().contains(cell);
        let already_requested = self.requested_tiles.borrow().contains(cell);

        if already_loaded {
            // If the heap already contain the cell,
            // we update its priority
            let time_received = //if depth_changed {
                utils::get_current_time();
            //} else {
            //    self.heap.borrow_mut()
            //        .get(cell)
            //        .unwrap()
            //        .time_received
            //};

            let time_request = utils::get_current_time();
            self.heap.borrow_mut().update_priority(cell, time_request, time_received);
        } else {
            // The cell is not already loaded 
            if !already_requested {
                let time_request = utils::get_current_time();

                let image = Rc::new(RefCell::new(HtmlImageElement::new().unwrap()));
                // Add to the tiles requested
                self.requested_tiles.borrow_mut()
                    .insert(*cell);
                // Prepare the url
                let url = {
                    let depth = cell.0;
                    let idx = cell.1;

                    let dir_idx = (idx / 10000) * 10000;

                    let mut url = hips.name.to_string() + "/";
                    url = url + "Norder" + &depth.to_string() + "/";
                    url = url + "Dir" + &dir_idx.to_string() + "/";
                    url = url + "Npix" + &idx.to_string();
                    url = url + "." + hips.format_str;
            
                    url
                };

                let onload = {
                    let requested_tiles = self.requested_tiles.clone();
                    let heap = self.heap.clone();
                    let hpx_texture = self.hpx_texture.clone();
                    let image = image.clone();
                    let cell = *cell;
                    let ready = self.ready.clone();
                    let update_sphere_vbo = self.update_sphere_vbo.clone();

                    Closure::wrap(Box::new(move || {
                        // Remove the cell from the requested cells set
                        requested_tiles.borrow_mut()
                            .remove(&cell);

                        // Append it to the heap
                        let time_received = utils::get_current_time();
                        //console::log_1(&format!("tile received").into());
                        heap.borrow_mut()
                            .push(Tile::new(cell.clone(), time_request, time_received, image.clone()));

                        // Preload the tile texture in the tile texture.
                        let mut tt = 0;
                        hpx_texture.borrow_mut()
                            .insert(
                                // The HEALPix cell that we need for drawing purposes
                                cell,
                                // Its texture stored in a HTMLElementImage
                                image.clone(),
                            );

                        ready.set(heap.borrow().is_ready());
                        // A cell has been received
                        update_sphere_vbo.set(true);
                    }) as Box<dyn Fn()>)
                };

                let onerror = {
                    let requested_tiles = self.requested_tiles.clone();
                    let cell = *cell;

                    Closure::wrap(Box::new(move || {
                        // Remove the cell from the requested cells set
                        requested_tiles.borrow_mut()
                            .remove(&cell);
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
    }

    pub fn is_ready(&self) -> bool {
        self.ready.get()
    }

    pub fn is_sphere_vbo_rebuild_necessary(&self) -> bool {
        self.update_sphere_vbo.get()
    }

    pub fn contains(&self, cell: &HEALPixCell) -> bool {
        self.heap.borrow()
            .contains(cell)
    }
    
    pub fn get_time_received(&self, cell: &HEALPixCell) -> Option<f32> {
        self.heap.borrow()
            .get(cell)
            .map(|tile| tile.time_received)
    }

    // Build the texture from the tiles needed by the
    // current view
    // This panic if the cells are not present in the buffer
    /*pub fn build_texture(&mut self, cells: HEALPixCells) {
        for (cell, idx) in cells.into_iter() {
            self.replace_texture_sampler_2d(&cell, idx as i32);
        }
    }*/

    /*fn replace_texture_sampler_2d(&self, cell: &HEALPixCell, idx: i32) {
        let idx_row = idx / 8;
        let idx_col = idx % 8;
    
        let dx = idx_col * WIDTH_TEXTURE;
        let dy = idx_row * HEIGHT_TEXTURE;
    
        // Get the tile texture
        let tile_texture = self.heap.borrow()
            .get(cell).unwrap()
            .texture.clone();
        /*self.canvas_ctx.draw_image_with_html_image_element_and_dw_and_dh(
            &texture.borrow(),
            dx as f64,
            dy as f64,
            WIDTH_TEXTURE as f64,
            HEIGHT_TEXTURE as f64,
        ).expect("Failing writing a tile to the CanvasRendering2dContext");*/
        self.texture.bind()
            .tex_sub_image_2d_with_u32_and_u32_and_html_image_element(
                dx as i32,
                dy as i32,
                &tile_texture.borrow(),
            );
    }*/

    /*pub fn refresh_requested_tiles_heap(&mut self) {
        self.requested_tiles = self.requested_tiles
            .clone()
            .into_vec()
            .into_iter()
            .collect::<BinaryHeap<_>>();
    }*/

    /*pub fn cancel_obsolete_tile_requests(&mut self, depth: u8) {
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
    }*/

    /*
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

        let mut buffer_cpy = self.buffer.clone();
        let mut idx = 0;

        let len_variable_tiles = std::cmp(max_size_binary_heap, self.buffer.len());
        while idx < len_variable_tiles {
            let tile = buffer_cpy.pop();

            tiles[idx] = tile.unwrap().into();
            idx += 1;
        }

        let off = idx;
        for (j, tile) in self.base_tiles.iter().enumerate() {
            tiles[off + j] = tile.into();
        }

        tiles.sort();

        /*for tile in tiles.iter() {
            console::log_1(&format!("idx tile {:?}", tile.texture_idx).into());
        }*/

        tiles
    }
    */
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



    pub fn send_texture_to_shader(&self, shader: &Shader) {
        self.hpx_texture.borrow()
            .get_texture()
            .bind()
            .send_to_shader(shader, "tex");
        /*let tiles = self.uniq_ordered_tiles();

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
        self.gl.uniform1i(location_size_buffer, self.len());*/
    }

    //pub fn len(&self) -> usize {
    //    self.buffer.len() + 12
    //}
}
