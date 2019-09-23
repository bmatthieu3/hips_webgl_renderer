use std::rc::Rc;
use std::cell::{Cell, RefCell};

use wasm_bindgen::prelude::Closure;
use wasm_bindgen::JsCast;

use web_sys::{HtmlImageElement, HtmlCanvasElement, CanvasRenderingContext2d};
use web_sys::WebGl2RenderingContext;

use web_sys::WebGlTexture;
use crate::renderable::hips_sphere::MAX_NUMBER_TEXTURE;
use std::collections::{BinaryHeap, HashSet};

use std::str;
use std::collections::VecDeque;

const HEIGHT_TEXTURE: f64 = 512_f64;
const WIDTH_TEXTURE: f64 = 512_f64;

#[derive(Clone, Debug)]
pub struct HEALPixCellRequest {
    texture: Rc<RefCell<HtmlImageElement>>,
    idx: i32,
    depth: i32,
    time_request: f64,
    idx_in_buffer: i32,
}

impl HEALPixCellRequest {
    pub fn new(depth: i32, idx: i32) -> HEALPixCellRequest {
        let window = web_sys::window().expect("should have a window in this context");
        let performance = window
            .performance()
            .expect("performance should be available");
        let time_request = performance.now();

        let texture = Rc::new(RefCell::new(HtmlImageElement::new().unwrap()));
        let idx_in_buffer = 0;
        HEALPixCellRequest {
            texture,
            idx,
            depth,
            time_request,
            idx_in_buffer,
        }
    }

    fn get_url(&self) -> String {
        let dir_idx = (self.idx / 10000) * 10000;

        let mut url = String::from("http://alasky.u-strasbg.fr/DSS/DSSColor/");
        url = url + "Norder" + &self.depth.to_string() + "/";
        url = url + "Dir" + &dir_idx.to_string() + "/";
        url = url + "Npix" + &self.idx.to_string() + ".jpg";

        url
    }
}

impl PartialEq for HEALPixCellRequest {
    fn eq(&self, other: &Self) -> bool {
        self.idx == other.idx && self.depth == other.depth
    }
}

use std::hash::{Hash, Hasher};
impl Hash for HEALPixCellRequest {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.idx.hash(state);
        self.depth.hash(state);
    }
}

impl Eq for HEALPixCellRequest {}
use std::cmp::Ordering;

impl PartialOrd for HEALPixCellRequest {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.time_request.partial_cmp(&other.time_request).unwrap().reverse())
    }
}

impl Ord for HEALPixCellRequest {
    fn cmp(&self, other: &Self) -> Ordering {
        self.partial_cmp(&other).unwrap().reverse()
    }
}

pub struct HEALPixTextureBuffer {
    buffer: Rc<RefCell<BinaryHeap<HEALPixCellRequest>>>,
    loaded_cells: Rc<RefCell<HashSet<HEALPixCellRequest>>>,
    ctx: Rc<RefCell<CanvasRenderingContext2d>>,

    num_load_tile: Rc<Cell<usize>>,
}

use web_sys::console;

impl HEALPixTextureBuffer {
    pub fn new() -> HEALPixTextureBuffer {
        let buffer = Rc::new(RefCell::new(BinaryHeap::with_capacity(MAX_NUMBER_TEXTURE)));
        let loaded_cells = Rc::new(RefCell::new(HashSet::with_capacity(MAX_NUMBER_TEXTURE)));

        // Initialize context rendering
        let window = web_sys::window().unwrap();
        let document = window.document().unwrap();
        let canvas = document.create_element("canvas").unwrap();
        let canvas: web_sys::HtmlCanvasElement = canvas.dyn_into::<web_sys::HtmlCanvasElement>().unwrap();
        canvas.set_width(512);
        canvas.set_height(512 * (MAX_NUMBER_TEXTURE as u32));
        let ctx = Rc::new(
            RefCell::new(
                canvas.get_context("2d")
                    .unwrap()
                    .unwrap()
                    .dyn_into::<CanvasRenderingContext2d>().unwrap()
            )
        );
        let num_load_tile = Rc::new(Cell::new(0));
        HEALPixTextureBuffer {
            buffer,
            loaded_cells,
            ctx,
            num_load_tile
        }
    }

