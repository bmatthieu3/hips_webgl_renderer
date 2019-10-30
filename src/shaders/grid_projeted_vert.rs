pub static CONTENT: &'static str = r#"#version 300 es
    precision highp float;

    layout (location = 0) in vec2 position;

    uniform float zoom_factor;
    uniform float resize_factor_x;
    uniform float resize_factor_y;

    uniform float aspect;

    void main() {
        vec2 p = position * vec2(1.f, aspect);
        gl_Position = vec4(p, 0.0, 1.0);
    }
"#;