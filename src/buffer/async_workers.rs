use cgmath::{Vector3, Vector2};

struct WriteTexture {
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

        WriteTexture {
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
/*
struct Workers {
    tasks: Vec<WriteTexture>
}*/