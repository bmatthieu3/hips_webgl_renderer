pub static CONTENT: &'static str = r#"#version 300 es
    precision highp float;
    precision highp int;

    layout (location = 0) in vec2 screen_position;
    layout (location = 1) in vec3 position;

    out vec3 out_vert_pos;

    uniform mat4 model;
    uniform vec2 ndc_to_clip;
    uniform float clip_zoom_factor;

    void main() {
        gl_Position = vec4((screen_position.xy / (ndc_to_clip * clip_zoom_factor)), 0.0, 1.0);
        out_vert_pos = vec3(model * vec4(position, 1.f));
    }
"#;