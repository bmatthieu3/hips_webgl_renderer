use cgmath::Vector4;
use cgmath::Vector3;

use cgmath::Rad;

use crate::renderable::Renderable;
use crate::renderable::hips_sphere::HiPSSphere;
use crate::renderable::grid::ProjetedGrid;
pub struct Move {
    start_world_pos: Vector4<f32>,
    axis: Vector3<f32>,
    x: Rad<f32>, // amount of displacement
}

use cgmath::Vector2;
use cgmath::InnerSpace;
use crate::projection::ProjectionType;
use crate::viewport::ViewPort;
pub fn screen_to_world_space(screen_pos: &Vector2<f32>, projection: &ProjectionType, viewport: &ViewPort) -> Option<Vector4<f32>> {
    let homogeneous_pos = crate::projection::screen_pixels_to_homogenous(&screen_pos, viewport);

    let world_pos = projection.screen_to_world_space(&homogeneous_pos);
    if let Some(world_pos) = world_pos {
        let world_pos = world_pos.normalize();

        Some(world_pos)
    } else {
        None
    }
}

use crate::math;
impl Move {
    pub fn new(
        start_screen_pos: Vector2<f32>,
        projection: &ProjectionType,
        viewport: &ViewPort,
    ) -> Option<Move> {
        let start_world_pos = screen_to_world_space(&start_screen_pos, projection, viewport);
        let result = if let Some(start_world_pos) = start_world_pos {
            let axis = Vector3::new(0_f32, 0_f32, 0_f32);
            let x = Rad(0_f32);
            let event = Move {
                start_world_pos,
                axis,
                x
            };
            Some(event)
        } else {
            None
        };

        result
    }

    pub fn update(&mut self, screen_pos: &Vector2<f32>,
        hips_sphere: &mut Renderable<HiPSSphere>,
        grid: &mut Renderable<ProjetedGrid>,
        projection: &ProjectionType,
        viewport: &mut ViewPort,
    ) {
        let world_pos = screen_to_world_space(screen_pos, projection, viewport);
        if let Some(world_pos) = world_pos {
            if world_pos != self.start_world_pos {
                let model_mat = hips_sphere.get_model_mat();

                let start_model_pos = model_mat * self.start_world_pos;
                let start_model_pos = cgmath::Vector3::<f32>::new(start_model_pos.x, start_model_pos.y, start_model_pos.z);

                let model_pos = model_mat * world_pos;
                let model_pos = cgmath::Vector3::<f32>::new(model_pos.x, model_pos.y, model_pos.z);

                let mut axis = start_model_pos.cross(model_pos);
                self.x = math::angular_distance_xyz(start_model_pos, model_pos);

                self.axis = axis.normalize();
                hips_sphere.apply_rotation(-self.axis, self.x);
                // Move the grid the opposite way of the hips sphere
                let inv_model_mat = hips_sphere.get_inverted_model_mat();
                grid.set_model_mat(inv_model_mat);

                self.start_world_pos = world_pos;
                viewport.displacement();
            }
        }
    }

    pub fn get_axis(&self) -> &Vector3<f32> {
        &self.axis
    }

    pub fn get_last_displacement_amount(&self) -> f32 {
        self.x.0
    }
}

/*
struct EventManager {
    move_event: Option<Move>,
    zoom_event: Option<Zoom>,
}

impl EventManager {
    fn new() -> EventManager {
        let move_event = None;
        let zoom_event = None;
        EventManager {
            move_event,
            zoom_event,
        }
    }

    fn move(&mut self, screen_mouse_pos: &Vector2<f32>) {
        let start_world_pos = screen_to_world_space(screen_mouse_pos);

        // Check if a move event is currently operating
        if let Some(ref mut move_event) = self.move_event {
            if let Some(ref pos) = start_world_pos {
                move_event.update(pos);
            } else {
                // The given position is not well projeted so we delete the move event
                self.move_event = None;
            }
        } else {
            // If it is not the case create a new one
            if let Some(pos) = start_world_pos {
                self.move_event = Some(Move::new(pos));
            }
        }
    }
}
*/
