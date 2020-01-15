pub static CONTENT: &'static str = r#"#version 300 es
    precision highp float;
    precision lowp sampler2D;
    precision highp int;

    in vec2 frag_uv;

    out vec4 out_frag_color;

    const float PI = 3.141592653589793f;

    struct Tile {
        uint uniq; // Healpix cell
        int texture_idx; // Index in the texture buffer
        float time_received; // Absolute time that the load has been done in ms
        float time_request;
    };

    /*uniform sampler2D textures_0;
    uniform Tile textures_0_tiles[12];*/

    uniform sampler2D textures;
    uniform Tile textures_tiles[64];

    uniform float current_time; // current time in ms

    void main() {
        vec3 color = texture(textures, frag_uv).rgb;
        out_frag_color = vec4(color, 1.f);
    }"#;