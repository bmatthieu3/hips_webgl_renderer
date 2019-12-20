pub static CONTENT: &'static str = r#"#version 300 es
    precision lowp float;

    in vec2 out_uv;
    out vec4 color;

    uniform sampler2D texture_fbo;
    uniform sampler2D colormap;
    uniform float alpha;

    void main() {
        float opacity = texture(texture_fbo, out_uv).r;

        float o = smoothstep(0.f, 0.1f, opacity);

        color = texture(colormap, vec2(opacity, 0.5f));
        color.a = alpha * o;
    }
"#;