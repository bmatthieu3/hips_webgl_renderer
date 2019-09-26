extern crate itertools_num;
extern crate slab;

use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::{WebGl2RenderingContext, console};
use cgmath;
use cgmath::{InnerSpace, Vector3};

mod shader;
mod renderable;
mod viewport;
mod texture;
mod math;
mod utils;

use shader::Shader;
use renderable::Renderable;
use renderable::hips_sphere::HiPSSphere;
use renderable::projection;
use renderable::projection::{ProjectionType, Aitoff, Orthographic};
use viewport::ViewPort;

fn request_animation_frame(f: &Closure<FnMut()>) {
    web_sys::window()
        .unwrap()
        .request_animation_frame(f.as_ref().unchecked_ref())
        .expect("should register `requestAnimationFrame` OK");
}

fn compute_speed(t_start: f32, speed_max: f32) -> f32 {
    let t = utils::get_current_time() as f32; 
    let t_duration = 1200_f32; // in ms
    let t_end = t_start + t_duration;

    if t > t_end {
        0_f32
    } else {
        let speed = (-t*speed_max / t_duration) + t_end*speed_max/t_duration;
        speed
    }
}

use std::time::SystemTime;

use std::rc::Rc;
use std::cell::{RefCell, Cell};
#[wasm_bindgen(start)]
pub fn start() -> Result<(), JsValue> {
    let window = web_sys::window().unwrap();
    let document = window.document().unwrap();
    let canvas = document.get_element_by_id("canvas").unwrap();
    let canvas: web_sys::HtmlCanvasElement = canvas.dyn_into::<web_sys::HtmlCanvasElement>()?;

    let inner_width = window.inner_width()?
        .as_f64()
        .unwrap();
    let inner_height = window.inner_height()?
        .as_f64()
        .unwrap();
    canvas.set_width(inner_width as u32);
    canvas.set_height(inner_height as u32);

    //let context_attributes = js_sys::Map::new();
    //let context_attributes2 = context_attributes.set(&"antialias".into(), &false.into());
    let context_options = js_sys::JSON::parse(&"{\"antialias\":true}").unwrap();

    let gl = Rc::new(
        canvas.get_context_with_context_options("webgl2", context_options.as_ref())?
            .unwrap()
            .dyn_into::<WebGl2RenderingContext>()?
    );

    let shader_texture = Rc::new(Shader::new(&gl, r#"#version 300 es
        precision highp float;
        precision highp int;

        in vec3 position;
        in vec2 screen_position;

        out vec3 out_vert_pos;

        uniform mat4 model;
        uniform mat4 view;

        uniform float zoom_factor;

        void main() {
            gl_Position = vec4(screen_position.x * zoom_factor, screen_position.y * zoom_factor, 0.0, 1.0);
            out_vert_pos = vec3(model * vec4(position, 1.f));
        }"#,
        r#"#version 300 es
        precision highp float;
        precision highp int;
        precision highp sampler3D;

        in vec3 out_vert_pos;

        out vec4 out_frag_color;

        const float PI = 3.1415926535897932384626433832795f;
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
        uniform int draw_grid;
        uniform sampler3D textures_buffer;
        uniform sampler3D textures_zero_depth_buffer;

        const int BUFFER_TEX_SIZE = 48;
        const int BUFFER_ZERO_TEX_SIZE = 12;

        struct HEALPixCell {
            int idx; // Healpix cell
            int buf_idx; // Index in the texture buffer
            float time_received; // Absolute time that the load has been done in ms
        };

        uniform int current_depth;
        uniform HEALPixCell hpx_current_depth[BUFFER_TEX_SIZE];
        uniform int num_current_depth_hpx_tiles;

        uniform int prev_depth;
        uniform HEALPixCell hpx_prev_depth[BUFFER_TEX_SIZE];
        uniform int num_prev_depth_hpx_tiles;

        uniform int next_depth;
        uniform HEALPixCell hpx_next_depth[BUFFER_TEX_SIZE];
        uniform int num_next_depth_hpx_tiles;

        uniform HEALPixCell hpx_zero_depth[BUFFER_ZERO_TEX_SIZE];

        const HEALPixCell no_cell = HEALPixCell(-1, -1, 0.f);
        struct HEALPixCellContrib {
            HEALPixCell cell;
            vec3 color;
        };

        HEALPixCellContrib compute_current_depth_color_from_hips(vec3 pos) {
            vec3 res = hash_with_dxdy(current_depth, pos.zxy);
            float tex_step = 1.f / float(BUFFER_TEX_SIZE - 1);

            int tile_idx = int(res.x);
            vec2 uv = res.zy;

            for(int i = 0; i < num_current_depth_hpx_tiles; i++) {
                if (hpx_current_depth[i].idx == tile_idx) {
                    HEALPixCell cell = hpx_current_depth[i];
                    int idx = cell.buf_idx;
                    vec3 color = texture(textures_buffer, vec3(uv, float(idx)*tex_step)).rgb;

                    return HEALPixCellContrib(cell, color);
                }
            }

            return HEALPixCellContrib(no_cell, vec3(0.f));
        }
        HEALPixCellContrib compute_prev_depth_color_from_hips(vec3 pos) {
            vec3 res = hash_with_dxdy(prev_depth, pos.zxy);
            float tex_step = 1.f / float(BUFFER_TEX_SIZE - 1);

            int tile_idx = int(res.x);
            vec2 uv = res.zy;

            for(int i = 0; i < num_prev_depth_hpx_tiles; i++) {
                if (hpx_prev_depth[i].idx == tile_idx) {
                    HEALPixCell cell = hpx_prev_depth[i];
                    int idx = cell.buf_idx;
                    vec3 color = texture(textures_buffer, vec3(uv, float(idx)*tex_step)).rgb;

                    return HEALPixCellContrib(cell, color);
                }
            }

            return HEALPixCellContrib(no_cell, vec3(0.f));
        }
        HEALPixCellContrib compute_next_depth_color_from_hips(vec3 pos) {
            vec3 res = hash_with_dxdy(next_depth, pos.zxy);
            float tex_step = 1.f / float(BUFFER_TEX_SIZE - 1);

            int tile_idx = int(res.x);
            vec2 uv = res.zy;

            for(int i = 0; i < num_next_depth_hpx_tiles; i++) {
                if (hpx_next_depth[i].idx == tile_idx) {
                    HEALPixCell cell = hpx_next_depth[i];
                    int idx = cell.buf_idx;
                    vec3 color = texture(textures_buffer, vec3(uv, float(idx)*tex_step)).rgb;

                    return HEALPixCellContrib(cell, color);
                }
            }

            return HEALPixCellContrib(no_cell, vec3(0.f));
        }
        HEALPixCellContrib compute_zero_depth_color_from_hips(vec3 pos) {
            vec3 res = hash_with_dxdy(0, pos.zxy);
            float tex_step = 1.f / float(BUFFER_ZERO_TEX_SIZE - 1);

            int tile_idx = int(res.x);
            vec2 uv = res.zy;

            for(int i = 0; i < 12; i++) {
                if (hpx_zero_depth[i].idx == tile_idx) {
                    HEALPixCell cell = hpx_zero_depth[i];
                    int idx = cell.buf_idx;
                    vec3 color = texture(textures_zero_depth_buffer, vec3(uv, float(idx)*tex_step)).rgb;

                    return HEALPixCellContrib(cell, color);
                }
            }

            // code unreachable
            return HEALPixCellContrib(no_cell, vec3(0.f));
        }

        const float duration = 500.f; // 1000 ms
        uniform float current_time; // current time in ms

        void main() {
            vec3 frag_pos = normalize(out_vert_pos);
            // Get the HEALPix cell idx and the uv in the texture
            HEALPixCellContrib current_cell = compute_current_depth_color_from_hips(frag_pos);
            float alpha = 0.f;
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
                if (current_depth < 9) {
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
            }
            out_color = out_color.brg;
            out_frag_color = vec4(out_color, 1.0f);

            if(draw_grid == 1) {
                vec2 lonlat = vec2(atan(frag_pos.x, frag_pos.z), atan(frag_pos.y, sqrt(frag_pos.x*frag_pos.x + frag_pos.z*frag_pos.z)));
                lonlat *= 180.0/PI;

                if(abs(lonlat.y) < 80.0) {
                    lonlat /= vec2(10.0, 10.0);

                    vec2 linePos = fract(lonlat + 0.5) - 0.5;
                    vec2 der = vec2(60.0f) * zoom_factor;
                    der = min(der, der / (abs(lonlat.y) * 0.25f));
                    linePos = max((1.0 - der*abs(linePos)), 0.0);

                    vec4 color_grid = vec4(1.f, 0.f, 0.f, 1.f);
                    out_frag_color = mix(out_frag_color, color_grid, (0.4 * max(linePos.x, linePos.y)));
                }
            }
        }"#
    ));
    console::log_1(&format!("sampler3D shader").into());

    /*let shader_direct_system = Rc::new(Shader::new(&gl,
        r#"#version 300 es
        in vec3 position;
        in vec4 color;

        out vec4 out_vert_color;

        uniform mat4 model;
        uniform mat4 projection;
        uniform mat4 view;

        void main() {
            gl_Position = projection * view *  model * vec4(position, 1.0);
            out_vert_color = color;
        }
        "#,
        r#"#version 300 es
        precision mediump float;

        in vec4 out_vert_color;
        out vec4 out_frag_color;

        void main() {
            out_frag_color = out_vert_color;
        }
        "#,
    ));*/

    // Viewport
    let viewport = Rc::new(RefCell::new(ViewPort::new()));
    let projection = Rc::new(RefCell::new(ProjectionType::Aitoff(Aitoff {})));

    let hips_sphere_mesh = Rc::new(RefCell::new(HiPSSphere::new(gl.clone())));
    let sphere = Rc::new(RefCell::new(Renderable::<HiPSSphere>::new(
        &gl,
        shader_texture.clone(),
        projection.clone(),
        hips_sphere_mesh.clone(),
    )));

    let model_mat = &sphere.as_ref().borrow().get_model_mat();
    //hips_sphere_mesh.borrow_mut().update_field_of_view(gl.clone(), &projection.as_ref().borrow(), &viewport.borrow(), model_mat);

    // Definition of the model matrix
    /*let mut direct_system = Rc::new(
        RefCell::new(
            Renderable::<DirectSystem>::new(&gl, shader_direct_system.clone(), &viewport.as_ref().borrow())
        )
    );*/
    //direct_system.borrow_mut().scale(0.5_f32);

    // Enable the depth test
    //gl.enable(WebGl2RenderingContext::DEPTH_TEST);
    // Enable back face culling
    //gl.enable(WebGl2RenderingContext::CULL_FACE);
    //gl.cull_face(WebGl2RenderingContext::BACK);

    gl.enable(WebGl2RenderingContext::BLEND);
    gl.blend_func(WebGl2RenderingContext::SRC_ALPHA, WebGl2RenderingContext::ONE_MINUS_SRC_ALPHA);

    let f = Rc::new(RefCell::new(None));
    let g = f.clone();

    // Mouse down pression event
    let pressed = Rc::new(Cell::new(false));

    let mouse_clic_x = Rc::new(Cell::new(0_f32));
    let mouse_clic_y = Rc::new(Cell::new(0_f32));

    let start_pos = Rc::new(Cell::new(Vector3::<f32>::new(0_f32, 0_f32, 0_f32)));

    let last_axis = Rc::new(Cell::new(Vector3::<f32>::new(0_f32, 0_f32, 0_f32)));
    let last_dist = Rc::new(Cell::new(0_f32));
    let roll = Rc::new(Cell::new(false));

    let time_last_move = Rc::new(Cell::new(utils::get_current_time() as f32));
    {
        let pressed = pressed.clone();

        let mouse_clic_x = mouse_clic_x.clone();
        let mouse_clic_y = mouse_clic_y.clone();

        let start_pos = start_pos.clone();
        let projection = projection.clone();

        let viewport = viewport.clone();

        let closure = Closure::wrap(Box::new(move |event: web_sys::MouseEvent| {
            console::log_1(&format!("mouse down").into());
            let event_x = event.offset_x() as f32;
            let event_y = event.offset_y() as f32;

            mouse_clic_x.set(event_x);
            mouse_clic_y.set(event_y);

            let (x_screen_homogeous_space, y_screen_homogeous_space) = projection::screen_pixels_to_homogeous(event_x, event_y, &viewport.borrow());
            let result = projection.as_ref().borrow().screen_to_world_space(x_screen_homogeous_space, y_screen_homogeous_space);
            if let Some(pos) = result {
                let pos = pos.normalize();

                start_pos.set(pos);

                pressed.set(true);
            }
        }) as Box<dyn FnMut(_)>);
        canvas.add_event_listener_with_callback("mousedown", closure.as_ref().unchecked_ref())?;
        closure.forget();
    }

    // Mouse up pression event
    {
        let pressed = pressed.clone();

        let closure = Closure::wrap(Box::new(move |_: web_sys::MouseEvent| {
            console::log_1(&format!("mouse up").into());
            pressed.set(false);
        }) as Box<dyn FnMut(_)>);
        canvas.add_event_listener_with_callback("mouseup", closure.as_ref().unchecked_ref())?;
        closure.forget();
    }

    // Mouse move event
    {
        let pressed = pressed.clone();

        let start_pos = start_pos.clone();
        let sphere = sphere.clone();

        let mouse_clic_x = mouse_clic_x.clone();
        let mouse_clic_y = mouse_clic_y.clone();

        let last_axis = last_axis.clone();
        let last_dist = last_dist.clone();

        let viewport = viewport.clone();
        let projection = projection.clone();
        let context = gl.clone();
        let hips_sphere_mesh = hips_sphere_mesh.clone();

        let time_last_move = time_last_move.clone();
        let roll = roll.clone();

        let closure = Closure::wrap(Box::new(move |event: web_sys::MouseEvent| {
            if pressed.get() {
                let event_x = event.offset_x() as f32;
                let event_y = event.offset_y() as f32;
                let model_mat = &sphere.as_ref().borrow().get_model_mat();

                let (x_screen_homogeous_space, y_screen_homogeous_space) = projection::screen_pixels_to_homogeous(event_x, event_y, &viewport.borrow());
                let result = projection.as_ref().borrow().screen_to_world_space(x_screen_homogeous_space, y_screen_homogeous_space);
                if let Some(pos) = result {
                    let pos = pos.normalize();

                    if event_x != mouse_clic_x.get() || event_y != mouse_clic_y.get() {
                        let start_pos_rotated = model_mat * cgmath::Vector4::<f32>::new(
                            start_pos.get().x,
                            start_pos.get().y,
                            start_pos.get().z,
                            1_f32
                        );

                        let start_pos_rotated = cgmath::Vector3::<f32>::new(start_pos_rotated.x, start_pos_rotated.y, start_pos_rotated.z);
                        
                        let pos_rotated = model_mat * cgmath::Vector4::<f32>::new(
                            pos.x,
                            pos.y,
                            pos.z,
                            1_f32
                        );

                        let pos_rotated = cgmath::Vector3::<f32>::new(pos_rotated.x, pos_rotated.y, pos_rotated.z);

                        console::log_1(&format!("REAL pos clic {:?}", start_pos_rotated).into());

                        let mut axis = start_pos_rotated.cross(pos_rotated);
                        let dist = math::angular_distance_xyz(start_pos_rotated, pos_rotated);

                        axis = axis.normalize();
                        sphere.borrow_mut().apply_rotation(-axis, cgmath::Rad(dist));
                        time_last_move.set(utils::get_current_time() as f32);
                        roll.set(true);

                        mouse_clic_x.set(event_x);
                        mouse_clic_y.set(event_y);

                        start_pos.set(pos);

                        last_axis.set(axis);
                        last_dist.set(dist);

                        // update the fov
                        hips_sphere_mesh.borrow_mut().update_field_of_view(context.clone(), &projection.as_ref().borrow(), &viewport.borrow(), model_mat, false);
                    }
                }
            }
        }) as Box<dyn FnMut(_)>);

        canvas.add_event_listener_with_callback("mousemove", closure.as_ref().unchecked_ref())?;
        closure.forget();
    }
    // Mouse wheel event
    {
        let context = gl.clone();
        let sphere = sphere.clone();
        let viewport = viewport.clone();

        let hips_sphere_mesh = hips_sphere_mesh.clone();
        let projection = projection.clone();

        let closure = Closure::wrap(Box::new(move |event: web_sys::WheelEvent| {
            let delta_y = event.delta_y() as f32;

            if delta_y < 0_f32 {
                viewport.borrow_mut().zoom(hips_sphere_mesh.borrow().current_depth);
            } else {
                viewport.borrow_mut().unzoom();
            }

            // update the fov
            let model_mat = &sphere.as_ref().borrow().get_model_mat();
            hips_sphere_mesh.borrow_mut().update_field_of_view(context.clone(), &projection.as_ref().borrow(), &viewport.borrow(), model_mat, true);
        }) as Box<dyn FnMut(_)>);
        canvas.add_event_listener_with_callback("wheel", closure.as_ref().unchecked_ref())?;
        closure.forget();
    }

    *g.borrow_mut() = Some(Closure::wrap(Box::new(move || {
        /*if !pressed.get() && roll.get() {
            let next_dist = compute_speed(time_last_move.get(), last_dist.get() * 0.5_f32);
            if next_dist > 1e-4 {
                sphere.borrow_mut().apply_rotation(-last_axis.get(), cgmath::Rad(next_dist));

                let model_mat = &sphere.as_ref().borrow().get_model_mat();
                hips_sphere_mesh.borrow_mut().update_field_of_view(gl.clone(), &projection.as_ref().borrow(), &viewport.borrow(), model_mat, false);
            }
        } else if pressed.get() {
            if (utils::get_current_time() as f32) - time_last_move.get() > 50_f32 {
                roll.set(false);
            }
        }*/

        // Render the scene
        gl.clear_color(0.08, 0.08, 0.08, 1.0);
        // Clear the color buffer bit
        // Clear the depth buffer bit
        gl.clear(WebGl2RenderingContext::COLOR_BUFFER_BIT | WebGl2RenderingContext::DEPTH_BUFFER_BIT);
        
        sphere.as_ref().borrow().draw(&gl, WebGl2RenderingContext::TRIANGLES, &viewport.as_ref().borrow());
        //direct_system.as_ref().borrow().draw(&gl, WebGl2RenderingContext::LINES, &viewport.as_ref().borrow());

        // Schedule ourself for another requestAnimationFrame callback.
        request_animation_frame(f.as_ref().borrow().as_ref().unwrap());
    }) as Box<dyn FnMut()>));

    request_animation_frame(g.as_ref().borrow().as_ref().unwrap());

    Ok(())
}