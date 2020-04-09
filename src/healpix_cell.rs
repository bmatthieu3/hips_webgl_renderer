use std::cmp::Ordering;

#[derive(Clone, Copy)]
#[derive(Debug)]
#[derive(PartialEq, Eq, Hash)]
pub struct HEALPixCell(pub u8, pub u64);

use crate::projection::Projection;
use healpix;
use crate::math;
use cgmath::Rad;
use cgmath::InnerSpace;
use crate::viewport::ViewPort;

pub enum Child {
    BottomLeft,
    BottomRight,
    TopLeft,
    TopRight,
}

use crate::utils;
use crate::buffer::TileConfig;
use crate::buffer::Texture;

impl HEALPixCell {
    // Build the parent cell
    #[inline]
    pub fn parent(self) -> HEALPixCell {
        let depth = self.depth();
        if depth == 0 {
            // If cell belongs to a root cell
            // we return it as a root cell do not have any parent
            self
        } else {
            HEALPixCell(depth - 1, self.1 >> 2)
        }
    }

    fn ancestor(self, delta_depth: u8) -> HEALPixCell {
        let HEALPixCell(depth, idx) = self;
        let delta_depth = std::cmp::min(delta_depth, depth);
        
        let ancestor = HEALPixCell(
            depth - delta_depth,
            idx >> (2*delta_depth)
        );
        ancestor
    }

    // Get the texture cell in which the tile is
    pub fn get_texture_cell(&self, config: &TileConfig) -> HEALPixCell {
        let delta_depth_to_texture = config.delta_depth();
        let texture_cell = self.ancestor(delta_depth_to_texture);
        texture_cell
    }
    pub fn get_offset_in_texture_cell(&self, config: &TileConfig) -> (u32, u32) {
        let texture_cell = self.get_texture_cell(config);
        self.offset_in_parent(&texture_cell)
    }

    pub fn offset_in_parent(&self, parent_cell: &HEALPixCell) -> (u32, u32) {
        let HEALPixCell(depth, idx) = *self;
        let HEALPixCell(parent_depth, parent_idx) = *parent_cell;

        let idx_off = parent_idx << (2*(depth - parent_depth));

        assert!(idx >= idx_off);
        assert!(depth >= parent_depth);
        let nside = 1 << (depth - parent_depth);

        let (x, y) = utils::unmortonize(idx - idx_off);
        assert!(x < nside);
        assert!(y < nside);
        //console::log_1(&format!("OFFSET {:?}", (x, y)).into());

        (x, y)
    }

    #[inline]
    pub fn uniq(&self) -> i32 {
        let HEALPixCell(depth, idx) = *self;
        ((16 << (depth << 1)) | idx) as i32
    }

    #[inline]
    pub fn idx(&self) -> u64 {
        self.1
    }

    #[inline]
    pub fn depth(&self) -> u8 {
        self.0
    }

    pub fn get(&self, child: Child) -> Self {
        if self.depth() == 29 {
            *self
        } else {
            let depth = self.depth() + 1;
            let idx_off = self.idx() << 2;

            match child {
                Child::BottomLeft => HEALPixCell(depth, idx_off),
                Child::BottomRight => HEALPixCell(depth, idx_off + 1),
                Child::TopLeft => HEALPixCell(depth, idx_off + 2),
                Child::TopRight => HEALPixCell(depth, idx_off + 3),
            }
        }
    }

    #[inline]
    pub fn is_root(&self) -> bool {
        self.depth() == 0
    }

    // Returns the tile cells being contained into self
    #[inline]
    pub fn get_tile_cells(&self, config: &TileConfig) -> HEALPixTiles {
        let delta_depth = config.delta_depth();

        let HEALPixCell(depth, idx) = *self;
        let first_idx = idx << (2*delta_depth);
        let last_idx = (idx + 1) << (2*delta_depth);

        let depth_tile = depth + delta_depth;
        HEALPixTiles(depth_tile, first_idx..last_idx)
    }
}

use std::ops::Range;
pub struct HEALPixTiles(pub u8, pub Range<u64>);

impl PartialOrd for HEALPixCell {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        let n1 = self.1 << ((29 - self.0) << 1);
        let n2 = other.1 << ((29 - other.0) << 1);

        n1.partial_cmp(&n2)
    }
}
impl Ord for HEALPixCell {
    fn cmp(&self, other: &Self) -> Ordering {
        self.partial_cmp(&other).unwrap()
    }
}

#[derive(Clone, Copy)]
#[derive(Debug)]
#[derive(PartialEq, Eq, Hash)]
struct HEALPixCellUniqOrd<'a> {
    cell: &'a HEALPixCell,
}

impl<'a> PartialOrd for HEALPixCellUniqOrd<'a> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        let u1 = self.cell.uniq();
        let u2 = other.cell.uniq();

        u1.partial_cmp(&u2)
    }
}
impl<'a> Ord for HEALPixCellUniqOrd<'a> {
    fn cmp(&self, other: &Self) -> Ordering {
        self.partial_cmp(&other).unwrap()
    }
}

use std::collections::HashSet;
pub fn allsky(depth: u8) -> HashSet<HEALPixCell> {
    let npix = 12 << (depth << 1);

    let mut cells = HashSet::with_capacity(npix);
    for ipix in 0..npix {
        cells.insert(HEALPixCell(depth, ipix as u64));
    }

    cells
}

