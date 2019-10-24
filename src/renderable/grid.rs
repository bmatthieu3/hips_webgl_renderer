struct Grid;

use crate::math;
use crate::renderable::buffers::buffer_data::BufferData;

impl Grid {
    fn new() -> Grid {
        Grid {}
    }

    /*fn create_iso_lon() -> BufferData<f32> {
        let num_points = 10;
        let screen_space_pos = (0..num_points).into_iter()
            .map(|i| {
                let theta = (2_f32 * 3.14_f32 * (i as f32)) / (num_points as f32);
                let delta = 0_f32;

                let pos = math::radec_to_xyz(theta, delta);
                // TODO world space to screen space
                let screen_space_pos = pos;
                screen_space_pos
            })
            .flatten()
            .collect::<Vec<_>>();
        screen_space_pos.try_into().unwrap()
    }*/
}

use crate::renderable::Mesh;
use std::rc::Rc;
use web_sys::WebGl2RenderingContext;
/*
impl Mesh for Grid {
    fn create_buffers(gl: Rc<WebGl2RenderingContext>, projection: &ProjectionType) -> VertexArrayObject {
        let mut vertex_array_object = VertexArrayObject::new(gl.clone());
        vertex_array_object.bind();

        // ARRAY buffer creation
        let vertices_data = Self::create_vertices_array(projection);
        let array_buffer = ArrayBuffer::new(
            gl.clone(),
            5 * std::mem::size_of::<f32>(),
            &[2, 3],
            &[0 * std::mem::size_of::<f32>(), 2 * std::mem::size_of::<f32>()],
            vertices_data
        );

        // ELEMENT ARRAY buffer creation
        let indexes_data = Self::create_index_array();
        let indexes_buffer = ElementArrayBuffer::new(
            gl,
            indexes_data
        );

        vertex_array_object.set_array_buffer(array_buffer);
        vertex_array_object.set_element_array_buffer(indexes_buffer);

        vertex_array_object.unbind();
        // Unbind the buffer
        //gl.bind_buffer(WebGl2RenderingContext::ELEMENT_ARRAY_BUFFER, None);
        vertex_array_object
    }

    fn send_uniforms(&self, gl: &WebGl2RenderingContext, shader: &Shader) {
        // Send grid enable
        //let location_enable_grid = shader.get_uniform_location(gl, "draw_grid").unwrap();
        //gl.uniform1i(Some(&location_enable_grid), 1);
        // Send max depth of the current HiPS
        let location_max_depth = shader.get_uniform_location(gl, "max_depth").unwrap();
        gl.uniform1i(Some(&location_max_depth), MAX_DEPTH);
        // Send sampler 3D
        // textures buffer
        let location_textures_buf = shader.get_uniform_location(gl, "textures_buffer").unwrap();
        gl.active_texture(WebGl2RenderingContext::TEXTURE0);
        gl.bind_texture(WebGl2RenderingContext::TEXTURE_3D, self.buffer_textures.webgl_texture0.as_ref());
        gl.uniform1i(Some(&location_textures_buf), 0);
        // base cell textures
        let location_textures_zero_depth_buf = shader.get_uniform_location(gl, "textures_zero_depth_buffer").unwrap();
        gl.active_texture(WebGl2RenderingContext::TEXTURE1);
        gl.bind_texture(WebGl2RenderingContext::TEXTURE_3D, self.buffer_textures.webgl_texture1.as_ref());
        gl.uniform1i(Some(&location_textures_zero_depth_buf), 1);
        let hpx_zero_depth_tiles = self.buffer_textures.get_zero_depth_tiles();
        for (i, hpx) in hpx_zero_depth_tiles.iter().enumerate() {
            let mut name = String::from("hpx_zero_depth");
            name += "[";
            name += &i.to_string();
            name += "].";

            let location_hpx_idx = shader.get_uniform_location(gl, &(name.clone() + "idx")).unwrap();
            gl.uniform1i(Some(location_hpx_idx.as_ref()), hpx.idx);

            let location_buf_idx = shader.get_uniform_location(gl, &(name.clone() + "buf_idx")).unwrap();
            gl.uniform1i(Some(location_buf_idx.as_ref()), hpx.buf_idx);

            let location_time_received = shader.get_uniform_location(gl, &(name + "time_received")).unwrap();
            gl.uniform1f(Some(location_time_received.as_ref()), hpx.time_received);
        }

        // Send current time in ms
        let location_current_time = shader.get_uniform_location(gl, "current_time").unwrap();
        gl.uniform1f(Some(&location_current_time), utils::get_current_time());

        let hpx_tiles = self.buffer_textures.get_tiles(self.current_depth);
        for (i, hpx) in hpx_tiles.iter().enumerate() {
            let mut name = String::from("hpx_current_depth");
            name += "[";
            name += &i.to_string();
            name += "].";

            let location_hpx_idx = shader.get_uniform_location(gl, &(name.clone() + "idx")).unwrap();
            gl.uniform1i(Some(location_hpx_idx.as_ref()), hpx.idx);

            let location_buf_idx = shader.get_uniform_location(gl, &(name.clone() + "buf_idx")).unwrap();
            gl.uniform1i(Some(location_buf_idx.as_ref()), hpx.buf_idx);

            let location_time_received = shader.get_uniform_location(gl, &(name + "time_received")).unwrap();
            gl.uniform1f(Some(location_time_received.as_ref()), hpx.time_received);
        }

        // Send number of HEALPix cells in the buffer whose depth equals current_depth
        let location_num_current_depth_tiles = shader.get_uniform_location(gl, "num_current_depth_hpx_tiles").unwrap();
        gl.uniform1i(Some(&location_num_current_depth_tiles), hpx_tiles.len() as i32);

        // Send current depth
        let location_current_depth = shader.get_uniform_location(gl, "current_depth").unwrap();
        gl.uniform1i(Some(&location_current_depth), self.current_depth);

        // PREVIOUS DEPTH TILES
        if self.current_depth > 0 {
            let prev_depth = self.current_depth - 1;

            let hpx_tiles = self.buffer_textures.get_tiles(prev_depth);
            for (i, hpx) in hpx_tiles.iter().enumerate() {
                let mut name = String::from("hpx_prev_depth");
                name += "[";
                name += &i.to_string();
                name += "].";

                let location_hpx_idx = shader.get_uniform_location(gl, &(name.clone() + "idx")).unwrap();
                gl.uniform1i(Some(location_hpx_idx.as_ref()), hpx.idx);

                let location_buf_idx = shader.get_uniform_location(gl, &(name.clone() + "buf_idx")).unwrap();
                gl.uniform1i(Some(location_buf_idx.as_ref()), hpx.buf_idx);

                let location_time_received = shader.get_uniform_location(gl, &(name + "time_received")).unwrap();
                gl.uniform1f(Some(location_time_received.as_ref()), hpx.time_received);
            }

            // Send number of HEALPix cells in the buffer whose depth equals current_depth
            let location_num_prev_depth_tiles = shader.get_uniform_location(gl, "num_prev_depth_hpx_tiles").unwrap();
            gl.uniform1i(Some(&location_num_prev_depth_tiles), hpx_tiles.len() as i32);

            // Send current depth
            let location_prev_depth = shader.get_uniform_location(gl, "prev_depth").unwrap();
            gl.uniform1i(Some(&location_prev_depth), prev_depth);
        }
        // NEXT DEPTH TILES
        if self.current_depth < MAX_DEPTH {
            let next_depth = self.current_depth + 1;

            let hpx_tiles = self.buffer_textures.get_tiles(next_depth);
            for (i, hpx) in hpx_tiles.iter().enumerate() {
                let mut name = String::from("hpx_next_depth");
                name += "[";
                name += &i.to_string();
                name += "].";

                let location_hpx_idx = shader.get_uniform_location(gl, &(name.clone() + "idx")).unwrap();
                gl.uniform1i(Some(location_hpx_idx.as_ref()), hpx.idx);

                let location_buf_idx = shader.get_uniform_location(gl, &(name.clone() + "buf_idx")).unwrap();
                gl.uniform1i(Some(location_buf_idx.as_ref()), hpx.buf_idx);

                let location_time_received = shader.get_uniform_location(gl, &(name + "time_received")).unwrap();
                gl.uniform1f(Some(location_time_received.as_ref()), hpx.time_received);
            }

            // Send number of HEALPix cells in the buffer whose depth equals current_depth
            let location_num_next_depth_tiles = shader.get_uniform_location(gl, "num_next_depth_hpx_tiles").unwrap();
            gl.uniform1i(Some(&location_num_next_depth_tiles), hpx_tiles.len() as i32);

            // Send current depth
            let location_next_depth = shader.get_uniform_location(gl, "next_depth").unwrap();
            gl.uniform1i(Some(&location_next_depth), next_depth);
        }
    }
}*/