use crate::{angle::{Angle, ArcDeg}, async_task::TaskExecutor, async_task::{BuildCatalogIndex, ParseTableTask, TaskResult, TaskType}, buffer::TileDownloader, camera::CameraViewPort, color::Color, coo_conversion::CooSystem, line, math, math::{LonLat, LonLatT}, projection::{Orthographic, Projection}, renderable::{
        catalog::{Manager, Source},
        grid::ProjetedGrid,
        survey::image_survey::ImageSurveys,
        labels::{RenderManager, TextRenderManager},
    }, shader::ShaderManager, shaders::Colormaps, time::DeltaTime, utils};

use al_core::resources::Resources;
use al_ui::hips::SimpleHiPS;
use al_core::{pixel::PixelType, WebGlContext};

use cgmath::Vector4;

use wasm_bindgen::prelude::*;
use web_sys::WebGl2RenderingContext;

use std::cell::RefCell;
use std::rc::Rc;

use std::collections::HashSet;

use serde::Deserialize;
#[derive(Deserialize, Debug)]
struct S {
    ra: f64,
    dec: f64,
}
use crate::renderable::final_pass::RenderPass;
use al_core::FrameBufferObject;
use al_ui::{Gui, GuiRef};
pub struct App {
    pub gl: WebGlContext,

    ui: GuiRef,

    shaders: ShaderManager,
    camera: CameraViewPort,

    downloader: TileDownloader,
    surveys: ImageSurveys,

    time_start_blending: Time,
    request_redraw: bool,
    rendering: bool,

    // The grid renderable
    grid: ProjetedGrid,
    // Catalog manager
    manager: Manager,

    // Task executor
    exec: Rc<RefCell<TaskExecutor>>,
    pub resources: Resources,

    move_animation: Option<MoveAnimation>,
    zoom_animation: Option<ZoomAnimation>,
    inertial_move_animation: Option<InertiaAnimation>,
    prev_cam_position: Vector3<f64>,
    prev_center: Vector3<f64>,
    out_of_fov: bool,
    tasks_finished: bool,
    catalog_loaded: bool,

    final_rendering_pass: RenderPass,
    fbo_view: FrameBufferObject,
    pub fbo_ui: FrameBufferObject,

    pub system: CooSystem,
    pub colormaps: Colormaps,
}

use cgmath::{Vector2, Vector3};
use futures::stream::StreamExt; // for `next`

use crate::rotation::Rotation;
use crate::shaders::Colormap;
struct MoveAnimation {
    start_anim_rot: Rotation<f64>,
    goal_anim_rot: Rotation<f64>,
    time_start_anim: Time,
}

/// State for inertia
struct InertiaAnimation {
    // Initial angular distance
    d0: Angle<f64>,
    // Vector of rotation
    axis: Vector3<f64>,
    // The time when the inertia begins
    time_start_anim: Time,
}