    pub fn load(&mut self, gl: Rc<WebGl2RenderingContext>, healpix_cell: HEALPixCellRequest) {
        // Check if the healpix_cell is loaded or has been currently asked
        // for being loaded
        if self.loaded_cells.borrow().contains(&healpix_cell) {
            // Change its priority in the buffer (if it is present!).
            let mut tmp_buffer = self.buffer.borrow_mut().clone()
                .into_sorted_vec();

            let idx_hpx_cell = tmp_buffer.iter().position(|x| x.idx == healpix_cell.idx && x.depth == healpix_cell.depth);

            if let Some(idx_hpx_cell) = idx_hpx_cell {
                //console::log_1(&format!("found cell: {:?}", healpix_cell).into());
                // Found
                let mut healpix_cell_to_change_priority = tmp_buffer.remove(idx_hpx_cell);
                healpix_cell_to_change_priority.time_request = healpix_cell.time_request;
                // Change time priority

                //let tmp_buffer = tmp_buffer.into_iter().collect::<BinaryHeap<_>>();
                self.buffer.borrow_mut().clear();
                for hpx_cell in tmp_buffer {
                    self.buffer.borrow_mut().push(hpx_cell);
                }
                //self.buffer = Rc::new(RefCell::new(tmp_buffer));

                self.buffer.borrow_mut().push(healpix_cell_to_change_priority);

                //console::log_1(&format!("BUFFER STATE: {:?}", self.buffer.borrow().clone().into_sorted_vec()).into());
            }
        } else {
            // Add it to the loaded cells hashset
            self.loaded_cells.borrow_mut().insert(healpix_cell.clone());

            let onerror = {
                let loaded_cells = self.loaded_cells.clone(); 
                let healpix_cell = healpix_cell.clone();

                Closure::wrap(Box::new(move || {
                    loaded_cells.borrow_mut().remove(&healpix_cell);
                }) as Box<dyn Fn()>)
            };

            let url = healpix_cell.get_url();
            let onload = {
                let buffer = self.buffer.clone();
                let loaded_cells = self.loaded_cells.clone();
                let ctx = self.ctx.clone();
                let num_load_tile = self.num_load_tile.clone();
                let texture_clone = healpix_cell.texture.clone();
                let url = url.clone();
                let healpix_cell = healpix_cell.clone();

                Closure::wrap(Box::new(move || {
                    //console::log_1(&format!("{:?} loaded", url).into());
                    //console::log_1(&format!("buffer, {:?}", buffer.borrow().clone().into_sorted_vec()).into());
                    let idx_y = if buffer.borrow().len() == MAX_NUMBER_TEXTURE {
                        // Remove the oldest tile from the buffer and from the
                        // hashset
                        let oldest_healpix_cell = buffer.borrow_mut().pop().unwrap();
                        loaded_cells.borrow_mut().remove(&oldest_healpix_cell);

                        //console::log_1(&format!("remove oldies, {:?}", oldest_healpix_cell).into());

                        oldest_healpix_cell.idx_in_buffer
                    } else {
                        let idx = num_load_tile.get() % MAX_NUMBER_TEXTURE;
                        idx as i32
                    };
                    //let idx_y = num_load_tile.get() % MAX_NUMBER_TEXTURE;
                    num_load_tile.set(num_load_tile.get() + 1);
                    let mut healpix_indexed = healpix_cell.clone();
                    healpix_indexed.idx_in_buffer = idx_y as i32;

                    buffer.borrow_mut().push(healpix_indexed);
                    let current_buffer_length = buffer.borrow().len();

                    //console::log_1(&format!("buffer after, {:?}", buffer.borrow().clone().into_sorted_vec()).into());

                    // Create the canvas containing the data
                    let dy = idx_y * 512;
                    let dx = 0;

                    ctx.borrow().draw_image_with_html_image_element(&texture_clone.borrow(), dx as f64, dy as f64).unwrap();

                    let width_texture_3d = WIDTH_TEXTURE;
                    let height_texture_3d = HEIGHT_TEXTURE * (current_buffer_length as f64);
                    let image_data = ctx.borrow().get_image_data(0_f64, 0_f64, width_texture_3d, height_texture_3d).unwrap();

                    let webgl_texture = gl.create_texture();
                    gl.active_texture(WebGl2RenderingContext::TEXTURE0);
                    gl.bind_texture(WebGl2RenderingContext::TEXTURE_3D, webgl_texture.as_ref());

                    gl.tex_parameteri(WebGl2RenderingContext::TEXTURE_3D, WebGl2RenderingContext::TEXTURE_MIN_FILTER, WebGl2RenderingContext::NEAREST as i32);
                    gl.tex_parameteri(WebGl2RenderingContext::TEXTURE_3D, WebGl2RenderingContext::TEXTURE_MAG_FILTER, WebGl2RenderingContext::NEAREST as i32);

                    // Prevents s-coordinate wrapping (repeating)
                    gl.tex_parameteri(WebGl2RenderingContext::TEXTURE_3D, WebGl2RenderingContext::TEXTURE_WRAP_S, WebGl2RenderingContext::CLAMP_TO_EDGE as i32);
                    // Prevents t-coordinate wrapping (repeating)
                    gl.tex_parameteri(WebGl2RenderingContext::TEXTURE_3D, WebGl2RenderingContext::TEXTURE_WRAP_T, WebGl2RenderingContext::CLAMP_TO_EDGE as i32);
                    // Prevents r-coordinate wrapping (repeating)
                    gl.tex_parameteri(WebGl2RenderingContext::TEXTURE_3D, WebGl2RenderingContext::TEXTURE_WRAP_R, WebGl2RenderingContext::CLAMP_TO_EDGE as i32);
                    gl.tex_image_3d_with_image_data(
                        WebGl2RenderingContext::TEXTURE_3D,
                        0,
                        WebGl2RenderingContext::RGBA as i32,
                        512,
                        512,
                        current_buffer_length as i32,
                        0,
                        WebGl2RenderingContext::RGBA,
                        WebGl2RenderingContext::UNSIGNED_BYTE,
                        &image_data,
                    )
                    .expect("Texture 3d");
                }) as Box<dyn Fn()>)
            };

            //let texture_clone_clone = healpix_cell.texture.clone();
            let image = healpix_cell.texture.borrow_mut();

            image.set_onload(Some(onload.as_ref().unchecked_ref()));
            image.set_onerror(Some(onerror.as_ref().unchecked_ref()));

            image.set_cross_origin(Some(""));
            image.set_src(&url);

            onload.forget();
        }
    }

