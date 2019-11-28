use cgmath::Vector3;

pub struct MouseInertia {
    x: f32, // rotation angle amount
    v0: f32, // initial velocity
    axis: Vector3<f32>,
}

use crate::renderable::Renderable;
use crate::renderable::hips_sphere::HiPSSphere;
use crate::renderable::grid::ProjetedGrid;

use crate::viewport::ViewPort;

use std::cmp;
impl MouseInertia {
    pub fn new(
        x0: f32,
        axis: Vector3<f32>,
    ) -> Option<MouseInertia> {
        if x0 < 1e-3 {
            None
        } else {
            let v0 = 0.0003_f32;
            let axis = axis;
            let inertia = MouseInertia {
                x: x0,
                v0: v0,
                axis: axis,
            };

            Some(inertia)
        }
    }

    // Returns whether the inertia is finished
    pub fn update(&mut self,
        hips_sphere: &mut Renderable<HiPSSphere>,
        grid: &mut Renderable<ProjetedGrid>,
        viewport: &mut ViewPort,
        dt: f32,
    ) -> bool {
        if self.x < 1e-3 {
            viewport.stop_displacement();
            return true;
        }

        let mut alpha = 0.8_f32;
        if alpha >= 1_f32 {
            alpha = 0.95_f32;
        }
        let dx = self.x * alpha;

        hips_sphere.apply_rotation(-self.axis, cgmath::Rad(dx));
        let inv_model_mat = hips_sphere.get_inverted_model_mat();
        grid.set_model_mat(inv_model_mat);

        self.x = dx;

        false
    }
}