struct ZoomAnimation {
    time_start_anim: Time,
    start_fov: Angle<f64>,
    goal_fov: Angle<f64>,
    w0: f64,
}
use al_core::log::log;
use cgmath::Rad;
const BLEND_TILE_ANIM_DURATION: f32 = 500.0; // in ms
use crate::buffer::Tile;
use crate::time::Time;
use cgmath::InnerSpace;
use wasm_bindgen::JsCast;
impl App {
    pub fn new(
        gl: &WebGlContext,
        aladin_div_name: &str,
        mut shaders: ShaderManager,
        resources: Resources,
    ) -> Result<Self, JsValue> {
        let gl = gl.clone();
        let exec = Rc::new(RefCell::new(TaskExecutor::new()));

        gl.blend_func_separate(
            WebGl2RenderingContext::SRC_ALPHA,
            WebGl2RenderingContext::ONE,
            WebGl2RenderingContext::ONE,
            WebGl2RenderingContext::ONE,
        );

        gl.enable(WebGl2RenderingContext::CULL_FACE);
        gl.cull_face(WebGl2RenderingContext::BACK);

        let system = CooSystem::ICRSJ2000;

        // The tile buffer responsible for the tile requests
        let downloader = TileDownloader::new();

        let camera = CameraViewPort::new::<Orthographic>(&gl, system);
        let screen_size = &camera.get_screen_size();

        let fbo_view = FrameBufferObject::new(&gl, screen_size.x as usize, screen_size.y as usize)?;
        let fbo_ui = FrameBufferObject::new(&gl, screen_size.x as usize, screen_size.y as usize)?;

        // The surveys storing the textures of the resolved tiles
        let surveys = ImageSurveys::new::<Orthographic>(&gl, &camera, &mut shaders, &system);

        let time_start_blending = Time::now();

        // Catalog definition
        let manager = Manager::new(&gl, &mut shaders, &camera, &resources)?;

        // Grid definition
        let grid = ProjetedGrid::new::<Orthographic>(&gl, &camera, &mut shaders)?;

        // Variable storing the location to move to
        let move_animation = None;
        let zoom_animation = None;
        let inertial_move_animation = None;
        let tasks_finished = false;
        let request_redraw = false;
        let _start_render_time = Time::now();
        let rendering = true;
        let prev_cam_position = camera.get_center().truncate();
        let prev_center = Vector3::new(0.0, 1.0, 0.0);
        let out_of_fov = false;
        let catalog_loaded = false;

        let colormaps = Colormaps::new(&gl, &resources)?;

        let final_rendering_pass = RenderPass::new(&gl, screen_size.x as i32, screen_size.y as i32)?;
        let ui = Gui::new(aladin_div_name, &gl)?;


        let app = App {
            gl,
            ui,
            
            shaders,

            camera,

            downloader,
            surveys,

            time_start_blending,
            rendering,
            request_redraw,
            // The grid renderable
            grid,
            // The catalog renderable
            manager,
            exec,
            resources,
            prev_center,

            fbo_view,
            fbo_ui,
            final_rendering_pass,

            move_animation,
            zoom_animation,
            inertial_move_animation,
            prev_cam_position,
            out_of_fov,

            tasks_finished,
            catalog_loaded,
            system,

            colormaps,
        };

        Ok(app)
    }

    pub fn pos_over_ui(&self) -> bool {
        self.ui.lock().pos_over_ui()
    }

    pub fn is_catalog_loaded(&mut self) -> bool {
        if self.catalog_loaded {
            self.catalog_loaded = false;

            true
        } else {
            false
        }
    }

    fn look_for_new_tiles(&mut self) {
        // Move the views of the different active surveys
        self.surveys.refresh_views(&self.camera);
        // Loop over the surveys
        let mut tiles = Vec::new();
        for (survey_id, survey) in self.surveys.iter_mut() {
            let already_available_cells = {
                let mut already_available_cells = HashSet::new();

                let textures = survey.get_textures();
                let view = survey.get_view();

                let texture_cells_in_fov = view.get_cells();

                for texture_cell in texture_cells_in_fov.iter() {
                    for cell in texture_cell.get_tile_cells(&textures.config()) {
                        let already_available = textures.contains_tile(&cell);
                        let is_cell_new = view.is_new(&cell);

                        if already_available {
                            // Remove and append the texture with an updated
                            // time_request
                            if is_cell_new {
                                // New cells are
                                self.time_start_blending = Time::now();
                            }
                            already_available_cells.insert((cell, is_cell_new));
                        } else {
                            // Submit the request to the buffer
                            let format = textures.config().format();
                            let root_url = survey_id.clone();
                            let tile = Tile {
                                root_url,
                                format,
                                cell,
                            };

                            tiles.push(tile);
                        }
                    }
                }

                already_available_cells
            };
            let textures = survey.get_textures_mut();

            for (cell, is_new_cell) in already_available_cells {
                textures.update_priority(&cell, is_new_cell);
            }
        }
        // Launch the new tile requests
        self.downloader.request_tiles(tiles);
    }