pub struct SphereSubdivided(Box<[u8; 196608]>);

fn subdivision(cell: &HEALPixCell) -> u8 {
    let mut subdivision = 0;
    let num_subdivision = recursive_sub(cell, subdivision);

    num_subdivision
}

fn recursive_sub(cell: &HEALPixCell, num_subdivide: u8) -> u8 {
    let mut ratio = compute_ratio_diag(cell);
    if ratio < 0.8 && cell.depth() < 10 {        
        let child_bl = cell.get(Child::BottomLeft);
        let child_br = cell.get(Child::BottomRight);
        let child_tl = cell.get(Child::TopLeft);
        let child_tr = cell.get(Child::TopRight);

        let n1 = recursive_sub(&child_bl, num_subdivide);
        let n2 = recursive_sub(&child_br, num_subdivide);
        let n3 = recursive_sub(&child_tl, num_subdivide);
        let n4 = recursive_sub(&child_tr, num_subdivide);

        let r = std::cmp::max(
            n1, std::cmp::max(
                n2, std::cmp::max(
                    n3, n4
                )
            )
        );

        r + 1
    } else {
        num_subdivide
    }
    /*let HEALPixCell(depth, idx) = *cell;

    let mut d = 11;
    if depth >= d {
        return 0;
    }

    let mut idx1 = idx << (2*(d - depth));
    let mut idx2 = (idx + 1) << (2*(d - depth));

    while d > depth {
        for i in idx1..idx2 {
            let r = compute_ratio_diag(&HEALPixCell(d, i));
            if r < 0.75_f32 {
                return d - depth;
            }
        }
        d -= 1;
        idx1 = idx1 >> 2;
        idx2 = idx2 >> 2;
    }

    0*/
}

fn compute_ratio_diag(cell: &HEALPixCell) -> f32 {
    let HEALPixCell(depth, idx) = *cell;
    let vertices = healpix::nested::vertices(depth, idx);

    let vertices_world_space = vertices.iter()
        .map(|vertex| {
            let (theta, delta) = (Rad(vertex.0 as f32), Rad(vertex.1 as f32));
            let vertex_world_space = math::radec_to_xyzw(theta, delta);

            vertex_world_space
        })
        .collect::<Vec<_>>();

    // Compute the diagonal of the cell
    let d1 = math::angular_distance_xyz(vertices_world_space[0].truncate(), vertices_world_space[2].truncate());
    let d2 = math::angular_distance_xyz(vertices_world_space[1].truncate(), vertices_world_space[3].truncate());
    
    let ratio = if d1 < d2 {
        d1 / d2 
    } else {
        d2 / d1
    };

    ratio
}

/*
fn compute_ratio_projeted_diag<P: Projection>(cell: &HEALPixCell, viewport: &ViewPort) -> f32 {
    let HEALPixCell(depth, idx) = *cell;
    let vertices = healpix::nested::vertices(depth, idx);
    let model = viewport.get_model_mat();

    let vertices_clip_space = vertices.iter()
        .map(|vertex| {
            let (theta, delta) = (Rad(vertex.0 as f32), Rad(vertex.1 as f32));
            let vertex_world_space = math::radec_to_xyzw(theta, delta);

            let vertex_world_space = (*model) * vertex_world_space;

            P::world_to_screen_space(vertex_world_space, viewport)
        })
        .collect::<Vec<_>>();

    // Compute the diagonal of the cell
    let d1 = (vertices_clip_space[0] - vertices_clip_space[2]).magnitude();
    let d2 = (vertices_clip_space[1] - vertices_clip_space[3]).magnitude();

    let ratio = if d1 < d2 {
        d1 / d2 
    } else {
        d2 / d1
    };

    ratio
}*/

use web_sys::console;
impl SphereSubdivided {
    pub fn new() -> SphereSubdivided {
        let mut hpx_sub = Box::new([0; 196608]);

        /*let d = 10;
        let */
        for idx in 0..hpx_sub.len() {
            hpx_sub[idx] = subdivision(&HEALPixCell(7, idx as u64));
        }

        console::log_1(&format!("sub {:?}", hpx_sub.to_vec()).into());

        SphereSubdivided(hpx_sub)
    }

    // Get the number of subdivision necessary for the given cell
    pub fn get_num_subdivide<P: Projection>(&self, cell: &HEALPixCell) -> u8 {
        let HEALPixCell(depth, idx) = *cell;
        let num_sub = if depth < 7 {
            // Get the 3 depth cells contained in it and add
            // each of them individually to the buffer
            let idx_off = (idx << (2*(7 - depth))) as usize;
            let idx_off2 = ((idx + 1) << (2*(7 - depth))) as usize;

            let num_sub_d3_cells = self.0[idx_off..idx_off2].iter().max();
            *num_sub_d3_cells.unwrap() + (7 - depth)
        } else {
            /*let idx_d3 = cell.idx() >> (2*(depth - 6));
            let num_sub_d3 = self[idx_d3 as usize];

            if depth > num_sub_d3 + 6 {
                0
            } else {
                num_sub_d3 - (depth - 6)
            }*/
            0
        };

        std::cmp::min(num_sub, 3)
    }
}

use std::ops::Index;
impl Index<usize> for SphereSubdivided {
    type Output = u8;

    fn index(&self, idx_d3: usize) -> &Self::Output {
        &self.0[idx_d3]
    }
}
