use cgmath::InnerSpace;

pub fn angular_distance_xyz(v1: cgmath::Vector3<f32>, v2: cgmath::Vector3<f32>) -> f32 {
    v1.cross(v2).magnitude().atan2(v1.dot(v2))
}

pub fn xyz_to_radec(v: cgmath::Vector3<f32>) -> (f32, f32) {
    (
        v.x.atan2(v.z),
        v.y.asin(),
    )
}

pub fn ang_per_pixel_to_depth(x: f32) -> i32 {
    let depth_pixel = (((4_f32 * std::f32::consts::PI) / (12_f32 * x * x)).log2() / 2_f32).floor() as i32;

    let mut depth = depth_pixel - 9;
    if depth < 0 {
        depth = 0;
    }
    depth
}

pub fn depth_to_resolution(depth: i32) -> f32 {
    let resolution = (4_f32 * std::f32::consts::PI / (12_f32 * 4_f32.powf(depth as f32))).sqrt();
    resolution
}