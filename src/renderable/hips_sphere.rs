use web_sys::console;

use wasm_bindgen::JsCast;

use js_sys::WebAssembly;

use web_sys::WebGl2RenderingContext;
use web_sys::WebGlBuffer;
use web_sys::WebGlTexture;
use web_sys::WebGlUniformLocation;
use web_sys::WebGlVertexArrayObject;

use crate::renderable::projection;
use crate::renderable::projection::ProjectionType;

use std::rc::Rc;

use crate::renderable::Mesh;
use crate::shader::Shader;

const MAX_NUMBER_TEXTURE: usize = 12;
const NUM_VERTICES_PER_STEP: usize = 70;
const NUM_STEPS: usize = 40;

use std::cell::RefCell;

pub struct HiPSSphere {
    idx_textures: Rc<RefCell<Vec<i32>>>,
    depth: i32,
    textures: Vec<Rc<Option<WebGlTexture>>>,
}

use crate::viewport::ViewPort;
use crate::math;
use healpix;
use itertools_num;
use std::iter;

use crate::texture;
impl HiPSSphere {
    pub fn new(gl: Rc<WebGl2RenderingContext>) -> HiPSSphere {
        let depth = 0;
        let idx_textures = (0..12).collect::<Vec<_>>();
        let mut hips_sphere = HiPSSphere {
            idx_textures: Rc::new(RefCell::new(idx_textures)),
            depth: depth,
            textures: vec![],
        };
        hips_sphere.load_healpix_tile_textures(gl, (0..12).collect::<Vec<_>>());
        hips_sphere
    }

    fn load_healpix_tile_textures(&mut self, gl: Rc<WebGl2RenderingContext>, idx_textures_next: Vec<i32>) {
        // Create the TEXTURE
        // 1. Update the FoV

        // 2. Compute the maximum depth for which there is at most
        // 15 tiles in the FoV
        // Get this list of HEALPix tiles too

        // 3. At this point we have at least 15 tiles selected at a specific depth
        // Now we load the textures
        self.textures.clear(); // TODO: do not clear the tiles that are still needed
        let mut num_tile = 0;
        for tile_idx_to_load in idx_textures_next.iter() {
            let dir_idx = (tile_idx_to_load / 10000) * 10000;

            let mut url = String::from("http://alasky.u-strasbg.fr/DSS/DSSColor/");
            url = url + "Norder" + &self.depth.to_string() + "/";
            url = url + "Dir" + &dir_idx.to_string() + "/";
            url = url + "Npix" + &tile_idx_to_load.to_string() + ".jpg";

            self.textures.push(texture::load(gl.clone(), &url, num_tile, tile_idx_to_load.clone(), self.idx_textures.clone()));
            num_tile += 1;
        }
    }

    pub fn update_field_of_view(&mut self, gl: Rc<WebGl2RenderingContext>, projection: &ProjectionType, viewport: &ViewPort, model: &cgmath::Matrix4<f32>) {
        let num_control_points_width = 4;
        let num_control_points_height = 3;
        let num_control_points = 4 + 2*num_control_points_width + 2*num_control_points_height;

        let mut x_ss = itertools_num::linspace::<f32>(-1., 1., num_control_points_width + 2)
            .collect::<Vec<_>>();

        x_ss.extend(iter::repeat(1_f32).take(num_control_points_height));
        x_ss.extend(itertools_num::linspace::<f32>(1., -1., num_control_points_width + 2));
        x_ss.extend(iter::repeat(-1_f32).take(num_control_points_height));

        let mut y_ss = iter::repeat(-1_f32).take(num_control_points_width + 1)
            .collect::<Vec<_>>();

        y_ss.extend(itertools_num::linspace::<f32>(-1., 1., num_control_points_height + 2));
        y_ss.extend(iter::repeat(1_f32).take(num_control_points_width));
        y_ss.extend(itertools_num::linspace::<f32>(1., -1., num_control_points_height + 2));
        y_ss.pop();

        let zoom_factor = viewport.get_zoom_factor();
        let pos_ws = x_ss.into_iter().zip(y_ss.into_iter())
            .filter_map(|(x_screen_space, y_screen_space)| {
                let (x_hs, y_hs) = (x_screen_space / zoom_factor, y_screen_space / zoom_factor);
                let pos_world_space = projection.screen_to_world_space(x_hs, y_hs);
                pos_world_space
            })
            .collect::<Vec<_>>();
        
        let (depth, healpix_cells) = if pos_ws.len() == num_control_points {
            let vertices = pos_ws.into_iter()
                .map(|pos_world_space| {
                    // Take into account the rotation of the sphere
                    let pos_world_space = model * cgmath::Vector4::<f32>::new(
                        pos_world_space.x,
                        pos_world_space.y,
                        pos_world_space.z,
                        1_f32
                    );

                    let pos_world_space = cgmath::Vector3::<f32>::new(pos_world_space.x, pos_world_space.y, pos_world_space.z);

                    let (ra, dec) = math::xyz_to_radec(pos_world_space);
                    (ra as f64, dec as f64)
                })
                .collect::<Vec<_>>();
            
            let mut depth = 1;
            let mut num_healpix_cells_in_fov = 12;
            let mut healpix_cells_in_fov = (0..12).collect::<Vec<_>>();
            while num_healpix_cells_in_fov <= 12 {
                let moc = healpix::nested::polygon_coverage(depth as u8, &vertices, true);
                let healpix_cells = moc.flat_iter()
                    .map(|hpx_idx_u64| hpx_idx_u64 as i32)
                    .collect::<Vec<_>>();
                
                console::log_1(&format!("current depth {:?}, current healpix_cells {:?}", depth, healpix_cells.len()).into());

                if healpix_cells.len() > 12 {
                    depth -= 1;
                    break;
                }

                depth += 1;
                num_healpix_cells_in_fov = healpix_cells.len();
                healpix_cells_in_fov = healpix_cells;
            }

            (depth, healpix_cells_in_fov)
        } else {
            let depth = 0;
            let healpix_cells_in_fov = (0..12).collect::<Vec<_>>(); 
            (depth, healpix_cells_in_fov)
        };
        console::log_1(&format!("depth {:?}, healpix_cells {:?}", depth, healpix_cells).into());

        //self.idx_textures = healpix_cells;
        
        if depth != self.depth || self.depth != 0 {
            self.depth = depth;
            self.load_healpix_tile_textures(gl, healpix_cells);
        }
    }
}

