use crate::shader::Shaderize;

#[derive(Shaderize)]
#[VertexShader = r#"#version 300 es
    precision lowp float;
    layout (location = 0) in vec2 offset;
    layout (location = 1) in vec2 uv;

    layout (location = 2) in vec3 center;
    layout (location = 3) in vec2 center_lonlat;
    layout (location = 4) in float mag;
    layout (location = 5) in float plx;

    uniform float current_time;
    uniform mat4 model;
    uniform float aspect;

    uniform vec2 ndc_to_clip;
    uniform float clip_zoom_factor;

    uniform float strength;

    uniform float min_plx;
    uniform float min_size_source;
    uniform float max_plx;
    uniform float max_size_source;

    const float PI = 3.1415926535897932384626433832795f;

    out vec2 out_uv;
    out vec3 out_p;

    vec2 world2clip_orthographic(vec3 p) {
        return vec2(-p.x, p.y);
    }

    void main() {
        vec3 p = vec3(model * vec4(center, 1.0f));
        vec2 center_pos_clip_space = world2clip_orthographic(p);

        float a = clamp((plx - min_plx)/(max_plx - min_plx), 0.f, 1.f);
        float size_source = mix(min_size_source, max_size_source, a);

        vec2 pos_clip_space = center_pos_clip_space + offset * (size_source * clip_zoom_factor);
        gl_Position = vec4(pos_clip_space / (ndc_to_clip * clip_zoom_factor), 0.f, 1.f);
        //gl_Position = vec4(screen_pos, 0.f, 1.f);
        out_uv = uv;
        out_p = p;
    }
"#]
#[FragmentShader = r#"#version 300 es
    precision lowp float;

    in vec2 out_uv;
    in vec3 out_p;

    out vec4 color;

    uniform mat4 model;
    uniform sampler2D kernel_texture;
    uniform float strength;

    void main() {
        if (out_p.z < 0.f) {
            discard;
        }

        color = texture(kernel_texture, out_uv);
        color.a *= strength;
    }
"#]
pub struct Catalog_Orthographic;