    // Run async tasks:
    // - parsing catalogs
    // - copying textures to GPU
    // Return true when a task is complete. This always lead
    // to a redraw of aladin lite
    fn run_tasks<P: Projection>(&mut self, dt: DeltaTime) -> Result<HashSet<Tile>, JsValue> {
        let tasks_time = (dt.0 * 0.5).min(8.3);
        let results = self.exec.borrow_mut().run(tasks_time);
        self.tasks_finished = !results.is_empty();

        // Retrieve back all the tiles that have been
        // copied to the GPU
        // This is important for the tile buffer to know which
        // requests can be reused to query more tiles
        let mut tiles_available = HashSet::new();
        for result in results {
            match result {
                TaskResult::TableParsed {
                    name,
                    sources,
                    colormap,
                } => {
                    self.manager.add_catalog::<P>(
                        name,
                        sources,
                        colormap,
                        &mut self.shaders,
                        &self.camera,
                        &self.surveys.get_view().unwrap(),
                    );
                    self.catalog_loaded = true;
                    self.request_redraw = true;
                }
                TaskResult::TileSentToGPU { tile } => {
                    tiles_available.insert(tile);
                }
            }
        }

        Ok(tiles_available)
    }

    pub fn is_ready(&self) -> Result<bool, JsValue> {
        let res = self.surveys.is_ready();

        Ok(res)
    }