    pub fn len(&self) -> usize {
        self.buffer.borrow().len()
    }

    pub fn get_tiles(&self) -> (Vec<i32>, Vec<i32>) {
        let (depth_tiles, idx_tiles): (Vec<_>, Vec<_>) = self.buffer.borrow()
            .clone()
            .into_sorted_vec()
            .into_iter()
            .map(|hpx| {
                (hpx.depth, hpx.idx)
            })
            .unzip();

        (depth_tiles, idx_tiles)
    }

    pub fn get_idx_tiles(&self) -> Vec<i32> {
        let idx_tiles = self.buffer.borrow()
            .clone()
            .into_sorted_vec()
            .into_iter()
            .map(|hpx| {
                hpx.idx_in_buffer
            })
            .collect::<Vec<_>>();

        idx_tiles
    }

    /*pub fn get_sampler_3d_canvas(&self, gl: Rc<WebGl2RenderingContext>) -> web_sys::HtmlCanvasElement {
        let window = web_sys::window().unwrap();
        let document = window.document().unwrap();

        let canvas = document.create_element("canvas").unwrap();
        let canvas: web_sys::HtmlCanvasElement = canvas.dyn_into::<web_sys::HtmlCanvasElement>().unwrap();

        let ctx = Rc::new(
            canvas.get_context("2d")
                .unwrap()
                .unwrap()
                .dyn_into::<CanvasRenderingContext2d>().unwrap()
        );

        let mut dy = 0_f64;
        for texture in self.buffer.borrow().iter() {
            ctx.draw_image_with_html_image_element(texture.borrow().as_ref(), 0_f64, dy).unwrap();
            dy += HEIGHT_TEXTURE;
        }

        canvas
    }*/
}
/*
pub fn load_sampler_3d(gl: Rc<WebGl2RenderingContext>, sampler: &HEALPixTextureBuffer) {
    let texture = gl.create_texture();
    gl.active_texture(WebGl2RenderingContext::TEXTURE0);
    gl.bind_texture(WebGl2RenderingContext::TEXTURE_3D, texture.as_ref());

    gl.tex_parameteri(WebGl2RenderingContext::TEXTURE_3D, WebGl2RenderingContext::TEXTURE_MIN_FILTER, WebGl2RenderingContext::NEAREST as i32);
    gl.tex_parameteri(WebGl2RenderingContext::TEXTURE_3D, WebGl2RenderingContext::TEXTURE_MAG_FILTER, WebGl2RenderingContext::NEAREST as i32);

    // Prevents s-coordinate wrapping (repeating)
    gl.tex_parameteri(WebGl2RenderingContext::TEXTURE_3D, WebGl2RenderingContext::TEXTURE_WRAP_S, WebGl2RenderingContext::CLAMP_TO_EDGE as i32);
    // Prevents t-coordinate wrapping (repeating)
    gl.tex_parameteri(WebGl2RenderingContext::TEXTURE_3D, WebGl2RenderingContext::TEXTURE_WRAP_T, WebGl2RenderingContext::CLAMP_TO_EDGE as i32);
    // Prevents r-coordinate wrapping (repeating)
    gl.tex_parameteri(WebGl2RenderingContext::TEXTURE_3D, WebGl2RenderingContext::TEXTURE_WRAP_R, WebGl2RenderingContext::CLAMP_TO_EDGE as i32);
    let canvas = sampler.get_sampler_3d_canvas(gl.clone());
    gl.tex_image_3d_with_html_canvas_element(
        WebGl2RenderingContext::TEXTURE_3D,
        0,
        WebGl2RenderingContext::RGB as i32,
        512,
        512,
        MAX_NUMBER_TEXTURE as i32,
        0,
        WebGl2RenderingContext::RGB,
        WebGl2RenderingContext::UNSIGNED_BYTE,
        &canvas,
    )
    .expect("Texture image 2d");
}
*/
pub fn load(
    gl: Rc<WebGl2RenderingContext>,
    src: &str,
    slot_texture: i32,
    idx_texture: i32,
    depth_texture: i32,
    idx_textures: Rc<RefCell<Vec<i32>>>,
    depth_textures: Rc<RefCell<Vec<i32>>>,
    num_textures: Rc<Cell<i32>>,
    time_async: f64,
    last_time: Rc<Cell<f64>>) {
    let image = Rc::new(RefCell::new(HtmlImageElement::new().unwrap()));
    let image_clone = Rc::clone(&image);
    let texture = Rc::new(gl.create_texture());

    let onload = Closure::wrap(Box::new(move || {
        if last_time.get() == time_async {
            gl.active_texture(WebGl2RenderingContext::TEXTURE0 + (slot_texture as u32));
            gl.bind_texture(WebGl2RenderingContext::TEXTURE_2D, (*texture).as_ref());

            gl.tex_parameteri(WebGl2RenderingContext::TEXTURE_2D, WebGl2RenderingContext::TEXTURE_MIN_FILTER, WebGl2RenderingContext::NEAREST as i32);
            gl.tex_parameteri(WebGl2RenderingContext::TEXTURE_2D, WebGl2RenderingContext::TEXTURE_MAG_FILTER, WebGl2RenderingContext::NEAREST as i32);

            // Prevents s-coordinate wrapping (repeating)
            gl.tex_parameteri(WebGl2RenderingContext::TEXTURE_2D, WebGl2RenderingContext::TEXTURE_WRAP_S, WebGl2RenderingContext::CLAMP_TO_EDGE as i32);
            // Prevents t-coordinate wrapping (repeating)
            gl.tex_parameteri(WebGl2RenderingContext::TEXTURE_2D, WebGl2RenderingContext::TEXTURE_WRAP_T, WebGl2RenderingContext::CLAMP_TO_EDGE as i32);
            gl.tex_image_2d_with_u32_and_u32_and_html_image_element(
                WebGl2RenderingContext::TEXTURE_2D,
                0,
                WebGl2RenderingContext::RGB as i32,
                WebGl2RenderingContext::RGB,
                WebGl2RenderingContext::UNSIGNED_BYTE,
                &image_clone.borrow(),
            )
            .expect("Texture image 2d");
            gl.generate_mipmap(WebGl2RenderingContext::TEXTURE_2D);

            idx_textures.borrow_mut()[slot_texture as usize] = idx_texture;
            depth_textures.borrow_mut()[slot_texture as usize] = depth_texture;

            if num_textures.get() < (MAX_NUMBER_TEXTURE as i32) {
                num_textures.set(num_textures.get() + 1);
            }
        }
    }) as Box<dyn Fn()>);

    let image = image.borrow_mut();

    image.set_onload(Some(onload.as_ref().unchecked_ref()));
    image.set_cross_origin(Some(""));
    image.set_src(src);

    onload.forget();
}
