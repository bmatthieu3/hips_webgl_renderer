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

    if gl.get_shader_parameter(&shader, WebGl2RenderingContext::COMPILE_STATUS)
        .as_bool()
        .unwrap_or(false) {
        Ok(shader)
    } else {
        Err(
            gl.get_shader_info_log(&shader)
                .unwrap_or_else(|| String::from("Unknown error creating shader"))
        )
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

    if gl.get_program_parameter(&program, WebGl2RenderingContext::LINK_STATUS)
        .as_bool()
        .unwrap_or(false) {
        Ok(program)
    } else {
        Err(
            gl.get_program_info_log(&program)
                .unwrap_or_else(|| String::from("Unknown error creating program object"))
        )
    }
}

use std::collections::HashMap;
pub struct Shader {
    program: WebGlProgram,
    uniform_locations: HashMap<&'static str, Option<WebGlUniformLocation>>,
}

use crate::WebGl2Context;
use web_sys::console;
impl Shader {
    pub fn new(gl: &WebGl2Context, vert_src: &str, frag_src: &str, name_uniforms: &[&'static str]) -> Result<Shader, String> {
        console::log_1(&format!("vert_src: {:?}", vert_src).into());
        let vert_shader = compile_shader(
            gl,
            WebGl2RenderingContext::VERTEX_SHADER,
            &vert_src,
        )?;
        let frag_shader = compile_shader(
            gl,
            WebGl2RenderingContext::FRAGMENT_SHADER,
            &frag_src,
        )?;

        let program = link_program(gl, &vert_shader, &frag_shader)?;

        let uniform_locations = name_uniforms.into_iter()
            .map(|&name| {
                let location_uniform = gl.get_uniform_location(&program, &name);
                //console::log_1(&format!("{:?}", *name).into());
                (name, location_uniform)
            })
            .collect::<HashMap<_, _>>();

        console::log_1(&format!("uniforms loaded").into());

        Ok(Shader {
            program,
            uniform_locations
        })
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

pub trait Shaderize {
    fn name() -> &'static str;

    fn vertex_shader_content() -> String;
    fn fragment_shader_content() -> String;
}

pub struct ShaderManager(HashMap<&'static str, Shader>);

impl ShaderManager {
    pub fn new() -> ShaderManager {
        ShaderManager(HashMap::new())
    }

    // Insert a shader inside the manager
    // Returns an error whether the shader compilation or linking failed
    // or if the manager already contains a shader
    pub fn insert<S: Shaderize>(&mut self, gl: &WebGl2Context, uniforms: &[&'static str]) {
        // Create the shader
        let shader = Shader::new(&gl,
            &S::vertex_shader_content(),
            &S::fragment_shader_content(),
            uniforms
        ).unwrap();

        // Insert it in the map
        let key = S::name();
        if self.0.contains_key(key) {
            // Already contained in the shader manager
            panic!(String::from(key) + " is already contained in the shaders manager");
        } else {
            self.0.insert(key, shader);
        }
    }

    pub fn get<S: Shaderize>(&self) -> Option<&Shader> {
        let key = S::name();
        self.0.get(key)
    }
}