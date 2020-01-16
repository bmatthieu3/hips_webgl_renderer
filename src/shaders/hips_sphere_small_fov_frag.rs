pub static CONTENT: &'static str = r#"#version 300 es
    precision highp float;
    precision lowp sampler2D;
    precision highp int;

    in vec2 frag_uv_start;
    in vec2 frag_uv_end;
    in float frag_blending_factor;

    out vec4 out_frag_color;

    uniform sampler2D textures;

    uniform float current_time; // current time in ms

    void main() {
        vec4 color_start = texture(textures, frag_uv_start);
        vec4 color_end = texture(textures, frag_uv_end);

        out_frag_color = mix(color_start, color_end, frag_blending_factor);
    }"#;