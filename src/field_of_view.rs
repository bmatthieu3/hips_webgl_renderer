use cgmath::Rad;
use cgmath::Deg;
use cgmath::Vector2;
use cgmath::Vector4;
use cgmath::Matrix4;

pub struct FieldOfView {
    num_vertices: usize,
    num_vertices_width: usize,
    num_vertices_height: usize,

    vertices_screen_space: Vec<Vector2<f32>>,
    vertices_world_space: Vec<Vector4<f32>>,

    value: Option<Rad<f32>>, // fov can be None if the camera is out of the projection
}

use itertools_num;
use std::iter;

use crate::projection::ProjectionType;
use crate::math;
use crate::MAX_DEPTH;
use crate::window_size_f32;

use std::sync::atomic::Ordering;

use web_sys::console;
impl FieldOfView {
    pub fn new() -> FieldOfView {
        let num_vertices_width = 5;
        let num_vertices_height = 5;
        let num_vertices = 4 + 2*num_vertices_width + 2*num_vertices_height;

        let mut x_screen_space = itertools_num::linspace::<f32>(-1., 1., num_vertices_width + 2)
            .collect::<Vec<_>>();

        x_screen_space.extend(iter::repeat(1_f32).take(num_vertices_height));
        x_screen_space.extend(itertools_num::linspace::<f32>(1., -1., num_vertices_width + 2));
        x_screen_space.extend(iter::repeat(-1_f32).take(num_vertices_height));

        let mut y_screen_space = iter::repeat(-1_f32).take(num_vertices_width + 1)
            .collect::<Vec<_>>();

        y_screen_space.extend(itertools_num::linspace::<f32>(-1., 1., num_vertices_height + 2));
        y_screen_space.extend(iter::repeat(1_f32).take(num_vertices_width));
        y_screen_space.extend(itertools_num::linspace::<f32>(1., -1., num_vertices_height + 2));
        y_screen_space.pop();

        let vertices_screen_space = x_screen_space.into_iter().zip(y_screen_space.into_iter()).map(|(x, y)| {
            Vector2::new(x, y)
        }).collect::<Vec<_>>();
        let vertices_world_space = vec![Vector4::new(0_f32, 0_f32, 0_f32, 1_f32); vertices_screen_space.len()];
        let value = None;

        FieldOfView {
            num_vertices,
            num_vertices_width,
            num_vertices_height,

            vertices_screen_space,
            vertices_world_space,

            value
        }
    }

    pub fn update(&mut self, zoom: f32, projection: &ProjectionType) {
        self.vertices_world_space = self.vertices_screen_space.iter()
            .filter_map(|vertex_screen_space| {
                let vertex_homogeneous_space = vertex_screen_space / zoom;
                let vertex_world_space = projection.screen_to_world_space(&vertex_homogeneous_space);
                vertex_world_space
            })
            .collect::<Vec<_>>();

        self.value = if self.vertices_world_space.len() == self.num_vertices {
            let idx_r = self.num_vertices_width + 1 + (((self.num_vertices_height as f32)/2_f32).ceil() as usize);
            let idx_l = 2 * self.num_vertices_width + 3 + self.num_vertices_height + (((self.num_vertices_height as f32)/2_f32).ceil() as usize);

            let pos_r_world_space = cgmath::Vector3::new(
                self.vertices_world_space[idx_r].x,
                self.vertices_world_space[idx_r].y,
                self.vertices_world_space[idx_r].z
            );
            let pos_l_world_space = cgmath::Vector3::new(
                self.vertices_world_space[idx_l].x,
                self.vertices_world_space[idx_l].y,
                self.vertices_world_space[idx_l].z
            );

            let fov = math::angular_distance_xyz(pos_r_world_space, pos_l_world_space);
            Some(fov)
        } else {
            None
        };
    }

    // Returns the HEALPix cells located in the
    // field of view
    pub fn get_healpix_cells(&self, model: &Matrix4<f32>, max_num_tiles: usize) -> (u8, Vec<u64>) {
        if let Some(fov) = self.value {
            // The fov does not cross the border of the projection
            if fov >= Deg(150_f32).into() {
                // The fov is >= 150°
                (0, (0..12).collect::<Vec<_>>())
            } else {
                let (width, _) = window_size_f32();
                let l = width;
                // Compute the depth corresponding to the angular resolution of a pixel
                // along the width of the screen
                let mut depth = std::cmp::min(math::ang_per_pixel_to_depth(fov.0 / l), MAX_DEPTH.load(Ordering::Relaxed));
                let idx = if depth == 0 {
                    (0..12).collect::<Vec<_>>()
                } else {
                    // The fov is not too big so we can get the HEALPix cells
                    // being in the fov
                    console::log_1(&format!("VERTICES world space, {:?}", self.vertices_world_space).into());
                    let lon_lat_world_space = self.vertices_world_space.iter()
                        .map(|vertex_world_space| {
                            // Take into account the rotation of the sphere
                            let vertex_model_space = (*model) * vertex_world_space;

                            let (ra, dec) = math::xyzw_to_radec(vertex_model_space);
                            (ra as f64, dec as f64)
                        })
                        .collect::<Vec<_>>();

                    console::log_1(&format!("LONLAT, {:?}", lon_lat_world_space).into());
                    let mut idx = vec![];
                    while depth > 0 {
                        let moc = healpix::nested::polygon_coverage(depth, &lon_lat_world_space, true);
                        let num_tiles = moc.entries.len();
                        // Stop when the number of tiles for this depth
                        // can be contained in the tile buffer
                        if num_tiles <= max_num_tiles {
                            idx = moc.flat_iter().collect::<Vec<_>>();
                            break;
                        }
    
                        depth -= 1;
                    }

                    if depth == 0 {
                        idx = (0..12).collect::<Vec<_>>();
                    }

                    idx
                };

                (depth, idx)
            }
        } else {
            // The fov is out the projection
            (0, (0..12).collect::<Vec<_>>())
        }
    }

    pub fn value(&self) -> &Option<Rad<f32>> {
        &self.value
    }
}