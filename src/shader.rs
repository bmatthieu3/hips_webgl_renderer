use web_sys::{WebGlProgram, WebGl2RenderingContext, WebGlShader, WebGlUniformLocation};

fn compile_shader(
    gl: &WebGl2Context,
    shader_type: u32,
    source: &str,
) -> Result<WebGlShader, String> {
    let shader = gl.create_shader(shader_type)
        .ok_or_else(|| String::from("Unable to create shader object"))?;
    gl.shader_source(&shader, source);
    gl.compile_shader(&shader);

    if gl
        .get_shader_parameter(&shader, WebGl2RenderingContext::COMPILE_STATUS)
        .as_bool()
        .unwrap_or(false)
    {
        Ok(shader)
    } else {
        Err(gl
            .get_shader_info_log(&shader)
            .unwrap_or_else(|| String::from("Unknown error creating shader")))
    }
}

fn link_program(
    gl: &WebGl2Context,
    vert_shader: &WebGlShader,
    frag_shader: &WebGlShader,
) -> Result<WebGlProgram, String> {
    let program = gl
        .create_program()
        .ok_or_else(|| String::from("Unable to create shader object"))?;

    gl.attach_shader(&program, vert_shader);
    gl.attach_shader(&program, frag_shader);
    gl.link_program(&program);

    if gl
        .get_program_parameter(&program, WebGl2RenderingContext::LINK_STATUS)
        .as_bool()
        .unwrap_or(false)
    {
        Ok(program)
    } else {
        Err(gl
            .get_program_info_log(&program)
            .unwrap_or_else(|| String::from("Unknown error creating program object")))
    }
}

use std::collections::HashMap;
pub struct Shader {
    program: WebGlProgram,
    uniform_locations: HashMap<String, Option<WebGlUniformLocation>>,
}

use crate::WebGl2Context;
use web_sys::console;
impl Shader {
    pub fn new(gl: &WebGl2Context, vert_src: &str, frag_src: &str, name_uniforms: Vec<String>) -> Shader {
        let vert_shader = compile_shader(
            gl,
            WebGl2RenderingContext::VERTEX_SHADER,
            &vert_src,
        ).unwrap();
        let frag_shader = compile_shader(
            gl,
            WebGl2RenderingContext::FRAGMENT_SHADER,
            &frag_src,
        ).unwrap();

        let program = link_program(gl, &vert_shader, &frag_shader).unwrap();

        let uniform_locations = name_uniforms.into_iter()
            .map(|name| {
                let location_uniform = gl.get_uniform_location(&program, &name);
                //console::log_1(&format!("{:?}", *name).into());
                (name, location_uniform)
            })
            .collect::<HashMap<_, _>>();

        console::log_1(&format!("uniforms loaded").into());

        Shader {
            program,
            uniform_locations
        }
    }

    pub fn bind(&self, gl: &WebGl2Context) {
        gl.use_program(Some(&self.program));
    }

    pub fn unbind(&self, gl: &WebGl2Context) {
        gl.use_program(None);
    }

    pub fn get_uniform_location(&self, name: &str) -> Option<&WebGlUniformLocation> {
        self.uniform_locations.get(name).unwrap().as_ref()
    }
}