use std::rc::Rc;
use std::cell::RefCell;

use wasm_bindgen::prelude::Closure;
use wasm_bindgen::JsCast;

use web_sys::HtmlImageElement;
use web_sys::WebGl2RenderingContext;

use std::collections::HashSet;

use crate::WebGl2Context;

use crate::healpix_cell::HEALPixCell;

use crate::core::Texture2DArray;

use std::collections::VecDeque;
use std::collections::HashMap;

use web_sys::console;
use futures::executor::LocalPool;

use futures::Future;
use futures::task::{Poll, Context};
use core::pin::Pin;


use std::sync::{Arc, Mutex};

use crate::buffer::{
 Textures,
 HiPSConfig
};
pub struct BufferTextures {
    gl: WebGl2Context,

    // The cells that are currently in the buffer.
    // The buffer is composed of two parts:
    // * A fixed part that will never change. The 12 base tiles are always
    //   stored
    // * A binary heap storing the most recent requested cells.
    textures: Rc<RefCell<Textures>>,
    // A set of the cells that have been requested but
    // not yet received
    requested_tiles: Rc<RefCell<HashSet<HEALPixCell>>>,

    // The bunch of textures needed by the current frame
    // Reinitialized each time in ``signals_new_frame``
    //textures_needed_per_frame: HashSet<HEALPixCell>,

    // The config of the current HiPS
    // This is needed whenever a new tile must be
    // requested
    config: HiPSConfig,

    // Flag telling whether the buffer has changed
    has_changed: bool,
    // LocalPool
    //pool: LocalPool,
    //futures: Vec<HEALPixTextureWrite>, // A LIFO containing the futures to run
}

use crate::utils;

use crate::buffer::{
 TileConfig,
 NUM_TEXTURES,
 Texture,
};

use std::cell::Cell;
use crate::buffer::Image;
use crate::healpix_cell::HEALPixTiles;
impl BufferTextures {
    pub fn new(gl: &WebGl2Context, config: &HiPSConfig) -> BufferTextures {
        // Arbitrary number decided here
        let requested_tiles = Rc::new(RefCell::new(HashSet::with_capacity(64)));

        let tile_config = config.tile_config();
        let textures = Rc::new(RefCell::new(Textures::new(gl, tile_config)));

        let gl = gl.clone();

        let has_changed = false;

        let config = config.clone();
        let mut buffer = BufferTextures {
            gl,

            textures,
            requested_tiles,


            config,
            has_changed,
        };

        buffer.initialize();

        buffer
    }

    /*pub fn poll_textures(&mut self) {
        if !self.futures.is_empty() {
            let future = self.futures.pop().unwrap();
            self.pool.run_until(future);
        }
    }*/

    pub fn reset(&mut self, config: &HiPSConfig) {
        self.config = config.clone();
        let tile_config = config.tile_config();
        self.textures.borrow_mut()
            .clear(tile_config);
        self.requested_tiles.borrow_mut()
            .clear();

        self.initialize();
    }

    // The textures has changed if:
    // * New tiles have been received
    // * Textures have been written to the textures array for the GPU
    // * New requests have been launched
    // The flag is reset to false when this method has been called
    // It must be called only one time per frame
    pub fn has_changed(&mut self) -> bool {
        let textures_written = if let Ok(mut textures) = self.textures.try_borrow_mut() {
            //console::log_1(&format!("aaaa: {:?}", textures_written).into());

            textures.write_textures()
        } else {
            false
        };

        let has_changed = self.has_changed | textures_written;
        self.has_changed = false;

        has_changed
    }

    // cell is contained into the buffer
    pub fn get_texture(&self, cell: &HEALPixCell) -> Texture {
        self.textures.borrow()
            .get(cell)
            .unwrap()
            .clone()
    }

    // Get the nearest parent tile found in the CPU buffer
    pub fn get_nearest_parent(&self, cell: &HEALPixCell) -> HEALPixCell {
        if cell.is_root() {
            // Root cells are in the buffer by definition
            *cell
        } else {
            let mut parent_cell = cell.parent();

            while !self.contains(&parent_cell) && !parent_cell.is_root() {
                parent_cell = parent_cell.parent();
            }

            parent_cell
        }
    }

    fn initialize(&mut self) {
        let root_textures = [
            (HEALPixCell(0, 0), true),
            (HEALPixCell(0, 1), true),
            (HEALPixCell(0, 2), true),
            (HEALPixCell(0, 3), true),
            (HEALPixCell(0, 4), true),
            (HEALPixCell(0, 5), true),
            (HEALPixCell(0, 6), true),
            (HEALPixCell(0, 7), true),
            (HEALPixCell(0, 8), true),
            (HEALPixCell(0, 9), true),
            (HEALPixCell(0, 10), true),
            (HEALPixCell(0, 11), true),
        ];

        for (root_texture, new) in root_textures.iter() {
            let root_tiles = root_texture.get_tile_cells(self.config.tile_config());

            let HEALPixTiles(depth, indexes) = root_tiles;
            for idx in indexes {
                let cell = HEALPixCell(depth, idx);
                self.load_tile(&cell, *new);
            }
        }
    }

