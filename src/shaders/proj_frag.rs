pub static CONTENT: &'static str = r#"#version 300 es
    precision mediump float;
    precision lowp sampler3D;
    precision lowp sampler2D;

    in vec3 out_vert_pos;

    out vec4 out_frag_color;

    const float PI = 3.1415f;
    const float TRANSITION_Z = 2.0f / 3.0f;
    const float TRANSITION_Z_INV = 3.0f / 2.0f;

    int quarter(vec2 p) {
        bool x_neg = (p.x < 0.0f);
        bool y_neg = (p.y < 0.0f);
        int q = (int(x_neg) + int(y_neg)) | (int(y_neg) << 1);
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
        float x02 = lon * 4.0f / PI;
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
    int ij2z(int i, int j) {
        int i1 = i | (j << 16);

        int j1 = (i1 ^ (i1 >> 8)) & 0x0000FF00;
        int i2 = i1 ^ j1 ^ (j1 << 8);

        int j2 = (i2 ^ (i2 >> 4)) & 0x00F000F0;
        int i3 = i2 ^ j2 ^ (j2 << 4);

        int j3 = (i3 ^ (i3 >> 2)) & 0x0C0C0C0C;
        int i4 = i3 ^ j3 ^ (j3 << 2);

        int j4 = (i4 ^ (i4 >> 1)) & 0x22222222;
        int i5 = i4 ^ j4 ^ (j4 << 1);

        return i5;
    }

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
    vec3 hash_with_dxdy(int depth, vec3 p) {
        //assert!(depth <= 14);
        //assert!(-1.0 <= x && x <= 1.0);
        //assert!(-1.0 <= y && y <= 1.0);
        //assert!(-1.0 <= z && z <= 1.0);
        //debug_assert!(1.0 - (x * x + y * y + z * z) < 1e-5);
        // A f32 mantissa contains 23 bits.
        // - it basically means that when storing (x, y) coordinates,
        //   we can go as deep as depth 24 (or maybe 25)
        //return vec3(0.f, 0.f, 0.f);
        
        int nside = 1 << depth;
        float half_nside = float(nside) * 0.5f;

        float x_pm1 = xpm1(p.xy);
        int q = quarter(p.xy);

        int d0h = 0;
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
            d0h = q + 8;
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
            int q01 = int(x_pm1 > y_pm1);  // 0/1
            //debug_assert!(q01 == 0 || q01 == 1);
            int q12 = int(x_pm1 >= -y_pm1); // 0\1
            //debug_assert!(q12 == 0 || q12 == 1);
            int q03 = 1 - q12; // 1\0
            //let q13 = q01 ^ q12; debug_assert!(q13 == 0 || q13 == 1);
            int q1 = q01 & q12; // = 1 if q1, 0 else
            //debug_assert!( q1 == 0 ||  q1 == 1);
            // x: xcea - 0 if q3 | xcea - 2 if q1 | xcea - 1 if q0 or q2
            //let x_proj = x_pm1 - ((q01 + q12) as i8 - 1) as f32;
            // y: y - 0 if q2 | y - 1 if q1 or q3 | y - 2 if q0 
            //let y_proj = y_pm1 + (q01 + q03) as f32;
            p_proj = vec2(
                x_pm1 - float(q01 + q12 - 1),
                y_pm1 + float(q01 + q03)
            );
            // d0h: +8 if q0 | +4 if q3 | +5 if q1
            d0h = ((q01 + q03) << 2) + ((q + q1) & 3);
        }

        // Coords inside the base cell
        float x = (half_nside * (p_proj.x + p_proj.y));
        float y = (half_nside * (p_proj.y - p_proj.x));
        int i = int(x);
        int j = int(y);
        // Deal with numerical inaccuracies, rare so branch miss-prediction negligible
        if (i == nside) {
            i = i - 1;
        }
        // Deal with numerical inaccuracies, rare so branch miss-prediction negligible
        if (j == nside) {
            j = j - 1;
        }

        return vec3(
            float((d0h << (depth << 1)) | ij2z(i, j)),
            x - float(i),
            y - float(j)
        );
    }

    uniform float zoom_factor;
    uniform sampler3D textures_buffer;
    uniform sampler3D textures_0;

    const int BUFFER_TEX_SIZE = 48;
    const int BUFFER_ZERO_TEX_SIZE = 24;

    struct Tile {
        int uniq; // Healpix cell
        int texture_idx; // Index in the texture buffer
        float time_received; // Absolute time that the load has been done in ms
        float time_request;
    };

    uniform int current_depth;
    /*uniform HEALPixCell hpx_current_depth[BUFFER_TEX_SIZE];
    uniform int num_current_depth_hpx_tiles;

    uniform int prev_depth;
    uniform HEALPixCell hpx_prev_depth[BUFFER_TEX_SIZE];
    uniform int num_prev_depth_hpx_tiles;

    uniform int next_depth;
    uniform HEALPixCell hpx_next_depth[BUFFER_TEX_SIZE];
    uniform int num_next_depth_hpx_tiles;*/

    uniform Tile hpx_zero_depth[BUFFER_ZERO_TEX_SIZE];

    const Tile no_cell = Tile(-1, -1, 0.f, 0.f);
    struct TileColor {
        Tile tile;
        vec3 color;
    };

    // ang2pix textures
    uniform sampler2D ang2pix_0_texture;
    uniform sampler2D ang2pix_1_texture;
    uniform sampler2D ang2pix_2_texture;

    const float tex_step_depth_zero = 1.f/(24.f - 1.f);
    TileColor get_tile_color(vec3 pos, float size, int depth) {
        vec3 res = hash_with_dxdy(depth, pos.zxy);
        int idx = int(res.x);
        int uniq = (1 << (2*(depth + 1))) + idx;

        /*vec2 radec = vec2(atan(pos.x, pos.z), asin(pos.y));
        radec = radec * vec2(-1.f/(2.f*PI), 1.f/PI) + 0.5f;
        vec3 res = texture(ang2pix_0_texture, radec).rgb;
        int idx = int(res.r * 255.f);*/

        vec2 uv = res.zy;

        float a = 0.f;
        float b = size - 1.f;

        int i = int((b + a) / 2.f);

        int h = int(log2(size)) + 1;
        // Binary search among the tile idx
        for(int step = 0; step < h; step++) {
            if (uniq == hpx_zero_depth[i].uniq) {
                Tile tile = hpx_zero_depth[i];
                float idx_texture = float(tile.texture_idx)*tex_step_depth_zero;
                vec3 color = texture(textures_0, vec3(uv, idx_texture)).rgb;

                return TileColor(tile, color);
            } else if (uniq < hpx_zero_depth[i].uniq) {
                // go to left
                b = float(i) - 1.f;
            } else {
                // go to right
                a = float(i) + 1.f;
            }
            i = int((a + b)/2.f);
        }

        // code unreachable
        return TileColor(hpx_zero_depth[0], vec3(0.f));
    }

    const float duration = 1000.f; // 500 ms
    uniform float current_time; // current time in ms
    uniform int max_depth; // max depth of the HiPS

    void main() {
        vec3 frag_pos = normalize(out_vert_pos);
        // Get the HEALPix cell idx and the uv in the texture
        //HEALPixCellContrib current_cell = compute_current_depth_color_from_hips(frag_pos);
        int prev_depth = max(0, current_depth - 1);
        int next_depth = min(29, current_depth + 1);
        //TileColor base_tile = get_tile_color(frag_pos, 24.f, 0);
        TileColor current_tile = get_tile_color(frag_pos, 24.f, current_depth);
        TileColor prev_tile = get_tile_color(frag_pos, 24.f, prev_depth);
        TileColor next_tile = get_tile_color(frag_pos, 24.f, next_depth);

        float alpha = clamp((current_time - current_tile.tile.time_received) / duration, 0.f, 1.f);
        vec3 out_color = vec3(0.f);
        if(prev_tile.tile.time_request > next_tile.tile.time_request) {
            // zoom
            out_color = mix(next_tile.color, current_tile.color, alpha);
        } else {
            // dezoom
            out_color = mix(prev_tile.color, current_tile.color, alpha);
        }

        /*float alpha = 0.f;
        if (current_cell.cell.idx > -1) { // tile downloaded
            alpha = clamp((current_time - current_cell.cell.time_received) / duration, 0.f, 1.f);
        }

        vec3 out_color = vec3(0.f);

        if (alpha == 1.f) {
            out_color = current_cell.color;
        } else {
            HEALPixCellContrib prev_depth_cell = HEALPixCellContrib(no_cell, vec3(0.f));
            HEALPixCellContrib next_depth_cell = HEALPixCellContrib(no_cell, vec3(0.f));
            if (current_depth > 0) {
                prev_depth_cell = compute_prev_depth_color_from_hips(frag_pos);
            }
            if (current_depth < max_depth) {
                next_depth_cell = compute_next_depth_color_from_hips(frag_pos);
            }

            HEALPixCellContrib past_cell = HEALPixCellContrib(no_cell, vec3(0.f));
            if (prev_depth_cell.cell.idx == -1 && next_depth_cell.cell.idx != -1) {
                past_cell = next_depth_cell;
            } else if (next_depth_cell.cell.idx == -1 && prev_depth_cell.cell.idx != -1) {
                past_cell = prev_depth_cell;
            } else if (next_depth_cell.cell.idx != -1 && prev_depth_cell.cell.idx != -1) {
                // the two are in the buffer
                if (prev_depth_cell.cell.time_received > next_depth_cell.cell.time_received) {
                    past_cell = prev_depth_cell;
                } else {
                    past_cell = next_depth_cell;
                }
            } else {
                // neither are in the buffer
                // We get an HEALPix tile at the depth 0 as for the past_cell

                past_cell = compute_zero_depth_color_from_hips(frag_pos);
            }

            out_color = mix(past_cell.color, current_cell.color, alpha);
        }*/
        out_frag_color = vec4(out_color, 1.f);
        //out_frag_color = vec4(1.f);
        //out_frag_color = vec4(vec3(1.f), 0.2f);
    }"#;