    pub fn update<P: Projection>(&mut self, dt: DeltaTime, force: bool) -> Result<(), JsValue> {
        let available_tiles = self.run_tasks::<P>(dt)?;
        let is_there_new_available_tiles = !available_tiles.is_empty();

        // Check if there is an move animation to do
        if let Some(MoveAnimation {
            start_anim_rot,
            goal_anim_rot,
            time_start_anim,
            ..
        }) = self.move_animation
        {
            let t = (utils::get_current_time() - time_start_anim.as_millis()) / 1000.0;

            // Undamped angular frequency of the oscillator
            // From wiki: https://en.wikipedia.org/wiki/Harmonic_oscillator
            //
            // In a damped harmonic oscillator system: w0 = sqrt(k / m)
            // where:
            // * k is the stiffness of the ressort
            // * m is its mass
            let alpha = 1.0 + (0.0 - 1.0) * (5.0 * t + 1.0) * (-5.0 * t).exp();
            let p = start_anim_rot.slerp(&goal_anim_rot, alpha as f64);

            self.camera.set_rotation::<P>(&p);
            self.look_for_new_tiles();

            // Animation stop criteria
            if 1.0 - alpha < 1e-5 {
                self.move_animation = None;
            }
        }

        // Check if there is an zoom animation to do
        if let Some(ZoomAnimation {
            time_start_anim,
            start_fov,
            goal_fov,
            w0,
            ..
        }) = self.zoom_animation
        {
            let t = ((utils::get_current_time() - time_start_anim.as_millis()) / 1000.0) as f64;

            // Undamped angular frequency of the oscillator
            // From wiki: https://en.wikipedia.org/wiki/Harmonic_oscillator
            //
            // In a damped harmonic oscillator system: w0 = sqrt(k / m)
            // where:
            // * k is the stiffness of the ressort
            // * m is its mass
            let fov = goal_fov + (start_fov - goal_fov) * (w0 * t + 1.0) * ((-w0 * t).exp());
            /*let alpha = 1_f32 + (0_f32 - 1_f32) * (10_f32 * t + 1_f32) * (-10_f32 * t).exp();
            let alpha = alpha * alpha;
            let fov = start_fov * (1_f32 - alpha) + goal_fov * alpha;*/

            self.camera.set_aperture::<P>(fov);
            self.look_for_new_tiles();

            // The threshold stopping criteria must be dependant
            // of the zoom level, in this case we stop when we get
            // to 1% before the goal fov
            let err = (fov - goal_fov).abs();
            let thresh = (start_fov - goal_fov).abs() * 1e-2;
            if err < thresh {
                self.zoom_animation = None;
            }
        }

        if let Some(InertiaAnimation {
            time_start_anim,
            d0,
            axis,
        }) = self.inertial_move_animation
        {
            let t = ((utils::get_current_time() - time_start_anim.as_millis()) / 1000.0) as f64;

            // Undamped angular frequency of the oscillator
            // From wiki: https://en.wikipedia.org/wiki/Harmonic_oscillator
            //
            // In a damped harmonic oscillator system: w0 = sqrt(k / m)
            // where:
            // * k is the stiffness of the ressort
            // * m is its mass
            let w0 = 5.0;
            let d1 = Angle(0.0);
            // The angular distance goes from d0 to 0.0
            let d = d1 + (d0 - d1) * (w0 * t + 1.0) * ((-w0 * t).exp());
            /*let alpha = 1_f32 + (0_f32 - 1_f32) * (10_f32 * t + 1_f32) * (-10_f32 * t).exp();
            let alpha = alpha * alpha;
            let fov = start_fov * (1_f32 - alpha) + goal_fov * alpha;*/

            self.camera.rotate::<P>(&axis, d);
            self.look_for_new_tiles();

            // The threshold stopping criteria must be dependant
            // of the zoom level, in this case the initial angular distance
            // speed
            let thresh: Angle<f64> = d0 * 1e-3;
            if d < thresh {
                self.inertial_move_animation = None;
            }
        }

        {
            // Newly available tiles must lead to
            if is_there_new_available_tiles {
                self.time_start_blending = Time::now();
            }

            // 1. Surveys must be aware of the new available tiles
            self.surveys.set_available_tiles(&available_tiles);
            // 2. Get the resolved tiles and push them to the image surveys
            let resolved_tiles = self
                .downloader
                .get_resolved_tiles(&available_tiles, &self.surveys);
            self.surveys.add_resolved_tiles(resolved_tiles);
            // 3. Try sending new tile requests after
            self.downloader.try_sending_tile_requests()?;
        }

        // The rendering is done following these different situations:
        // - the camera has moved
        let has_camera_moved = self.camera.has_moved();

        // - there is at least one tile in its blending phase
        let blending_anim_occuring =
            (Time::now().0 - self.time_start_blending.0) < BLEND_TILE_ANIM_DURATION;
        self.rendering = blending_anim_occuring
            | has_camera_moved
            | self.request_redraw;
        self.request_redraw = false;

        // Finally update the camera that reset the flag camera changed
        if has_camera_moved {
            if let Some(view) = self.surveys.get_view() {
                self.manager.update::<P>(&self.camera, view);
            }
        }

        self.grid.update::<P>(&self.camera, force);        
        {
            let events = self.ui.lock().update();
            let mut events = events.lock().unwrap();

            for event in events.drain(..) {
                match event {
                    al_ui::Event::ImageSurveys(surveys) => self.set_image_surveys(surveys)?,
                    _ => ()
                }
            }
        }

        Ok(())
    }


    pub fn reset_north_orientation<P: Projection>(&mut self) {
        // Reset the rotation around the center if there is one
        self.camera.set_rotation_around_center::<P>(Angle(0.0));
        // Reset the camera position to its current position
        // this will keep the current position but reset the orientation
        // so that the north pole is at the top of the center.
        let center = self.get_center::<P>();
        self.set_center::<P>(&center);
    }

    pub fn read_pixel<P: Projection>(
        &self,
        x: f64,
        y: f64,
        layer: &str,
    ) -> Result<PixelType, JsValue> {
        let pos = Vector2::new(x, y);
        if let Some(lonlat) = self.screen_to_world::<P>(&pos) {
            self.surveys.read_pixel(&lonlat, layer)
        } else {
            Err(JsValue::from_str(&format!(
                "{:?} is out of projection",
                pos
            )))
        }
    }