#[derive(Shaderize)]
#[VertexShader = r#"#version 300 es
    precision lowp float;
    layout (location = 0) in vec2 offset;
    layout (location = 1) in vec2 uv;

    layout (location = 2) in vec3 center;
    layout (location = 3) in vec2 center_lonlat;
    layout (location = 4) in float mag;
    layout (location = 5) in float plx;

    uniform float current_time;
    uniform mat4 model;
    uniform float aspect;

    uniform vec2 ndc_to_clip;
    uniform float clip_zoom_factor;

    uniform float strength;

    uniform float min_plx;
    uniform float min_size_source;
    uniform float max_plx;
    uniform float max_size_source;

    const float PI = 3.1415926535897932384626433832795f;

    out vec2 out_uv;
    out vec3 out_p;

    vec2 world2clip_aitoff(vec3 p) {
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

    void main() {
        vec3 p = vec3(model * vec4(center, 1.0f));
        vec2 center_pos_clip_space = world2clip_aitoff(p);

        float a = clamp((plx - min_plx)/(max_plx - min_plx), 0.f, 1.f);
        float size_source = mix(min_size_source, max_size_source, a);

        vec2 pos_clip_space = center_pos_clip_space + offset * (size_source * clip_zoom_factor);
        gl_Position = vec4(pos_clip_space / (ndc_to_clip * clip_zoom_factor), 0.f, 1.f);
        //gl_Position = vec4(screen_pos, 0.f, 1.f);
        out_uv = uv;
        out_p = p;
    }
"#]
#[FragmentShader = r#"#version 300 es
    precision lowp float;

    in vec2 out_uv;
    in vec3 out_p;

    out vec4 color;

    uniform mat4 model;
    uniform sampler2D kernel_texture;
    uniform float strength;

    void main() {
        color = texture(kernel_texture, out_uv);
        color.a *= strength;
    }
"#]
pub struct Catalog_Aitoff;

#[derive(Shaderize)]
#[VertexShader = r#"#version 300 es
    precision lowp float;
    layout (location = 0) in vec2 offset;
    layout (location = 1) in vec2 uv;

    layout (location = 2) in vec3 center;
    layout (location = 3) in vec2 center_lonlat;
    layout (location = 4) in float mag;
    layout (location = 5) in float plx;

    uniform float current_time;
    uniform mat4 model;
    uniform float aspect;

    uniform vec2 ndc_to_clip;
    uniform float clip_zoom_factor;

    uniform float strength;

    uniform float min_plx;
    uniform float min_size_source;
    uniform float max_plx;
    uniform float max_size_source;

    const float PI = 3.1415926535897932384626433832795f;

    out vec2 out_uv;
    out vec3 out_p;
    
    vec2 world2clip_mollweide(vec3 p) {
        // X in [-1, 1]
        // Y in [-1/2; 1/2] and scaled by the screen width/height ratio
        int max_iter = 10;

        float delta = asin(p.y);
        float theta = atan(p.x, p.z);
 
        float cst = PI * sin(delta);

        float phi = delta;
        float f = phi + sin(phi) - cst;

        int k = 0;
        while (abs(f) > 1e-4 && k < max_iter) {
            phi = phi - f / (1.f + cos(phi));
            f = phi + sin(phi) - cst;

            k = k + 1;
        }

        phi = phi * 0.5f;

        // The minus is an astronomical convention.
        // longitudes are increasing from right to left
        float x = -(theta / PI) * cos(phi);
        float y = 0.5f * sin(phi);

        return vec2(x, y);
    }

    void main() {
        vec3 p = vec3(model * vec4(center, 1.0f));
        vec2 center_pos_clip_space = world2clip_mollweide(p);

        float a = clamp((plx - min_plx)/(max_plx - min_plx), 0.f, 1.f);
        float size_source = mix(min_size_source, max_size_source, a);

        vec2 pos_clip_space = center_pos_clip_space + offset * (size_source * clip_zoom_factor);
        gl_Position = vec4(pos_clip_space / (ndc_to_clip * clip_zoom_factor), 0.f, 1.f);
        //gl_Position = vec4(screen_pos, 0.f, 1.f);
        out_uv = uv;
        out_p = p;
    }
"#]
#[FragmentShader = r#"#version 300 es
    precision lowp float;

    in vec2 out_uv;
    in vec3 out_p;

    out vec4 color;

    uniform mat4 model;
    uniform sampler2D kernel_texture;
    uniform float strength;

    void main() {
        color = texture(kernel_texture, out_uv);
        color.a *= strength;
    }
"#]
pub struct Catalog_MollWeide;

#[derive(Shaderize)]
#[VertexShader = r#"#version 300 es
    precision lowp float;
    layout (location = 0) in vec2 offset;
    layout (location = 1) in vec2 uv;

    layout (location = 2) in vec3 center;
    layout (location = 3) in vec2 center_lonlat;
    layout (location = 4) in float mag;
    layout (location = 5) in float plx;

    uniform float current_time;
    uniform mat4 model;
    uniform float aspect;

    uniform vec2 ndc_to_clip;
    uniform float clip_zoom_factor;

    uniform float strength;

    uniform float min_plx;
    uniform float min_size_source;
    uniform float max_plx;
    uniform float max_size_source;

    const float PI = 3.1415926535897932384626433832795f;

    out vec2 out_uv;
    out vec3 out_p;
    
    vec2 world2clip_mercator(vec3 p) {
        // X in [-1, 1]
        // Y in [-1/2; 1/2] and scaled by the screen width/height ratio
        int max_iter = 10;

        float delta = asin(p.y);
        float theta = atan(p.x, p.z);

        float x = -theta / PI;
        float y = log(tan(PI * 0.25f + delta * 0.5f)) / PI;

        return vec2(x, y);
    }

    void main() {
        vec3 p = vec3(model * vec4(center, 1.0f));
        vec2 center_pos_clip_space = world2clip_mercator(p);

        float a = clamp((plx - min_plx)/(max_plx - min_plx), 0.f, 1.f);
        float size_source = mix(min_size_source, max_size_source, a);

        vec2 pos_clip_space = center_pos_clip_space + offset * (size_source * clip_zoom_factor);
        gl_Position = vec4(pos_clip_space / (ndc_to_clip * clip_zoom_factor), 0.f, 1.f);
        //gl_Position = vec4(screen_pos, 0.f, 1.f);
        out_uv = uv;
        out_p = p;
    }
"#]
#[FragmentShader = r#"#version 300 es
    precision lowp float;

    in vec2 out_uv;
    in vec3 out_p;

    out vec4 color;

    uniform mat4 model;
    uniform sampler2D kernel_texture;
    uniform float strength;

    void main() {
        color = texture(kernel_texture, out_uv);
        color.a *= strength;
    }
"#]
pub struct Catalog_Mercator;