impl Mesh for HiPSSphere {
    fn create_color_array() -> js_sys::Float32Array {
        unreachable!()
    }

    fn create_uv_array() -> js_sys::Float32Array {
        unreachable!()
    }

    fn create_vertices_array(projection: &ProjectionType) -> js_sys::Float32Array {
        let vertices_screen_space = projection.build_screen_map();

        let vertices_world_space = vertices_screen_space
            .iter()
            .map(|pos_screen_space| {
                // Perform the inverse projection that converts
                // screen position to the 3D space position
                let pos_world_space = projection.screen_to_world_space(
                    pos_screen_space.x, pos_screen_space.y,
                ).unwrap();

                vec![pos_world_space.x, pos_world_space.y, pos_world_space.z]
            })
            .flatten()
            .collect::<Vec<_>>();

        let vertices_world_space_array = {
            let memory_buffer = wasm_bindgen::memory()
                .dyn_into::<WebAssembly::Memory>().unwrap()
                .buffer();
            let vertices_location = vertices_world_space.as_ptr() as u32 / 4;
            js_sys::Float32Array::new(&memory_buffer)
                .subarray(vertices_location, vertices_location + vertices_world_space.len() as u32)
        };

        vertices_world_space_array
    }

    fn create_index_array() -> js_sys::Uint32Array {
        let mut indices = Vec::with_capacity(3 * NUM_VERTICES_PER_STEP * NUM_STEPS);

        for j in 0..NUM_STEPS {
            if j == 0 {
                for i in 1..NUM_VERTICES_PER_STEP {
                    indices.push(0 as u32);
                    indices.push(i as u32);
                    indices.push((i + 1) as u32);
                }
                
                indices.push(0 as u32);
                indices.push(NUM_VERTICES_PER_STEP as u32);
                indices.push(1 as u32);
            } else {
                for i in 0..NUM_VERTICES_PER_STEP {
                    let start_p_idx = (j - 1) * NUM_VERTICES_PER_STEP + i + 1;
                    let next_p_idx = if i + 1 == NUM_VERTICES_PER_STEP {
                        (j - 1) * NUM_VERTICES_PER_STEP + 1
                    } else {
                        (j - 1) * NUM_VERTICES_PER_STEP + i + 2
                    };

                    let start_c_idx = j * NUM_VERTICES_PER_STEP + i + 1;
                    let next_c_idx = if i + 1 == NUM_VERTICES_PER_STEP {
                        j * NUM_VERTICES_PER_STEP + 1
                    } else {
                        j * NUM_VERTICES_PER_STEP + i + 2
                    };

                    // Triangle touching the prec circle
                    indices.push(start_p_idx as u32);
                    indices.push(next_p_idx as u32);
                    indices.push(start_c_idx as u32);
                    // Triangle touching the next circle
                    indices.push(start_c_idx as u32);
                    indices.push(next_p_idx as u32);
                    indices.push(next_c_idx as u32);
                }
            }
        }
        
        let indices_array = {
            let memory_buffer = wasm_bindgen::memory()
                .dyn_into::<WebAssembly::Memory>().unwrap()
                .buffer();
            let indices_location = indices.as_ptr() as u32 / 4;
            js_sys::Uint32Array::new(&memory_buffer)
                .subarray(indices_location, indices_location + indices.len() as u32)
        };

        indices_array
    }

