use core::ops::{Div, Neg};

/// Trait that provides all the math methods that we need from std.
/// This is private because it's too easy to silently end up using the std methods silently if both
/// std and libm are enabled.
trait Float : Copy + PartialEq + Neg<Output = Self> + Div<Output = Self> {
    #[inline]
    fn abs(self) -> Self {
        if self.is_sign_positive() {
            return self;
        }
        if self.is_sign_negative() {
            return -self;
        }
        Self::nan()
    }
    fn acos_clamped(self) -> Self;
    /// Returns a very close approximation of `self.clamp(-1.0, 1.0).acos()`.
    #[inline(always)]
    fn acos_approx(self) -> Self {
        Self::acos_clamped(self)
    }
    fn asin(self) -> Self;
    fn atan2(self, other: Self) -> Self;
    fn cos(self) -> Self;
    fn sin(self) -> Self;
    fn sin_cos(self) -> (Self, Self);
    #[inline(always)]
    fn rsqrt(self) -> Self { Self::one() / Self::sqrt(self) }
    fn sqrt(self) -> Self;
    fn one() -> Self;
    fn nan() -> Self;
    #[inline]
    fn is_nan(self) -> bool {
        self != self
    }
    fn is_sign_negative(self) -> bool;
    fn is_sign_positive(self) -> bool;
    fn copysign(self, sign: Self) -> Self;
    #[inline]
    fn signum(self) -> Self {
        if self.is_nan() { Self::nan() } else { Self::one().copysign(self) }
    }
}

#[inline(always)]
fn is_sign_negative_f32(v: f32) -> bool {
    let bits: u32 = v.to_bits();
    bits >> 31 != 0
}

#[inline(always)]
fn is_sign_positive_f32(v: f32) -> bool {
    let bits: u32 = v.to_bits();
    bits >> 31 == 0
}

#[inline(always)]
fn is_sign_negative_f64(v: f64) -> bool {
    let bits: u64 = v.to_bits();
    bits >> 63 != 0
}

#[inline(always)]
fn is_sign_positive_f64(v: f64) -> bool {
    let bits: u64 = v.to_bits();
    bits >> 63 == 0
}

#[inline]
fn acos_approx_f32(v: f32) -> f32 {
    // Based on https://github.com/microsoft/DirectXMath `XMScalarAcos`
    // Clamp input to [-1,1].
    let nonnegative = v >= 0.0;
    let x = abs(v);
    let mut omx = 1.0 - x;
    if omx < 0.0 {
        omx = 0.0;
    }
    let root = sqrt(omx);

    // 7-degree minimax approximation
    #[allow(clippy::approx_constant)]
    let mut result = ((((((-0.001_262_491_1 * x + 0.006_670_09) * x - 0.017_088_126) * x
                    + 0.030_891_88)
                * x
                - 0.050_174_303)
            * x
            + 0.088_978_99)
        * x
        - 0.214_598_8)
        * x
        + 1.570_796_3;
    result *= root;

    // acos(x) = pi - acos(-x) when x < 0
    if nonnegative {
        result
    } else {
        core::f32::consts::PI - result
    }
}

#[cfg(feature = "libm")]
impl Float for f32 {
    #[inline(always)]
    fn one() -> Self {
        1.0
    }
    #[inline(always)]
    fn nan() -> Self {
        f32::NAN
    }
    #[inline(always)]
    fn is_sign_negative(self) -> bool {
        is_sign_negative_f32(self)
    }
    #[inline(always)]
    fn is_sign_positive(self) -> bool {
        is_sign_positive_f32(self)
    }
    #[inline(always)]
    fn copysign(self, sign: Self) -> Self {
        libm::copysignf(self, sign)
    }
    #[inline(always)]
    fn asin(self) -> Self {
        libm::asinf(self)
    }
    #[inline(always)]
    fn atan2(self, other: Self) -> Self {
        libm::atan2f(self, other)
    }
    #[inline(always)]
    fn cos(self) -> Self {
        libm::cosf(self)
    }
    #[inline(always)]
    fn sin(self) -> Self {
        libm::sinf(self)
    }
    #[inline(always)]
    fn sin_cos(self) -> (Self, Self) {
        libm::sincosf(self)
    }
    #[inline(always)]
    fn sqrt(self) -> Self {
        libm::sqrtf(self)
    }
    #[inline(always)]
    fn acos_clamped(self) -> Self {
        libm::acosf(self.clamp(-1.0, 1.0))
    }
    #[inline(always)]
    fn acos_approx(self) -> Self {
        acos_approx_f32(self)
    }
}

