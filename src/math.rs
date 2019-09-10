use cgmath::{Vector2, Vector3, InnerSpace};

pub fn angular_distance_xyz(v1: cgmath::Vector3<f32>, v2: cgmath::Vector3<f32>) -> f32 {
    v1.cross(v2).magnitude().atan2(v1.dot(v2))
}

pub fn to_ra_dec(v: cgmath::Vector3<f32>) -> (f32, f32) {
    (
        v.x.atan2(v.z),
        v.y.asin()
    )
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

// Projection methods
pub fn orthographic_projection(xh: f32, yh: f32, zh: f32) -> cgmath::Vector4<f32> {
    cgmath::Vector4::new(xh, yh, zh, 1_f32)
}

pub fn aitoff_projection(xh: f32, yh: f32, zh: f32) -> cgmath::Vector4<f32> {
    /*let mut z = 1_f32 - (0.25_f32 * xh) * (0.25_f32 * xh) - (0.5_f32 * yh) * (0.5_f32 * yh);
    z = z.sqrt();

    let theta = 2_f32 * (z*xh / (2_f32 * (2_f32*z*z - 1_f32))).atan();
    let phi = (yh * z).asin();
    */

    let u = xh * std::f32::consts::PI * 0.50_f32 ;
    let v = yh * std::f32::consts::PI ;
    //da uv a lat/lon
    let mut phi = 0_f32;
    let mut theta = 0_f32;
    let c = (v*v + u*u).sqrt();	
    if c != 0_f32{
        phi = (v * c.sin() / c).asin();
        theta = (u * c.sin()).atan2(c * c.cos());
    }
    theta *= 2_f32;

    cgmath::Vector4::new(
        theta.sin() * phi.cos(),
        phi.sin(),
        theta.cos() * phi.cos(),
        1_f32
    )
}
