pub static CONTENT: &'static str = r#"#version 300 es
    precision highp float;
    layout (location = 0) in vec2 offset;
    layout (location = 1) in vec2 uv;

    layout (location = 2) in vec3 center;

    uniform float current_time;
    uniform mat4 model;
    uniform float aspect;
    uniform float zoom_factor;

    const float PI = 3.1415926535897932384626433832795f;

    out vec2 out_uv;
    out vec3 out_p;

    vec2 world2screen_aitoff(vec3 p) {
        float delta = asin(p.y);
        float theta = atan(p.x, p.z);

        float theta_by_two = theta / 2.f;

        float alpha = acos(cos(delta)*cos(theta_by_two));
        float inv_sinc_alpha = 1.f;
        if (alpha > 1e-3f) {
            inv_sinc_alpha = alpha / sin(alpha);
        }

        // The minus is an astronomical convention.
        // longitudes are increasing from right to left
        float x = -2.f * inv_sinc_alpha * cos(delta) * sin(theta_by_two);
        float y = inv_sinc_alpha * sin(delta);

        return vec2(x / PI, aspect * y / PI);
    }
    vec2 world2screen_orthographic(vec3 p) {
        return vec2(-p.x / aspect, p.y);
    }

    void main() {
        vec3 p = vec3(model * vec4(center, 1.0f));

        vec2 screen_pos = world2screen_orthographic(p) + offset * (0.02f / zoom_factor) * vec2(1.f, aspect);
        gl_Position = vec4(screen_pos * zoom_factor, 0.f, 1.f);
        out_uv = uv;
        out_p = p;
    }
"#;