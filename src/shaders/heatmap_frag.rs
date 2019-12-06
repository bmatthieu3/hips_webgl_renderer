pub static CONTENT: &'static str = r#"#version 300 es
    precision lowp float;

    in vec2 out_uv;
    out vec4 color;

    uniform sampler2D texture_fbo;
    uniform sampler2D colormap;

    void main() {
        float opacity = texture(texture_fbo, out_uv).r;

        color = texture(colormap, vec2(opacity, 0.5f));
        color.a *= 10.f*opacity;

        //color = mix(vec4(0.0, 0.0, 0.0, 0.0), texture(colormap, vec2(opacity, 0.5f)), opacity);
    }
"#;