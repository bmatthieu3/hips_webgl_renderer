use cgmath::{Vector3, Vector2};

#[derive(Clone)]
struct WriteTexture {
    tile: HEALPixCell, // The tile cell that has been written
    texture: HEALPixCell, // the texture cell that contains tile
    offset: Vector3<i32>,
    size: Vector2<i32>,
    image: Rc<Image>,
    textures_array: Rc<Texture2DArray>
}

use crate::core::Texture2DArray;
use crate::buffer::{Image, Texture, TileConfig};
use crate::buffer::{
 NUM_TEXTURES_BY_SLICE,
 NUM_TEXTURES_BY_SIDE_SLICE
};
use std::rc::Rc;
use crate::healpix_cell::HEALPixCell;
impl WriteTexture {
    fn new(
     cell: &HEALPixCell, // The tile cell. It must lie in the texture
     texture: &Texture,
     image: Rc<Image>,
     textures_array: Rc<Texture2DArray>,
     config: &TileConfig
    ) -> WriteTexture {
        // Index of the texture in the total set of textures
        let texture_idx = texture.idx();
        // Index of the slice of textures
        let idx_slice = texture_idx / NUM_TEXTURES_BY_SLICE;
        // Index of the texture in its slice
        let idx_in_slice = texture_idx % NUM_TEXTURES_BY_SLICE;

        // Index of the column of the texture in its slice
        let idx_col_in_slice = idx_in_slice / NUM_TEXTURES_BY_SIDE_SLICE;
        // Index of the row of the texture in its slice
        let idx_row_in_slice = idx_in_slice % NUM_TEXTURES_BY_SIDE_SLICE;

        // Row and column indexes of the tile in its texture
        let (idx_col_in_tex, idx_row_in_tex) = cell.get_offset_in_texture_cell(config);

        // The size of the global texture containing the tiles
        let texture_size = config.get_texture_size();
        // The size of a tile in its texture
        let tile_size = config.get_tile_size();

        // Offset in the slice in pixels
        let offset = Vector3::new(
            (idx_row_in_slice as i32) * texture_size + (idx_row_in_tex as i32) * tile_size,
            (idx_col_in_slice as i32) * texture_size + (idx_col_in_tex as i32) * tile_size,
            idx_slice
        );
        let size = Vector2::new(
            tile_size,
            tile_size,
        );

        let tile = *cell;
        let texture = *texture.cell();
        WriteTexture {
            tile,
            texture,
            offset,
            size,
            image,
            textures_array,
        }
    }
}

use futures::Future;
use futures::task::{Poll, Context};
use core::pin::Pin;
impl Future for WriteTexture {
    type Output = ();

    fn poll(self: Pin<&mut Self>, cx: &mut Context) -> Poll<Self::Output> {
        let ref offset = self.offset;
        let ref size = self.size;
        let textures_array = &self.textures_array;

        // Costly call here that copy the tile pixels in its texture
        // for the GPU
        self.image.write(offset, size, textures_array);

        Poll::Ready(())
    }
}

pub struct Worker {
    tasks: Vec<WriteTexture>,
    textures_array: Rc<Texture2DArray>,
    //num_root_textures_written: usize,
}

use crate::utils;
use futures::executor::block_on;
use web_sys::console;
use std::collections::HashSet;
impl Worker {
    pub fn new(textures_array: Rc<Texture2DArray>) -> Worker {
        let tasks = Vec::new();
        //let num_root_textures_written = 0;
        Worker {
            tasks,
            textures_array,
            //num_root_textures_written
        }
    }

    // Used for running tasks immediately i.e. write root texture cells
    pub fn block_on_task(&self,
     cell: &HEALPixCell,
     texture: &mut Texture,
     image: Rc<Image>,
     config: &TileConfig
    ) {
        let task = WriteTexture::new(cell, texture, image, self.textures_array.clone(), config);
        block_on(task);
        // Register immediatly the tile as 'having being written'
        texture.register_written_tile(cell, config);
    }

    pub fn append_task(&mut self, 
     cell: &HEALPixCell,
     texture: &Texture,
     image: Rc<Image>,
     config: &TileConfig
    ) {
        let task = WriteTexture::new(cell, texture, image, self.textures_array.clone(), config);
        self.tasks.push(task);
    }

    // Return the set of tile cells that have been written (i.e. tiles within it)
    pub fn run(&mut self) -> HashSet<HEALPixCell> {
        // Get the current time
        let start_time = utils::get_current_time();
        // Define a maximum time duration in which tasks are polled
        let duration = 10_f32; // in milliseconds

        let mut tiles_written = HashSet::new();

        while !self.tasks_finished() && (utils::get_current_time() - start_time) < duration {
            let task = self.tasks.pop().unwrap();
            tiles_written.insert(task.tile);

            block_on(task);
        }

        //console::log_1(&format!("num textures: {:?}", num_textures_written).into());

        tiles_written
    }

    // This method is called when a texture is cleared but
    // some tasks involving tiles in it are still in the queue
    // to be processed.
    //
    // This ensures these remaining tasks are removed from the worker
    // and will not be processed anymore.
    pub fn clear_texture_tasks(&mut self, texture_cell: &HEALPixCell) {
        self.tasks = self.tasks.clone()
            .into_iter()
            .filter(|task| {
                task.texture != *texture_cell
            })
            .collect();
    }

    pub fn clear(&mut self) {
        //self.num_root_textures_written = 0;
        self.tasks.clear();
    }

    fn tasks_finished(&self) -> bool {
        self.tasks.is_empty()
    }
}