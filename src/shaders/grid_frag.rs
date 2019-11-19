pub static CONTENT: &'static str = r#"#version 300 es
    precision lowp float;

    out vec4 color;

    uniform vec4 location_color;

    uniform float current_time;
    uniform int last_user_action;

    void main() {
        color = location_color;
    }"#;