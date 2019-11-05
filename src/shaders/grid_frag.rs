pub static CONTENT: &'static str = r#"#version 300 es
    precision highp float;

    out vec4 color;

    uniform vec4 location_color;

    uniform float current_time;

    void main() {
        color = location_color;
    }"#;