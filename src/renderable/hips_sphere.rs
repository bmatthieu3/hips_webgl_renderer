use web_sys::console;
use crate::shader::ShaderBound;

pub const NUM_VERTICES_PER_STEP: usize = 50;
pub const NUM_STEPS: usize = 20;

use crate::buffer_tiles::BufferTiles;

use std::sync::Arc;
use std::sync::atomic::AtomicU8;
lazy_static! {
    pub static ref DEPTH: Arc<AtomicU8> = Arc::new(AtomicU8::new(0));
}


use crate::viewport::ViewPort;
use crate::WebGl2Context;

use crate::projection::Projection;
use crate::event_manager::EventManager;
pub trait RenderingMode {
    fn new(gl: &WebGl2Context, viewport: &ViewPort, shaders: &mut ShaderManager) -> Self;

    fn update<P: Projection>(
        &mut self,
        buffer: &mut BufferTiles,
        viewport: &ViewPort,
        events: &EventManager,
    );

    fn draw<P: Projection>(&self, gl: &WebGl2Context, shader: &ShaderBound);

    //fn send_to_shader(buffer: &BufferTiles, shader: &Shader);
}

use crate::renderable::RayTracer;
use crate::renderable::Rasterizer;
use crate::projection::*;
pub struct HiPSSphere {
    // Some information about the HiPS
    pub config: HiPSConfig,
    
    // The buffer responsible for: 
    // * Performing the async request of tiles
    // * Storing the most recently asked texture tiles
    // * Sending them to the GPU
    buffer: BufferTiles,

    raster: Rasterizer,
    raytracer: RayTracer,

    gl: WebGl2Context,

    depth: u8,
}

use crate::buffer_tiles::HiPSConfig;
use crate::shader::ShaderManager;
impl HiPSSphere {
    pub fn new(gl: &WebGl2Context, viewport: &ViewPort, config: HiPSConfig, shaders: &mut ShaderManager) -> Self {
        let buffer = BufferTiles::new(gl, &config);
        let gl = gl.clone();

        let raster = Rasterizer::new(&gl, viewport, shaders);
        let raytracer = RayTracer::new(&gl, viewport, shaders);

        let depth = 0;

        HiPSSphere {
            config,
            buffer,

            raster,
            raytracer,

            gl,

            depth,
        }
    }

    pub fn set_hips_config<P: Projection>(&mut self, config: HiPSConfig, viewport: &mut ViewPort) {        
        // Tell the viewport the config has changed
        viewport.set_max_depth(&config);
        // Clear the buffer
        self.buffer.reset(&config);
        // Erase the old config with the new one
        self.config = config;
    
        self.request_tiles(viewport);
    }

    pub fn request_tiles(&mut self, viewport: &ViewPort) {
        let field_of_view = viewport.field_of_view();
        let cells = field_of_view.new_healpix_cells();
        
        let depth = field_of_view.current_depth();
        self.depth = depth;

        self.buffer.request_tiles(cells, &self.config);
    }

    pub fn update<P: Projection>(&mut self, viewport: &ViewPort, events: &EventManager, shaders: &ShaderManager) {
        /*if viewport.screen_inside_of_projection::<P>() {
        
        } else {

        }*/

        self.raster.update::<P>(
            &mut self.buffer,
            viewport,
            events,
        );
    }

    pub fn draw<P: Projection>(
        &self,
        gl: &WebGl2Context,
        shaders: &ShaderManager,
        viewport: &ViewPort,
    ) {
        /*// Big field of view, check the projections
        match P::name() {
            "Aitoff" => {
                let shader = RayTracer::get_shader(shaders); // TODO: The same shader for all projection
                shader.bind(gl);

                self.send_global_uniforms(gl, shader, viewport);
                self.aitoff_perpixel.draw(gl, shader)
            },
            "MollWeide" => {
                let shader = RayTracing::get_shader(shaders); // TODO: The same shader for all projection
                shader.bind(gl);

                self.send_global_uniforms(gl, shader, viewport);
                self.moll_perpixel.draw(gl, shader)
            },
            "Arc" => {
                let shader = RayTracing::get_shader(shaders); // TODO: The same shader for all projection
                shader.bind(gl);

                self.send_global_uniforms(gl, shader, viewport);
                self.arc_perpixel.draw(gl, shader)
            },
            "Mercator" => {
                let shader = RayTracing::get_shader(shaders); // TODO: The same shader for all projection
                shader.bind(gl);

                self.send_global_uniforms(gl, shader, viewport);
                self.mercator_perpixel.draw(gl, shader)
            },
            // By construction, we are in orthographic projection when we have zoomed or the ortho projection selected
            "Orthographic" => {
                let shader = Rasterization::get_shader(shaders); // TODO: The same shader for all projection
                shader.bind(gl);

                self.send_global_uniforms(gl, shader, viewport);
                self.raster.draw(gl, shader)
            },
            _ => panic!("Not all projection are handled!"),
        }*/

        /*if viewport.screen_inside_of_projection::<P>() {
        
        } else {

        }*/
        // TEXTURES TILES BUFFER
        //Rasterizer::send_to_shader(&self.buffer, shader);

        // Rasterization
        let shader = Rasterizer::get_shader::<P>(shaders);
        let shader_bound = shader.bind(gl);
            shader_bound.attach_uniforms_from(viewport)
            .attach_uniforms_from(&self.config)
            .attach_uniforms_from(&self.buffer)
            .attach_uniform("model", viewport.get_model_mat())
            .attach_uniform("current_time", &utils::get_current_time())
            .attach_uniform("current_depth", &(self.depth as i32));

        self.raster.draw::<P>(gl, &shader_bound);
    }
}

use crate::utils;

use crate::renderable::DisableDrawing;
impl DisableDrawing for HiPSSphere {
    fn disable(&mut self, _: &ViewPort) {
    }
}