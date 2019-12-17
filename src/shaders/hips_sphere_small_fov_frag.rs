pub static CONTENT: &'static str = r#"#version 300 es
    precision highp float;
    precision lowp sampler3D;
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

    uniform sampler2D textures_0;
    uniform Tile textures_0_tiles[12];

    uniform float current_time; // current time in ms

    void main() {
        Tile tile = textures_0_tiles[0];
        float idx_row = float(tile.texture_idx / 8); // in [0; 7]
        float idx_col = float(tile.texture_idx % 8); // in [0; 7]

        vec2 offset = (vec2(idx_col, idx_row) + frag_uv)/8.f;

        vec3 color = texture(textures_0, offset).rgb;
        out_frag_color = vec4(color, 1.f);
    }"#;