    pub fn draw<P: Projection>(&mut self, force_render: bool) -> Result<(), JsValue> {
        let scene_redraw = self.rendering | force_render;
        if scene_redraw {
            let shaders = &mut self.shaders;
            let gl = self.gl.clone();
            let camera = &self.camera;

            let grid = &mut self.grid;
            let surveys = &mut self.surveys;
            let catalogs = &self.manager;
            let colormaps = &self.colormaps;
            let fbo_view = &self.fbo_view;

            fbo_view.draw_onto(move || {
                // Render the scene
                gl.clear_color(0.08, 0.08, 0.08, 1.0);
                gl.clear(WebGl2RenderingContext::COLOR_BUFFER_BIT);

                surveys.draw::<P>(camera, shaders, colormaps);

                // Draw the catalog
                catalogs.draw::<P>(&gl, shaders, camera, colormaps, fbo_view)?;

                grid.draw::<P>(camera, shaders)?;

                Ok(())
            }, None)?;

            // Reset the flags about the user action
            self.camera.reset();
        }

        // Tell if the ui has been redrawn
        let mut ui_redraw = false;
        {
            let mut ui = self.ui.lock();
            let dpi  = self.camera.get_dpi();
            ui_redraw = ui.redraw_needed();
            if ui_redraw {
                let gl = self.gl.clone();

                self.fbo_ui.draw_onto(move || {
                    ui.draw(&gl, dpi)?;

                    Ok(())
                }, None)?;
            }
        }

        // If neither of the scene or the ui has been redraw then do nothing
        // otherwise, redraw both fbos on the screen
        if scene_redraw || ui_redraw {
            self.final_rendering_pass.draw_on_screen(&self.fbo_view);
            self.final_rendering_pass.draw_on_screen(&self.fbo_ui);
        }

        Ok(())
    }

    pub fn set_image_surveys(&mut self, hipses: Vec<SimpleHiPS>) -> Result<(), JsValue> {
        let new_survey_ids = self.surveys.set_image_surveys(
            hipses,
            &self.gl,
            &self.camera,
            self.exec.clone(),
            &self.colormaps,
        )?;
        self.downloader.clear_requests();

        if !new_survey_ids.is_empty() {
            for id in new_survey_ids.iter() {
                let config = &self.surveys.get(id).unwrap().get_textures().config;
                al_core::log::log(&format!("config: {:?}", config));
                self.downloader.request_base_tiles(config);
            }
            // Once its added, request its tiles
            self.look_for_new_tiles();
        }
        self.request_redraw = true;

        Ok(())
    }

    /*pub fn move_image_surveys_layer_forward(&mut self, layer_name: &str) -> Result<(), JsValue> {
        self.surveys.move_image_surveys_layer_forward(layer_name)?;
        self.request_redraw = true;

        Ok(())
    }*/

    pub fn set_projection<P: Projection>(&mut self) {
        self.camera.set_projection::<P>();
        self.surveys.set_projection::<P>(
            &self.camera,
            &mut self.shaders,
            &self.resources,
            &self.system,
        );

        self.look_for_new_tiles();
        self.request_redraw = true;
    }

    pub fn get_max_fov<P: Projection>(&self) -> f64 {
        P::aperture_start().0
    }

    pub fn set_longitude_reversed<P: Projection>(&mut self, reversed: bool) {
        self.camera.set_longitude_reversed(reversed);
        self.surveys.set_longitude_reversed::<P>(
            reversed,
            &self.camera,
            &mut self.shaders,
            &self.resources,
            &self.system,
        );

        self.look_for_new_tiles();

        self.request_redraw = true;
    }

