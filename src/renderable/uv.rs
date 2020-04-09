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

use crate::buffer::{
 NUM_TEXTURES_BY_SIDE_SLICE,
 NUM_TEXTURES_BY_SLICE
};
use crate::healpix_cell::HEALPixCell;
use crate::buffer::Texture;
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

    // The texture cell passed must be a child of texture
    pub fn new(child_texture_cell: &HEALPixCell, texture: &Texture) -> TileUVW {
        let HEALPixCell(depth, idx) = *child_texture_cell;
        let HEALPixCell(parent_depth, parent_idx) = *texture.cell();

        let idx_off = parent_idx << (2*(depth - parent_depth));

        assert!(idx >= idx_off);
        assert!(depth >= parent_depth);
        let nside = (1 << (depth - parent_depth)) as f32;

        let (x, y) = utils::unmortonize(idx - idx_off);
        let x = x as f32;
        let y = y as f32;
        assert!(x < nside);
        assert!(y < nside);

        let parent_idx_texture = texture.idx();
        let idx_texture = (parent_idx_texture / NUM_TEXTURES_BY_SLICE) as f32;
        let parent_idx_in_texture = parent_idx_texture % NUM_TEXTURES_BY_SLICE;

        let parent_idx_row = (parent_idx_in_texture / NUM_TEXTURES_BY_SIDE_SLICE) as f32; // in [0; 7]
        let parent_idx_col = (parent_idx_in_texture % NUM_TEXTURES_BY_SIDE_SLICE) as f32; // in [0; 7]

        let num_textures_by_side_slice_f32 = NUM_TEXTURES_BY_SIDE_SLICE as f32;
        let u = (parent_idx_col + (y/nside)) / num_textures_by_side_slice_f32;
        let v = (parent_idx_row + (x/nside)) / num_textures_by_side_slice_f32;

        let ds = 1_f32 / (num_textures_by_side_slice_f32 * nside);

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