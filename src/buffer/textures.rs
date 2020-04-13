use crate::healpix_cell::HEALPixCell;

#[derive(Clone)]
#[derive(Debug)]
pub struct TextureNode {
    cell: HEALPixCell,
    time_request: f32,
}

impl TextureNode {
    fn is_root(&self) -> bool {
        self.cell.is_root()
    }
}

impl PartialEq for TextureNode {
    fn eq(&self, other: &Self) -> bool {
        self.cell == other.cell
    }
}
impl Eq for TextureNode {}

use std::cmp::Ordering;
// Ordering based on the time the tile has been requested
impl PartialOrd for TextureNode {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        other.time_request.partial_cmp(&self.time_request)
    }
}
impl Ord for TextureNode {
    fn cmp(&self, other: &Self) -> Ordering {
        self.partial_cmp(&other).unwrap()
    }
}

use crate::buffer::Texture;
impl From<Texture> for TextureNode {
    fn from(texture: Texture) -> Self {
        let time_request = texture.time_request();
        let cell = *texture.cell();

        TextureNode {
            cell,
            time_request,
        }
    }
}
impl From<&Texture> for TextureNode {
    fn from(texture: &Texture) -> Self {
        let time_request = texture.time_request();
        let cell = *texture.cell();

        TextureNode {
            cell,
            time_request,
        }
    }
}
impl From<&mut Texture> for TextureNode {
    fn from(texture: &mut Texture) -> Self {
        let time_request = texture.time_request();
        let cell = *texture.cell();

        TextureNode {
            cell,
            time_request,
        }
    }
}

pub const BLENDING_DURATION_MS: f32 = 500_f32;
/*
use std::sync::{Arc, Mutex};
#[derive(Debug)]
pub struct Tile {
    cell: HEALPixCell,
    uniq: i32,

    time_request: f32,

    image: Arc<Mutex<Image>>,
}

impl Tile {
    pub fn new(cell: &HEALPixCell,
        time_request: f32,
        image: Arc<Mutex<Image>>,
    ) -> Tile {
        let HEALPixCell(depth, idx) = *cell;
        let uniq = ((16 << (depth << 1)) | idx) as i32;

        Tile {
            cell: *cell,
            uniq,

            time_request,
            //time_received,

            image
        }
    }

    pub fn depth(&self) -> u8 {
        self.cell.depth()
    }

    pub fn idx(&self) -> u64 {
        self.cell.idx()
    }

    pub fn uniq(&self) -> i32 {
        self.uniq
    }

    pub fn texture(&self) -> Arc<Mutex<Image>> {
        self.image.clone()
    }
}

impl PartialEq for Tile {
    fn eq(&self, other: &Self) -> bool {
        self.cell == other.cell
    }
}
impl Eq for Tile {}
*/
use std::collections::BinaryHeap;
use std::collections::HashMap;
use crate::core::Texture2DArray;
use crate::buffer::TileConfig;

use crate::buffer::Worker;
// Fixed sized binary heap
pub struct Textures {
    heap: BinaryHeap<TextureNode>,

    pub root_textures: Vec<TextureNode>,
    num_root_textures_available: usize,

    size: usize,

    pub textures: HashMap<HEALPixCell, Texture>,

    // Array of 2D textures
    texture_2d_array: Rc<Texture2DArray>,

    // A config describing the tile (its storage format, extension, number of color channels)
    config: TileConfig,

    // Write textures worker
    worker: Worker,

    // A boolean ensuring the root textures
    // have already been loaded
    ready: bool
}
use crate::buffer::NUM_TEXTURES;
use crate::WebGl2Context;
use crate::utils;
use crate::buffer::Image;
use web_sys::console;
use std::rc::Rc;
impl Textures {
    pub fn new(gl: &WebGl2Context, config: &TileConfig) -> Textures {
        let size = NUM_TEXTURES;
        // Ensures there is at least space for the 12
        // root textures
        assert!(size >= 12);
        let heap = BinaryHeap::with_capacity(size - 12);
        let root_textures = Vec::with_capacity(12);

        let textures = HashMap::with_capacity(size);
        
        let texture_2d_array = Rc::new(config.create_texture_array(gl));
        console::log_1(&format!("config: {:?}", config).into());

        // The root textures have not been loaded
        let ready = false;
        let num_root_textures_available = 0;
        let config = *config;

        let worker = Worker::new(texture_2d_array.clone());
        // Push the 
        Textures {
            heap,
            root_textures,

            size,
            num_root_textures_available,

            textures,
            texture_2d_array,
            config,

            ready,

            worker,
        }
    }

