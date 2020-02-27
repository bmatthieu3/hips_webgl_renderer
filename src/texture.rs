use std::rc::Rc;
use std::cell::RefCell;
use std::convert::TryInto;

use web_sys::WebGl2RenderingContext;
use web_sys::HtmlImageElement;
use wasm_bindgen::prelude::Closure;
use wasm_bindgen::JsCast;
use web_sys::ImageData;
use web_sys::console;

use crate::WebGl2Context;
use crate::shader::Shader;

#[derive(Clone)]
enum TextureType {
    // The image containing the width and height
    // of the texture
    ImageElement(Rc<RefCell<HtmlImageElement>>),
    // Width and Height
    Bytes(u32, u32)
}

impl TextureType {
    fn get_width(&self) -> u32 {
        match self {
            TextureType::ImageElement(image) => image.borrow().width() as u32,
            TextureType::Bytes(width, _) => *width
        }
    }

    fn get_height(&self) -> u32 {
        match self {
            TextureType::ImageElement(image) => image.borrow().height() as u32,
            TextureType::Bytes(_, height) => *height
        }
    }
}

use web_sys::WebGlTexture;
pub struct Texture2D {
    texture: Rc<RefCell<Option<WebGlTexture>>>,
    idx_texture_unit: u32,

    gl: WebGl2Context,
    tex_params: &'static [(u32, u32)],

    data: TextureType,
}

static mut NUM_TEXTURE_UNIT: u32 = WebGl2RenderingContext::TEXTURE0;

impl Texture2D {
    pub fn create(gl: &WebGl2Context, src: &'static str, tex_params: &'static [(u32, u32)]) -> Texture2D {
        let image = Rc::new(RefCell::new(HtmlImageElement::new().unwrap()));

        let texture = Rc::new(RefCell::new(gl.create_texture()));
        let idx_texture_unit = unsafe { NUM_TEXTURE_UNIT };
        unsafe {
            NUM_TEXTURE_UNIT += 1;
        }
        let onerror = {
            Closure::wrap(Box::new(move || {
                console::log_1(&format!("Cannot load texture located at: {:?}", src).into());
            }) as Box<dyn Fn()>)
        };

        let onload = {
            let image = image.clone();
            let gl = gl.clone();
            let texture = texture.clone();

            Closure::wrap(Box::new(move || {
                gl.active_texture(idx_texture_unit);
                gl.bind_texture(WebGl2RenderingContext::TEXTURE_2D, texture.borrow().as_ref());

                for (pname, param) in tex_params.iter() {
                    gl.tex_parameteri(WebGl2RenderingContext::TEXTURE_2D, *pname, *param as i32);
                }

                gl.tex_image_2d_with_u32_and_u32_and_html_image_element(
                    WebGl2RenderingContext::TEXTURE_2D,
                    0,
                    WebGl2RenderingContext::RGBA as i32,
                    WebGl2RenderingContext::RGBA,
                    WebGl2RenderingContext::UNSIGNED_BYTE,
                    &image.borrow()
                ).expect("Texture 2D");
                gl.generate_mipmap(WebGl2RenderingContext::TEXTURE_2D);
            }) as Box<dyn Fn()>)
        };

        image.borrow_mut().set_onload(Some(onload.as_ref().unchecked_ref()));
        image.borrow_mut().set_onerror(Some(onerror.as_ref().unchecked_ref()));

        image.borrow_mut().set_cross_origin(Some(""));
        image.borrow_mut().set_src(src);

        onload.forget();
        onerror.forget();
        
        let data = TextureType::ImageElement(image);
        let gl = gl.clone();
        Texture2D {
            texture,
            idx_texture_unit,

            gl,
            tex_params,

            data
        }
    }

