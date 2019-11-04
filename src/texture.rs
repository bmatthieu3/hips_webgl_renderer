use std::rc::Rc;
use std::cell::{Cell, RefCell};

use wasm_bindgen::prelude::Closure;
use wasm_bindgen::JsCast;

use web_sys::{HtmlImageElement, CanvasRenderingContext2d};
use web_sys::WebGl2RenderingContext;

use crate::renderable::hips_sphere::MAX_NUMBER_TEXTURE;
use std::collections::{BinaryHeap, HashSet};

const HEIGHT_TEXTURE: i32 = 512;
const WIDTH_TEXTURE: i32 = 512;

#[derive(Clone, Debug)]
pub struct HEALPixCellRequest {
    texture: Rc<RefCell<HtmlImageElement>>,
    idx: i32,
    depth: i32,
    time_request: f32,
    idx_in_buffer: i32,
    time_received: Option<f32>,
}

pub struct HEALPixCellGPUData {
    pub idx: i32,
    pub buf_idx: i32,
    pub time_received: f32,
}

impl From<HEALPixCellRequest> for HEALPixCellGPUData {
    fn from(cell: HEALPixCellRequest) -> Self {
        HEALPixCellGPUData {
            idx: cell.idx,
            buf_idx: cell.idx_in_buffer,
            time_received: cell.time_received.unwrap()
        }
    }
}

