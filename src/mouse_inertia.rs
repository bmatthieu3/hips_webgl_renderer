use cgmath::Vector3;

#[derive(Clone)]
pub struct MouseInertia {
    x: f32, // rotation angle amount
    axis: Vector3<f32>,
}

use crate::renderable::Renderable;
use crate::renderable::hips_sphere::HiPSSphere;
use crate::renderable::grid::ProjetedGrid;
use crate::renderable::catalog::Catalog;

use crate::viewport::ViewPort;

use crate::event::Move;
use crate::utils;
use web_sys::console;
const DURATION: f32 = 10_f32;

use crate::renderable::hips_sphere::RenderingMode;
use crate::projection::Projection;

impl MouseInertia {
    pub fn new(event: Move, dt: f32) -> Option<MouseInertia> {
        let speed = 7_f32;
        let x0 = speed * event.get_last_displacement_amount() / dt;
        let t0 = event.get_last_time();

        // If the user has not moved in the last **duration** ms then the inertia
        // is disabled
        // TODO: maybe that would be good for the value of duration to be dependant of the
        // framerate of the user machine. For the moment the value is a constant.
        if x0 < 1e-5 || (utils::get_current_time() - t0) > DURATION + dt {
            None
        } else {
            // Tell the viewport we enter in the inertia
            // mode
            let axis = event.get_axis();
            let inertia = MouseInertia {
                x: x0,
                axis: *axis,
            };

            Some(inertia)
        }
    }

    // Returns whether the inertia is finished
    pub fn update<P: Projection>(mut self,
        hips_sphere: &mut Renderable<HiPSSphere>,
        grid: &mut Renderable<ProjetedGrid>,
        catalog: &mut Renderable<Catalog>,

        viewport: &mut ViewPort
    ) -> Option<Self> {
        console::log_1(&format!("inertia").into());
        if self.x < 1e-5 {
            // Stop inertia
            None
        } else {
            let alpha = 0.9_f32;

            let dx = self.x * alpha;

            hips_sphere.apply_rotation(-self.axis, cgmath::Rad(dx));
            viewport.displacement::<P>(hips_sphere, catalog);

            self.x = dx;

            Some(MouseInertia {
                x: self.x,
                axis: self.axis,
            })
        }
    }
}