    /*pub fn set_opacity_layer(&mut self, layer_name: &str, opacity: f32) -> Result<(), JsValue> {
        self.surveys.set_opacity_layer(layer_name, opacity)?;
        self.request_redraw = true;

        Ok(())
    }*/

    pub fn add_catalog(&mut self, name: String, table: JsValue, colormap: String) {
        let mut exec_ref = self.exec.borrow_mut();
        let table = table;
        let c = self.colormaps.get(&colormap);

        exec_ref
            .spawner()
            .spawn(TaskType::ParseTableTask, async move {
                let mut stream = ParseTableTask::<[f32; 2]>::new(table);
                let mut results: Vec<Source> = vec![];

                while let Some(item) = stream.next().await {
                    let item: &[f32] = item.as_ref();
                    results.push(item.into());
                }

                let mut stream_sort = BuildCatalogIndex::new(results);
                while stream_sort.next().await.is_some() {}

                // The stream is finished, we get the sorted sources
                let results = stream_sort.sources;

                TaskResult::TableParsed {
                    name,
                    sources: results,
                    colormap: c,
                }
            });
    }

    pub fn resize<P: Projection>(&mut self, width: f32, height: f32) {
        let dpi = self.camera.get_dpi();
        

        let canvas = self.gl
            .canvas()
            .unwrap()
            .dyn_into::<web_sys::HtmlCanvasElement>()
            .unwrap();
        canvas.style().set_property("width", &format!("{}px", width.to_string())).unwrap();
        canvas.style().set_property("height", &format!("{}px", height.to_string())).unwrap();

        let w = (width as f32) * dpi;
        let h = (height as f32 ) * dpi;
        self.camera.set_screen_size::<P>(w, h);
        // resize the view fbo
        self.fbo_view.resize(w as usize, h as usize);
        // resize the ui fbo
        self.fbo_ui.resize(w as usize, h as usize);

        // launch the new tile requests
        self.look_for_new_tiles();
        self.manager.set_kernel_size(&self.camera);
    }

    pub fn set_catalog_colormap(
        &mut self,
        name: String,
        colormap: Colormap,
    ) -> Result<(), JsValue> {
        let catalog = self.manager.get_mut_catalog(&name).map_err(|e| {
            let err: JsValue = e.into();
            err
        })?;
        catalog.set_colormap(colormap);

        self.request_redraw = true;

        Ok(())
    }

    pub fn set_heatmap_opacity(&mut self, name: String, opacity: f32) -> Result<(), JsValue> {
        let catalog = self.manager.get_mut_catalog(&name).map_err(|e| {
            let err: JsValue = e.into();
            err
        })?;
        catalog.set_alpha(opacity);

        self.request_redraw = true;

        Ok(())
    }

    pub fn set_kernel_strength(&mut self, name: String, strength: f32) -> Result<(), JsValue> {
        let catalog = self.manager.get_mut_catalog(&name).map_err(|e| {
            let err: JsValue = e.into();
            err
        })?;
        catalog.set_strength(strength);

        self.request_redraw = true;

        Ok(())
    }

    pub fn set_grid_color(&mut self, color: Color) {
        self.grid.set_color(color);
        self.request_redraw = true;
    }

    pub fn enable_grid<P: Projection>(&mut self) {
        self.grid.enable::<P>(&self.camera);
        self.request_redraw = true;
    }

    pub fn hide_grid_labels(&mut self) {
        self.grid.hide_labels(&self.camera);
        self.request_redraw = true;
    }

    pub fn show_grid_labels(&mut self) {
        self.grid.show_labels();
        self.request_redraw = true;
    }

    pub fn disable_grid(&mut self) {
        self.grid.disable(&self.camera);
        self.request_redraw = true;
    }

