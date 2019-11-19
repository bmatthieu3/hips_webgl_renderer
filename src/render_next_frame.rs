use std::sync::atomic::{AtomicBool, Ordering};
use crate::utils;

pub struct RenderNextFrame {
    render: bool,
    next_time: f32,
}

impl RenderNextFrame {
    pub fn new() -> RenderNextFrame {
        let next_time = 0_f32;
        let render = true;

        RenderNextFrame {
            render,
            next_time,
        }
    }

    pub fn get(&self) -> bool {
        self.render
    }

    pub fn set_for_duration_seconds(&mut self, duration: f32) {
        self.next_time = utils::get_current_time() + duration;
        self.set(true);
    }

    pub fn set(&mut self, val: bool) {
        self.render = val;
    }

    pub fn update(&mut self) {
        let current_time = utils::get_current_time();
        if current_time >= self.next_time {
            self.set(false);
        }
    }
}

use std::sync::Mutex;
use std::sync::Arc;
lazy_static! {
    // Note: Render_next_frame is global for the moment
    // A Rc cannot be instanciated as global because it cannot be shared between
    // threads (Rc does not impl the Sync trait)
    // Arc can be shared between threads => it is used here.
    pub static ref RENDER_NEXT_FRAME: Arc<Mutex<RenderNextFrame>> = Arc::new(Mutex::new(RenderNextFrame::new()));
}