    // This method pushes a new downloaded tile into the buffer
    // It must be ensured that the tile is not already contained into the buffer
    pub fn push(&mut self, cell: &HEALPixCell, time_request: f32, image: Rc<Image>) {
        // Assert here to prevent pushing doublons
        assert!(!self.contains_tile(cell));

        // Get the texture cell in which the tile has to be
        let texture_cell = cell.get_texture_cell(&self.config);

        // Check whether the texture is a new texture
        if !self.textures.contains_key(&texture_cell) {
            let HEALPixCell(depth, idx) = texture_cell;
            let texture = if texture_cell.is_root() {
                let texture = Texture::new(&self.config, &texture_cell, idx as i32, time_request);
                // The texture is a root texture
                let texture_node = (&texture).into();

                // Push it to the buffer
                self.root_textures.push(texture_node);

                texture
            } else {
                // The texture is not among the essential ones
                // (i.e. is not a root texture)
                let texture = if self.is_heap_full() {
                    // Pop the oldest requested texture
                    let texture_node = self.heap.pop()
                        .unwrap();
                    // Ensure this is not a base texture
                    assert!(!texture_node.is_root());

                    // Remove it from the textures HashMap
                    if let Some(mut texture) = self.textures.remove(&texture_node.cell) {
                        // Clear the texture to assign it to texture_cell
                        texture.clear(&texture_cell, time_request, &mut self.worker);

                        texture
                    } else {
                        // The hashmap must contain the texture by construction
                        unreachable!()
                    }
                } else {
                    // The heap buffer is not full, let's create a new
                    // texture with an unique idx
                    // The idx is computed based on the current size of the buffer
                    let root_texture_off_idx = 12;
                    let idx = root_texture_off_idx + self.heap.len();

                    let texture = Texture::new(&self.config, &texture_cell, idx as i32, time_request);
                    texture
                };
                // Push it to the buffer
                let texture_node = (&texture).into();
                self.heap.push(texture_node);

                texture
            };
            // Insert it the texture
            self.textures.insert(texture_cell, texture);
        }

        // At this point, the texture that should contain the tile
        // is in the buffer
        // and the tile is not already in any textures of the buffer
        // We can safely push it
        // First get the texture
        if let Some(texture) = self.textures.get_mut(&texture_cell) {
            texture.append(
                cell, // The tile cell
                &self.config
            );

            if texture_cell.is_root() {
                // Root texture cells are pushed only one time at the beginning
                // We need to ensure they are directly available
                self.worker.block_on_task(
                    cell,
                    texture,
                    image,
                    &self.config
                );
                texture.register_written_tile(cell, &self.config);
                if texture.is_available() && texture.is_full() {
                    self.num_root_textures_available += 1;

                    assert!(self.num_root_textures_available <= 12);
                    console::log_1(&format!("aass {:?}", self.num_root_textures_available).into());

                    if self.num_root_textures_available == 12 {
                        self.ready = true;
                    }
                }
            } else {
                // Append new async task responsible for writing
                // the image into the texture 2d array for the GPU
                self.worker.append_task(
                    cell,
                    texture,
                    image,
                    &self.config
                );
            }
        } else {
            unreachable!()
        }
    }

    // Return true if at least one task has been processed
    pub fn write_textures(&mut self) -> bool {
        let tiles_written = self.worker.run();

        for cell in &tiles_written {
            let texture_cell = cell.get_texture_cell(&self.config);

            if let Some(texture) = self.textures.get_mut(&texture_cell) {
                texture.register_written_tile(cell, &self.config);
            } else {
                // Textures written have to be in the textures collection
                unreachable!();
            }
        }

        !tiles_written.is_empty()
    }