    // This is called whenever the user:
    // * zoom
    // * unzoom
    // * moves
    pub fn request_textures(&mut self, texture_cells: &HashMap<HEALPixCell, bool>) {
        // Until the buffer is ready, we do not send texture requests
        if self.is_ready() {
            // We set the changed flag to true
            // This will cause the VAO vertices to be recomputed
            self.has_changed = true;

            for (texture_cell, new) in texture_cells.iter() {
                let tiles = texture_cell.get_tile_cells(self.config.tile_config());

                let HEALPixTiles(depth, indexes) = tiles;
                for idx in indexes {
                    let cell = HEALPixCell(depth, idx);
                    self.load_tile(&cell, *new);
                }
            }
        }
    }

    fn load_tile(&mut self,
        // The HEALPix cell to load. First check whether it is already in the buffer
        cell: &HEALPixCell,
        // A flag telling whether the HEALPix cell to load is new (i.e. not contained in the previous
        // field of view).
        new: bool
    ) {
        let already_loaded = self.textures.borrow()
            .contains_tile(cell);

        if already_loaded {
            let time_request = utils::get_current_time();

            // Remove and append the texture with an updated
            // time_request
            self.textures.borrow_mut()
                .update_priority(cell, new, time_request);
        } else {
            let already_requested = self.requested_tiles.borrow()
                .contains(cell);
            // The cell is not already requested
            if !already_requested {
                // Let is do the request
                let time_request = utils::get_current_time();

                let image = Rc::new(
                    Image::HtmlImageElement(
                        HtmlImageElement::new().unwrap()
                    )
                );
                // Add to the tiles requested
                self.requested_tiles.borrow_mut()
                    .insert(*cell);
                // Get the tile config
                let tile_config = self.config.tile_config();
                // Prepare the url
                let url = {
                    let HEALPixCell(depth, idx) = *cell;

                    let dir_idx = (idx / 10000) * 10000;

                    let mut url = self.config.name.to_string() + "/";
                    url = url + "Norder" + &depth.to_string() + "/";
                    url = url + "Dir" + &dir_idx.to_string() + "/";
                    url = url + "Npix" + &idx.to_string();

                    url = url + "." + tile_config.get_ext();
            
                    url
                };

                let onload = {
                    let requested_tiles = self.requested_tiles.clone();
                    let textures = self.textures.clone();
                    let image = image.clone();
                    let cell = *cell;

                    Closure::wrap(Box::new(move || {
                        // Remove the cell from the requested cells set
                        requested_tiles.borrow_mut()
                            .remove(&cell);

                        textures.borrow_mut()
                            .push(&cell, time_request, image.clone());
                    }) as Box<dyn Fn()>)
                };

                let onerror = {
                    let requested_tiles = self.requested_tiles.clone();
                    let textures = self.textures.clone();

                    let cell = *cell;
                    let num_bytes = tile_config.get_num_bytes();

                    Closure::wrap(Box::new(move || {
                        // Remove the cell from the requested cells set
                        requested_tiles.borrow_mut()
                            .remove(&cell);

                        if cell.is_root() {
                            let data = vec![125 as u8; num_bytes].into_boxed_slice();
                            textures.borrow_mut()
                                .push(&cell, time_request, Rc::new(Image::Bytes(data)));
                        }
                    }) as Box<dyn Fn()>)
                };

                match &*image {
                    Image::HtmlImageElement(image) => {
                        image.set_onload(Some(onload.as_ref().unchecked_ref()));
                        image.set_onerror(Some(onerror.as_ref().unchecked_ref()));

                        image.set_cross_origin(Some(""));
                        image.set_src(&url);
                    },
                    _ => unreachable!()
                }

                onload.forget();
                onerror.forget();
            }
        }
    }

    /*pub fn tile_texture_remaining_processes(&self) -> bool {
        !self.futures.is_empty()
    }*/

    // Tell if a texture is available meaning all its sub tiles
    // must have been written for the GPU
    pub fn contains(&self, texture_cell: &HEALPixCell) -> bool {
        if let Some(texture) = self.textures.borrow().get(texture_cell) {
            // The texture is in the buffer i.e. there is at least one
            // sub tile received

            // It is possible that it is not available. Available means
            // all its sub tiles have been received and written to the
            // textures array!
            texture.is_available()
        } else {
            // The texture is not contained in the buffer i.e.
            // even not one sub tile that has been received
            false
        }
    }

    pub fn is_ready(&self) -> bool {
        self.textures.borrow().is_ready()
    }
}

use crate::shader::HasUniforms;
use crate::shader::ShaderBound;
impl HasUniforms for BufferTextures {
    fn attach_uniforms<'a>(&self, shader: &'a ShaderBound<'a>) -> &'a ShaderBound<'a> {
        let textures = self.textures.borrow();
        shader.attach_uniforms_from(&*textures);

        shader
    }
}

