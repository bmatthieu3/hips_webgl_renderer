use cgmath::{InnerSpace, BaseFloat, Angle};
use cgmath::Rad;
use cgmath::{Vector4, Vector3};
#[inline]
pub fn ang_between_vect<S: BaseFloat>(x: &Vector3<S>, y: &cgmath::Vector3<S>) -> Rad<S> {
    Rad(x.cross(*y).magnitude().atan2(x.dot(*y)))
}

#[inline]
pub fn ang_between_lonlat<S: BaseFloat>(lon1: Rad<S>, lat1: Rad<S>, lon2: Rad<S>, lat2: Rad<S>) -> Rad<S> {
    let abs_diff_lon = (lon1 - lon2).0.abs();
    Rad(
        (lat1.sin()*lat2.sin() + lat1.cos()*lat2.cos()*abs_diff_lon.cos()).acos()
    )
}

#[inline]
pub fn xyz_to_radec<S: BaseFloat>(v: &cgmath::Vector3<S>) -> (Rad<S>, Rad<S>) {
    let lon = Rad(v.x.atan2(v.z));
    let lat = Rad(v.y.atan2((v.x*v.x + v.z*v.z).sqrt()));

    (lon, lat)
}

#[inline]
pub fn xyzw_to_radec<S: BaseFloat>(v: &cgmath::Vector4<S>) -> (Rad<S>, Rad<S>) {
    let lon = Rad(v.x.atan2(v.z));
    let lat = Rad(v.y.atan2((v.x*v.x + v.z*v.z).sqrt()));

    (lon, lat)
}

#[inline]
pub fn radec_to_xyzw<S: BaseFloat>(theta: Rad<S>, delta: Rad<S>) -> Vector4<S> {
    Vector4::<S>::new(
        delta.cos() * theta.sin(),
        delta.sin(),
        delta.cos() * theta.cos(),
        S::one()
    )
}

#[inline]
pub fn radec_to_xyz<S: BaseFloat>(theta: Rad<S>, delta: Rad<S>) -> Vector3<S> {
    Vector3::<S>::new(
        delta.cos() * theta.sin(),
        delta.sin(),
        delta.cos() * theta.cos(),
    )
}

/*pub fn ang_per_pixel_to_depth(x: f32) -> u8 {
    let depth_pixel = (((4_f32 * std::f32::consts::PI) / (12_f32 * x * x)).log2() / 2_f32).floor() as i32;

    let mut depth = depth_pixel - 9;
    if depth < 0 {
        depth = 0;
    }
    depth as u8
}*/

// Used for selecting the current depth for a given FOV
// We need to select a depth so that we do not see any pixels
// This takes into account the screen resolution and can impact
// the number of healpix cells to load. Bigger resolution will need
// more cells which can overfit the buffer!
use crate::buffer::TileConfig;
pub fn fov_to_depth(fov: Rad<f32>, width: f32, tile_config: &TileConfig) -> u8 {
    let pixel_ang = fov.0 / width;

    let depth_pixel = (((4_f32 * std::f32::consts::PI) / (12_f32 * pixel_ang * pixel_ang)).log2() / 2_f32).ceil() as i8;

    // The texture size in pixels
    let texture_size = tile_config.get_texture_size();
    // The depth of the texture
    // A texture of 512x512 pixels will have a depth of 9
    let depth_offset_texture = log_2(texture_size);
    // The depth of the texture corresponds to the depth of a pixel
    // minus the offset depth of the texture
    let mut depth_texture = depth_pixel - depth_offset_texture;
    if depth_texture < 0 {
        depth_texture = 0;
    }
    depth_texture as u8
}
/*
pub fn depth_to_fov(depth: u8) -> Rad<f32> {
    let sphere_area = 4_f32 * std::f32::consts::PI;
    let num_hpx_cells = 12_f32 * 4_f32.powf(depth as f32);
    let hpx_cell_ang = Rad((sphere_area / num_hpx_cells).sqrt());

    hpx_cell_ang
}
*/
use cgmath::Vector2;
pub fn is_inside_ellipse(screen_pos: &Vector2<f32>, a: f32, b: f32) -> bool {
    let a2 = a * a;
    let b2 = b * b;
    let px2 = screen_pos.x * screen_pos.x;
    let py2 = screen_pos.y * screen_pos.y;

    return (px2 * b2 + py2 * a2) <= a2 * b2;
}

#[inline]
const fn num_bits<T>() -> usize { std::mem::size_of::<T>() * 8 }

#[inline]
pub fn log_2(x: i32) -> i8 {
    assert!(x > 0);
    (num_bits::<i32>() as u32 - x.leading_zeros() - 1) as i8
}
