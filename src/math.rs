use cgmath::InnerSpace;
use cgmath::Rad;

pub fn angular_distance_xyz(v1: cgmath::Vector3<f32>, v2: cgmath::Vector3<f32>) -> Rad<f32> {
    Rad(v1.cross(v2).magnitude().atan2(v1.dot(v2)))
}

pub fn angular_distance_lonlat(lon1: f32, lat1: f32, lon2: f32, lat2: f32) -> f32 {
    let abs_diff_lon = (lon1 - lon2).abs();
    (lat1.sin()*lat2.sin() + lat1.cos()*lat2.cos()*abs_diff_lon.cos()).acos()
}

pub fn xyz_to_radec(v: cgmath::Vector3<f32>) -> (f32, f32) {
    (
        v.x.atan2(v.z),
        v.y.atan2((v.x*v.x + v.z*v.z).sqrt()),
    )
}

pub fn xyzw_to_radec(v: cgmath::Vector4<f32>) -> (f32, f32) {
    (
        v.x.atan2(v.z),
        v.y.atan2((v.x*v.x + v.z*v.z).sqrt()),
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

pub fn ang_per_pixel_to_depth(x: f32) -> u8 {
    let depth_pixel = (((4_f32 * std::f32::consts::PI) / (12_f32 * x * x)).log2() / 2_f32).floor() as i32;

    let mut depth = depth_pixel - 9;
    if depth < 0 {
        depth = 0;
    }
    depth as u8
}

use crate::window_size_f32;
pub fn fov_to_depth(fov: Rad<f32>) -> u8 {
    let (width_screen, _) = window_size_f32();
    let pixel_ang = fov.0 / width_screen;

    let depth_pixel = (((4_f32 * std::f32::consts::PI) / (12_f32 * pixel_ang * pixel_ang)).log2() / 2_f32).floor() as i32;

    let mut depth = depth_pixel - 9;
    if depth < 0 {
        depth = 0;
    }
    depth as u8
}

use web_sys::console;
pub fn depth_to_fov(depth: u8) -> Rad<f32> {
    let depth_pixel = depth + 9;

    let pixel_ang = Rad((4_f32 * std::f32::consts::PI / (12_f32 * ((1 << (2*depth_pixel)) as f32))).sqrt());

    let (width_screen, _) = window_size_f32();
    let fov = pixel_ang * width_screen;
    //print_to_console!("{:?} fov", fov);
    fov
}

use cgmath::Vector2;
pub fn is_inside_ellipse(screen_pos: &Vector2<f32>, a: f32, b: f32) -> bool {
    let a2 = a * a;
    let b2 = b * b;
    let px2 = screen_pos.x * screen_pos.x;
    let py2 = screen_pos.y * screen_pos.y;

    return (px2 * b2 + py2 * a2) <= a2 * b2;
}