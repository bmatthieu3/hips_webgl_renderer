use crate::healpix_cell::HEALPixCell;

#[derive(PartialEq)]
#[derive(Clone)]
struct TileNode {
    cell: HEALPixCell,
    time_request: f32,
}

impl Eq for TileNode {}

use std::cmp::Ordering;
// Ordering based on the time the tile has been requested
impl PartialOrd for TileNode {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        other.time_request.partial_cmp(&self.time_request)
    }
}
impl Ord for TileNode {
    fn cmp(&self, other: &Self) -> Ordering {
        self.partial_cmp(&other).unwrap()
    }
}

impl From<Tile> for TileNode {
    fn from(tile: Tile) -> Self {
        let time_request = tile.time_request;
        let cell = tile.cell;

        TileNode {
            cell,
            time_request,
        }
    }
}
impl From<&Tile> for TileNode {
    fn from(tile: &Tile) -> Self {
        let time_request = tile.time_request;
        let cell = tile.cell;

        TileNode {
            cell,
            time_request,
        }
    }
}
impl From<&mut Tile> for TileNode {
    fn from(tile: &mut Tile) -> Self {
        let time_request = tile.time_request;
        let cell = tile.cell;

        TileNode {
            cell,
            time_request,
        }
    }
}

#[derive(Clone)]
pub struct Tile {
    pub cell: HEALPixCell,
    pub uniq: u32,

    pub time_request: f32,
    pub time_received: f32,

    pub texture: Rc<RefCell<HtmlImageElement>>,
}

pub const BLENDING_DURATION_MS: f32 = 500_f32;
use web_sys::HtmlImageElement;
use std::cell::RefCell;
use std::rc::Rc;
use crate::utils;
impl Tile {
    pub fn new(cell: HEALPixCell,
        time_request: f32,
        time_received: f32,
        texture: Rc<RefCell<HtmlImageElement>>,
    ) -> Tile {
        let (depth, idx) = (cell.0, cell.1);
        let uniq = ((16 << (depth << 1)) | idx) as u32;

        Tile {
            cell,
            uniq,

            time_request,
            time_received,

            texture
        }
    }

    pub fn blending_factor(&self) -> f32 {
        let mut t = (utils::get_current_time() - self.time_received) / BLENDING_DURATION_MS;

        if t > 1_f32 {
            t = 1_f32;
        } else if t < 0_f32 {
            t = 0_f32;
        }

        t
    }
}

impl PartialEq for Tile {
    fn eq(&self, other: &Self) -> bool {
        self.cell == other.cell
    }
}
impl Eq for Tile {}

/*
use std::hash::{Hash, Hasher};
impl Hash for Tile {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.cell.hash(state);
    }
}
*/

use std::collections::BinaryHeap;
use std::collections::HashMap;
// Fixed sized binary heap
pub struct BinaryHeapTiles {
    heap: BinaryHeap<TileNode>,
    base_cells: Vec<TileNode>,

    max_length: usize,

    tiles: HashMap<HEALPixCell, Tile>,
}

impl BinaryHeapTiles {
    pub fn new(max_length: usize) -> BinaryHeapTiles {
        assert!(max_length >= 12);
        let heap = BinaryHeap::with_capacity(max_length - 12);
        let base_cells = Vec::with_capacity(12);

        let tiles = HashMap::with_capacity(max_length);

        // Push the 
        BinaryHeapTiles {
            heap,
            base_cells,

            max_length,

            tiles,
        }
    }

    // Do not push tiles being already in the buffer
    pub fn push(&mut self, tile: Tile) {
        if !self.contains(&tile.cell) {
            let depth = tile.cell.0;
            if depth == 0 {
                let node = (&tile).into();
                self.base_cells.push(node);
            } else {
                if self.heap.len() == self.max_length {
                    // Pop the oldest requested tile
                    let node = self.heap.pop()
                        .unwrap();
                    self.tiles.remove(&node.cell);
                }
                // Add the new one
                let node = (&tile).into();
                self.heap.push(node);
            }
            self.tiles.insert(tile.cell, tile);
        }
    }

    pub fn contains(&self, cell: &HEALPixCell) -> bool {
        self.tiles.contains_key(cell)
    }

    pub fn get(&self, cell: &HEALPixCell) -> Option<&Tile> {
        self.tiles.get(cell)
    }

    // Panic if cell is not in the binary heap
    pub fn update_priority(&mut self, cell: &HEALPixCell, time_request: f32, time_received: f32) {
        let tile = self.tiles.get_mut(cell).unwrap();
        tile.time_request = time_request;
        tile.time_received = time_received;

        self.heap = self.heap.iter()
            // Remove the cell
            .filter(|node| node.cell != *cell)
            // Clone the iterator to get an iterator of TileNode
            .cloned()
            // Collect to a new binary heap that do not have cell anymore
            .collect::<BinaryHeap<_>>();
        let node = tile.into();
        // Push the cell again that has the new time_request
        self.heap.push(node);
    }

    pub fn clear(&mut self) {
        self.base_cells.clear();
        self.heap.clear();

        self.tiles.clear();
    }
}