    pub fn create_empty(gl: &WebGl2Context, width: i32, height: i32, tex_params: &'static [(u32, u32)]) -> Texture2D {
        let texture = Rc::new(RefCell::new(gl.create_texture()));
        let idx_texture_unit = unsafe { NUM_TEXTURE_UNIT };
        unsafe {
            NUM_TEXTURE_UNIT += 1;
        }
        gl.active_texture(idx_texture_unit);
        gl.bind_texture(WebGl2RenderingContext::TEXTURE_2D, texture.borrow().as_ref());

        for (pname, param) in tex_params.iter() {
            gl.tex_parameteri(WebGl2RenderingContext::TEXTURE_2D, *pname, *param as i32);
        }

        gl.tex_image_2d_with_i32_and_i32_and_i32_and_format_and_type_and_opt_u8_array(
            WebGl2RenderingContext::TEXTURE_2D,
            0,
            WebGl2RenderingContext::RGB as i32,
            width,
            height,
            0,
            WebGl2RenderingContext::RGB,
            WebGl2RenderingContext::UNSIGNED_BYTE,
            None
        ).expect("Texture 2D");
        gl.generate_mipmap(WebGl2RenderingContext::TEXTURE_2D);

        let gl = gl.clone();
        let data = TextureType::Bytes(width as u32, height as u32);

        Texture2D {
            texture,
            idx_texture_unit,

            gl,
            tex_params,

            data
        }
    }
    pub fn create_from_image_data(gl: &WebGl2Context, image: &ImageData, tex_params: &'static [(u32, u32)]) -> Texture2D {
        let texture = Rc::new(RefCell::new(gl.create_texture()));
        let idx_texture_unit = unsafe { NUM_TEXTURE_UNIT };
        unsafe {
            NUM_TEXTURE_UNIT += 1;
        }
        gl.active_texture(idx_texture_unit);
        gl.bind_texture(WebGl2RenderingContext::TEXTURE_2D, texture.borrow().as_ref());

        for (pname, param) in tex_params.iter() {
            gl.tex_parameteri(WebGl2RenderingContext::TEXTURE_2D, *pname, *param as i32);
        }

        let width = image.width() as i32;
        let height = image.height() as i32;
        gl.tex_image_2d_with_i32_and_i32_and_i32_and_format_and_type_and_image_data(
            WebGl2RenderingContext::TEXTURE_2D,
            0,
            WebGl2RenderingContext::RGB as i32,
            width,
            height,
            0,
            WebGl2RenderingContext::RGB,
            WebGl2RenderingContext::UNSIGNED_BYTE,
            image
        ).expect("Failing writing ImageData to texture");
        //gl.generate_mipmap(WebGl2RenderingContext::TEXTURE_2D);

        let gl = gl.clone();
        let data = TextureType::Bytes(width as u32, height as u32);

        Texture2D {
            texture,
            idx_texture_unit,

            gl,
            tex_params,

            data
        }
    }

    pub fn attach_to_framebuffer(&self) {
        self.gl.framebuffer_texture_2d(
            WebGl2RenderingContext::FRAMEBUFFER,
            WebGl2RenderingContext::COLOR_ATTACHMENT0,
            WebGl2RenderingContext::TEXTURE_2D,
            self.texture.borrow().as_ref(),
            0
        );
    }

    pub fn bind(&self) -> Texture2DBound {
        let texture_unit = self.idx_texture_unit;
        let webgl_texture = self.texture.borrow();

        self.gl.active_texture(texture_unit);
        self.gl.bind_texture(WebGl2RenderingContext::TEXTURE_2D, webgl_texture.as_ref());

        let gl = self.gl.clone();
        let data = self.data.clone();
        let idx_texture_unit = self.idx_texture_unit;
        Texture2DBound {
            gl,
            data,
            idx_texture_unit,
        }
    }
}

pub struct Texture2DBound {
    gl: WebGl2Context,

    idx_texture_unit: u32,
    data: TextureType,
}
impl Texture2DBound {
    pub fn send_to_shader(&self, shader: &Shader, name: &'static str) {        
        let idx_sampler: i32 = (self.idx_texture_unit - WebGl2RenderingContext::TEXTURE0).try_into().unwrap();

        let location_tex = shader.get_uniform_location(name);
        self.gl.uniform1i(location_tex, idx_sampler);
    }

    pub fn clear(&self) {
        let (width, height) = (self.data.get_width(), self.data.get_height());

        let data = vec![0 as u8; 3 * (height as usize) * (width as usize)];
        self.gl.tex_sub_image_2d_with_i32_and_i32_and_u32_and_type_and_opt_u8_array(
            WebGl2RenderingContext::TEXTURE_2D,
            0,
            0,
            0,

            width as i32,
            height as i32,

            WebGl2RenderingContext::RGB,
            WebGl2RenderingContext::UNSIGNED_BYTE,
            Some(&data),
        )
        .expect("Sub texture 2d");
    }

    pub fn tex_sub_image_2d_with_u32_and_u32_and_html_image_element(&self, dx: i32, dy: i32, image: &HtmlImageElement) {
        self.gl.tex_sub_image_2d_with_u32_and_u32_and_html_image_element(
            WebGl2RenderingContext::TEXTURE_2D,
            0,
            dx,
            dy,
            WebGl2RenderingContext::RGB,
            WebGl2RenderingContext::UNSIGNED_BYTE,
            &image,
        )
        .expect("Sub texture 2d");
    }
}