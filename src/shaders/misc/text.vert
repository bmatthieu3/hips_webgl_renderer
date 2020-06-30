#version 300 es
precision lowp float;
precision lowp sampler2DArray;

layout (location = 0) in vec2 offset_screen;
layout (location = 1) in vec3 uv;

out vec3 out_uv;

uniform vec2 window_size;
uniform vec2 center_screen;
uniform float scaling;

uniform vec2 ndc_to_clip;
uniform float clip_zoom_factor;

vec2 screen_to_ndc(vec2 pos_screen) {
    // Change of origin
    vec2 origin = pos_screen - window_size/2.0;

    // Scale to fit in [-1, 1]
    return vec2(2.0 * (origin.x/window_size.x), -2.0 * (origin.y/window_size.y));
}

void main() {
    vec2 ndc_pos = screen_to_ndc(center_screen + scaling*offset_screen);

    gl_Position = vec4(ndc_pos, 0.f, 1.f);
    out_uv = uv;
}