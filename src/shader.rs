use web_sys::{WebGlProgram, WebGl2RenderingContext, WebGlShader, WebGlUniformLocation};

fn compile_shader(
    context: &WebGl2RenderingContext,
    shader_type: u32,
    source: &str,
) -> Result<WebGlShader, String> {
    let shader = context
        .create_shader(shader_type)
        .ok_or_else(|| String::from("Unable to create shader object"))?;
    context.shader_source(&shader, source);
    context.compile_shader(&shader);

    if context
        .get_shader_parameter(&shader, WebGl2RenderingContext::COMPILE_STATUS)
        .as_bool()
        .unwrap_or(false)
    {
        Ok(shader)
    } else {
        Err(context
            .get_shader_info_log(&shader)
            .unwrap_or_else(|| String::from("Unknown error creating shader")))
    }
}

fn link_program(
    context: &WebGl2RenderingContext,
    vert_shader: &WebGlShader,
    frag_shader: &WebGlShader,
) -> Result<WebGlProgram, String> {
    let program = context
        .create_program()
        .ok_or_else(|| String::from("Unable to create shader object"))?;

    context.attach_shader(&program, vert_shader);
    context.attach_shader(&program, frag_shader);
    context.link_program(&program);

    if context
        .get_program_parameter(&program, WebGl2RenderingContext::LINK_STATUS)
        .as_bool()
        .unwrap_or(false)
    {
        Ok(program)
    } else {
        Err(context
            .get_program_info_log(&program)
            .unwrap_or_else(|| String::from("Unknown error creating program object")))
    }
}

pub struct Shader {
    program: WebGlProgram,
}

impl Shader {
    pub fn new(gl: &WebGl2RenderingContext, vert_src: &str, frag_src: &str) -> Shader {
        let vert_shader = compile_shader(
            &gl,
            WebGl2RenderingContext::VERTEX_SHADER,
            &vert_src,
        ).unwrap();
        let frag_shader = compile_shader(
            &gl,
            WebGl2RenderingContext::FRAGMENT_SHADER,
            &frag_src,
        ).unwrap();

        let program = link_program(&gl, &vert_shader, &frag_shader).unwrap();
        
        Shader {
            program
        }
    }

    pub fn bind(&self, gl: &WebGl2RenderingContext) {
        gl.use_program(Some(&self.program));
    }

    pub fn unbind(&self, gl: &WebGl2RenderingContext) {
        gl.use_program(None);
    }

    pub fn get_uniform_location(&self, gl: &WebGl2RenderingContext, name: &str) -> Option<WebGlUniformLocation> {
        gl.get_uniform_location(&self.program, name)
    }
}