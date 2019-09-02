use cgmath::{Vector2, Vector3, InnerSpace};

pub fn angular_distance_xyz(v1: cgmath::Vector3<f32>, v2: cgmath::Vector3<f32>) -> f32 {
    v1.cross(v2).magnitude().atan2(v1.dot(v2))
}

pub fn angular_distance(a: cgmath::Vector2<f32>, b: cgmath::Vector2<f32>) -> f32 {
    let r = a.y.sin()*b.y.sin() + a.y.cos()*b.y.cos()*(a.x - b.x).cos();
    r.acos()
}

pub fn angular_distance_haversine(a: cgmath::Vector2<f32>, b: cgmath::Vector2<f32>) -> f32 {
    let A = ((b.y - a.y)/2_f32).sin();
    let B = ((b.x - a.x)/2_f32).sin();

    let r = A*A + a.y.cos()*b.y.cos()*B*B;
    2_f32*r.sqrt().asin()
}