    pub fn set_coo_system<P: Projection>(&mut self, coo_system: CooSystem) {
        //let icrs2gal = coo_system == CooSystem::GAL && self.system == CooSystem::ICRSJ2000;
        //let gal2icrs = coo_system == CooSystem::ICRSJ2000 && self.system == CooSystem::GAL;

        self.system = coo_system;
        self.camera.set_coo_system::<P>(coo_system);

        /*if icrs2gal {
            // rotate the camera around the center axis
            // to move the galactic plane straight to the center
            self.camera
                .set_rotation_around_center::<P>(ArcDeg(58.6).into());
        } else if gal2icrs {
            self.camera
                .set_rotation_around_center::<P>(ArcDeg(0.0).into());
        }*/

        self.request_redraw = true;
    }

    pub fn world_to_screen<P: Projection>(
        &self,
        lonlat: &LonLatT<f64>,
    ) -> Result<Option<Vector2<f64>>, String> {
        //let lonlat = crate::coo_conversion::to_galactic(*lonlat);
        let model_pos_xyz = lonlat.vector();

        let screen_pos = P::model_to_screen_space(&model_pos_xyz, &self.camera);
        Ok(screen_pos)
    }

    /// World to screen projection
    ///
    /// sources coordinates are given in ICRS j2000
    pub fn world_to_screen_vec<P: Projection>(
        &self,
        sources: &Vec<JsValue>,
    ) -> Result<Vec<f64>, JsValue> {
        let res: Vec<f64> = sources
            .into_iter()
            .filter_map(|s| {
                let source: S = s
                    .into_serde()
                    .map_err(|e| JsValue::from_str(&e.to_string()))
                    .unwrap();
                let lonlat = LonLatT::new(ArcDeg(source.ra).into(), ArcDeg(source.dec).into());
                //let lonlat = self.app.system.icrs_to_system(lonlat);

                let xyz = lonlat.vector();

                if let Some(s_xy) = P::model_to_screen_space(&xyz, &self.camera) {
                    Some(vec![s_xy.x, s_xy.y])
                } else {
                    None
                }
            })
            .flatten()
            .collect::<Vec<_>>();
        Ok(res)
    }

    pub fn screen_to_world<P: Projection>(&self, pos: &Vector2<f64>) -> Option<LonLatT<f64>> {
        if let Some(model_pos) = P::screen_to_model_space(&pos, &self.camera) {
            //let model_pos = self.system.system_to_icrs_coo(model_pos);
            Some(model_pos.lonlat())
        } else {
            None
        }
    }

    pub fn set_center<P: Projection>(&mut self, lonlat: &LonLatT<f64>) {
        self.prev_cam_position = self.camera.get_center().truncate();

        let xyz: Vector4<_> = lonlat.vector();
        let rot = Rotation::from_sky_position(&xyz);

        // Apply the rotation to the camera to go
        // to the next lonlat
        self.camera.set_rotation::<P>(&rot);
        self.look_for_new_tiles();

        // Stop the current animation if there is one
        self.move_animation = None;
        // And stop the current inertia as well if there is one
        self.inertial_move_animation = None;
    }

    pub fn press_left_button_mouse(&mut self, sx: f32, sy: f32) {
        self.prev_center = self.camera.get_center().truncate();
        self.inertial_move_animation = None;
        self.move_animation = None;
        self.out_of_fov = false;
    }

    pub fn release_left_button_mouse(&mut self, sx: f32, sy: f32) {
        let dpi = self.camera.get_dpi();
        
        let sx = (sx as f32) * dpi;
        let sy = (sy as f32) * dpi;
        // Check whether the center has moved
        // between the pressing and releasing
        // of the left button.
        //
        // Do not start inerting if:
        // * the mouse has not moved
        // * the mouse is out of the projection
        // * the mouse has not been moved since a certain
        //   amount of time
        let center = self.camera.get_center().truncate();
        let now = Time::now();
        let time_of_last_move = self.camera.get_time_of_last_move();
        //debug!(now);
        //debug!(time_of_last_move);
        if self.out_of_fov
            || self.prev_center == center
            || (now - time_of_last_move) >= DeltaTime::from_millis(30.0)
        {
            return;
        }

        if self.ui.lock().pos_over_ui() {
            return;
        }
        // Start inertia here

        // Angular distance between the previous and current
        // center position
        let x = self.prev_cam_position;
        let axis = x.cross(center).normalize();
        let d0 = math::ang_between_vect(&x, &center);

        self.inertial_move_animation = Some(InertiaAnimation {
            d0,
            axis,
            time_start_anim: Time::now(),
        });
    }

