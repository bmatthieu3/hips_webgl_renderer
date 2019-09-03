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
use web_sys::{Request, RequestInit, RequestMode, Response, XmlHttpRequest};


pub fn load(gl: Rc<WebGl2RenderingContext>, src: &str, idx_texture: u32) {
    let xhr = Rc::new(XmlHttpRequest::new().unwrap());
    let xhr_clone = xhr.clone();
    let window = web_sys::window().unwrap();

    xhr.open_with_async("GET", src, true)
        .unwrap();

    let closure = Closure::wrap(Box::new(move |event: JsValue| {
        console::log_1(&format!("TEXT aaaaa").into());
        if xhr_clone.ready_state() == XmlHttpRequest::DONE && xhr_clone.status().unwrap() == 200 {
            let dd = xhr_clone.response().unwrap();
            let dd: String = dd.as_string().unwrap();
            //console::log_1(&dd.into());

            let encoded_uri: String = js_sys::encode_uri_component(&dd).into();
            let unescaped_uri: String = js_sys::unescape(&encoded_uri).into();
            let data = "data:image/jpeg;base64,".to_owned() + &window.btoa(&unescaped_uri).unwrap();
            //console::log_1(&format!("url {:?}", data).into());
            console::log_1(&format!("TEXT aaaaa2").into());

            create_texture(gl.clone(), &data, idx_texture);
        }
    }) as Box<dyn FnMut(_)>);

    xhr.set_onreadystatechange(Some(&closure.as_ref().unchecked_ref()));
    xhr.send().unwrap();

    closure.forget();
    /*
    let image = Rc::new(RefCell::new(HtmlImageElement::new().unwrap()));
    let image_clone = Rc::clone(&image);
    
    console::log_1(&format!("url {:?}", src).into());
    */
    //console::log_1(&format!("TEXT {:?}", future).into());

    /*let onload = Closure::wrap(Box::new(move || {
        let texture = gl.create_texture();

        gl.active_texture(idx_texture);
        gl.bind_texture(WebGl2RenderingContext::TEXTURE_2D, texture.as_ref());

        gl.tex_parameteri(WebGl2RenderingContext::TEXTURE_2D, WebGl2RenderingContext::TEXTURE_MIN_FILTER, WebGl2RenderingContext::NEAREST as i32);
        gl.tex_parameteri(WebGl2RenderingContext::TEXTURE_2D, WebGl2RenderingContext::TEXTURE_MAG_FILTER, WebGl2RenderingContext::NEAREST as i32);
        
        //gl.generate_mipmap(WebGl2RenderingContext::TEXTURE_2D);
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
    image.set_src(src);

    onload.forget();*/
}

fn create_texture(gl: Rc<WebGl2RenderingContext>, data_uri: &str, idx_texture: u32) {
    let image = Rc::new(RefCell::new(HtmlImageElement::new().unwrap()));
    let image_clone = Rc::clone(&image);
    console::log_1(&format!("data uri {:?}", data_uri).into());

    let onload = Closure::wrap(Box::new(move || {
        console::log_1(&format!("AAAAA").into());
        let texture = gl.create_texture();

        gl.active_texture(idx_texture);
        gl.bind_texture(WebGl2RenderingContext::TEXTURE_2D, texture.as_ref());

        gl.tex_parameteri(WebGl2RenderingContext::TEXTURE_2D, WebGl2RenderingContext::TEXTURE_MIN_FILTER, WebGl2RenderingContext::NEAREST as i32);
        gl.tex_parameteri(WebGl2RenderingContext::TEXTURE_2D, WebGl2RenderingContext::TEXTURE_MAG_FILTER, WebGl2RenderingContext::NEAREST as i32);
        
        //gl.generate_mipmap(WebGl2RenderingContext::TEXTURE_2D);
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
    image.set_src(data_uri);

    onload.forget();
}