use crate::utils;
impl HEALPixCellRequest {
    pub fn new(depth: i32, idx: i32) -> HEALPixCellRequest {
        let time_request = utils::get_current_time();
        let time_received = None;

        let texture = Rc::new(RefCell::new(HtmlImageElement::new().unwrap()));
        let idx_in_buffer = 0;
        HEALPixCellRequest {
            texture,
            idx,
            depth,
            time_request,
            idx_in_buffer,
            time_received
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
    buffer_zero_depth: Rc<RefCell<Vec<HEALPixCellRequest>>>,

    loaded_cells: Rc<RefCell<HashSet<HEALPixCellRequest>>>,

    pub webgl_texture0: Option<web_sys::WebGlTexture>,
    pub webgl_texture1: Option<web_sys::WebGlTexture>,
}

use web_sys::console;

impl HEALPixTextureBuffer {
    pub fn new(gl: Rc<WebGl2RenderingContext>) -> HEALPixTextureBuffer {
        let buffer = Rc::new(RefCell::new(BinaryHeap::with_capacity(MAX_NUMBER_TEXTURE)));
        let buffer_zero_depth = Rc::new(RefCell::new(Vec::with_capacity(12)));
        let loaded_cells = Rc::new(RefCell::new(HashSet::with_capacity(MAX_NUMBER_TEXTURE)));

        // Initialize context rendering
        let webgl_texture0 = create_sampler_3d(gl.clone(), WebGl2RenderingContext::TEXTURE0, MAX_NUMBER_TEXTURE as u32);
        let webgl_texture1 = create_sampler_3d(gl.clone(), WebGl2RenderingContext::TEXTURE1, 12);

        let mut healpix_texture_buf = HEALPixTextureBuffer {
            buffer,
            buffer_zero_depth,
            loaded_cells,
            webgl_texture0,
            webgl_texture1,
        };

        for base_cell_idx in 0..12 {
            let healpix_cell = HEALPixCellRequest::new(0, base_cell_idx);
            healpix_texture_buf.load_zero_depth_cells(gl.clone(), healpix_cell);
        }

        healpix_texture_buf
    }

    fn load_zero_depth_cells(&mut self, gl: Rc<WebGl2RenderingContext>, healpix_cell: HEALPixCellRequest) {
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
            let buffer = self.buffer_zero_depth.clone();
            let texture_clone = healpix_cell.texture.clone();
            let healpix_cell = healpix_cell.clone();
            let webgl_texture = self.webgl_texture1.clone();

            Closure::wrap(Box::new(move || {
                console::log_1(&format!("sampler3D shader").into());

                let idx = buffer.borrow().len() as i32;
                gl.active_texture(WebGl2RenderingContext::TEXTURE1);
                gl.bind_texture(WebGl2RenderingContext::TEXTURE_3D, webgl_texture.as_ref());
                gl.tex_sub_image_3d_with_html_image_element(
                    WebGl2RenderingContext::TEXTURE_3D,
                    0,
                    0,
                    0,
                    idx,
                    WIDTH_TEXTURE,
                    HEIGHT_TEXTURE,
                    1,
                    WebGl2RenderingContext::RGB,
                    WebGl2RenderingContext::UNSIGNED_BYTE,
                    &texture_clone.borrow(),
                )
                .expect("Texture 3d");

                let mut healpix_indexed = healpix_cell.clone();
                healpix_indexed.idx_in_buffer = idx as i32;
                healpix_indexed.time_received = Some(utils::get_current_time());

                buffer.borrow_mut().push(healpix_indexed);
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

    pub fn load(&mut self, gl: Rc<WebGl2RenderingContext>, mut healpix_cell: HEALPixCellRequest, zoom: bool) {
        // discard base cells, they are stored in the buffer_zero_depth
        if healpix_cell.depth == 0 {
            return;
        }

        // Check if the healpix_cell is loaded or has been currently asked
        // for being loaded
        if self.loaded_cells.borrow().contains(&healpix_cell) {
            // Change its priority in the buffer (if it is present!).
            let mut tmp_buffer = self.buffer.borrow_mut().clone()
                .into_sorted_vec();

            let idx_hpx_cell = tmp_buffer.iter().position(|x| x.idx == healpix_cell.idx && x.depth == healpix_cell.depth);

            if let Some(idx_hpx_cell) = idx_hpx_cell {
                console::log_1(&format!("found healpix cell").into());
                // Found
                let mut healpix_cell_to_change_priority = tmp_buffer.remove(idx_hpx_cell);
                healpix_cell_to_change_priority.time_request = healpix_cell.time_request;
                if zoom {
                    healpix_cell_to_change_priority.time_received = Some(utils::get_current_time());
                }
                // Change time priority
                self.buffer.borrow_mut().clear();
                for hpx_cell in tmp_buffer {
                    self.buffer.borrow_mut().push(hpx_cell);
                }

                self.buffer.borrow_mut().push(healpix_cell_to_change_priority);
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
                let texture_clone = healpix_cell.texture.clone();
                let healpix_cell = healpix_cell.clone();
                let webgl_texture = self.webgl_texture0.clone();

                Closure::wrap(Box::new(move || {
                    console::log_1(&format!("load new tile").into());
                    let idx = if buffer.borrow().len() == MAX_NUMBER_TEXTURE {
                        // Remove the oldest tile from the buffer and from the
                        // hashset
                        let oldest_healpix_cell = buffer.borrow_mut().pop().unwrap();
                        loaded_cells.borrow_mut().remove(&oldest_healpix_cell);

                        oldest_healpix_cell.idx_in_buffer
                    } else {
                        buffer.borrow().len() as i32
                    };
                    gl.active_texture(WebGl2RenderingContext::TEXTURE0);
                    gl.bind_texture(WebGl2RenderingContext::TEXTURE_3D, webgl_texture.as_ref());
                    gl.tex_sub_image_3d_with_html_image_element(
                        WebGl2RenderingContext::TEXTURE_3D,
                        0,
                        0,
                        0,
                        idx,
                        WIDTH_TEXTURE,
                        HEIGHT_TEXTURE,
                        1,
                        WebGl2RenderingContext::RGB,
                        WebGl2RenderingContext::UNSIGNED_BYTE,
                        &texture_clone.borrow(),
                    )
                    .expect("Texture 3d");

                    let mut healpix_indexed = healpix_cell.clone();
                    healpix_indexed.idx_in_buffer = idx as i32;
                    healpix_indexed.time_received = Some(utils::get_current_time());

                    buffer.borrow_mut().push(healpix_indexed);

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

    pub fn get_tiles(&self, depth: i32) -> Vec<HEALPixCellGPUData> {
        self.buffer.borrow()
            .clone()
            .into_sorted_vec()
            .into_iter()
            .filter_map(|hpx| {
                if hpx.depth == depth {
                    Some(hpx.into())
                } else {
                    None
                }
            })
            .collect::<Vec<_>>()
    }
    
    pub fn get_zero_depth_tiles(&self) -> Vec<HEALPixCellGPUData> {
        self.buffer_zero_depth.borrow()
            .clone()
            .into_iter()
            .map(|hpx| {
                hpx.into()
            })
            .collect::<Vec<_>>()
    }
}


fn create_sampler_3d(gl: Rc<WebGl2RenderingContext>, texture_unit: u32, size_buffer: u32) -> Option<web_sys::WebGlTexture> {
    let window = web_sys::window().unwrap();
    let document = window.document().unwrap();
    let canvas = document.create_element("canvas").unwrap();
    let canvas: web_sys::HtmlCanvasElement = canvas.dyn_into::<web_sys::HtmlCanvasElement>().unwrap();
    canvas.set_width(WIDTH_TEXTURE as u32);
    canvas.set_height((HEIGHT_TEXTURE as u32) * size_buffer);
    
    let ctx = Rc::new(
        RefCell::new(
            canvas.get_context("2d")
                .unwrap()
                .unwrap()
                .dyn_into::<CanvasRenderingContext2d>().unwrap()
        )
    );
    let webgl_texture = gl.create_texture();
    gl.active_texture(texture_unit);
    gl.bind_texture(WebGl2RenderingContext::TEXTURE_3D, webgl_texture.as_ref());

    gl.tex_parameteri(WebGl2RenderingContext::TEXTURE_3D, WebGl2RenderingContext::TEXTURE_MIN_FILTER, WebGl2RenderingContext::NEAREST as i32);
    gl.tex_parameteri(WebGl2RenderingContext::TEXTURE_3D, WebGl2RenderingContext::TEXTURE_MAG_FILTER, WebGl2RenderingContext::NEAREST as i32);

    // Prevents s-coordinate wrapping (repeating)
    gl.tex_parameteri(WebGl2RenderingContext::TEXTURE_3D, WebGl2RenderingContext::TEXTURE_WRAP_S, WebGl2RenderingContext::CLAMP_TO_EDGE as i32);
    // Prevents t-coordinate wrapping (repeating)
    gl.tex_parameteri(WebGl2RenderingContext::TEXTURE_3D, WebGl2RenderingContext::TEXTURE_WRAP_T, WebGl2RenderingContext::CLAMP_TO_EDGE as i32);
    // Prevents r-coordinate wrapping (repeating)
    gl.tex_parameteri(WebGl2RenderingContext::TEXTURE_3D, WebGl2RenderingContext::TEXTURE_WRAP_R, WebGl2RenderingContext::CLAMP_TO_EDGE as i32);

    gl.tex_image_3d_with_html_canvas_element(
        WebGl2RenderingContext::TEXTURE_3D,
        0,
        WebGl2RenderingContext::RGB as i32,
        WIDTH_TEXTURE,
        HEIGHT_TEXTURE,
        size_buffer as i32,
        0,
        WebGl2RenderingContext::RGB,
        WebGl2RenderingContext::UNSIGNED_BYTE,
        &ctx.borrow().canvas().unwrap(),
    )
    .expect("Texture 3d");
    gl.generate_mipmap(WebGl2RenderingContext::TEXTURE_3D);
    webgl_texture
}