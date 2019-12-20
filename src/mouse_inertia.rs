use cgmath::Vector3;

pub struct MouseInertia {
    x: f32, // rotation angle amount
    v0: f32, // initial velocity
    axis: Vector3<f32>,
}

use crate::renderable::Renderable;
use crate::renderable::hips_sphere::HiPSSphere;
use crate::renderable::grid::ProjetedGrid;
use crate::renderable::catalog::Catalog;

use crate::viewport::ViewPort;

use crate::event::Move;

impl MouseInertia {
    pub fn new(
        event: &Move,
        viewport: &mut ViewPort,
    ) -> Option<MouseInertia> {
        let x0 = event.get_last_displacement_amount();
        if x0 < 1e-5 {
            None
        } else {
            // Tell the viewport we enter in the inertia
            // mode
            viewport.start_inertia();

            let v0 = 0.0003_f32;
            let axis = event.get_axis();
            let inertia = MouseInertia {
                x: x0,
                v0: v0,
                axis: *axis,
            };

            Some(inertia)
        }
    }

    // Returns whether the inertia is finished
    pub fn update(&mut self,
        hips_sphere: &mut Renderable<HiPSSphere>,
        grid: &mut Renderable<ProjetedGrid>,
        catalog: &mut Renderable<Catalog>,
        viewport: &mut ViewPort,
        dt: f32,
    ) -> bool {
        if self.x < 1e-5 {
            // Stop inertia
            viewport.stop_inertia();
            return true;
        }

        let alpha = 0.9_f32;
        /*if alpha >= 1_f32 {
            alpha = 0.95_f32;
        }*/
        let dx = self.x * alpha;

        hips_sphere.apply_rotation(-self.axis, cgmath::Rad(dx));
        let inv_model_mat = hips_sphere.get_inverted_model_mat();
        grid.set_model_mat(inv_model_mat);
        catalog.set_model_mat(inv_model_mat);

        self.x = dx;

        false
    }
}