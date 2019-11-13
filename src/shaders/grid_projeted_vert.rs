pub static CONTENT: &'static str = r#"#version 300 es
    precision lowp float;

    layout (location = 0) in vec2 position;

    uniform float zoom_factor;
    uniform float aspect;

    void main() {
        vec2 p = position * vec2(1.f, aspect) * zoom_factor;
        gl_Position = vec4(p, 0.0, 1.0);
    }
"#;