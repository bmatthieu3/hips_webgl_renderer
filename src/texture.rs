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

use crate::binary_heap_tiles::BinaryHeapTiles;
use crate::healpix_cell::HEALPixCell;
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

    // The texture storing the 64 tiles
    texture: Texture2D,
}

use crate::utils;
use crate::shader::Shader;
use std::convert::TryInto;
use crate::binary_heap_tiles::Tile;

impl BufferTiles {
    pub fn new(gl: &WebGl2Context) -> BufferTiles {
        let heap = Rc::new(RefCell::new(BinaryHeapTiles::new(512)));
        let requested_tiles = Rc::new(RefCell::new(HashSet::with_capacity(64)));

        let texture = Texture2D::create_empty(
            gl,
            WIDTH_TEXTURE * 8,
            HEIGHT_TEXTURE * 8,
            &[
                // The HiPS tiles sampling is NEAREST
                (WebGl2RenderingContext::TEXTURE_MIN_FILTER, WebGl2RenderingContext::LINEAR),
                (WebGl2RenderingContext::TEXTURE_MAG_FILTER, WebGl2RenderingContext::LINEAR),
                
                // Prevents s-coordinate wrapping (repeating)
                (WebGl2RenderingContext::TEXTURE_WRAP_S, WebGl2RenderingContext::CLAMP_TO_EDGE),
                // Prevents t-coordinate wrapping (repeating)
                (WebGl2RenderingContext::TEXTURE_WRAP_T, WebGl2RenderingContext::CLAMP_TO_EDGE),
            ]
        );

        let gl = gl.clone();
        let mut buffer = BufferTiles {
            gl,

            heap,
            requested_tiles,

            texture,
        };

        buffer.request_tiles(&vec![
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
            // Depth change
            true,
            // Sync request, we are waiting to receive the tiles
            true
        );

        buffer
    }

    pub fn request_tiles(&mut self, cells: &Vec<HEALPixCell>, depth_changed: bool, sync: bool) {
        for cell in cells.iter() {
            self.load_tile(cell, depth_changed);
        }

        if sync {
            // Loop until all the base tiles have been received
            while self.requested_tiles.borrow().len() > 0 {}
        }
    }

    fn load_tile(&mut self, cell: &HEALPixCell, depth_changed: bool) {
        let already_loaded = self.heap.borrow().contains(cell);
        let already_requested = self.requested_tiles.borrow().contains(cell);

        if already_loaded {
            // If the heap already contain the cell,
            // we update its priority
            let time_received = if depth_changed {
                utils::get_current_time()
            } else {
                self.heap.borrow_mut()
                    .get(cell)
                    .unwrap()
                    .time_received
            };
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
            
                    let mut url = HIPS_NAME.lock().unwrap().clone() + "/";
                    url = url + "Norder" + &depth.to_string() + "/";
                    url = url + "Dir" + &dir_idx.to_string() + "/";
                    url = url + "Npix" + &idx.to_string() + ".jpg";
            
                    url
                };

                let onload = {
                    let requested_tiles = self.requested_tiles.clone();
                    let heap = self.heap.clone();
                    let image = image.clone();
                    let cell = *cell;
            
                    Closure::wrap(Box::new(move || {
                        // Remove the cell from the requested cells set
                        requested_tiles.borrow_mut()
                            .remove(&cell);
                        
                        // Append it to the heap
                        let time_received = utils::get_current_time();
                        heap.borrow_mut()
                            .push(Tile::new(cell, time_request, time_received, image.clone()));
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
            // Remove the cell from the requested cells set
        }
    }

    pub fn contains(&self, cell: &HEALPixCell) -> bool {
        self.heap.borrow()
            .contains(cell)
    }
    
    pub fn get(&self, cell: &HEALPixCell) -> Option<Tile> {
        self.heap.borrow()
            .get(cell)
            // References on a temporary value cannot be returned
            // out of a function
            .cloned()
    }

    // Build the texture from the tiles needed by the
    // current view
    // This panic if the cells are not present in the buffer
    pub fn build_texture(&mut self, cells: Vec<HEALPixCell>) {
        for (idx, cell) in cells.iter().enumerate() {
            self.replace_texture_sampler_2d(cell, idx as i32);
        }
    }

    fn replace_texture_sampler_2d(&mut self, cell: &HEALPixCell, idx: i32) {
        let texture_unit = self.texture.idx_texture_unit;
        let webgl_texture = self.texture.texture.borrow();
    
        self.gl.active_texture(texture_unit);
        self.gl.bind_texture(WebGl2RenderingContext::TEXTURE_2D, webgl_texture.as_ref());
    
        let idx_row = idx / 8;
        let idx_col = idx % 8;
    
        let xoffset = idx_col * WIDTH_TEXTURE;
        let yoffset = idx_row * HEIGHT_TEXTURE;
    
        // Get the tile texture
        let tile = self.get(cell)
            .unwrap();
        self.gl.tex_sub_image_2d_with_u32_and_u32_and_html_image_element(
            WebGl2RenderingContext::TEXTURE_2D,
            0,
            xoffset,
            yoffset,
            WebGl2RenderingContext::RGB,
            WebGl2RenderingContext::UNSIGNED_BYTE,
            &tile.texture.borrow(),
        )
        .expect("Sub texture 2d");
    }
    

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
        self.texture.send_to_shader(&self.gl, shader, "tex");
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

    pub fn clear(&mut self) {
        // Clear the texture
        self.texture.clear(&self.gl);
        
        self.heap.borrow_mut()
            .clear();
        self.requested_tiles.borrow_mut()
            .clear();
    }
}

use crate::HIPS_NAME;

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