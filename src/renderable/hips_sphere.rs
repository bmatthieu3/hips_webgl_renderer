use web_sys::console;
use crate::shader::ShaderBound;

pub const NUM_VERTICES_PER_STEP: usize = 50;
pub const NUM_STEPS: usize = 20;

use crate::buffer::BufferTextures;

use std::sync::Arc;
use std::sync::atomic::AtomicU8;
lazy_static! {
    pub static ref DEPTH: Arc<AtomicU8> = Arc::new(AtomicU8::new(0));
}

use crate::event_manager::Event;
use crate::buffer::Texture;
use crate::healpix_cell::HEALPixCell;
pub struct TextureState {
    pub starting_texture: Texture,
    pub ending_texture: Texture,
}

impl TextureState {
    fn new(starting_texture: Texture, ending_texture: Texture) -> TextureState {
        TextureState {
            starting_texture,
            ending_texture
        }
    }
}

use std::collections::HashMap;
pub struct TextureStates(HashMap<HEALPixCell, TextureState>);

impl TextureStates {
    fn new(cap: usize) -> TextureStates {
        let states = HashMap::with_capacity(cap);

        TextureStates(states)
    }
}

impl core::ops::Deref for TextureStates {
    type Target = HashMap<HEALPixCell, TextureState>;

    fn deref (self: &'_ Self) -> &'_ Self::Target {
        &self.0
    }
}
impl core::ops::DerefMut for TextureStates {
    fn deref_mut (self: &'_  mut Self) -> &'_ mut Self::Target {
        &mut self.0
    }
}

use crate::healpix_cell::SphereSubdivided;
pub trait UpdateTextureBufferEvent: Event {
    // Returns:
    // * The UV of the starting tile in the global 4096x4096 texture
    // * The UV of the ending tile in the global 4096x4096 texture
    // * the blending factor between the two tiles in the texture
    fn update_texture_buffer<P: Projection>(
        // The buffer that will be modified due to the need of specific tile textures by the GPU
        buffer: &BufferTextures,
        // The HEALPix cells located in the FOV
        viewport: &ViewPort,
    ) -> TextureStates;

    fn num_subdivision<P: Projection>(cell: &HEALPixCell, sphere_sub: &SphereSubdivided) -> u8;
}

use crate::event_manager::{
 MouseMove,
 MouseWheelDown,
 MouseWheelUp
};

impl UpdateTextureBufferEvent for MouseMove  {
    // Returns:
    // * The UV of the starting tile in the global 4096x4096 texture
    // * The UV of the ending tile in the global 4096x4096 texture
    // * the blending factor between the two tiles in the texture
    fn update_texture_buffer<P: Projection>(
        // The buffer that will be modified due to the need of specific tile textures by the GPU
        buffer: &BufferTextures,
        // The HEALPix cells located in the FOV
        viewport: &ViewPort,
    ) -> TextureStates {
        let cells_fov = viewport.field_of_view()
            .healpix_cells();

        let mut textures = TextureStates::new(cells_fov.len());

        for &cell in cells_fov {
            if buffer.contains(&cell) {
                let parent_cell = buffer.get_nearest_parent(&cell);

                let ending_cell_in_tex = buffer.get_texture(&cell);
                let starting_cell_in_tex = buffer.get_texture(&parent_cell);

                textures.insert(cell, TextureState::new(starting_cell_in_tex, ending_cell_in_tex));
            } else {
                let parent_cell = buffer.get_nearest_parent(&cell);
                let grand_parent_cell = buffer.get_nearest_parent(&parent_cell);

                let ending_cell_in_tex = buffer.get_texture(&parent_cell);
                let starting_cell_in_tex = buffer.get_texture(&grand_parent_cell);

                textures.insert(cell, TextureState::new(starting_cell_in_tex, ending_cell_in_tex));
            }
        }

        textures
    }
    fn num_subdivision<P: Projection>(cell: &HEALPixCell, sphere_sub: &SphereSubdivided) -> u8 {
        sphere_sub.get_num_subdivide::<P>(cell)
    }
}

impl UpdateTextureBufferEvent for MouseWheelUp {
    // Returns:
    // * The UV of the starting tile in the global 4096x4096 texture
    // * The UV of the ending tile in the global 4096x4096 texture
    // * the blending factor between the two tiles in the texture
    fn update_texture_buffer<P: Projection>(
        // The buffer that will be modified due to the need of specific tile textures by the GPU
        buffer: &BufferTextures,
        // The HEALPix cells located in the FOV
        viewport: &ViewPort,
    ) -> TextureStates {
        let cells_fov = viewport.field_of_view()
            .healpix_cells();

        let mut textures = TextureStates::new(cells_fov.len());

        for &cell in cells_fov {
            if buffer.contains(&cell) {
                let parent_cell = buffer.get_nearest_parent(&cell);

                let ending_cell_in_tex = buffer.get_texture(&cell);
                let starting_cell_in_tex = buffer.get_texture(&parent_cell);

                textures.insert(cell, TextureState::new(starting_cell_in_tex, ending_cell_in_tex));
            } else {
                let parent_cell = buffer.get_nearest_parent(&cell);
                let grand_parent_cell = buffer.get_nearest_parent(&parent_cell);

                let ending_cell_in_tex = buffer.get_texture(&parent_cell);
                let starting_cell_in_tex = buffer.get_texture(&grand_parent_cell);

                textures.insert(cell, TextureState::new(starting_cell_in_tex, ending_cell_in_tex));
            }
        }

        textures
    }

