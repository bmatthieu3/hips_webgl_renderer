#version 300 es
precision lowp float;
layout (location = 0) in vec2 offset;
layout (location = 1) in vec2 uv;

layout (location = 2) in vec3 center;
layout (location = 3) in vec2 center_lonlat;

uniform float current_time;
uniform mat4 inv_model;

uniform vec2 ndc_to_clip;
uniform float czf;
uniform vec2 kernel_size;

out vec2 out_uv;
out vec3 out_p;

@import ../hips/projection;

void main() {
    vec3 p = vec3(inv_model * vec4(center, 1.0f));
    p = check_inversed_longitude(p);

    vec2 center_pos_clip_space = world2clip_arc(p);

    vec2 pos_clip_space = center_pos_clip_space;
    gl_Position = vec4((pos_clip_space / (ndc_to_clip * czf)) + offset * kernel_size , 0.f, 1.f);

    out_uv = uv;
    out_p = p;
}