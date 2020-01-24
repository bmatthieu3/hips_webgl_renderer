pub static CONTENT: &'static str = r#"#version 300 es
    precision highp float;

    layout (location = 0) in vec3 position;

    uniform mat4 model;
    uniform mat4 view;

    uniform vec2 ndc_to_clip;
    uniform float clip_zoom_factor;

    uniform float resize_factor_x;
    uniform float resize_factor_y;

    uniform float aspect;

    const float PI = 3.1415926535897932384626433832795f;

    vec3 world_to_screen_space(vec3 p) {
        float delta = asin(p.y);
        float theta = atan(p.x, p.z);

        float alpha = acos(cos(delta)*cos(theta / 2.f));
        float X = 2.f * (alpha / sin(alpha)) * cos(delta) * sin(theta / 2.f);
        float Y = (alpha / sin(alpha)) * sin(delta);

        // X in [-1, 1]
        // Y in [-1/2; 1/2] and scaled by the screen width/height ratio
        return vec3(X / PI, aspect * Y / PI, 0.f);
    }

    void main() {
        vec3 p = position;
        vec3 world_space_pos = vec3(inverse(model) * vec4(p, 1.f));

        vec3 screen_space_pos = world_to_screen_space(world_space_pos);
        vec2 pos_xy = screen_space_pos.xy;
        gl_Position = vec4(pos_xy, 0, 1.0);
    }
"#;