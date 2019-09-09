use std::rc::Rc;
use std::cell::RefCell;

use wasm_bindgen::prelude::*;
use wasm_bindgen::prelude::Closure;
use wasm_bindgen::JsCast;
use futures::future::Future;
use wasm_bindgen_futures::JsFuture;

use js_sys::{Promise, Function};

use web_sys::HtmlImageElement;
use web_sys::WebGl2RenderingContext;

use web_sys::console;
use web_sys::{Request, RequestInit, RequestMode, Response, XmlHttpRequest, WebGlTexture};

use std::str;
use base64::{encode, decode};

pub fn load(gl: Rc<WebGl2RenderingContext>, src: &str, idx_texture: u32) -> Rc<Option<WebGlTexture>> {
    let image = Rc::new(RefCell::new(HtmlImageElement::new().unwrap()));
    let image_clone = Rc::clone(&image);
    let texture = Rc::new(gl.create_texture());
    let texture_clone = texture.clone();

    let onload = Closure::wrap(Box::new(move || {
        gl.active_texture(idx_texture);
        gl.bind_texture(WebGl2RenderingContext::TEXTURE_2D, (*texture_clone).as_ref());

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
    }) as Box<dyn Fn()>);

    let image = image.borrow_mut();

    image.set_onload(Some(onload.as_ref().unchecked_ref()));
    image.set_cross_origin(Some(""));
    image.set_src(src);

    onload.forget();
    
    texture
}
