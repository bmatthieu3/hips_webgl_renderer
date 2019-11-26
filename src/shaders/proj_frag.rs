pub static CONTENT: &'static str = r#"#version 300 es
    precision highp float;
    precision lowp sampler3D;
    precision lowp sampler2D;
    precision highp int;

    in vec3 out_vert_pos;

    out vec4 out_frag_color;

    const float PI = 3.141592653589793f;
    const float FOUR_OVER_PI = 1.27323954474f;
    const float TRANSITION_Z = 0.66666666666f;
    const float TRANSITION_Z_INV = 1.5f;

    uint quarter(vec2 p) {
        uint x_neg = uint(p.x < 0.0f);
        uint y_neg = uint(p.y < 0.0f);
        uint q = (x_neg + y_neg) | (y_neg << 1U);
        return q;
    }

    float xpm1(vec2 p) {
        bool x_neg = (p.x < 0.0f);
        //debug_assert!(x_neg <= 1);
        bool y_neg = (p.y < 0.0f);
        //debug_assert!(y_neg <= 1);
        // The purpose it to have the same numerical precision for each base cell
        // by avoiding subtraction by 1 or 3 or 5 or 7
        float lon = atan(abs(p.y), abs(p.x));
        //debug_assert!(0.0 <= lon && lon <= PI / 2.0);
        float x02 = lon * FOUR_OVER_PI;
        //debug_assert!(0.0 <= x02 && x02 <= 2.0);
        if (x_neg != y_neg) { // Could be replaced by a sign copy from (x_neg ^ y_neg) << 32
            return 1.0f - x02;
        } else {
            return x02 - 1.0f;
        }
    }
    
    float one_minus_z_pos(vec3 p) {
        //debug_assert!(z > 0.0);
        float d2 = dot(p.xy, p.xy); // z = sqrt(1 - d2) AND sqrt(1 - x) = 1 - x / 2 - x^2 / 8 - x^3 / 16 - 5 x^4/128 - 7 * x^5/256

        if (d2 < 1e-1f) { // <=> dec > 84.27 deg
            return d2 * (0.5f + d2 * (0.125f + d2 * (0.0625f + d2 * (0.0390625f + d2 * 0.02734375f))));
        }
        return 1.0f - p.z;
    }

    float one_minus_z_neg(vec3 p) {
        //debug_assert!(z < 0.0);
        float d2 = dot(p.xy, p.xy); // z = sqrt(1 - d2) AND sqrt(1 - x) = 1 - x / 2 - x^2 / 8 - x^3 / 16 - 5 x^4/128 - 7 * x^5/256
        if (d2 < 1e-1f) { // <=> dec < -84.27 deg
            // 0.5 * d2 + 0.125 * d2 * d2
            return d2 * (0.5f + d2 * (0.125f + d2 * (0.0625f + d2 * (0.0390625f + d2 * 0.02734375f))));
        }
        return p.z + 1.0f;
    }

    // Z-Order curve projection.
    uint ij2z(uint i, uint j) {
        uint i1 = i | (j << 16U);

        uint j1 = (i1 ^ (i1 >> 8U)) & 0x0000FF00U;
        uint i2 = i1 ^ j1 ^ (j1 << 8U);

        uint j2 = (i2 ^ (i2 >> 4U)) & 0x00F000F0U;
        uint i3 = i2 ^ j2 ^ (j2 << 4U);

        uint j3 = (i3 ^ (i3 >> 2U)) & 0x0C0C0C0CU;
        uint i4 = i3 ^ j3 ^ (j3 << 2U);

        uint j4 = (i4 ^ (i4 >> 1U)) & 0x22222222U;
        uint i5 = i4 ^ j4 ^ (j4 << 1U);

        return i5;
    }

    struct HashDxDy {
        uint idx;
        float dx;
        float dy;
    };

    // Returns the cell number (hash value) associated with the given position on the unit sphere, 
    // together with the offset `(dx, dy)` on the Euclidean plane of the projected position with
    // respect to the origin of the cell (South vertex).
    // # Inputs:
    // - `depth` in `[0, 14]` (so that and HEALPix cell number can be stored on an unsigned integer)
    // - `x`: in `[-1.0, 1.0]`
    // - `y`: in `[-1.0, 1.0]`
    // - `z`: in `[-1.0, 1.0]`
    // # Output
    // - the cell number (hash value) associated with the given position on the unit sphere,
    //   in `[0, 12*nside^2[`
    // - `dx`: the positional offset $\in [0, 1[$ along the south-to-east axis
    // - `dy`: the positional offset $\in [0, 1[$ along the south-to-west axis
    // # WARNING
    // - The function assumes, without checking, that the input vector is a unit vector 
    //   (hence `x^2 + y^2 + z^2 = 1`) !!
    // - Operations being made on simple precision float, the precision is lower than `~0.2 arcsec` only!!
    // - At depth 13, the precision on `(dx, dy)` is better than `(1/512, 1/512)`, i.e. 2e-3.
    HashDxDy hash_with_dxdy(int depth, vec3 p) {
        //assert!(depth <= 14);
        //assert!(-1.0 <= x && x <= 1.0);
        //assert!(-1.0 <= y && y <= 1.0);
        //assert!(-1.0 <= z && z <= 1.0);
        //debug_assert!(1.0 - (x * x + y * y + z * z) < 1e-5);
        // A f32 mantissa contains 23 bits.
        // - it basically means that when storing (x, y) coordinates,
        //   we can go as deep as depth 24 (or maybe 25)
        
        uint nside = 1U << uint(depth);
        float half_nside = float(nside) * 0.5f;

        float x_pm1 = xpm1(p.xy);
        uint q = quarter(p.xy);

        uint d0h = 0U;
        vec2 p_proj = vec2(0.f);
        if (p.z > TRANSITION_Z) {
            // North polar cap, Collignon projection.
            // - set the origin to (PI/4, 0)
            float sqrt_3_one_min_z = sqrt(3.0f * one_minus_z_pos(p));
            p_proj = vec2(x_pm1 * sqrt_3_one_min_z, 2.0f - sqrt_3_one_min_z);
            d0h = q;
        } else if (p.z < -TRANSITION_Z) {
            // South polar cap, Collignon projection
            // - set the origin to (PI/4, -PI/2)
            float sqrt_3_one_min_z = sqrt(3.0f * one_minus_z_neg(p));
            p_proj = vec2(x_pm1 * sqrt_3_one_min_z, sqrt_3_one_min_z);
            d0h = q + 8U;
        } else {
            // Equatorial region, Cylindrical equal area projection
            // - set the origin to (PI/4, 0)               if q = 2
            // - set the origin to (PI/4, -PI/2)           if q = 0
            // - set the origin to (0, -TRANSITION_LAT)    if q = 3
            // - set the origin to (PI/2, -TRANSITION_LAT) if q = 1
            // let zero_or_one = (x_cea as u8) & 1;
            float y_pm1 = p.z * TRANSITION_Z_INV;
            // |\2/|
            // .3X1.
            // |/0\|
            uint q01 = uint(x_pm1 > y_pm1);  // 0/1
            //debug_assert!(q01 == 0 || q01 == 1);
            uint q12 = uint(x_pm1 >= -y_pm1); // 0\1
            //debug_assert!(q12 == 0 || q12 == 1);
            uint q03 = 1U - q12; // 1\0
            //let q13 = q01 ^ q12; debug_assert!(q13 == 0 || q13 == 1);
            uint q1 = q01 & q12; // = 1 if q1, 0 else
            //debug_assert!( q1 == 0 ||  q1 == 1);
            // x: xcea - 0 if q3 | xcea - 2 if q1 | xcea - 1 if q0 or q2
            //let x_proj = x_pm1 - ((q01 + q12) as i8 - 1) as f32;
            // y: y - 0 if q2 | y - 1 if q1 or q3 | y - 2 if q0 
            //let y_proj = y_pm1 + (q01 + q03) as f32;
            p_proj = vec2(
                x_pm1 - float(int(q01 + q12) - 1),
                y_pm1 + float(q01 + q03)
            );
            // d0h: +8 if q0 | +4 if q3 | +5 if q1
            d0h = ((q01 + q03) << 2U) + ((q + q1) & 3U);
        }

        // Coords inside the base cell
        float x = (half_nside * (p_proj.x + p_proj.y));
        float y = (half_nside * (p_proj.y - p_proj.x));
        uint i = uint(x);
        uint j = uint(y);
        // Deal with numerical inaccuracies, rare so branch miss-prediction negligible
        /*if (i == nside) {
            i = i - 1U;
        }
        // Deal with numerical inaccuracies, rare so branch miss-prediction negligible
        if (j == nside) {
            j = j - 1U;
        }*/

        return HashDxDy(
            (d0h << (uint(depth) << 1U)) | ij2z(i, j),
            x - float(i),
            y - float(j)
        );
    }

    uniform float zoom_factor;
    uniform int last_zoom_action;

    struct Tile {
        uint uniq; // Healpix cell
        int texture_idx; // Index in the texture buffer
        float time_received; // Absolute time that the load has been done in ms
        float time_request;
    };

    uniform int current_depth;
    
    uniform sampler3D textures;
    uniform Tile textures_tiles[20];

    uniform sampler3D textures_0;
    uniform Tile textures_0_tiles[12];

    uniform float current_time; // current time in ms
    struct TileColor {
        Tile tile;
        vec3 color;
        bool found;
    };

    TileColor get_tile_color(vec3 pos, float size, int depth) {
        HashDxDy result = hash_with_dxdy(depth, pos.zxy);
        uint idx = result.idx;
        uint uniq = (1U << (2U*(uint(depth) + 1U))) + idx;

        vec2 uv = vec2(result.dy, result.dx);

        int a = 0;
        int b = int(size) - 1;

        int i = (b + a) / 2;

        int h = int(log2(size)) + 1;
        // Binary search among the tile idx
        for(int step = 0; step < h; step++) {
            if (uniq == textures_tiles[i].uniq) {
                Tile tile = textures_tiles[i];
                float step_depth_texture = 1.f / size;
                float idx_texture = (float(tile.texture_idx) + 0.5f) * step_depth_texture;
                vec3 color = texture(textures, vec3(uv, idx_texture)).rgb;

                return TileColor(tile, color, true);
            } else if (uniq < textures_tiles[i].uniq) {
                // go to left
                b = i - 1;
            } else {
                // go to right
                a = i + 1;
            }
            i = (a + b)/2;
        }

        // code unreachable
        Tile empty = Tile(0U, -1, current_time, 0.f);
        return TileColor(empty, vec3(0.f), false);
    }

    TileColor get_depth_0_tile(vec3 pos) {
        HashDxDy result = hash_with_dxdy(0, pos.zxy);
        uint idx = result.idx;
        uint uniq = 4U + idx;

        vec2 uv = vec2(result.dy, result.dx);

        int a = 0;
        int b = 11;

        int i = 5;

        int h = 4;
        // Binary search among the tile idx
        for(int step = 0; step < h; step++) {
            if (uniq == textures_0_tiles[i].uniq) {
                Tile tile = textures_0_tiles[i];
                float step_depth_texture = 1.f / 12.f;
                float idx_texture = (float(tile.texture_idx) + 0.5f) * step_depth_texture;
                vec3 color = texture(textures_0, vec3(uv, idx_texture)).rgb;

                return TileColor(tile, color, true);
            } else if (uniq < textures_0_tiles[i].uniq) {
                // go to left
                b = i - 1;
            } else {
                // go to right
                a = i + 1;
            }
            i = (a + b)/2;
        }

        // code unreachable
        Tile empty = Tile(0U, -1, current_time, 0.f);
        return TileColor(empty, vec3(0.f), false);
    }

    const float duration = 500.f; // 500ms
    uniform int max_depth; // max depth of the HiPS

    void main() {
        vec3 frag_pos = normalize(out_vert_pos);
        // Get the HEALPix cell idx and the uv in the texture
        TileColor current_tile = get_tile_color(frag_pos, 20.f, current_depth);

        float alpha = clamp((current_time - current_tile.tile.time_received) / duration, 0.f, 1.f);

        // Little optimization: if the current tile is loaded since the time duration
        // then we do not need to evaluate the frag position for the previous/next depth
        if (alpha == 1.f) {
            out_frag_color = vec4(current_tile.color, 1.f);
            return;
        }
        vec3 out_color = vec3(0.f);
        int depth = 0;
        if (last_zoom_action == 1) {
            // zoom
            depth = max(0, current_depth - 1);
        } else {
            // unzoom
            depth = min(29, current_depth + 1);
        }

        TileColor tile = get_tile_color(frag_pos, 20.f, depth);
        if (!tile.found) {
            tile = get_depth_0_tile(frag_pos);
        }

        out_color = mix(tile.color, current_tile.color, alpha);
        out_frag_color = vec4(out_color, 1.f);
    }"#;