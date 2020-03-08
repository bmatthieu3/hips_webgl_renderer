use std::rc::Rc;
use std::cell::RefCell;
use std::convert::TryInto;

use web_sys::WebGl2RenderingContext;
use web_sys::HtmlImageElement;
use wasm_bindgen::prelude::Closure;
use wasm_bindgen::JsCast;
use web_sys::console;

use crate::WebGl2Context;
use crate::shader::Shader;

use web_sys::WebGlTexture;
pub struct Texture2DArray {
    gl: WebGl2Context,

    texture: Rc<RefCell<Option<WebGlTexture>>>, // The texture data
    idx_texture_unit: u32, // Internal index of the texture array
    format: u32, // The storage format (e.g. RGB, RGBA)

    tex_params: &'static [(u32, u32)], // texture array parameters

    width: i32, // Width of a texture element
    height: i32, // Height of a texture element
    num_textures: i32 // number of texture elements
}

use crate::core::NUM_TEXTURE_UNIT;

impl Texture2DArray {
    // Create a Texture2DArray from an image
    //
    // The number of texture is defined from the height of the image.
    pub fn create(gl: &WebGl2Context,
        // The path to the image
        src: &'static str,
        // The weight of the individual textures
        width: i32,
        // Their height
        height: i32,
        // How many textures it contains
        num_textures: i32,
        tex_params: &'static [(u32, u32)],
        // Texture format
        format: u32,
    ) -> Texture2DArray {
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
                gl.bind_texture(WebGl2RenderingContext::TEXTURE_2D_ARRAY, texture.borrow().as_ref());

                for (pname, param) in tex_params.iter() {
                    gl.tex_parameteri(WebGl2RenderingContext::TEXTURE_2D_ARRAY, *pname, *param as i32);
                }

                gl.tex_image_3d_with_html_image_element(
                    WebGl2RenderingContext::TEXTURE_2D_ARRAY, // target
                    0, // level
                    format as i32, // internalformat
                    width, // width
                    height, // height
                    num_textures, // depth
                    0, // border
                    format, // format
                    WebGl2RenderingContext::UNSIGNED_BYTE, // type
                    &image.borrow() // source
                ).expect("Texture Array 2D");
                gl.generate_mipmap(WebGl2RenderingContext::TEXTURE_2D_ARRAY);
            }) as Box<dyn Fn()>)
        };

        image.borrow_mut().set_onload(Some(onload.as_ref().unchecked_ref()));
        image.borrow_mut().set_onerror(Some(onerror.as_ref().unchecked_ref()));

        image.borrow_mut().set_cross_origin(Some(""));
        image.borrow_mut().set_src(src);

        onload.forget();
        onerror.forget();
        
        let gl = gl.clone();
        Texture2DArray {
            gl,

            texture,
            idx_texture_unit,
            format,

            tex_params,

            width,
            height,
            num_textures
        }
    }

    pub fn create_empty(gl: &WebGl2Context,
        // The weight of the individual textures
        width: i32,
        // Their height
        height: i32,
        // How many textures it contains
        num_textures: i32,
        tex_params: &'static [(u32, u32)],
        // Texture format
        format: u32,
    ) -> Texture2DArray {
        let texture = Rc::new(RefCell::new(gl.create_texture()));
        let idx_texture_unit = unsafe { NUM_TEXTURE_UNIT };
        unsafe {
            NUM_TEXTURE_UNIT += 1;
        }
        gl.active_texture(idx_texture_unit);
        gl.bind_texture(WebGl2RenderingContext::TEXTURE_2D_ARRAY, texture.borrow().as_ref());

        for (pname, param) in tex_params.iter() {
            gl.tex_parameteri(WebGl2RenderingContext::TEXTURE_2D_ARRAY, *pname, *param as i32);
        }

        gl.tex_image_3d_with_opt_u8_array(
            WebGl2RenderingContext::TEXTURE_2D_ARRAY, // target
            0, // level
            format as i32, // internalformat
            width, // width
            height, // height
            num_textures, // depth
            0, // border
            format, // format
            WebGl2RenderingContext::UNSIGNED_BYTE, // type
            None, // source
        ).expect("Texture 2D Array");
        gl.generate_mipmap(WebGl2RenderingContext::TEXTURE_2D_ARRAY);

        let gl = gl.clone();

        Texture2DArray {
            gl,

            texture,
            idx_texture_unit,
            format,

            tex_params,

            width,
            height,
            num_textures
        }
    }

    pub fn bind(&self) -> Texture2DArrayBound {
        let idx_texture_unit = self.idx_texture_unit;
        let webgl_texture = self.texture.borrow();

        self.gl.active_texture(idx_texture_unit);
        self.gl.bind_texture(WebGl2RenderingContext::TEXTURE_2D_ARRAY, webgl_texture.as_ref());

        let gl = self.gl.clone();
        Texture2DArrayBound {
            gl,
            idx_texture_unit,

            format: self.format,
            width: self.width,
            height: self.height,
            num_textures: self.num_textures,
        }
    }
}

pub struct Texture2DArrayBound {
    gl: WebGl2Context,

    idx_texture_unit: u32,

    format: u32,

    width: i32,
    height: i32,
    num_textures: i32
}
impl Texture2DArrayBound {
    pub fn send_to_shader(&self, shader: &Shader, name: &'static str) {        
        let idx_sampler: i32 = (self.idx_texture_unit - WebGl2RenderingContext::TEXTURE0).try_into().unwrap();

        let location_tex = shader.get_uniform_location(name);
        self.gl.uniform1i(location_tex, idx_sampler);
    }

    pub fn clear(&self) {
        let data = vec![0 as u8; 3 * (self.height as usize) * (self.width as usize) * (self.num_textures as usize)];
        self.gl.tex_sub_image_3d_with_opt_u8_array(
            WebGl2RenderingContext::TEXTURE_2D_ARRAY, // target: u32,
            0, // level: i32,
            0, // xoffset: i32,
            0, // yoffset: i32,
            0, // zoffset: i32,

            self.width, // width: i32,
            self.height, // height: i32,
            self.num_textures, // depth: i32,

            self.format, // format: u32,
            WebGl2RenderingContext::UNSIGNED_BYTE, // type: u32
            Some(&data),
        )
        .expect("Sub texture 2d");
    }

    pub fn tex_sub_image_3d_with_html_image_element(&self,
        xoffset: i32, yoffset: i32,
        idx_texture: i32, // Idx of the texture to replace
        width: i32, // Width of the image
        height: i32, // Height of the image
        image: &HtmlImageElement // image data
    ) {
        self.gl.tex_sub_image_3d_with_html_image_element(
            WebGl2RenderingContext::TEXTURE_2D_ARRAY, // target: u32,
            0, // level: i32,
            xoffset, // xoffset: i32,
            yoffset, // yoffset: i32,
            idx_texture, // zoffset: i32,
            width, // width: i32,
            height, // height: i32,
            1, // depth: i32,
            self.format, // format: u32,
            WebGl2RenderingContext::UNSIGNED_BYTE, // type: u32
            image,
        )
        .expect("Sub texture 2d");
    }
}