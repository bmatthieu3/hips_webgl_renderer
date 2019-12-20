pub static CONTENT: &'static str = r#"#version 300 es
    precision highp float;
    precision lowp sampler3D;
    precision lowp sampler2D;
    precision highp int;

    layout (location = 0) in vec2 lonlat;
    layout (location = 1) in vec3 position;
    layout (location = 2) in vec2 uv;
    layout (location = 3) in int idx_texture;

    out vec2 frag_uv;
    out float aaa;
    //layout (location = 1) out int frag_idx_texture;

    uniform mat4 model;
    uniform float zoom_factor;
    uniform float aspect;

    vec2 world2screen_orthographic(vec3 p) {
        return vec2(-p.x / aspect, p.y);
    }

    void main() {
        vec3 world_pos = vec3(model * vec4(position, 1.f));
        gl_Position = vec4((world2screen_orthographic(world_pos) * zoom_factor), 0.0, 1.0);

        frag_uv = uv;
        aaa = float(idx_texture);
    }
"#;