#[cfg(feature = "libm")]
impl Float for f64 {
    #[inline(always)]
    fn one() -> Self {
        1.0
    }
    #[inline(always)]
    fn nan() -> Self {
        f64::NAN
    }
    #[inline(always)]
    fn is_sign_negative(self) -> bool {
        is_sign_negative_f64(self)
    }
    #[inline(always)]
    fn is_sign_positive(self) -> bool {
        is_sign_positive_f64(self)
    }
    #[inline(always)]
    fn copysign(self, sign: Self) -> Self {
        libm::copysign(self, sign)
    }
    #[inline(always)]
    fn asin(self) -> Self {
        libm::asin(self)
    }
    #[inline(always)]
    fn atan2(self, other: Self) -> Self {
        libm::atan2(self, other)
    }
    #[inline(always)]
    fn cos(self) -> Self {
        libm::cos(self)
    }
    #[inline(always)]
    fn sin(self) -> Self {
        libm::sin(self)
    }
    #[inline(always)]
    fn sin_cos(self) -> (Self, Self) {
        libm::sincos(self)
    }
    #[inline(always)]
    fn sqrt(self) -> Self {
        libm::sqrt(self)
    }
    #[inline(always)]
    fn acos_clamped(self) -> Self {
        libm::acos(self.clamp(-1.0, 1.0))
    }
}

#[cfg(not(feature = "libm"))]
impl Float for f32 {
    #[inline(always)]
    fn one() -> Self {
        1.0
    }
    #[inline(always)]
    fn nan() -> Self {
        f32::NAN
    }
    #[inline(always)]
    fn is_sign_negative(self) -> bool {
        is_sign_negative_f32(self)
    }
    #[inline(always)]
    fn is_sign_positive(self) -> bool {
        is_sign_positive_f32(self)
    }
    #[inline(always)]
    fn copysign(self, sign: Self) -> Self {
        f32::copysign(self, sign)
    }
    #[inline(always)]
    fn asin(self) -> Self {
        f32::asin(self)
    }
    #[inline(always)]
    fn atan2(self, other: Self) -> Self {
        f32::atan2(self, other)
    }
    #[inline(always)]
    fn cos(self) -> Self {
        f32::cos(self)
    }
    #[inline(always)]
    fn sin(self) -> Self {
        f32::sin(self)
    }
    #[inline(always)]
    fn sin_cos(self) -> (Self, Self) {
        f32::sin_cos(self)
    }
    #[inline(always)]
    fn sqrt(self) -> Self {
        f32::sqrt(self)
    }
    #[inline(always)]
    fn acos_clamped(self) -> Self {
        f32::acos(self.clamp(-1.0, 1.0))
    }
    #[inline(always)]
    fn acos_approx(self) -> Self {
        acos_approx_f32(self)
    }
}

#[cfg(not(feature = "libm"))]
impl Float for f64 {
    #[inline(always)]
    fn one() -> Self {
        1.0
    }
    #[inline(always)]
    fn nan() -> Self {
        f64::NAN
    }
    #[inline(always)]
    fn is_sign_negative(self) -> bool {
        is_sign_negative_f64(self)
    }
    #[inline(always)]
    fn is_sign_positive(self) -> bool {
        is_sign_positive_f64(self)
    }
    #[inline(always)]
    fn copysign(self, sign: Self) -> Self {
        f64::copysign(self, sign)
    }
    #[inline(always)]
    fn asin(self) -> Self {
        f64::asin(self)
    }
    #[inline(always)]
    fn atan2(self, other: Self) -> Self {
        f64::atan2(self, other)
    }
    #[inline(always)]
    fn cos(self) -> Self {
        f64::cos(self)
    }
    #[inline(always)]
    fn sin(self) -> Self {
        f64::sin(self)
    }
    #[inline(always)]
    fn sin_cos(self) -> (Self, Self) {
        f64::sin_cos(self)
    }
    #[inline(always)]
    fn sqrt(self) -> Self {
        f64::sqrt(self)
    }
    #[inline(always)]
    fn acos_clamped(self) -> Self {
        f64::acos(self)
    }
}

#[inline(always)]
pub(crate) fn abs<T: Float>(f: T) -> T {
    Float::abs(f)
}

#[inline(always)]
pub(crate) fn atan2<T: Float>(f: T, other: T) -> T {
    Float::atan2(f, other)
}

#[inline(always)]
pub(crate) fn asin<T: Float>(f: T) -> T {
    Float::asin(f)
}

#[inline(always)]
pub(crate) fn sin<T: Float>(f: T) -> T {
    Float::sin(f)
}

#[inline(always)]
pub(crate) fn cos<T: Float>(f: T) -> T {
    Float::cos(f)
}

#[inline(always)]
pub(crate) fn sin_cos<T: Float>(f: T) -> (T, T) {
    Float::sin_cos(f)
}

#[inline(always)]
pub(crate) fn acos_clamped<T: Float>(f: T) -> T {
    Float::acos_clamped(f)
}

#[inline(always)]
pub(crate) fn acos_approx<T: Float>(f: T) -> T {
    Float::acos_approx(f)
}

#[inline(always)]
pub(crate) fn rsqrt<T: Float>(f: T) -> T {
    Float::rsqrt(f)
}

#[inline(always)]
pub(crate) fn sqrt<T: Float>(f: T) -> T {
    Float::sqrt(f)
}

#[inline(always)]
pub(crate) fn copysign<T: Float>(f: T, sign: T) -> T {
    Float::copysign(f, sign)
}

#[inline(always)]
pub(crate) fn signum<T: Float>(f: T) -> T {
    Float::signum(f)
}

