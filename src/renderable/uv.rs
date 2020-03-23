use num::{Float, Zero};
use cgmath::{Vector2, Vector3};

pub struct UV<T: Float + Zero>([Vector2<T>; 4]);
impl<T> UV<T>
where T: Float + Zero {
    pub fn empty() -> UV<T> {
        UV([Vector2::new(T::zero(), T::zero()); 4])
    }

    // The idx of the tile in the texture
    pub fn new(u0: T, v0: T, size: T) -> UV<T> {
        UV::<T>([
            Vector2::new(u0, v0),
            Vector2::new(u0 + size, v0),
            Vector2::new(u0, v0 + size),
            Vector2::new(u0 + size, v0 + size)
        ])
    }
}

use crate::buffer_tiles::{
 NUM_TILES_BY_TEXTURE,
 NUM_CELLS_BY_TEXTURE_SIDE
};
use crate::healpix_cell::HEALPixCell;
use crate::buffer_tiles::TileInTextureBuffer;
use crate::utils;
pub struct TileUVW([Vector3<f32>; 4]);
impl TileUVW {
    pub fn empty() -> TileUVW {
        TileUVW([Vector3::new(0_f32, 0_f32, 0_f32); 4])
    } 

    /*pub fn new(u: f32, v: f32, ds: f32, w: f32) -> TileUVW {
        TileUVW([
            Vector3::new(u, v, w),
            Vector3::new(u + ds, v, w),
            Vector3::new(u, v + ds, w),
            Vector3::new(u + ds, v + ds, w)
        ])
    }*/

    // Search in the buffer the UV of the cell
    // At this point, cell is in the CPU buffer, but we do not know if the tile is 
    // in the GPU texture
    //
    // We should be able to return the nearest parent cell loaded in the GPU buffer
    // whilst the needed tile texture is moved into the big GPU textures
    pub fn look(idx: usize) -> TileUVW {
        let idx_texture = (idx / NUM_TILES_BY_TEXTURE) as f32;
        let idx_in_texture = idx % NUM_TILES_BY_TEXTURE;

        let idx_row = (idx_in_texture / NUM_CELLS_BY_TEXTURE_SIDE) as f32;
        let idx_col = (idx_in_texture % NUM_CELLS_BY_TEXTURE_SIDE) as f32;

        let u = idx_col / (NUM_CELLS_BY_TEXTURE_SIDE as f32);
        let v = idx_row / (NUM_CELLS_BY_TEXTURE_SIDE as f32);

        let ds = 1_f32 / (NUM_CELLS_BY_TEXTURE_SIDE as f32);
        TileUVW([
            Vector3::new(u, v, idx_texture),
            Vector3::new(u + ds, v, idx_texture),
            Vector3::new(u, v + ds, idx_texture),
            Vector3::new(u + ds, v + ds, idx_texture)
        ])
    }

    // Return the uv in the gpu texture of cell.
    // Cell might not be in the gpu texture, if so, the cell_in_buf uv are given instead.
    pub fn new(cell: &HEALPixCell, cell_in_buf: &TileInTextureBuffer) -> TileUVW {
        /*if *cell  == cell_in_buf.cell {
            let idx_texture = (cell_in_buf.idx_texture / NUM_TILES_BY_TEXTURE) as f32;
            let idx_in_texture = cell_in_buf.idx_texture % NUM_TILES_BY_TEXTURE;

            let idx_row = (idx_in_texture / NUM_CELLS_BY_TEXTURE_SIDE) as f32;
            let idx_col = (idx_in_texture % NUM_CELLS_BY_TEXTURE_SIDE) as f32;

            let u = idx_col / (NUM_CELLS_BY_TEXTURE_SIDE as f32);
            let v = idx_row / (NUM_CELLS_BY_TEXTURE_SIDE as f32);

            let ds = 1_f32 / (NUM_CELLS_BY_TEXTURE_SIDE as f32);
            return TileUVW([
                Vector3::new(u, v, idx_texture),
                Vector3::new(u + ds, v, idx_texture),
                Vector3::new(u, v + ds, idx_texture),
                Vector3::new(u + ds, v + ds, idx_texture)
            ]);
        }*/

        let HEALPixCell(depth, idx) = *cell;
        let HEALPixCell(parent_depth, parent_idx) = cell_in_buf.cell;

        let idx_off = parent_idx << (2*(depth - parent_depth));

        assert!(idx >= idx_off);
        assert!(depth >= parent_depth);
        let nside = (1 << (depth - parent_depth));

        let (x, y) = utils::unmortonize(idx - idx_off);
        assert!(x < nside);
        assert!(y < nside);
        //console_log!("x {:?}, y {:?}, nside {:?}", x, y, nside);

        let parent_idx_texture = cell_in_buf.idx_texture;
        let idx_texture = (parent_idx_texture / NUM_TILES_BY_TEXTURE) as f32;
        let parent_idx_in_texture = parent_idx_texture % NUM_TILES_BY_TEXTURE;

        let parent_idx_row = (parent_idx_in_texture / NUM_CELLS_BY_TEXTURE_SIDE) as f32; // in [0; 7]
        let parent_idx_col = (parent_idx_in_texture % NUM_CELLS_BY_TEXTURE_SIDE) as f32; // in [0; 7]
        let u = (parent_idx_col + ((y as f32)/(nside as f32))) / (NUM_CELLS_BY_TEXTURE_SIDE as f32);
        let v = (parent_idx_row + ((x as f32)/(nside as f32))) / (NUM_CELLS_BY_TEXTURE_SIDE as f32);

        let ds = 1_f32 / ((NUM_CELLS_BY_TEXTURE_SIDE as f32) * (nside as f32));

        TileUVW([
            Vector3::new(u, v, idx_texture),
            Vector3::new(u + ds, v, idx_texture),
            Vector3::new(u, v + ds, idx_texture),
            Vector3::new(u + ds, v + ds, idx_texture)
        ])
    }
}

pub enum TileCorner {
    BottomLeft,
    BottomRight,
    TopLeft,
    TopRight,
}
use std::ops::Index;
impl<T> Index<TileCorner> for UV<T>
where T: Float + Zero {
    type Output = Vector2<T>;

    fn index(&self, corner: TileCorner) -> &Self::Output {
        match corner {
            TileCorner::BottomLeft => &self.0[0],
            TileCorner::BottomRight => &self.0[1],
            TileCorner::TopLeft => &self.0[2],
            TileCorner::TopRight => &self.0[3],
        }
    }
}
impl Index<TileCorner> for TileUVW {
    type Output = Vector3<f32>;

    fn index(&self, corner: TileCorner) -> &Self::Output {
        match corner {
            TileCorner::BottomLeft => &self.0[0],
            TileCorner::BottomRight => &self.0[1],
            TileCorner::TopLeft => &self.0[2],
            TileCorner::TopRight => &self.0[3],
        }
    }
}