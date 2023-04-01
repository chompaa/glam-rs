/*
Conversion from quaternions to Euler rotation sequences.

From: http://bediyap.com/programming/convert-quaternion-to-euler-rotations/
*/

use crate::{math, DQuat, Quat};

/// Euler rotation sequences.
///
/// The angles are applied starting from the right.
/// E.g. XYZ will first apply the z-axis rotation.
///
/// YXZ can be used for yaw (y-axis), pitch (x-axis), roll (z-axis).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum EulerRot {
    /// Intrinsic three-axis rotation ZYX
    ZYX,
    /// Intrinsic three-axis rotation ZXY
    ZXY,
    /// Intrinsic three-axis rotation YXZ
    YXZ,
    /// Intrinsic three-axis rotation YZX
    YZX,
    /// Intrinsic three-axis rotation XYZ
    XYZ,
    /// Intrinsic three-axis rotation XZY
    XZY,
}

impl Default for EulerRot {
    /// Default `YXZ` as yaw (y-axis), pitch (x-axis), roll (z-axis).
    fn default() -> Self {
        Self::YXZ
    }
}

/// Conversion from quaternion to euler angles.
pub(crate) trait EulerFromQuaternion<Q: Copy>: Sized + Copy {
    type Output;
    /// Compute the angle of the first axis (X-x-x)
    fn first(self, q: Q) -> Self::Output;
    /// Compute then angle of the second axis (x-X-x)
    fn second(self, q: Q) -> Self::Output;
    /// Compute then angle of the third axis (x-x-X)
    fn third(self, q: Q) -> Self::Output;

    /// Compute all angles of a rotation in the notation order
    fn convert_quat(self, q: Q) -> (Self::Output, Self::Output, Self::Output) {
        (self.first(q), self.second(q), self.third(q))
    }
}

/// Conversion from euler angles to quaternion.
pub(crate) trait EulerToQuaternion<T>: Copy {
    type Output;
    /// Create the rotation quaternion for the three angles of this euler rotation sequence.
    fn new_quat(self, u: T, v: T, w: T) -> Self::Output;
}

macro_rules! impl_from_quat {
    ($t:ty, $quat:ident) => {
        impl EulerFromQuaternion<$quat> for EulerRot {
            type Output = $t;
            fn first(self, q: $quat) -> $t {
                use EulerRot::*;
                match self {
                    ZYX => math::atan2(2.0 * (q.x * q.y + q.w * q.z),
                        q.w * q.w + q.x * q.x - q.y * q.y - q.z * q.z),
                    ZXY => math::atan2(-2.0 * (q.x * q.y - q.w * q.z),
                        q.w * q.w - q.x * q.x + q.y * q.y - q.z * q.z),
                    YXZ => math::atan2(2.0 * (q.x * q.z + q.w * q.y),
                        q.w * q.w - q.x * q.x - q.y * q.y + q.z * q.z),
                    YZX => math::atan2(-2.0 * (q.x * q.z - q.w * q.y),
                        q.w * q.w + q.x * q.x - q.y * q.y - q.z * q.z),
                    XYZ => math::atan2(-2.0 * (q.y * q.z - q.w * q.x),
                        q.w * q.w - q.x * q.x - q.y * q.y + q.z * q.z),
                    XZY => math::atan2(2.0 * (q.y * q.z + q.w * q.x),
                        q.w * q.w - q.x * q.x + q.y * q.y - q.z * q.z),
                }
            }

            fn second(self, q: $quat) -> $t {
                use EulerRot::*;

                /// Clamp number to range [-1,1](-1,1) for asin() and acos(), else NaN is possible.
                #[inline(always)]
                fn arc_clamp(val: $t) -> $t {
                    <$t>::min(<$t>::max(val, -1.0), 1.0)
                }

                match self {
                    ZYX => arc_clamp(math::asin(-2.0 * (q.x * q.z - q.w * q.y))),
                    ZXY => arc_clamp(math::asin(2.0 * (q.y * q.z + q.w * q.x))),
                    YXZ => arc_clamp(math::asin(-2.0 * (q.y * q.z - q.w * q.x))),
                    YZX => arc_clamp(math::asin(2.0 * (q.x * q.y + q.w * q.z))),
                    XYZ => arc_clamp(math::asin(2.0 * (q.x * q.z + q.w * q.y))),
                    XZY => arc_clamp(math::asin(-2.0 * (q.x * q.y - q.w * q.z))),
                }
            }

            fn third(self, q: $quat) -> $t {
                use EulerRot::*;
                match self {
                    ZYX => math::atan2(2.0 * (q.y * q.z + q.w * q.x),
                        q.w * q.w - q.x * q.x - q.y * q.y + q.z * q.z),
                    ZXY => math::atan2(-2.0 * (q.x * q.z - q.w * q.y),
                        q.w * q.w - q.x * q.x - q.y * q.y + q.z * q.z),
                    YXZ => math::atan2(2.0 * (q.x * q.y + q.w * q.z),
                        q.w * q.w - q.x * q.x + q.y * q.y - q.z * q.z),
                    YZX => math::atan2(-2.0 * (q.y * q.z - q.w * q.x),
                        q.w * q.w - q.x * q.x + q.y * q.y - q.z * q.z),
                    XYZ => math::atan2(-2.0 * (q.x * q.y - q.w * q.z),
                        q.w * q.w + q.x * q.x - q.y * q.y - q.z * q.z),
                    XZY => math::atan2(2.0 * (q.x * q.z + q.w * q.y),
                        q.w * q.w + q.x * q.x - q.y * q.y - q.z * q.z),
                }
            }
        }
        // End - impl EulerFromQuaternion
    };
}

macro_rules! impl_to_quat {
    ($t:ty, $quat:ident) => {
        impl EulerToQuaternion<$t> for EulerRot {
            type Output = $quat;
            #[inline(always)]
            fn new_quat(self, u: $t, v: $t, w: $t) -> $quat {
                use EulerRot::*;
                #[inline(always)]
                fn rot_x(a: $t) -> $quat {
                    $quat::from_rotation_x(a)
                }
                #[inline(always)]
                fn rot_y(a: $t) -> $quat {
                    $quat::from_rotation_y(a)
                }
                #[inline(always)]
                fn rot_z(a: $t) -> $quat {
                    $quat::from_rotation_z(a)
                }
                match self {
                    ZYX => rot_z(u) * rot_y(v) * rot_x(w),
                    ZXY => rot_z(u) * rot_x(v) * rot_y(w),
                    YXZ => rot_y(u) * rot_x(v) * rot_z(w),
                    YZX => rot_y(u) * rot_z(v) * rot_x(w),
                    XYZ => rot_x(u) * rot_y(v) * rot_z(w),
                    XZY => rot_x(u) * rot_z(v) * rot_y(w),
                }
                .normalize()
            }
        }
        // End - impl EulerToQuaternion
    };
}

impl_from_quat!(f32, Quat);
impl_from_quat!(f64, DQuat);
impl_to_quat!(f32, Quat);
impl_to_quat!(f64, DQuat);