    pub fn start_moving_to<P: Projection>(&mut self, lonlat: &LonLatT<f64>) {
        // Get the XYZ cartesian position from the lonlat
        let goal_pos: Vector4<f64> = lonlat.vector();

        // Convert these positions to rotations
        let start_anim_rot = *self.camera.get_rotation();
        let goal_anim_rot = Rotation::from_sky_position(&goal_pos);

        // Set the moving animation object
        self.move_animation = Some(MoveAnimation {
            time_start_anim: Time::now(),
            start_anim_rot,
            goal_anim_rot,
        });
    }

    pub fn rotate_around_center<P: Projection>(&mut self, theta: ArcDeg<f64>) {
        self.camera.set_rotation_around_center::<P>(theta.into());
        // New tiles can be needed and some tiles can be removed
        self.look_for_new_tiles();

        self.request_redraw = true;
    }

    pub fn get_rotation_around_center(&self) -> &Angle<f64> {
        self.camera.get_rotation_around_center()
    }

    pub fn start_zooming_to<P: Projection>(&mut self, fov: Angle<f64>) {
        // For the moment, no animation is triggered.
        // The fov is directly set
        self.camera.set_aperture::<P>(fov);
        self.look_for_new_tiles();
    }

    pub fn project_line<P: Projection>(
        &self,
        lon1: f64,
        lat1: f64,
        lon2: f64,
        lat2: f64,
    ) -> Vec<Vector2<f64>> {
        let v1: Vector3<f64> = LonLatT::new(ArcDeg(lon1).into(), ArcDeg(lat1).into()).vector();
        let v2: Vector3<f64> = LonLatT::new(ArcDeg(lon2).into(), ArcDeg(lat2).into()).vector();

        line::project::<P>(&v1, &v2, &self.camera)
    }

    pub fn go_from_to<P: Projection>(&mut self, s1x: f64, s1y: f64, s2x: f64, s2y: f64) {
        if let Some(w1) = P::screen_to_world_space(&Vector2::new(s1x, s1y), &self.camera) {
            if let Some(w2) = P::screen_to_world_space(&Vector2::new(s2x, s2y), &self.camera) {
                let r = self.camera.get_final_rotation();

                let cur_pos = r.rotate(&w1).truncate();
                //let cur_pos = w1.truncate();
                let next_pos = r.rotate(&w2).truncate();
                //let next_pos = w2.truncate();
                if cur_pos != next_pos {
                    let axis = cur_pos.cross(next_pos).normalize();
                    let d = math::ang_between_vect(&cur_pos, &next_pos);
                    self.prev_cam_position = self.camera.get_center().truncate();

                    // Apply the rotation to the camera to
                    // go from the current pos to the next position
                    self.camera.rotate::<P>(&(-axis), d);
                    self.look_for_new_tiles();
                }
                return;
            }
        }

        self.out_of_fov = true;
    }

    // Accessors
    pub fn get_center<P: Projection>(&self) -> LonLatT<f64> {
        self.camera.get_center().lonlat()
    }

    pub fn get_clip_zoom_factor(&self) -> f64 {
        self.camera.get_clip_zoom_factor()
    }

    pub fn get_fov(&self) -> f64 {
        let deg: ArcDeg<f64> = self.camera.get_aperture().into();
        deg.0
    }

    pub fn get_gl_canvas(&self) -> Option<js_sys::Object> {
        self.gl.canvas()
    }
}