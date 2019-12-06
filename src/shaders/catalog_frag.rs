pub static CONTENT: &'static str = r#"#version 300 es
    precision lowp float;

    in vec2 out_uv;
    in vec3 out_p;

    out vec4 color;

    uniform mat4 model;
    uniform float zoom_factor;

    uniform sampler2D kernel_texture;

    void main() {
        if (out_p.z < 0.f) {
            discard;
        }

        color = texture(kernel_texture, out_uv);
        //float opacity = 0.1f * zoom_factor;
        //color.rgb *= 0.6f;
        //color = vec4(1.f, 0.f, 0.f, 1.f);
    }
"#;