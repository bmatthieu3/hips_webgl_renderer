use crate::healpix_cell::HEALPixCell;

#[derive(Clone)]
#[derive(Debug)]
pub struct TileNode {
    cell: HEALPixCell,
    time_request: f32,
}

impl PartialEq for TileNode {
    fn eq(&self, other: &Self) -> bool {
        self.cell == other.cell
    }
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

use web_sys::HtmlImageElement;
#[derive(Debug)]
pub enum TileTexture {
    HtmlImageElement(HtmlImageElement),
    Black,
}

use std::sync::{Arc, Mutex};
#[derive(Debug)]
pub struct Tile {
    pub cell: HEALPixCell,
    pub uniq: u32,

    pub time_request: f32,
    pub time_received: f32,

    pub texture: Arc<Mutex<TileTexture>>,
}

pub const BLENDING_DURATION_MS: f32 = 500_f32;

impl Tile {
    pub fn new(cell: HEALPixCell,
        time_request: f32,
        time_received: f32,
        texture: Arc<Mutex<TileTexture>>,
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

    pub fn depth(&self) -> u8 {
        self.cell.depth()
    }

    pub fn idx(&self) -> u64 {
        self.cell.idx()
    }
}

impl PartialEq for Tile {
    fn eq(&self, other: &Self) -> bool {
        self.cell == other.cell
    }
}
impl Eq for Tile {}


use std::collections::BinaryHeap;
use std::collections::HashMap;
// Fixed sized binary heap
pub struct BinaryHeapTiles {
    heap: BinaryHeap<TileNode>,
    pub base_cells: Vec<TileNode>,

    max_length: usize,

    pub tiles: HashMap<HEALPixCell, Tile>,

    // A boolean ensuring the base tiles
    // have already been loaded
    ready: bool
}

impl BinaryHeapTiles {
    pub fn new(max_length: usize) -> BinaryHeapTiles {
        assert!(max_length >= 12);
        let heap = BinaryHeap::with_capacity(max_length - 12);
        let base_cells = Vec::with_capacity(12);

        let tiles = HashMap::with_capacity(max_length);

        // The base tiles have not been loaded
        let ready = false;

        // Push the 
        BinaryHeapTiles {
            heap,
            base_cells,

            max_length,

            tiles,

            ready,
        }
    }

    // Do not push tiles being already in the buffer
    pub fn push(&mut self, tile: Tile) {
        if !self.contains(&tile.cell) {
            let depth = tile.cell.0;
            if depth == 0 {
                let node = (&tile).into();
                self.base_cells.push(node);

                // If the base tiles have all been loaded
                if self.base_cells.len() == 12 {
                    // Then the tile buffer is ready
                    // to be queried
                    self.ready = true;
                }
            } else {
                if self.heap.len() == self.max_length {
                    // Pop the oldest requested tile
                    let node = self.heap.pop()
                        .unwrap();
                    assert!(node.cell.0 != 0);

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

    pub fn get_mut(&mut self, cell: &HEALPixCell) -> Option<&mut Tile> {
        self.tiles.get_mut(cell)
    }

    // Panic if cell is not in the binary heap
    pub fn update_priority(&mut self, cell: &HEALPixCell, time_request: f32, time_received: f32) {
        let tile = self.tiles.get_mut(cell).unwrap();
        tile.time_request = time_request;
        tile.time_received = time_received;
        
        if cell.depth() == 0 {
            return;
        }

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

        self.ready = false;
    }

    pub fn is_ready(&self) -> bool {
        self.ready
    }
}
