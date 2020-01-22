use crate::utils;

pub struct RenderNextFrame {
    render: bool,
    next_time: f32,
}
use web_sys::console;
use crate::viewport::ViewPort;
impl RenderNextFrame {
    pub fn new() -> RenderNextFrame {
        let next_time = 0_f32;
        let render = true;

        RenderNextFrame {
            render,
            next_time,
        }
    }

    pub fn update(&mut self, viewport: &ViewPort) -> bool {
        let mut update = true;
        
        let current_time = utils::get_current_time();
        if current_time >= self.next_time {
            update = false;
        }

        update
    }
}

use std::sync::Mutex;
use std::sync::Arc;

use std::sync::atomic::{AtomicBool};
lazy_static! {
    // Note: Render_next_frame is global for the moment
    // A Rc cannot be instanciated as global because it cannot be shared between
    // threads (Rc does not impl the Sync trait)
    // Arc can be shared between threads => it is used here.
    pub static ref LATEST_TIME_TILE_RECEIVED: Arc<Mutex<f32>> = Arc::new(Mutex::new(utils::get_current_time()));
    //pub static ref UPDATE_USER_INTERFACE: Arc<AtomicBool> = Arc::new(AtomicBool::new(false));
}