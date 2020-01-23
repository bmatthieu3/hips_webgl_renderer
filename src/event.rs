use cgmath::Vector4;
use cgmath::Vector3;

use cgmath::Rad;

use crate::renderable::Renderable;
use crate::renderable::hips_sphere::HiPSSphere;
use crate::renderable::grid::ProjetedGrid;
use crate::renderable::catalog::Catalog;

pub struct Move {
    start_world_pos: Vector4<f32>,
    axis: Vector3<f32>,
    x: Rad<f32>, // amount of displacement
    last_time: f32,
}

use cgmath::Vector2;
use cgmath::InnerSpace;
use crate::projection::ProjectionType;
use crate::viewport::ViewPort;
use crate::math;
use crate::utils;

impl Move {
    pub fn new(start_world_pos: Vector4<f32>) -> Move {
        let axis = Vector3::new(0_f32, 0_f32, 0_f32);
        let x = Rad(0_f32);
        let last_time = utils::get_current_time();

        Move {
            start_world_pos,
            axis,
            x,
            last_time,
        }
    }

    pub fn apply_to_renderables(&mut self, world_pos: Vector4<f32>,
        hips_sphere: &mut Renderable<HiPSSphere>,
        grid: &mut Renderable<ProjetedGrid>,
        catalog: &mut Renderable<Catalog>,
    ) {
        if world_pos == self.start_world_pos {
            return;
        }

        let model_mat = hips_sphere.get_model_mat();

        let start_model_pos = model_mat * self.start_world_pos;
        let start_model_pos = cgmath::Vector3::<f32>::new(start_model_pos.x, start_model_pos.y, start_model_pos.z);

        let model_pos = model_mat * world_pos;
        let model_pos = cgmath::Vector3::<f32>::new(model_pos.x, model_pos.y, model_pos.z);

        let axis = start_model_pos.cross(model_pos);
        self.x = math::angular_distance_xyz(start_model_pos, model_pos);

        self.axis = axis.normalize();
        hips_sphere.apply_rotation(-self.axis, self.x);
        //catalog.apply_rotation(-self.axis, self.x);
        // Move the grid the opposite way of the hips sphere
        let inv_model_mat = hips_sphere.get_inverted_model_mat();
        grid.set_model_mat(inv_model_mat);
        catalog.set_model_mat(inv_model_mat);

        self.start_world_pos = world_pos;

        // Update the time to the current move
        self.last_time = utils::get_current_time();
    }

    pub fn get_axis(&self) -> &Vector3<f32> {
        &self.axis
    }

    pub fn get_last_displacement_amount(&self) -> f32 {
        self.x.0
    }

    pub fn get_last_time(&self) -> f32 {
        self.last_time
    }
}