    fn num_subdivision<P: Projection>(cell: &HEALPixCell, sphere_sub: &SphereSubdivided) -> u8 {
        sphere_sub.get_num_subdivide::<P>(cell)
    }
}

impl UpdateTextureBufferEvent for MouseWheelDown {
    // Returns:
    // * The UV of the starting tile in the global 4096x4096 texture
    // * The UV of the ending tile in the global 4096x4096 texture
    // * the blending factor between the two tiles in the texture
    fn update_texture_buffer<P: Projection>(
        // The buffer that will be modified due to the need of specific tile textures by the GPU
        buffer: &BufferTextures,
        // The HEALPix cells located in the FOV
        viewport: &ViewPort,
    ) -> TextureStates {
        let depth_plus_two = viewport.field_of_view()
            .current_depth() + 2;

        let cells_fov = viewport.field_of_view()
            .get_cells_in_fov::<P>(depth_plus_two);

        let mut textures = TextureStates::new(cells_fov.len());

        for cell in cells_fov {
            let parent_cell = cell.parent();
            let grand_parent_cell = parent_cell.parent();

            if buffer.contains(&grand_parent_cell) {
                let starting_cell = if buffer.contains(&cell) {
                    cell
                } else {
                    buffer.get_nearest_parent(&cell)
                };
                let starting_cell_in_tex = buffer.get_texture(&starting_cell);
                let ending_cell_in_tex = buffer.get_texture(&grand_parent_cell);

                textures.insert(cell, TextureState::new(starting_cell_in_tex, ending_cell_in_tex));
            } else {
                let ending_cell = if buffer.contains(&cell) {
                    cell
                } else {
                    buffer.get_nearest_parent(&cell)
                };

                let starting_cell = buffer.get_nearest_parent(&ending_cell);

                let starting_cell_in_tex = buffer.get_texture(&starting_cell);
                let ending_cell_in_tex = buffer.get_texture(&ending_cell);

                textures.insert(cell, TextureState::new(starting_cell_in_tex, ending_cell_in_tex));
            }
        }

        textures
    }

    fn num_subdivision<P: Projection>(cell: &HEALPixCell, sphere_sub: &SphereSubdivided) -> u8 {
        let num_subdivision = sphere_sub.get_num_subdivide::<P>(cell);
        if num_subdivision <= 2 {
            0
        } else {
            num_subdivision - 2
        }
    }
}

use crate::viewport::ViewPort;
use crate::WebGl2Context;

use crate::projection::Projection;
use crate::event_manager::EventManager;
pub trait RenderingMode {
    fn new(gl: &WebGl2Context, viewport: &ViewPort, shaders: &mut ShaderManager) -> Self;

    /*fn update<P: Projection>(
        &mut self,
        buffer: &mut BufferTiles,
        viewport: &ViewPort,
        events: &EventManager,
    );*/

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
    buffer: BufferTextures,

    raster: Rasterizer,
    raytracer: RayTracer,

    gl: WebGl2Context,

    depth: u8,
}

use cgmath::Deg;

use crate::buffer::HiPSConfig;
use crate::shader::ShaderManager;
use crate::shaders;
use crate::viewport::LastAction;
impl HiPSSphere {
    pub fn new(gl: &WebGl2Context, viewport: &ViewPort, config: HiPSConfig, shaders: &mut ShaderManager) -> Self {
        let buffer = BufferTextures::new(gl, &config);
        console::log_1(&format!("config: {:?}", config).into());

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
        viewport.set_hips_config(&config);
        console::log_1(&format!("config: {:?}", config).into());

        // Clear the buffer
        self.buffer.reset(&config);
        // Erase the old config with the new one
        self.config = config;
    

        self.request_tiles(viewport);
    }

    pub fn request_tiles(&mut self, viewport: &ViewPort) {
        let field_of_view = viewport.field_of_view();
        
        let depth = field_of_view.current_depth();
        self.depth = depth;

        let cells = field_of_view.new_healpix_cells();
        self.buffer.request_textures(cells);
    }

