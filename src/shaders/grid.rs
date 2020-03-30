use crate::shader::Shaderize;

#[derive(Shaderize)]
#[VertexShader = r#"#version 300 es
    precision lowp float;

    layout (location = 0) in vec2 position;

    uniform vec2 ndc_to_clip;
    uniform float clip_zoom_factor;

    uniform float aspect;

    void main() {
        vec2 p = position / (ndc_to_clip * clip_zoom_factor);
        gl_Position = vec4(p, 0.0, 1.0);
    }
"#]
#[FragmentShader = r#"#version 300 es
    precision lowp float;

    out vec4 color;

    uniform vec4 grid_color;

    uniform float current_time;
    uniform int last_zoom_action;

    void main() {
        color = grid_color;
    }
"#]
pub struct Grid;