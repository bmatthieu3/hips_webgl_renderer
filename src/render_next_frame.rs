use crate::utils;

pub struct RenderNextFrame {
    render: bool,
    next_time: f32,
}

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

    pub fn update(&mut self, viewport: &ViewPort) {
        // Priority to mouse/wheel events
        let is_action = viewport.is_user_action();

        if !is_action {
            // If no action by the user we check whether there is
            // more tiles to load.
            let current_time = utils::get_current_time();
            if current_time >= self.next_time {
                // If not, we stop rendering the next frame
                self.set(false);
            }
        } else {
            self.set(true);
        }
    }
}

pub struct UpdateCurrentFrame {
    update: bool,
}
impl UpdateCurrentFrame {
    pub fn new() -> UpdateCurrentFrame {
        let update = true;

        UpdateCurrentFrame {
            update,
        }
    }

    pub fn get(&self) -> bool {
        self.update
    }

    pub fn set(&mut self, new_value: bool) {
        self.update = new_value;
    }

    pub fn update(&mut self, viewport: &ViewPort) {
        // Priority to mouse/wheel events
        let is_action = viewport.is_user_action();

        if !is_action {
            // No action by the user -> no updates
            self.set(false);
        } else {
            self.set(true);
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
    pub static ref RENDER_FRAME: Arc<Mutex<RenderNextFrame>> = Arc::new(Mutex::new(RenderNextFrame::new()));
    pub static ref UPDATE_FRAME: Arc<Mutex<UpdateCurrentFrame>> = Arc::new(Mutex::new(UpdateCurrentFrame::new()));
}