    pub fn update<P: Projection>(&mut self, viewport: &ViewPort, events: &EventManager, shaders: &ShaderManager) {
        //if self.buffer.ready() {
            /*if let Some(_) = events.get::<MouseMove>() {
                console::log_1(&format!("mouse move").into());
            }

            if let Some(_) = events.get::<MouseWheelUp>() {
                console::log_1(&format!("zoom").into());
            }

            if let Some(_) = events.get::<MouseWheelDown>() {
                console::log_1(&format!("unzoom").into());
            }*/

            let aperture: Deg<f32> = viewport
                .get_aperture()
                .into();

            // A tile has been received
            // Signals a new frame to the buffer
            //self.buffer.signals_new_frame();

            //self.buffer.poll_textures();

            let new_cells_in_fov = viewport.field_of_view().is_new_cells();

            let last_user_action = viewport.get_last_action();
            match last_user_action {
                LastAction::Unzooming => {
                    if /*self.buffer.tile_texture_remaining_processes() ||*/ new_cells_in_fov {
                        let tile_textures = MouseWheelDown::update_texture_buffer::<P>(&mut self.buffer, viewport);
                        self.raster.update_vertex_array_object::<P, MouseWheelDown>(&tile_textures);
                    }
                },
                LastAction::Zooming => {
                    if /*self.buffer.tile_texture_remaining_processes() ||*/ new_cells_in_fov {
                        let tile_textures = MouseWheelUp::update_texture_buffer::<P>(&mut self.buffer, viewport);
                        self.raster.update_vertex_array_object::<P, MouseWheelUp>(&tile_textures);
                    }
                },
                LastAction::Moving => {
                    if /*self.buffer.tile_texture_remaining_processes() ||*/ new_cells_in_fov {
                        let tile_textures = MouseMove::update_texture_buffer::<P>(&mut self.buffer, viewport);
                        self.raster.update_vertex_array_object::<P, MouseMove>(&tile_textures);
                    }
                }
            }

            //self.buffer.signals_end_frame();
        //}
        /*// if self.buffer.is_ready() {
        if self.buffer.tile_texture_remaining_processes() && self.buffer.is_ready() {
            let aperture: Deg<f32> = viewport
                .get_aperture()
                .into();

            // A tile has been received
            // Signals a new frame to the buffer
            self.buffer.signals_new_frame();

            self.buffer.poll_textures();

            let last_user_action = viewport.get_last_action();
            match last_user_action {
                LastAction::Unzooming => {
                    let tile_textures = MouseWheelDown::update_texture_buffer::<P>(&mut self.buffer, viewport);
                    //if aperture < Deg(110_f32) {
                        self.raster.update_vertex_array_object::<P, MouseWheelDown>(&tile_textures);
                    //}
                },
                LastAction::Zooming => {
                    let tile_textures = MouseWheelUp::update_texture_buffer::<P>(&mut self.buffer, viewport);
                    //if aperture < Deg(110_f32) {
                        self.raster.update_vertex_array_object::<P, MouseWheelUp>(&tile_textures);
                    //}
                },
                LastAction::Moving => {
                    let tile_textures = MouseMove::update_texture_buffer::<P>(&mut self.buffer, viewport);
                    //if aperture < Deg(110_f32) {
                        self.raster.update_vertex_array_object::<P, MouseMove>(&tile_textures);
                    //}
                }
            }

            self.buffer.signals_end_frame();
        }*/
        


        //if aperture < Deg(110_f32) {
            // Rasterization


        //}
    }

    pub fn draw<P: Projection>(
        &self,
        gl: &WebGl2Context,
        shaders: &ShaderManager,
        viewport: &ViewPort,
    ) {
        let aperture: Deg<f32> = viewport
            .get_aperture()
            .into();

        if aperture < Deg(110_f32) {
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
        } else {
            // Ray-tracing
            let shader = shaders.get::<shaders::Raytracing>().unwrap();
            let shader_bound = shader.bind(gl);
            shader_bound.attach_uniforms_from(viewport)
                .attach_uniforms_from(&self.config)
                .attach_uniforms_from(&self.buffer)
                .attach_uniform("model", viewport.get_model_mat())
                .attach_uniform("current_time", &utils::get_current_time())
                .attach_uniform("current_depth", &(self.depth as i32));

            self.raytracer.draw::<P>(gl, &shader_bound);
        }        
    }
}

use crate::utils;

use crate::renderable::DisableDrawing;
impl DisableDrawing for HiPSSphere {
    fn disable(&mut self, _: &ViewPort) {
    }
}