    fn create_buffers(gl: &WebGl2RenderingContext, projection: &ProjectionType) -> (Box<[(u32, i32, WebGlBuffer)]>, i32, WebGlVertexArrayObject) {
        let vao = gl.create_vertex_array()
            .ok_or("failed to create the vertex array buffer")
            .unwrap();
        gl.bind_vertex_array(Some(&vao));

        let vertices_world_space = Self::create_vertices_array(projection);

        // VERTEX buffer creation
        let vertex_buffer = gl.create_buffer()
            .ok_or("failed to create buffer")
            .unwrap();
        gl.bind_buffer(WebGl2RenderingContext::ARRAY_BUFFER, Some(&vertex_buffer));
        // Pass the vertices data to the buffer
        gl.buffer_data_with_array_buffer_view(
            WebGl2RenderingContext::ARRAY_BUFFER,
            &vertices_world_space,
            WebGl2RenderingContext::STATIC_DRAW,
        );

        // Unbind the buffer
        gl.bind_buffer(WebGl2RenderingContext::ARRAY_BUFFER, None);
        
        let vertices_screen_space = {
            let vertices_screen = projection
                .build_screen_map()
                .iter()
                .map(|pos_screen_space| {
                    vec![pos_screen_space.x, pos_screen_space.y]
                })
                .flatten()
                .collect::<Vec<_>>();

            let memory_buffer = wasm_bindgen::memory()
                .dyn_into::<WebAssembly::Memory>().unwrap()
                .buffer();
            let vertices_location = vertices_screen.as_ptr() as u32 / 4;
            js_sys::Float32Array::new(&memory_buffer)
                .subarray(vertices_location, vertices_location + vertices_screen.len() as u32)
        };

        // SCREEN VERTICES buffer creation
        let screen_vertices_buffer = gl.create_buffer()
            .ok_or("failed to create buffer")
            .unwrap();
        gl.bind_buffer(WebGl2RenderingContext::ARRAY_BUFFER, Some(&screen_vertices_buffer));
        // Pass the vertices data to the buffer
        gl.buffer_data_with_array_buffer_view(
            WebGl2RenderingContext::ARRAY_BUFFER,
            &vertices_screen_space,
            WebGl2RenderingContext::STATIC_DRAW,
        );

        // Unbind the buffer
        gl.bind_buffer(WebGl2RenderingContext::ARRAY_BUFFER, None);
        
        // INDEX buffer creation
        let indices_array = Self::create_index_array();
        let index_buffer = gl.create_buffer()
            .ok_or("failed to create buffer")
            .unwrap();
        gl.bind_buffer(WebGl2RenderingContext::ELEMENT_ARRAY_BUFFER, Some(&index_buffer));
        // Pass the indices data to the buffer
        gl.buffer_data_with_array_buffer_view(
            WebGl2RenderingContext::ELEMENT_ARRAY_BUFFER,
            &indices_array,
            WebGl2RenderingContext::STATIC_DRAW,
        );

        // Unbind the buffer
        //gl.bind_buffer(WebGl2RenderingContext::ELEMENT_ARRAY_BUFFER, None);
        let num_indices = indices_array.length() as i32;
        console::log_1(&format!("num indices {:?}", num_indices).into());
        (Box::new([
            (WebGl2RenderingContext::ARRAY_BUFFER, 3, vertex_buffer),
            (WebGl2RenderingContext::ARRAY_BUFFER, 2, screen_vertices_buffer),
            (WebGl2RenderingContext::ELEMENT_ARRAY_BUFFER, 1, index_buffer),
        ]),
        num_indices,
        vao)
    }

    fn init_uniforms(gl: &WebGl2RenderingContext, shader: &Shader) -> Box<[WebGlUniformLocation]> {
        // Enable (lon, lat) grid
        let grid_uniform = shader.get_uniform_location(gl, "draw_grid").unwrap();

        // Get the attribute location of the matrices from the Vertex shader
        let textures_data_uniform = shader.get_uniform_location(gl, "texture_hips_tile").unwrap();
        // Array storing the visible HEALPix indexes
        let healpix_idx_uniform = shader.get_uniform_location(gl, "idx_textures").unwrap();
        // Depth of the visible HEALPix cells
        let healpix_depth_uniform = shader.get_uniform_location(gl, "depth").unwrap();

        Box::new([
            textures_data_uniform,
            healpix_depth_uniform,
            healpix_idx_uniform,
            grid_uniform
        ])
    }

    fn send_uniforms(
        &self,
        gl: &WebGl2RenderingContext,
        uniform_locations: &Box<[WebGlUniformLocation]>,
    ) {
        // Send textures
        let sampler_idxs = ((0 as i32)..(MAX_NUMBER_TEXTURE as i32)).collect::<Vec<_>>();
        gl.uniform1iv_with_i32_array(Some(uniform_locations[0].as_ref()), &sampler_idxs);

        // Send current depth
        gl.uniform1i(Some(uniform_locations[1].as_ref()), self.depth);

        // Send the HEALPix cell indexes
        gl.uniform1iv_with_i32_array(Some(uniform_locations[2].as_ref()), &self.idx_textures.borrow());

        // Send grid enable
        gl.uniform1i(Some(uniform_locations[3].as_ref()), 1);
    }
}