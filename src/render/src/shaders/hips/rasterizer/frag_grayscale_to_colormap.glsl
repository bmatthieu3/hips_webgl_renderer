#version 300 es
precision highp float;
precision lowp sampler2DArray;
precision lowp isampler2DArray;
precision highp int;

in vec3 frag_uv_start;
in vec3 frag_uv_end;
in float frag_blending_factor;

out vec4 out_frag_color;

@import ../color;

void main() {
    vec4 color_start = vec4(colormap_f(get_grayscale_from_texture(frag_uv_start)).rgb, 1.0);
    vec4 color_end = vec4(colormap_f(get_grayscale_from_texture(frag_uv_end)).rgb, 1.0);

    out_frag_color = mix(color_start, color_end, frag_blending_factor);
}