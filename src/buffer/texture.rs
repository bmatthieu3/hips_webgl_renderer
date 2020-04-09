use crate::healpix_cell::HEALPixCell;
use std::collections::HashSet;
/*
#[derive(Clone)]
pub struct Tile {
    pub cell: HEALPixCell,
    // The idx of the ancestor in which the cell is located
    pub idx: i32,

    pub off_x: i32,
    pub off_y: i32,
}

impl Tile {
    fn new(cell: &HEALPixCell, idx: i32, off_x: i32, off_y: i32) -> Tile {
        Tile {
            cell: *cell,
            off_x,
            off_y,
            idx,
        }
    }
}*/

#[derive(Clone)]
pub struct Texture {
    texture_cell: HEALPixCell,
    // Precomputed uniq number
    uniq: i32,
    // The cells located in the Texture
    tiles: HashSet<HEALPixCell>,
    // Position of the texture in the buffer
    idx: i32,
    // The time the texture has been received
    // If the texture contains multiple tiles, then the receiving time
    // is set when all the tiles have been copied to the buffer
    start_time: Option<f32>,
    // The time request of the texture is the time request
    // of the first tile being inserted in it
    // It is then only given in the constructor of Texture
    // This is approximate, it should correspond to the minimum
    // of the time requests of the cells currenlty contained in the 
    // texture. But this is too expensive because at each tile inserted
    // in the buffer, one should reevalute the priority of the texture
    // in the buffer's binary heap.
    time_request: f32,

    // Full flag telling the texture has been filled
    full: bool,
}

use crate::buffer::TileConfig;
use crate::utils;
use crate::core::Texture2DArray;
use crate::buffer::{
 NUM_TEXTURES_BY_SLICE,
 NUM_TEXTURES_BY_SIDE_SLICE
};
use crate::buffer::Image;
use cgmath::{Vector3, Vector2};
use std::rc::Rc;
impl Texture {
    pub fn new(config: &TileConfig, texture_cell: &HEALPixCell, idx: i32, time_request: f32) -> Texture {
        let tiles = HashSet::with_capacity(config.num_tiles_per_texture());

        let start_time = None;
        let full = false;
        let texture_cell = *texture_cell;
        let uniq = texture_cell.uniq();
        Texture {
            texture_cell,
            uniq,
            time_request,
            tiles,
            idx,
            start_time,
            full
        }
    }

    /*fn get_tile(&self, cell: &HEALPixCell) -> Option<Tile> {
        if self.tiles.contains(cell) {
            let (off_x, off_y) = cell.offset_in_parent(&self.texture_cell);

            let tile = Tile::new(cell, self.idx, off_x as i32, off_y as i32);
            Some(tile)
        } else {
            None
        }
    }

    fn get_tiles(&self) -> &HashSet<HEALPixCell> {
        &self.tiles
    }*/

    // Panic if cell is not contained in the texture
    // Do nothing if the texture is full
    // Return true if the tile is newly added
    pub fn append(&mut self,
        cell: &HEALPixCell,
        image: Rc<Image>,
        textures_array: Rc<Texture2DArray>,
        config: &TileConfig
    ) {
        let texture_cell = cell.get_texture_cell(config);
        assert!(texture_cell == self.texture_cell);
        assert!(!self.full);

        // cell has the good ancestor for this texture
        let new_tile = self.tiles.insert(*cell);
        // Ensures the tile was not already present in the buffer
        // This is the case because already contained cells do not
        // lead to new requests
        assert!(new_tile);

        // Copy the tile image into the texture array
        if cell.is_root() {
            self.write_to_textures_array(cell, image, textures_array, config);
        }

        if self.tiles.len() == config.num_tiles_per_texture() {
            // The texture is full, we set its start time
            self.full = true;
            self.start_time = Some(utils::get_current_time());
        }
    }

    // It must be ensured that cell is in the texture
    fn write_to_textures_array(&self,
        cell: &HEALPixCell,
        image: Rc<Image>,
        textures_array: Rc<Texture2DArray>,
        config: &TileConfig
    ) {
        assert!(self.contains(cell));

        // Index of the texture in the total set of textures
        let texture_idx = self.idx;
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

        image.write(&offset, &size, &textures_array);
    }


    pub fn contains(&self, cell: &HEALPixCell) -> bool {
        self.tiles.contains(cell)
    }

    fn is_full(&self) -> bool {
        self.full
    }

    // Getter
    // Returns the current time if the texture is not full
    pub fn start_time(&self) -> f32 {
        if let Some(start_time) = self.start_time {
            start_time
        } else {
            utils::get_current_time()
        }
    }
    pub fn time_request(&self) -> f32 {
        self.time_request
    }
    pub fn cell(&self) -> &HEALPixCell {
        &self.texture_cell
    }
    pub fn idx(&self) -> i32 {
        self.idx
    }

    // Setter
    pub fn update_start_time(&mut self, start_time: f32) {
        // Asserts the texture is full
        assert!(self.is_full());
        self.start_time = Some(start_time);
    }
    pub fn set_time_request(&mut self, time_request: f32) {
        self.time_request = time_request;
    }



    pub fn clear(&mut self, texture_cell: &HEALPixCell, time_request: f32) {
        self.texture_cell = *texture_cell;
        self.uniq = texture_cell.uniq();
        self.full = false;
        self.start_time = None;
        self.time_request = time_request;
        self.tiles.clear();
    }
}

use std::cmp::Ordering;
impl PartialOrd for Texture {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.uniq.partial_cmp(&other.uniq)
    }
}
impl Ord for Texture {
    fn cmp(&self, other: &Self) -> Ordering {
        self.partial_cmp(&other).unwrap()
    }
}

impl PartialEq for Texture {
    fn eq(&self, other: &Self) -> bool {
        self.uniq == other.uniq
    }
}
impl Eq for Texture {}

pub struct TextureUniforms<'a> {
    texture: &'a Texture,
    name: String,
}

impl<'a> TextureUniforms<'a> {
    pub fn new(texture: &Texture, idx_texture: i32) -> TextureUniforms {
        let mut name = String::from("textures");
        name += "_tiles";
        name += "[";
        name += &idx_texture.to_string();
        name += "].";
        TextureUniforms {
            texture,
            name
        }
    }
}

use crate::shader::HasUniforms;
use crate::shader::ShaderBound;
impl<'a> HasUniforms for TextureUniforms<'a> {
    fn attach_uniforms<'b>(&self, shader: &'b ShaderBound<'b>) -> &'b ShaderBound<'b> {
        shader.attach_uniform(&(self.name.clone() + "uniq"), &self.texture.uniq)
            .attach_uniform(&(self.name.clone() + "texture_idx"), &self.texture.idx)
            .attach_uniform(&(self.name.clone() + "start_time"), &self.texture.start_time());

        shader
    }
}