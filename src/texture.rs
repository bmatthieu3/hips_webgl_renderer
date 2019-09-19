use std::rc::Rc;
use std::cell::{Cell, RefCell};

use wasm_bindgen::prelude::Closure;
use wasm_bindgen::JsCast;

use web_sys::{HtmlImageElement, HtmlCanvasElement, CanvasRenderingContext2d};
use web_sys::WebGl2RenderingContext;

use web_sys::WebGlTexture;
use crate::renderable::hips_sphere::MAX_NUMBER_TEXTURE;
use std::collections::BinaryHeap;


use std::str;
use std::collections::VecDeque;

const HEIGHT_TEXTURE: f64 = 512_f64;
const WIDTH_TEXTURE: f64 = 512_f64;

struct HEALPixCellRequest {
    idx: i32,
    depth: i32,
    time_request: f64,
}

impl PartialEq for HEALPixCellRequest {
    fn eq(&self, other: &Self) -> bool {
        self.idx == other.idx && self.depth == other.depth
    }
}

impl Eq for HEALPixCellRequest {}
use std::cmp::Ordering;
/*
impl PartialOrd for HEALPixCellRequest {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        let ordering = self.time_request.partial_cmp(&other.time_request);

        if let Some(ordering) = ordering {
            Some(ordering.reverse())
        } else {
            None
        }
    }
}
*/
pub struct HEALPixTextureBuffer {
    buffer: Rc<RefCell<BinaryHeap<Rc<RefCell<HtmlImageElement>>>>>,
}

use web_sys::console;

impl HEALPixTextureBuffer {
    pub fn new() -> HEALPixTextureBuffer {
        let buffer = Rc::new(RefCell::new(VecDeque::with_capacity(MAX_NUMBER_TEXTURE)));
        HEALPixTextureBuffer {
            buffer
        }
    }

    pub fn load(&mut self, gl: Rc<WebGl2RenderingContext>, src: &str) {
        let image = Rc::new(RefCell::new(HtmlImageElement::new().unwrap()));
        let image_clone = image.clone();
        let buffer_clone = self.buffer.clone();
        let src_clone = src.to_string();

        let onload = Closure::wrap(Box::new(move || {
            console::log_1(&format!("{:?} loaded", src_clone).into());
            //if last_time.get() == time_async {
            // We should need a priority queue as a buffer with
            // a score being the time at which the tile is needed!
            if buffer_clone.borrow().len() == MAX_NUMBER_TEXTURE {
                buffer_clone.borrow_mut().pop_back();
            }
            buffer_clone.borrow_mut().push_back(image_clone.clone());
            
            // Create the canvas containing the data
            let image_data = {
                let window = web_sys::window().unwrap();
                let document = window.document().unwrap();
                
                let canvas = document.create_element("canvas").unwrap();
                let canvas: web_sys::HtmlCanvasElement = canvas.dyn_into::<web_sys::HtmlCanvasElement>().unwrap();
                canvas.set_width(512);
                canvas.set_height(512 * (buffer_clone.borrow().len() as u32));
                let ctx = Rc::new(
                    canvas.get_context("2d")
                        .unwrap()
                        .unwrap()
                        .dyn_into::<CanvasRenderingContext2d>().unwrap()
                );
                
                let mut dy = 0_f64;
                for texture in buffer_clone.borrow().iter() {
                    ctx.draw_image_with_html_image_element(texture.borrow().as_ref(), 0_f64, dy).unwrap();
                    dy += HEIGHT_TEXTURE;
                }
                
                let image_data = ctx.get_image_data(0_f64, 0_f64, WIDTH_TEXTURE, HEIGHT_TEXTURE * (buffer_clone.borrow().len() as f64)).unwrap();
                image_data
            };

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
            gl.tex_image_3d_with_image_data(
                WebGl2RenderingContext::TEXTURE_3D,
                0,
                WebGl2RenderingContext::RGBA as i32,
                512,
                512,
                buffer_clone.borrow().len() as i32,
                0,
                WebGl2RenderingContext::RGBA,
                WebGl2RenderingContext::UNSIGNED_BYTE,
                &image_data,
            )
            .expect("Texture image 2d");
        }) as Box<dyn Fn()>);

        let image = image.borrow_mut();

        image.set_onload(Some(onload.as_ref().unchecked_ref()));
        image.set_cross_origin(Some(""));
        image.set_src(src);

        onload.forget();
    }

    pub fn len(&self) -> usize {
        self.buffer.borrow().len()
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
pub fn load(gl: Rc<WebGl2RenderingContext>, src: &str,
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
