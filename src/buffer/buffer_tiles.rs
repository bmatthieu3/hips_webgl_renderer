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
/*
struct HEALPixTextureWrite {
    buf: Arc<Mutex<HEALPixTexture>>,
    tiles: Arc<Mutex<BinaryHeapTiles>>,

    cell: HEALPixCell,
    ancestor: HEALPixCell,
}

impl Future for HEALPixTextureWrite {
    type Output = ();

    fn poll(self: Pin<&mut Self>, cx: &mut Context) -> Poll<Self::Output> {
        let cell = self.cell;
        let ancestor = self.ancestor;

        // We ensure the cell is still contained in the cpu buffer
        // e.g. has not been removed from it
        let tiles = self.tiles.lock().unwrap();
        if tiles.contains(&cell) {
            let mut buffer = self.buf.lock().unwrap();
            let start_time = utils::get_current_time();

            let new_tile = if cell.depth() == 0 {
                // 0 depth tiles are put at the indexes 0 -> 11
                Tile::new(&cell, ancestor.idx() as i32, start_time)
            } else {
                // 1 -> 29 depth tiles are put at indexes 12 -> NUM_TILES
                let num_tiles_per_texture_side = 1 << buffer.config.delta_depth;
                let num_tiles_per_texture = num_tiles_per_texture_side * num_tiles_per_texture_side;
                let num_tiles = NUM_TEXTURES * num_tiles_per_texture;

                let tile = if buffer.cells.len() == num_tiles {
                    let lost_cell = buffer.cells.pop_front().unwrap();
                    buffer.task_launched.remove(&lost_cell);

                    let mut lost_tile = buffer.tile_textures.remove(&lost_cell).unwrap();
                    lost_tile.start_time = start_time;
                    //lost_tile.cell = ancestor;

                    lost_tile
                } else {
                    Tile::new(&cell, buffer.cells.len() as i32 + 12, start_time)
                };
                
                // We need to avoid having doublon ancestors here
                // If the delta_depth is 0, (i.e. there is one tile per texture)
                // Then the ancestor will be unique so we do nothing
                if buffer.config.delta_depth > 0 {
                    // Otherwise, several cells can have the same ancestor

                    // Seek the position of the cell in the vecqueue
                    let idx_ancestor = buffer.cells.iter().position(|c| *c == ancestor).unwrap();
                    // Remove it
                    buffer.cells.remove(idx_ancestor);
                }
                // Push the ancestor to the queue
                buffer.cells.push_back(ancestor);

                tile
            };

            // The tile is contained in the cpu buffer so we can get it
            let texture = tiles.get(&cell).unwrap().texture();
            let idx = new_tile.idx;
            let (off_x, off_y) = cell.offset_in_parent(&ancestor);

            buffer.tex_sub_image_2d(
                texture, // The texture data
                idx, // The idx of the ancestor tile

                off_y, // X off of the tile in ancestor 
                off_x, // Y off of the tile in ancestor
            );

            buffer.tile_textures.insert(ancestor, new_tile);
        } else {
            let mut buffer = self.buf.lock().unwrap();
            buffer.task_launched.remove(&ancestor);
        }

        // If it has been removed, we exit the future without doing
        // anything
        Poll::Ready(())
    }
}
*/

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
    // LocalPool
    //pool: LocalPool,
    //futures: Vec<HEALPixTextureWrite>, // A LIFO containing the futures to run
}

use crate::utils;

macro_rules! console_error {
    ( $( $t:tt )* ) => {
        web_sys::console::error(
            &js_sys::Array::from(
                &format!( $( $t )* ).into()
            )
        );
    }
}

use crate::buffer::{
 TileConfig,
 NUM_TEXTURES,
 Texture,
};

use crate::buffer::Image;
use crate::healpix_cell::HEALPixTiles;
impl BufferTextures {
    pub fn new(gl: &WebGl2Context, config: &HiPSConfig) -> BufferTextures {
        // Arbitrary number decided here
        let requested_tiles = Rc::new(RefCell::new(HashSet::with_capacity(64)));

        let tile_config = config.tile_config();
        let textures = Rc::new(RefCell::new(Textures::new(gl, tile_config)));

        let gl = gl.clone();
        //let textures_needed_per_frame = HashSet::with_capacity(NUM_TEXTURES);

        //let pool = LocalPool::new();
        //let futures = vec![];

        let config = config.clone();
        let mut buffer = BufferTextures {
            gl,

            textures,
            requested_tiles,

            //textures_needed_per_frame,

            config,

            //pool,
            //futures,
        };

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
            ].iter()
            .cloned()
            .collect();

        buffer.request_textures(&root_textures);

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
            ].iter()
            .cloned()
            .collect();

        self.request_textures(&root_textures);
    }
/*
    pub fn signals_new_frame(&mut self) {
        self.tiles_needed_per_frame.clear();
    }
    pub fn signals_end_frame(&mut self) {
        // Ensure the current frame does not need more than
        // 64 different tiles. Otherwise it will exceed the texture
        // size capacity!
        let num_tiles_needed = self.tiles_needed_per_frame.len();
        if num_tiles_needed > NUM_TILES {
            console_error!(
                "The number of tiles needed to be plot ({:?}) exceeds the GPU texture buffer size ({:?})",
                num_tiles_needed, NUM_TILES
            );
        }
    }
*/
    /*// This method must be called between ``signals_new_frame`` and ``signals_end_frame``
    // It counts the number of tiles needed by the current frame by added successively the cells
    // in a HashSet. The number of tiles needed cannot exceed the maximum number of tiles
    // that can be sent to the GPU (e.g. NUM_TILES)
    pub fn get_cell_in_texture(&self, cell: &HEALPixCell) -> Tile {
        /*let delta_depth = self.hpx_texture.lock().unwrap()
            // Access to the tile config
            .config
            .delta_depth;
        let ancestor = cell.ancestor(delta_depth);
        self.tiles_needed_per_frame.insert(ancestor);

        let mut spawn_future = false;
        let tile = self.hpx_texture.lock().unwrap().insert(*cell, &mut spawn_future);

        if spawn_future {
            // Launch the async call to copy the cell to the GPU texture
            // This is done only if the tile texture is not already in the GPU texture
            let future = HEALPixTextureWrite {
                buf: self.hpx_texture.clone(),
                tiles: self.heap.clone(),
                cell: *cell,
                ancestor: ancestor,
            };
            self.futures.push(future);
        }

        tile*/

        // The cell is contained in the buffer
        // its image has been written to the buffer
        self.texture_buf.lock().unwrap()
            .get_tile(cell)
            .unwrap()
    }*/

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

    pub fn request_textures(&mut self, texture_cells: &HashMap<HEALPixCell, bool>) {
        for (texture_cell, new) in texture_cells.iter() {
            let tiles = texture_cell.get_tile_cells(self.config.tile_config());

            let HEALPixTiles(depth, indexes) = tiles;
            for idx in indexes {
                let cell = HEALPixCell(depth, idx);
                self.load_tile(&cell, *new);
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

    pub fn contains(&self, texture_cell: &HEALPixCell) -> bool {
        self.textures.borrow().contains_texture(texture_cell)
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

