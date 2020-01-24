pub static CONTENT: &'static str = r#"#version 300 es
    precision lowp float;

    layout (location = 0) in vec2 position;

    uniform vec2 ndc_to_clip;
    uniform float clip_zoom_factor;

    uniform float aspect;

    void main() {
        vec2 p = position;
        gl_Position = vec4(p, 0.0, 1.0);
    }
"#;