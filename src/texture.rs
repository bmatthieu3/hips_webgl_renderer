use std::rc::Rc;
use std::cell::{Cell, RefCell};

use wasm_bindgen::prelude::Closure;
use wasm_bindgen::JsCast;

use web_sys::HtmlImageElement;
use web_sys::WebGl2RenderingContext;

use web_sys::WebGlTexture;
use crate::renderable::hips_sphere::MAX_NUMBER_TEXTURE;

use std::str;

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

            gl.generate_mipmap(WebGl2RenderingContext::TEXTURE_2D);
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
