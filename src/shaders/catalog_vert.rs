pub static CONTENT: &'static str = r#"#version 300 es
    precision lowp float;
    layout (location = 0) in vec2 offset;
    layout (location = 1) in vec2 uv;

    layout (location = 2) in vec3 center;
    layout (location = 3) in vec2 center_lonlat;
    layout (location = 4) in float score;

    uniform float current_time;
    uniform mat4 model;
    uniform float aspect;

    uniform vec2 ndc_to_clip;
    uniform float clip_zoom_factor;

    uniform float strength;

    const float PI = 3.1415926535897932384626433832795f;

    out vec2 out_uv;
    out vec3 out_p;

    vec2 world2screen_aitoff(vec3 p) {
        float delta = asin(p.y);
        float theta = atan(p.x, p.z);

        float theta_by_two = theta * 0.5f;

        float alpha = acos(cos(delta)*cos(theta_by_two));
        float inv_sinc_alpha = 1.f;
        if (alpha > 1e-3f) {
            inv_sinc_alpha = alpha / sin(alpha);
        }

        // The minus is an astronomical convention.
        // longitudes are increasing from right to left
        float x = -2.f * inv_sinc_alpha * cos(delta) * sin(theta_by_two);
        float y = inv_sinc_alpha * sin(delta);

        return vec2(x / PI, y / PI);
    }
    vec2 world2screen_orthographic(vec3 p) {
        return vec2(-p.x, p.y);
    }

    void main() {
        vec3 p = vec3(model * vec4(center, 1.0f));

        vec2 pos_clip_space = world2screen_orthographic(p) + offset * (0.02f * clip_zoom_factor);
        gl_Position = vec4(pos_clip_space / (ndc_to_clip * clip_zoom_factor), 0.f, 1.f);
        //gl_Position = vec4(screen_pos, 0.f, 1.f);
        out_uv = uv;
        out_p = p;
    }
"#;