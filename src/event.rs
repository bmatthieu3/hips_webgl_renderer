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
use crate::viewport::ViewPort;
use crate::math;
use crate::utils;

use crate::projection::Projection;
use crate::renderable::hips_sphere::RenderingMode;

use cgmath::Matrix4;
use cgmath::Quaternion;
use cgmath::Euler;
use cgmath::SquareMatrix;

impl Move {
    pub fn new<P: Projection>(start_world_pos: Vector4<f32>, hips_sphere: &Renderable<HiPSSphere>) -> Move {
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

    pub fn apply_to_renderables<P: Projection>(&mut self, world_pos: Vector4<f32>,
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

        /*let q = Quaternion::from_arc(model_pos, start_model_pos, None);
        let m: Matrix4<f32> = q.into();
        hips_sphere.apply_quarternion_rotation(&q);
        hips_sphere.set_model_mat(&(m * model_mat));*/

        hips_sphere.apply_rotation(-self.axis, self.x);
        //catalog.apply_rotation(-self.axis, self.x);
        // Move the grid the opposite way of the hips sphere

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