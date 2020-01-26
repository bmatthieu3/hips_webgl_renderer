pub static CONTENT: &'static str = r#"#version 300 es
    precision highp float;
    precision lowp sampler2D;
    precision highp int;

    in vec2 frag_uv_start;
    in vec2 frag_uv_end;
    in float frag_blending_factor;
    in vec2 frag_idx_texture;

    out vec4 out_frag_color;

    uniform sampler2D textures[2];

    uniform float current_time; // current time in ms

    void main() {
        uint idx_texture_start = uint(frag_idx_texture.x);
        uint idx_texture_end = uint(frag_idx_texture.y);

        vec4 color_start = vec4(0.f);
        if (idx_texture_start == 0U) {
            color_start = texture(textures[0], frag_uv_start);
        } else {
            color_start = texture(textures[1], frag_uv_start);
        }

        vec4 color_end = vec4(0.f);
        if (idx_texture_end == 0U) {
            color_end = texture(textures[0], frag_uv_end);
        } else {
            color_end = texture(textures[1], frag_uv_end);
        }

        out_frag_color = mix(color_start, color_end, frag_blending_factor);
    }"#;