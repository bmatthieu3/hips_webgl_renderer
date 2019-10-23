pub static CONTENT: &'static str = r#"#version 300 es
    precision highp float;
    precision highp int;

    layout (location = 0) in vec2 screen_position;
    layout (location = 1) in vec3 position;

    out vec3 out_vert_pos;

    uniform mat4 model;
    uniform mat4 view;

    uniform float zoom_factor;

    void main() {
        gl_Position = vec4(screen_position.x * zoom_factor, screen_position.y * zoom_factor, 0.0, 1.0);
        out_vert_pos = vec3(model * vec4(position, 1.f));
    }
"#;