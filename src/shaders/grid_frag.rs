pub static CONTENT: &'static str = r#"#version 300 es
    precision highp float;

    out vec4 color;

    uniform vec4 location_color;

    void main() {
        color = location_color;
    }"#;