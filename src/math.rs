use cgmath::InnerSpace;

pub fn angular_distance_xyz(v1: cgmath::Vector3<f32>, v2: cgmath::Vector3<f32>) -> f32 {
    v1.cross(v2).magnitude().atan2(v1.dot(v2))
}

pub fn angular_distance_lonlat(lon1: f32, lat1: f32, lon2: f32, lat2: f32) -> f32 {
    let abs_diff_lon = (lon1 - lon2).abs();
    (lat1.sin()*lat2.sin() + lat1.cos()*lat2.cos()*abs_diff_lon.cos()).acos()
}

pub fn xyz_to_radec(v: cgmath::Vector3<f32>) -> (f32, f32) {
    (
        v.x.atan2(v.z),
        v.y.asin(),
    )
}

pub fn radec_to_xyz(theta: cgmath::Rad<f32>, delta: cgmath::Rad<f32>) -> cgmath::Vector4<f32> {
    cgmath::Vector4::<f32>::new(
        delta.0.cos() * theta.0.sin(),
        delta.0.sin(),
        delta.0.cos() * theta.0.cos(),
        1_f32
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