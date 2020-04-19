use cgmath::Quaternion;
use cgmath::{BaseFloat, InnerSpace};
use crate::math;
use cgmath::{Vector3, Vector4};

#[derive(Clone, Copy)]
// Internal structure of a rotation, a quaternion
// All operations are done on it
pub struct SphericalRotation<S: BaseFloat>(
    Quaternion<S>,
);

use cgmath::{Matrix4, Matrix3};
impl<S> From<&Matrix4<S>> for SphericalRotation<S>
where S: BaseFloat {
    fn from(m: &Matrix4<S>) -> Self {
        let m: [[S; 4]; 4] = (*m).into();

        let t = Matrix3::new(
            m[0][0], m[0][1], m[0][2],
            m[1][0], m[1][1], m[1][2],
            m[2][0], m[2][1], m[2][2]
        );

        SphericalRotation(t.into())
    }
}
impl<S> From<&SphericalRotation<S>> for Matrix4<S>
where S: BaseFloat {
    fn from(s: &SphericalRotation<S>) -> Self {
        s.0.into()
    }
}

use cgmath::Rad;
impl<S> SphericalRotation<S>
where S: BaseFloat {
    pub fn slerp(&self, other: &SphericalRotation<S>, alpha: S) -> SphericalRotation<S> {        
        // Check if the dot of the two quaternions is negative
        // If so, negative one:
        // This comes from https://www.gamedev.net/forums/topic/551663-quaternion-rotation-issue-sometimes-take-longest-path/
        let d = self.0.dot(other.0);
        let q = if d < S::zero() {
            self.0.slerp(-other.0, alpha)  
        } else {
            self.0.slerp(other.0, alpha)
        };

        SphericalRotation(q)
    }

    pub fn get_rot_x(&self) -> Matrix4<S> {
        let mut q = self.0.clone();

        q.v.z = S::zero();
        q.v.y = S::zero();

        let norm = (q.v.x * q.v.x + q.s * q.s).sqrt();
        q.v.x /= norm;
        q.s /= norm;

        q.into()
    }

    pub fn get_rot_y(&self) -> Matrix4<S> {
        let mut q = self.0.clone();

        q.v.x = S::zero();
        q.v.z = S::zero();

        let norm = (q.v.y * q.v.y + q.s * q.s).sqrt();
        q.v.y /= norm;
        q.s /= norm;

        q.into()
    }

    pub fn get_rot_z(&self) -> Matrix4<S> {
        let mut q = self.0.clone();

        q.v.x = S::zero();
        q.v.y = S::zero();

        let norm = (q.v.z * q.v.z + q.s * q.s).sqrt();
        q.v.z /= norm;
        q.s /= norm;

        q.into()
    }

    pub fn zero() -> SphericalRotation<S> {
        let q = Quaternion::new(S::one(), S::zero(), S::zero(), S::zero());
        SphericalRotation(q)
    }

    // Define a rotation from a quaternion
    pub fn from_quaternion(q: &Quaternion<S>) -> SphericalRotation<S> {
        SphericalRotation(*q)
    }

    // Define a rotation from an axis and a angle 
    pub fn from_axis_angle<A: Into<Rad<S>>>(axis: &Vector3<S>, angle: A) -> SphericalRotation<S> {
        let mat4 = Matrix4::from_axis_angle(*axis, angle);
        (&mat4).into()
    }

    // Define a rotation from a normalized vector
    pub fn from_sky_position(pos: &Vector4<S>) -> SphericalRotation<S> {
        let (lon, lat) = math::xyzw_to_radec(&pos.normalize());

        let rot_y = Matrix4::from_angle_y(lon);
        let rot_x = Matrix4::from_angle_x(-lat);

        let mat4 = rot_y * rot_x;
        (&mat4).into()
    }

    // Apply a rotation to a position
    pub fn rotate(&self, pos_model_space: &Vector4<S>) -> Vector4<S> {
        let model2world: &Matrix4<S> = &self.into();

        let pos_world_space = model2world * pos_model_space;
        pos_world_space
    }
}

use std::ops::Mul;
impl<S> Mul<SphericalRotation<S>> for SphericalRotation<S>
where S: BaseFloat {
    type Output = Self;

    fn mul(self, rhs: SphericalRotation<S>) -> Self::Output {
        let q = self.0 * rhs.0;
        SphericalRotation(q)
    }
}