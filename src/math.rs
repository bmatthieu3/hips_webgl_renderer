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