    fn is_heap_full(&self) -> bool {
        // Check that there are no more than num_textures
        // textures in the buffer
        let root_texture_off_idx = 12;
        let num_textures_heap = self.heap.len();
        let full_heap = num_textures_heap == (self.size - root_texture_off_idx);
        full_heap
    }

    // Check whether the buffer has a tile
    // For that purpose, we first need to verify that its
    // texture ancestor exists and then, it it contains the tile
    pub fn contains_tile(&self, cell: &HEALPixCell) -> bool {
        let texture_cell = cell.get_texture_cell(&self.config);
        if let Some(texture) = self.textures.get(&texture_cell) {
            // The texture is present in the buffer
            // We must check whether it contains the tile
            texture.contains(cell)
        } else {
            // The texture in which cell should be is not present
            false
        }
    }

    pub fn contains_texture(&self, texture_cell: &HEALPixCell) -> bool {
        self.textures.contains_key(texture_cell)
    }

    pub fn get(&self, texture_cell: &HEALPixCell) -> Option<&Texture> {
        self.textures.get(texture_cell)
    }

    // Update the priority of the texture containing the tile
    // It must be ensured that the tile is already contained in the buffer
    pub fn update_priority(&mut self, cell: &HEALPixCell, new_fov_cell: bool, time_request: f32) {
        assert!(self.contains_tile(cell));

        // Get the texture cell in which the tile has to be
        let texture_cell = cell.get_texture_cell(&self.config);

        if let Some(texture) = self.textures.get_mut(&texture_cell) {
            texture.set_time_request(time_request);

            // Reset the time the tile has been received if it is a new cell present in the fov
            if new_fov_cell {
                let start_time = utils::get_current_time();
                texture.update_start_time(start_time);
            }

            // Root textures are always in the buffer
            // But other textures can be removed thanks to the heap
            // data-structure. We have to update the time_request of the texture
            // and push it again in the heap to update its position.
            if !texture_cell.is_root() {
                self.heap = self.heap.iter()
                    // Remove the cell
                    .filter(|texture_node| texture_node.cell != texture_cell)
                    // Clone the iterator to get an iterator of TextureNode
                    .cloned()
                    // Collect to a new binary heap that do not have cell anymore
                    .collect::<BinaryHeap<_>>();
                let texture_node = texture.into();
                // Push the cell again that has the new time_request
                self.heap.push(texture_node);
            }
        } else {
            unreachable!()
        }
    }

    // This is called when the HiPS changes
    pub fn clear(&mut self, config: &TileConfig) {
        // Size i.e. the num of textures is the same
        // no matter the HiPS config
        self.root_textures.clear();
        self.heap.clear();
        self.worker.clear();

        self.textures.clear();
        self.texture_2d_array.bind()
            .clear();

        self.ready = false;
        self.num_root_textures_available = 0;

        self.config = *config;
    }

    pub fn is_ready(&self) -> bool {
        self.ready
    }

    // Get the textures in the buffer
    // The resulting array is uniq sorted
    fn get_sorted_textures(&self) -> Vec<&Texture> {
        let mut textures = self.textures.values().collect::<Vec<_>>();
        textures.sort();
        textures
    }
}

use crate::shader::HasUniforms;
use crate::shader::ShaderBound;
use crate::buffer::TextureUniforms;
impl HasUniforms for Textures {
    fn attach_uniforms<'a>(&self, shader: &'a ShaderBound<'a>) -> &'a ShaderBound<'a> {
        // Send the textures
        let textures = self.get_sorted_textures();
        let mut num_textures = 0;
        for texture in textures.iter() {
            if texture.is_available() {
                let texture_uniforms = TextureUniforms::new(
                    texture,
                    num_textures as i32
                );

                shader.attach_uniforms_from(&texture_uniforms);
                num_textures += 1;
                // TODO: send more tiles to the ray tracer
                // As it is now, it only send the 64 min uniq tiles
                // from the texture buffer i.e. all the 0 and 1 depth tiles
                // + 4 tiles of depth 2: 12 + 48 + 4 = 64
                if num_textures == 63 {
                    break;
                }
            }
        }
        num_textures += 1;
        shader.attach_uniform("num_textures", &(num_textures as i32));

        // Texture 2d array
        shader.attach_uniform("tex", &*self.texture_2d_array);

        shader
    }
}
