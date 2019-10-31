use crate::renderable::Renderable;
use crate::renderable::grid::IsoLatitudeLine;
use crate::projection::ProjectionType;
/*
pub struct ProjetedGrid {
    step_lat: cgmath::Rad<f32>, // The number of lines in the view
    projection: Rc<RefCell<ProjectionType>>,
    iso_lat_lines: Vec<Renderable<IsoLatitudeLine>>,

    model_mat: cgmath::Matrix4<f32>,
}

use std::cell::RefCell;
use std::rc::Rc;
use crate::shader::Shader;
use crate::viewport::ViewPort;
use web_sys::WebGl2RenderingContext;

use cgmath::SquareMatrix;
impl ProjetedGrid {
    pub fn new(gl: Rc<WebGl2RenderingContext>, shader: Rc<Shader>, projection: Rc<RefCell<ProjectionType>>, step_lat: cgmath::Rad<f32>) -> ProjetedGrid {
        let mut num_lat = (std::f32::consts::PI / step_lat.0) as usize;
        num_lat = num_lat - 1;

        let lat_start = (-std::f32::consts::PI / 2_f32) + step_lat.0;

        let mut iso_lat_lines = Vec::with_capacity(num_lat);
        for i in 0..num_lat {
            let lat = lat_start + (i as f32) * step_lat.0;

            let iso_lat = Rc::new(RefCell::new(IsoLatitudeLine::new(lat)));

            let renderable = Renderable::<IsoLatitudeLine>::new(
                gl.clone(),
                shader.clone(),
                projection.clone(),
                iso_lat.clone(),
            );
            iso_lat_lines.push(renderable);
        }

        let model_mat = cgmath::Matrix4::identity();
        ProjetedGrid {
            step_lat,
            projection,
            iso_lat_lines,
            model_mat,
        }
    }

    pub fn apply_rotation(&mut self, axis: cgmath::Vector3<f32>, angle: cgmath::Rad<f32>) {
        //self.model_mat = cgmath::Matrix4::<f32>::from_axis_angle(axis, angle) * self.model_mat;
        for iso_lat_line in self.iso_lat_lines.iter_mut() {
            iso_lat_line.apply_rotation(axis, angle);
            //iso_lat_line.update_vertex_array_object(self.projection.clone());
        }
    }

    pub fn update(&mut self) {
        for iso_lat_line in self.iso_lat_lines.iter_mut() {
            iso_lat_line.update_vertex_array_object(self.projection.clone());
        }
    }

    pub fn draw(&self, mode: u32, viewport: &ViewPort) {
        for iso_lat_line in self.iso_lat_lines.iter() {
            iso_lat_line.draw(mode, viewport);
        }
    }
}
*/



