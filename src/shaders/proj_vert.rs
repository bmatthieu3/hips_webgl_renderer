pub static CONTENT: &'static str = r#"#version 300 es
    precision mediump float;

    layout (location = 0) in vec2 screen_position;
    layout (location = 1) in vec3 position;

    out vec3 out_vert_pos;

    uniform mat4 model;
    uniform mat4 view;

    uniform float zoom_factor;
    uniform float resize_factor_x;
    uniform float resize_factor_y;

    //uniform vec2 window_size_default;
    //uniform vec2 current_window_size;

    void main() {
        //vec2 screen_ratio = current_window_size / window_size_default;
        //vec2 offset = (window_size_default - current_window_size)/current_window_size;

        gl_Position = vec4((screen_position.xy * vec2(resize_factor_x, resize_factor_y) * zoom_factor), 0.0, 1.0);
        out_vert_pos = vec3(model * vec4(position, 1.f));
    }
"#;