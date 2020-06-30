#version 300 es
precision lowp float;

in vec2 out_uv;
in vec3 out_p;

out vec4 color;

uniform mat4 model;
uniform sampler2D kernel_texture;
uniform float max_density;

void main() {
    if (out_p.z < 0.f) {
        discard;
    }

    color += texture(kernel_texture, out_uv) / max(log2(max_density), 1.0);
}