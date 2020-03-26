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

impl HEALPixCell {
    // Build the parent cell
    pub fn parent(self) -> HEALPixCell {
        if self.depth() == 0 {
            // If cell belongs to a root cell
            // we return it as a root cell do not have any parent
            self
        } else {
            HEALPixCell(self.0 - 1, self.1 >> 2)
        }
    }

    pub fn idx(&self) -> u64 {
        self.1
    }

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
}

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

pub struct SphereSubdivided(Box<[u8; 196608]>);

/*const fn num_bits<T>() -> usize { std::mem::size_of::<T>() * 8 }

fn log_2(x: u8) -> u32 {
    assert!(x > 0);
    num_bits::<u8>() as u32 - x.leading_zeros() - 1
}*/ 

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
    pub fn get_num_subdivide<P: Projection>(&self, cell: &HEALPixCell, viewport: &ViewPort, depth_start: u8) -> u8 {
        /*if cell.depth() == 29 || (cell.depth() - depth_start) == 3 {
            return 0;
        }

        //let ratio = compute_ratio_projeted_diag::<P>(cell, viewport);
        let ratio = compute_ratio_diag(cell);
        if ratio < 0.8 {
            let cbl = &cell.get(Child::BottomLeft);
            let n1 = self.get_num_subdivide::<P>(cbl, viewport, depth_start);
            let cbr = &cell.get(Child::BottomRight);
            let n2 = self.get_num_subdivide::<P>(cbr, viewport, depth_start);
            let ctl = &cell.get(Child::TopLeft);
            let n3 = self.get_num_subdivide::<P>(ctl, viewport, depth_start);
            let ctr = &cell.get(Child::TopRight);
            let n4 = self.get_num_subdivide::<P>(ctr, viewport, depth_start);

            let r = std::cmp::max(n1, std::cmp::max(n2, std::cmp::max(n3, n4)));
            r + 1
        } else {
            if cell.depth() < 3 {
                3 - cell.depth()
            } else {
                0
            }
        }*/
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
