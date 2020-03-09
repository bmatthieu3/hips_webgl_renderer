pub static CONTENT: &'static str = r#"#version 300 es
    precision highp float;
    precision lowp sampler2DArray;
    precision highp int;

    in vec3 frag_uv_start;
    in vec3 frag_uv_end;
    in float frag_blending_factor;

    out vec4 out_frag_color;

    uniform sampler2DArray tex;

    uniform float current_time; // current time in ms

    void main() {
        vec4 color_start = vec4(0.f);
        color_start = texture(tex, frag_uv_start);

        vec4 color_end = vec4(0.f);
        color_end = texture(tex, frag_uv_end);

        out_frag_color = mix(color_start, color_end, frag_blending_factor);
    }"#;