use core::fmt;
use core::ops;

use super::Float2;
use super::Float4;

/// Vector containing 3 floating point values.
#[derive(Copy, Clone, Debug)]
pub struct Float3 {
    /// The x-component of the vector.
    pub x: f32,
    /// The y-component of the vector.
    pub y: f32,
    /// The z-component of the vector.
    pub z: f32,
}

impl fmt::Display for Float3 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Float3 ({}, {}, {})", self.x, self.y, self.z)
    }
}

impl ops::Add<Self> for Float3 {
    type Output = Self;

    fn add(self, rhs: Self) -> Self {
        Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z,
        }
    }
}

impl ops::Add<f32> for Float3 {
    type Output = Self;

    fn add(self, rhs: f32) -> Self {
        Self {
            x: self.x + rhs,
            y: self.y + rhs,
            z: self.z + rhs,
        }
    }
}

impl ops::AddAssign<Self> for Float3 {
    fn add_assign(&mut self, rhs: Self) {
        self.x += rhs.x;
        self.y += rhs.y;
        self.z += rhs.z;
    }
}

impl ops::AddAssign<f32> for Float3 {
    fn add_assign(&mut self, rhs: f32) {
        self.x += rhs;
        self.y += rhs;
        self.z += rhs;
    }
}

impl ops::Sub<Self> for Float3 {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self {
        Self {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
            z: self.z - rhs.z,
        }
    }
}

impl ops::Sub<f32> for Float3 {
    type Output = Self;

    fn sub(self, rhs: f32) -> Self {
        Self {
            x: self.x - rhs,
            y: self.y - rhs,
            z: self.z - rhs,
        }
    }
}

impl ops::SubAssign<Self> for Float3 {
    fn sub_assign(&mut self, rhs: Self) {
        self.x -= rhs.x;
        self.y -= rhs.y;
        self.z -= rhs.z;
    }
}

impl ops::SubAssign<f32> for Float3 {
    fn sub_assign(&mut self, rhs: f32) {
        self.x -= rhs;
        self.y -= rhs;
        self.z -= rhs;
    }
}

impl ops::Mul<Self> for Float3 {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self {
        Self {
            x: self.x * rhs.x,
            y: self.y * rhs.y,
            z: self.z * rhs.z,
        }
    }
}

impl ops::Mul<f32> for Float3 {
    type Output = Self;

    fn mul(self, rhs: f32) -> Self {
        Self {
            x: self.x * rhs,
            y: self.y * rhs,
            z: self.z * rhs,
        }
    }
}

impl ops::MulAssign<Self> for Float3 {
    fn mul_assign(&mut self, rhs: Self) {
        self.x *= rhs.x;
        self.y *= rhs.y;
        self.z *= rhs.z;
    }
}

impl ops::MulAssign<f32> for Float3 {
    fn mul_assign(&mut self, rhs: f32) {
        self.x *= rhs;
        self.y *= rhs;
        self.z *= rhs;
    }
}

impl ops::Div<Self> for Float3 {
    type Output = Self;

    fn div(self, rhs: Self) -> Self {
        Self {
            x: self.x / rhs.x,
            y: self.y / rhs.y,
            z: self.z / rhs.z,
        }
    }
}

impl ops::Div<f32> for Float3 {
    type Output = Self;

    fn div(self, rhs: f32) -> Self {
        Self {
            x: self.x / rhs,
            y: self.y / rhs,
            z: self.z / rhs,
        }
    }
}

impl ops::DivAssign<Self> for Float3 {
    fn div_assign(&mut self, rhs: Self) {
        self.x /= rhs.x;
        self.y /= rhs.y;
        self.z /= rhs.z;
    }
}

impl ops::DivAssign<f32> for Float3 {
    fn div_assign(&mut self, rhs: f32) {
        self.x /= rhs;
        self.y /= rhs;
        self.z /= rhs;
    }
}

impl ops::Neg for Float3 {
    type Output = Self;

    fn neg(self) -> Self {
        Self {
            x: -self.x,
            y: -self.y,
            z: -self.z,
        }
    }
}

impl PartialEq for Float3 {
    fn eq(&self, rhs: &Self) -> bool {
        self.x == rhs.x && self.y == rhs.y && self.z == rhs.z
    }
}

impl From<f32> for Float3 {
    fn from(lhs: f32) -> Self {
        Self {
            x: lhs,
            y: lhs,
            z: lhs,
        }
    }
}

impl From<(f32, f32, f32)> for Float3 {
    fn from(lhs: (f32, f32, f32)) -> Self {
        Self {
            x: lhs.0,
            y: lhs.1,
            z: lhs.2,
        }
    }
}

impl Float3 {
    /// Creates a vector from 3 floating point values.
    pub fn new(x: f32, y: f32, z: f32) -> Self {
        Self { x, y, z }
    }

    /// Computes the per-component absolute numbers.
    pub fn abs(&self) -> Self {
        Self {
            x: self.x.abs(),
            y: self.y.abs(),
            z: self.z.abs(),
        }
    }

    /// Computes the per-component arccosine numbers (in radians).
    /// Each component should be a value within the range of -1 to 1.
    /// Return values are in the range 0, pi or NaN if the number is outside the range -1 to 1.
    pub fn acos(&self) -> Self {
        Self {
            x: self.x.acos(),
            y: self.y.acos(),
            z: self.z.acos(),
        }
    }

    /// Determines if all components of the vector are non-zero.
    pub fn all(&self) -> bool {
        self.x != 0.0 && self.y != 0.0 && self.z != 0.0
    }

    /// Determines if any components of the vector are non-zero.
    pub fn any(&self) -> bool {
        self.x != 0.0 || self.y != 0.0 || self.z != 0.0
    }

    /// Each component should be a value within the range of -1 to 1.
    /// Return values are in the range -pi/2 to pi/2 or NaN if the number is outside the range -1 to 1.
    pub fn asin(&self) -> Self {
        Self {
            x: self.x.asin(),
            y: self.y.asin(),
            z: self.z.asin(),
        }
    }

    /// Computes the per-component arctangent numbers (in radians).
    /// Return values are in the range -pi/2 to pi/2.
    pub fn atan(&self) -> Self {
        Self {
            x: self.x.atan(),
            y: self.y.atan(),
            z: self.z.atan(),
        }
    }

    /// Computes the per-component smallest integers greater than or equal to `self.x` and `self.y` and `self.z` respectively.
    pub fn ceil(&self) -> Self {
        Self {
            x: self.x.ceil(),
            y: self.y.ceil(),
            z: self.z.ceil(),
        }
    }

    /// Computes the per-component clamped numbers between `min` and `max`.
    pub fn clamp(&self, min: f32, max: f32) -> Self {
        Self {
            x: self.x.clamp(min, max),
            y: self.y.clamp(min, max),
            z: self.z.clamp(min, max),
        }
    }

    /// Computes the per-component cosine numbers (in radians).
    pub fn cos(&self) -> Self {
        Self {
            x: self.x.cos(),
            y: self.y.cos(),
            z: self.z.cos(),
        }
    }

    /// Computes the per-component hyperbolic cosine numbers.
    pub fn cosh(&self) -> Self {
        Self {
            x: self.x.cosh(),
            y: self.y.cosh(),
            z: self.z.cosh(),
        }
    }

    /// Converts the per-component numbers from radians to degrees.
    pub fn degrees(&self) -> Self {
        Self {
            x: self.x.to_degrees(),
            y: self.y.to_degrees(),
            z: self.z.to_degrees(),
        }
    }

    /// Computes the distance scalar between two vectors.
    pub fn distance(&self, rhs: &Self) -> f32 {
        let dx = self.x - rhs.x;
        let dy = self.y - rhs.y;
        let dz = self.z - rhs.z;
        (dx * dx + dy * dy + dz * dz).sqrt()
    }

    /// Computes the dot product of two vectors.
    pub fn dot(&self, rhs: &Self) -> f32 {
        self.x * rhs.x + self.y * rhs.y + self.z * rhs.z
    }

    /// Computes the per-component e^(self), the exponential function.
    pub fn exp(&self) -> Self {
        Self {
            x: self.x.exp(),
            y: self.y.exp(),
            z: self.z.exp(),
        }
    }

    /// Computes the per-component 2^(self).
    pub fn exp2(&self) -> Self {
        Self {
            x: self.x.exp2(),
            y: self.y.exp2(),
            z: self.z.exp2(),
        }
    }

    /// Computes the per-component largest integers less than or equal to `self.x` and `self.y` and `self.z` respectively.
    pub fn floor(&self) -> Self {
        Self {
            x: self.x.floor(),
            y: self.y.floor(),
            z: self.z.floor(),
        }
    }

    /// Computes the floating-point remainder of division for each component.
    pub fn fmod(&self, rhs: &Self) -> Self {
        Self {
            x: self.x % rhs.x,
            y: self.y % rhs.y,
            z: self.z % rhs.z,
        }
    }

    /// Computes the fractional (or decimal) part of each component; which is greater than or equal to 0 and less than 1.
    pub fn frac(&self) -> Self {
        Self {
            x: self.x.fract(),
            y: self.y.fract(),
            z: self.z.fract(),
        }
    }

    /// Computes `value * 2^exponent` for each component of the vector.
    pub fn ldexp(&self, exponent: &Self) -> Self {
        Self {
            x: self.x * (2.0f32).powi(exponent.x as i32),
            y: self.y * (2.0f32).powi(exponent.y as i32),
            z: self.z * (2.0f32).powi(exponent.z as i32),
        }
    }

    /// Computes the length scalar between two vectors.
    pub fn length(&self) -> f32 {
        (self.x * self.x + self.y * self.y + self.z * self.z).sqrt()
    }

    /// Computes a linear interpolation between two vectors.
    pub fn lerp(&self, rhs: &Self, t: f32) -> Self {
        Self {
            x: self.x + t * (rhs.x - self.x),
            y: self.y + t * (rhs.y - self.y),
            z: self.z + t * (rhs.z - self.z),
        }
    }

    /// Computes the natural logarithm (base e) of each component of the vector.
    pub fn log(&self) -> Self {
        Self {
            x: self.x.ln(),
            y: self.y.ln(),
            z: self.z.ln(),
        }
    }

    /// Computes the natural logarithm (base 10) of each component of the vector.
    pub fn log10(&self) -> Self {
        Self {
            x: self.x.log10(),
            y: self.y.log10(),
            z: self.z.log10(),
        }
    }

    /// Computes the natural logarithm (base 2) of each component of the vector.
    pub fn log2(&self) -> Self {
        Self {
            x: self.x.log2(),
            y: self.y.log2(),
            z: self.z.log2(),
        }
    }

    /// Computes the multiply-add operation: (self * b) + c.
    pub fn mad(&self, b: &Self, c: &Self) -> Self {
        Self {
            x: self.x * b.x + c.x,
            y: self.y * b.y + c.y,
            z: self.z * b.z + c.z,
        }
    }

    /// Computes the component-wise maximum of two vectors.
    pub fn max(&self, rhs: &Self) -> Self {
        Self {
            x: self.x.max(rhs.x),
            y: self.y.max(rhs.y),
            z: self.z.max(rhs.z),
        }
    }

    /// Computes the component-wise minimum of two vectors.
    pub fn min(&self, rhs: &Self) -> Self {
        Self {
            x: self.x.min(rhs.x),
            y: self.y.min(rhs.y),
            z: self.z.min(rhs.z),
        }
    }

    /// Computes the normalized vector (unit vector) of `self`.
    pub fn normalize(&self) -> Self {
        let length = self.length();
        if length != 0.0 {
            Self {
                x: self.x / length,
                y: self.y / length,
                z: self.z / length,
            }
        } else {
            // return the zero vector if the input vector has zero length.
            Self {
                x: 0.0,
                y: 0.0,
                z: 0.0,
            }
        }
    }

    /// Computes the component-wise power: `self^exponent`.
    pub fn pow(&self, exponent: f32) -> Self {
        Self {
            x: self.x.powf(exponent),
            y: self.y.powf(exponent),
            z: self.z.powf(exponent),
        }
    }

    /// Converts the per-component numbers from degrees to radians.
    pub fn radians(&self) -> Self {
        Self {
            x: self.x.to_radians(),
            y: self.y.to_radians(),
            z: self.z.to_radians(),
        }
    }

    /// Computes the reciprocal of each component of the vector.
    /// Equivalent to `1 / self`.
    pub fn rcp(&self) -> Self {
        Self {
            x: 1.0 / self.x,
            y: 1.0 / self.y,
            z: 1.0 / self.z,
        }
    }

    /// Computes the reciprocal of each component of the vector.
    /// Equivalent to `1 / self` and returning 0 when self is 0.
    pub fn rcp_safe(&self) -> Self {
        Self {
            x: if self.x != 0.0 { 1.0 / self.x } else { 0.0 },
            y: if self.y != 0.0 { 1.0 / self.y } else { 0.0 },
            z: if self.z != 0.0 { 1.0 / self.z } else { 0.0 },
        }
    }

    /// Computes the reflection of an incident vector `self` about a normal vector `normal`.
    pub fn reflect(&self, normal: &Self) -> Self {
        let dot = self.dot(normal);
        Self {
            x: self.x - 2.0 * dot * normal.x,
            y: self.y - 2.0 * dot * normal.y,
            z: self.z - 2.0 * dot * normal.z,
        }
    }

    /// Computes the refraction vector for the given incident vector, normal, and refraction index.
    pub fn refract(&self, normal: &Self, eta: f32) -> Self {
        let dot_n_i = self.dot(normal);
        let k = 1.0 - eta * eta * (1.0 - dot_n_i * dot_n_i);
        if k < 0.0 {
            Self {
                x: 0.0,
                y: 0.0,
                z: 0.0,
            }
        } else {
            let scale_i = eta;
            let scale_n = eta * dot_n_i + k.sqrt();
            Self {
                x: scale_i * self.x - scale_n * normal.x,
                y: scale_i * self.y - scale_n * normal.y,
                z: scale_i * self.z - scale_n * normal.z,
            }
        }
    }

    /// Rounds each component of the vector to the nearest integer.
    pub fn round(&self) -> Self {
        Self {
            x: self.x.round(),
            y: self.y.round(),
            z: self.z.round(),
        }
    }

    /// Computes the reciprocal square root of each component of the vector.
    pub fn rsqrt(&self) -> Self {
        Self {
            x: if self.x != 0.0 {
                1.0 / self.x.sqrt()
            } else {
                f32::INFINITY
            },
            y: if self.y != 0.0 {
                1.0 / self.y.sqrt()
            } else {
                f32::INFINITY
            },
            z: if self.z != 0.0 {
                1.0 / self.z.sqrt()
            } else {
                f32::INFINITY
            },
        }
    }

    /// Clamps each component of the vector to the range [0, 1].
    pub fn saturate(&self) -> Self {
        Self {
            x: self.x.clamp(0.0, 1.0),
            y: self.y.clamp(0.0, 1.0),
            z: self.z.clamp(0.0, 1.0),
        }
    }

    /// Computes the sign of each component of the vector.
    pub fn sign(&self) -> Self {
        Self {
            x: self.x.signum(),
            y: self.y.signum(),
            z: self.z.signum(),
        }
    }

    /// Computes the per-component sine numbers (in radians).
    pub fn sin(&self) -> Self {
        Self {
            x: self.x.sin(),
            y: self.y.sin(),
            z: self.z.sin(),
        }
    }

    /// Computes the per-component hyperbolic sine numbers.
    pub fn sinh(&self) -> Self {
        Self {
            x: self.x.sinh(),
            y: self.y.sinh(),
            z: self.z.sinh(),
        }
    }

    /// Performs smoothstep interpolation on each component of the vector.
    pub fn smoothstep(&self, min: &Self, max: &Self) -> Self {
        fn smoothstep_component(min: f32, max: f32, value: f32) -> f32 {
            if value <= min {
                0.0
            } else if value >= max {
                1.0
            } else {
                let t = (value - min) / (max - min);
                t * t * (3.0 - 2.0 * t)
            }
        }

        Self {
            x: smoothstep_component(min.x, max.x, self.x),
            y: smoothstep_component(min.y, max.y, self.y),
            z: smoothstep_component(min.z, max.z, self.z),
        }
    }

    /// Computes the square root of each component of the vector.
    pub fn sqrt(&self) -> Self {
        Self {
            x: self.x.sqrt(),
            y: self.y.sqrt(),
            z: self.z.sqrt(),
        }
    }

    /// Computes the component-wise step function.
    /// For each component: returns 0.0 if `self < edge`, else returns 1.0.
    pub fn step(&self, edge: &Self) -> Self {
        Self {
            x: if self.x < edge.x { 0.0 } else { 1.0 },
            y: if self.y < edge.y { 0.0 } else { 1.0 },
            z: if self.z < edge.z { 0.0 } else { 1.0 },
        }
    }

    /// Computes the per-component tangent numbers (in radians).
    pub fn tan(&self) -> Self {
        Self {
            x: self.x.tan(),
            y: self.y.tan(),
            z: self.z.tan(),
        }
    }

    /// Computes the per-component hyperbolic tangent numbers.
    pub fn tanh(&self) -> Self {
        Self {
            x: self.x.tanh(),
            y: self.y.tanh(),
            z: self.z.tanh(),
        }
    }

    /// Truncates each component of the vector to its integer portion.
    pub fn trunc(&self) -> Self {
        Self {
            x: self.x.trunc(),
            y: self.y.trunc(),
            z: self.z.trunc(),
        }
    }

    /// Returns a swizzled vector.
    pub fn xx(&self) -> Float2 {
        Float2 {
            x: self.x,
            y: self.x,
        }
    }

    /// Returns a swizzled vector.
    pub fn xy(&self) -> Float2 {
        Float2 {
            x: self.x,
            y: self.y,
        }
    }

    /// Returns a swizzled vector.
    pub fn xz(&self) -> Float2 {
        Float2 {
            x: self.x,
            y: self.z,
        }
    }

    /// Returns a swizzled vector.
    pub fn yx(&self) -> Float2 {
        Float2 {
            x: self.y,
            y: self.x,
        }
    }

    /// Returns a swizzled vector.
    pub fn yy(&self) -> Float2 {
        Float2 {
            x: self.y,
            y: self.y,
        }
    }

    /// Returns a swizzled vector.
    pub fn yz(&self) -> Float2 {
        Float2 {
            x: self.y,
            y: self.z,
        }
    }

    /// Returns a swizzled vector.
    pub fn zx(&self) -> Float2 {
        Float2 {
            x: self.z,
            y: self.x,
        }
    }

    /// Returns a swizzled vector.
    pub fn zy(&self) -> Float2 {
        Float2 {
            x: self.z,
            y: self.y,
        }
    }

    /// Returns a swizzled vector.
    pub fn zz(&self) -> Float2 {
        Float2 {
            x: self.z,
            y: self.z,
        }
    }

    /// Returns a swizzled vector.
    pub fn xxx(&self) -> Float3 {
        Float3 {
            x: self.x,
            y: self.x,
            z: self.x,
        }
    }

    /// Returns a swizzled vector.
    pub fn xxy(&self) -> Float3 {
        Float3 {
            x: self.x,
            y: self.x,
            z: self.y,
        }
    }

    /// Returns a swizzled vector.
    pub fn xxz(&self) -> Float3 {
        Float3 {
            x: self.x,
            y: self.x,
            z: self.z,
        }
    }

    /// Returns a swizzled vector.
    pub fn xyx(&self) -> Float3 {
        Float3 {
            x: self.x,
            y: self.y,
            z: self.x,
        }
    }

    /// Returns a swizzled vector.
    pub fn xyy(&self) -> Float3 {
        Float3 {
            x: self.x,
            y: self.y,
            z: self.y,
        }
    }

    /// Returns a swizzled vector.
    pub fn xyz(&self) -> Float3 {
        Float3 {
            x: self.x,
            y: self.y,
            z: self.z,
        }
    }

    /// Returns a swizzled vector.
    pub fn xzx(&self) -> Float3 {
        Float3 {
            x: self.x,
            y: self.z,
            z: self.x,
        }
    }

    /// Returns a swizzled vector.
    pub fn xzy(&self) -> Float3 {
        Float3 {
            x: self.x,
            y: self.z,
            z: self.y,
        }
    }

    /// Returns a swizzled vector.
    pub fn xzz(&self) -> Float3 {
        Float3 {
            x: self.x,
            y: self.z,
            z: self.z,
        }
    }

    /// Returns a swizzled vector.
    pub fn yxx(&self) -> Float3 {
        Float3 {
            x: self.y,
            y: self.x,
            z: self.x,
        }
    }

    /// Returns a swizzled vector.
    pub fn yxy(&self) -> Float3 {
        Float3 {
            x: self.y,
            y: self.x,
            z: self.y,
        }
    }

    /// Returns a swizzled vector.
    pub fn yxz(&self) -> Float3 {
        Float3 {
            x: self.y,
            y: self.x,
            z: self.z,
        }
    }

    /// Returns a swizzled vector.
    pub fn yyx(&self) -> Float3 {
        Float3 {
            x: self.y,
            y: self.y,
            z: self.x,
        }
    }

    /// Returns a swizzled vector.
    pub fn yyy(&self) -> Float3 {
        Float3 {
            x: self.y,
            y: self.y,
            z: self.y,
        }
    }

    /// Returns a swizzled vector.
    pub fn yyz(&self) -> Float3 {
        Float3 {
            x: self.y,
            y: self.y,
            z: self.z,
        }
    }

    /// Returns a swizzled vector.
    pub fn yzx(&self) -> Float3 {
        Float3 {
            x: self.y,
            y: self.z,
            z: self.x,
        }
    }

    /// Returns a swizzled vector.
    pub fn yzy(&self) -> Float3 {
        Float3 {
            x: self.y,
            y: self.z,
            z: self.y,
        }
    }

    /// Returns a swizzled vector.
    pub fn yzz(&self) -> Float3 {
        Float3 {
            x: self.y,
            y: self.z,
            z: self.z,
        }
    }

    /// Returns a swizzled vector.
    pub fn zxx(&self) -> Float3 {
        Float3 {
            x: self.z,
            y: self.x,
            z: self.x,
        }
    }

    /// Returns a swizzled vector.
    pub fn zxy(&self) -> Float3 {
        Float3 {
            x: self.z,
            y: self.x,
            z: self.y,
        }
    }

    /// Returns a swizzled vector.
    pub fn zxz(&self) -> Float3 {
        Float3 {
            x: self.z,
            y: self.x,
            z: self.z,
        }
    }

    /// Returns a swizzled vector.
    pub fn zyx(&self) -> Float3 {
        Float3 {
            x: self.z,
            y: self.y,
            z: self.x,
        }
    }

    /// Returns a swizzled vector.
    pub fn zyy(&self) -> Float3 {
        Float3 {
            x: self.z,
            y: self.y,
            z: self.y,
        }
    }

    /// Returns a swizzled vector.
    pub fn zyz(&self) -> Float3 {
        Float3 {
            x: self.z,
            y: self.y,
            z: self.z,
        }
    }

    /// Returns a swizzled vector.
    pub fn zzx(&self) -> Float3 {
        Float3 {
            x: self.z,
            y: self.z,
            z: self.x,
        }
    }

    /// Returns a swizzled vector.
    pub fn zzy(&self) -> Float3 {
        Float3 {
            x: self.z,
            y: self.z,
            z: self.y,
        }
    }

    /// Returns a swizzled vector.
    pub fn zzz(&self) -> Float3 {
        Float3 {
            x: self.z,
            y: self.z,
            z: self.z,
        }
    }

    /// Returns a swizzled vector.
    pub fn xxxx(&self) -> Float4 {
        Float4 {
            x: self.x,
            y: self.x,
            z: self.x,
            w: self.x,
        }
    }

    /// Returns a swizzled vector.
    pub fn xxxy(&self) -> Float4 {
        Float4 {
            x: self.x,
            y: self.x,
            z: self.x,
            w: self.y,
        }
    }

    /// Returns a swizzled vector.
    pub fn xxxz(&self) -> Float4 {
        Float4 {
            x: self.x,
            y: self.x,
            z: self.x,
            w: self.z,
        }
    }

    /// Returns a swizzled vector.
    pub fn xxyx(&self) -> Float4 {
        Float4 {
            x: self.x,
            y: self.x,
            z: self.y,
            w: self.x,
        }
    }

    /// Returns a swizzled vector.
    pub fn xxyy(&self) -> Float4 {
        Float4 {
            x: self.x,
            y: self.x,
            z: self.y,
            w: self.y,
        }
    }

    /// Returns a swizzled vector.
    pub fn xxyz(&self) -> Float4 {
        Float4 {
            x: self.x,
            y: self.x,
            z: self.y,
            w: self.z,
        }
    }

    /// Returns a swizzled vector.
    pub fn xxzx(&self) -> Float4 {
        Float4 {
            x: self.x,
            y: self.x,
            z: self.z,
            w: self.x,
        }
    }

    /// Returns a swizzled vector.
    pub fn xxzy(&self) -> Float4 {
        Float4 {
            x: self.x,
            y: self.x,
            z: self.z,
            w: self.y,
        }
    }

    /// Returns a swizzled vector.
    pub fn xxzz(&self) -> Float4 {
        Float4 {
            x: self.x,
            y: self.x,
            z: self.z,
            w: self.z,
        }
    }

    /// Returns a swizzled vector.
    pub fn xyxx(&self) -> Float4 {
        Float4 {
            x: self.x,
            y: self.y,
            z: self.x,
            w: self.x,
        }
    }

    /// Returns a swizzled vector.
    pub fn xyxy(&self) -> Float4 {
        Float4 {
            x: self.x,
            y: self.y,
            z: self.x,
            w: self.y,
        }
    }

    /// Returns a swizzled vector.
    pub fn xyxz(&self) -> Float4 {
        Float4 {
            x: self.x,
            y: self.y,
            z: self.x,
            w: self.z,
        }
    }

    /// Returns a swizzled vector.
    pub fn xyyx(&self) -> Float4 {
        Float4 {
            x: self.x,
            y: self.y,
            z: self.y,
            w: self.x,
        }
    }

    /// Returns a swizzled vector.
    pub fn xyyy(&self) -> Float4 {
        Float4 {
            x: self.x,
            y: self.y,
            z: self.y,
            w: self.y,
        }
    }

    /// Returns a swizzled vector.
    pub fn xyyz(&self) -> Float4 {
        Float4 {
            x: self.x,
            y: self.y,
            z: self.y,
            w: self.z,
        }
    }

    /// Returns a swizzled vector.
    pub fn xyzx(&self) -> Float4 {
        Float4 {
            x: self.x,
            y: self.y,
            z: self.z,
            w: self.x,
        }
    }

    /// Returns a swizzled vector.
    pub fn xyzy(&self) -> Float4 {
        Float4 {
            x: self.x,
            y: self.y,
            z: self.z,
            w: self.y,
        }
    }

    /// Returns a swizzled vector.
    pub fn xyzz(&self) -> Float4 {
        Float4 {
            x: self.x,
            y: self.y,
            z: self.z,
            w: self.z,
        }
    }

    /// Returns a swizzled vector.
    pub fn xzxx(&self) -> Float4 {
        Float4 {
            x: self.x,
            y: self.z,
            z: self.x,
            w: self.x,
        }
    }

    /// Returns a swizzled vector.
    pub fn xzxy(&self) -> Float4 {
        Float4 {
            x: self.x,
            y: self.z,
            z: self.x,
            w: self.y,
        }
    }

    /// Returns a swizzled vector.
    pub fn xzxz(&self) -> Float4 {
        Float4 {
            x: self.x,
            y: self.z,
            z: self.x,
            w: self.z,
        }
    }

    /// Returns a swizzled vector.
    pub fn xzyx(&self) -> Float4 {
        Float4 {
            x: self.x,
            y: self.z,
            z: self.y,
            w: self.x,
        }
    }

    /// Returns a swizzled vector.
    pub fn xzyy(&self) -> Float4 {
        Float4 {
            x: self.x,
            y: self.z,
            z: self.y,
            w: self.y,
        }
    }

    /// Returns a swizzled vector.
    pub fn xzyz(&self) -> Float4 {
        Float4 {
            x: self.x,
            y: self.z,
            z: self.y,
            w: self.z,
        }
    }

    /// Returns a swizzled vector.
    pub fn xzzx(&self) -> Float4 {
        Float4 {
            x: self.x,
            y: self.z,
            z: self.z,
            w: self.x,
        }
    }

    /// Returns a swizzled vector.
    pub fn xzzy(&self) -> Float4 {
        Float4 {
            x: self.x,
            y: self.z,
            z: self.z,
            w: self.y,
        }
    }

    /// Returns a swizzled vector.
    pub fn xzzz(&self) -> Float4 {
        Float4 {
            x: self.x,
            y: self.z,
            z: self.z,
            w: self.z,
        }
    }

    /// Returns a swizzled vector.
    pub fn yxxx(&self) -> Float4 {
        Float4 {
            x: self.y,
            y: self.x,
            z: self.x,
            w: self.x,
        }
    }

    /// Returns a swizzled vector.
    pub fn yxxy(&self) -> Float4 {
        Float4 {
            x: self.y,
            y: self.x,
            z: self.x,
            w: self.y,
        }
    }

    /// Returns a swizzled vector.
    pub fn yxxz(&self) -> Float4 {
        Float4 {
            x: self.y,
            y: self.x,
            z: self.x,
            w: self.z,
        }
    }

    /// Returns a swizzled vector.
    pub fn yxyx(&self) -> Float4 {
        Float4 {
            x: self.y,
            y: self.x,
            z: self.y,
            w: self.x,
        }
    }

    /// Returns a swizzled vector.
    pub fn yxyy(&self) -> Float4 {
        Float4 {
            x: self.y,
            y: self.x,
            z: self.y,
            w: self.y,
        }
    }

    /// Returns a swizzled vector.
    pub fn yxyz(&self) -> Float4 {
        Float4 {
            x: self.y,
            y: self.x,
            z: self.y,
            w: self.z,
        }
    }

    /// Returns a swizzled vector.
    pub fn yxzx(&self) -> Float4 {
        Float4 {
            x: self.y,
            y: self.x,
            z: self.z,
            w: self.x,
        }
    }

    /// Returns a swizzled vector.
    pub fn yxzy(&self) -> Float4 {
        Float4 {
            x: self.y,
            y: self.x,
            z: self.z,
            w: self.y,
        }
    }

    /// Returns a swizzled vector.
    pub fn yxzz(&self) -> Float4 {
        Float4 {
            x: self.y,
            y: self.x,
            z: self.z,
            w: self.z,
        }
    }

    /// Returns a swizzled vector.
    pub fn yyxx(&self) -> Float4 {
        Float4 {
            x: self.y,
            y: self.y,
            z: self.x,
            w: self.x,
        }
    }

    /// Returns a swizzled vector.
    pub fn yyxy(&self) -> Float4 {
        Float4 {
            x: self.y,
            y: self.y,
            z: self.x,
            w: self.y,
        }
    }

    /// Returns a swizzled vector.
    pub fn yyxz(&self) -> Float4 {
        Float4 {
            x: self.y,
            y: self.y,
            z: self.x,
            w: self.z,
        }
    }

    /// Returns a swizzled vector.
    pub fn yyyx(&self) -> Float4 {
        Float4 {
            x: self.y,
            y: self.y,
            z: self.y,
            w: self.x,
        }
    }

    /// Returns a swizzled vector.
    pub fn yyyy(&self) -> Float4 {
        Float4 {
            x: self.y,
            y: self.y,
            z: self.y,
            w: self.y,
        }
    }

    /// Returns a swizzled vector.
    pub fn yyyz(&self) -> Float4 {
        Float4 {
            x: self.y,
            y: self.y,
            z: self.y,
            w: self.z,
        }
    }

    /// Returns a swizzled vector.
    pub fn yyzx(&self) -> Float4 {
        Float4 {
            x: self.y,
            y: self.y,
            z: self.z,
            w: self.x,
        }
    }

    /// Returns a swizzled vector.
    pub fn yyzy(&self) -> Float4 {
        Float4 {
            x: self.y,
            y: self.y,
            z: self.z,
            w: self.y,
        }
    }

    /// Returns a swizzled vector.
    pub fn yyzz(&self) -> Float4 {
        Float4 {
            x: self.y,
            y: self.y,
            z: self.z,
            w: self.z,
        }
    }

    /// Returns a swizzled vector.
    pub fn yzxx(&self) -> Float4 {
        Float4 {
            x: self.y,
            y: self.z,
            z: self.x,
            w: self.x,
        }
    }

    /// Returns a swizzled vector.
    pub fn yzxy(&self) -> Float4 {
        Float4 {
            x: self.y,
            y: self.z,
            z: self.x,
            w: self.y,
        }
    }

    /// Returns a swizzled vector.
    pub fn yzxz(&self) -> Float4 {
        Float4 {
            x: self.y,
            y: self.z,
            z: self.x,
            w: self.z,
        }
    }

    /// Returns a swizzled vector.
    pub fn yzyx(&self) -> Float4 {
        Float4 {
            x: self.y,
            y: self.z,
            z: self.y,
            w: self.x,
        }
    }

    /// Returns a swizzled vector.
    pub fn yzyy(&self) -> Float4 {
        Float4 {
            x: self.y,
            y: self.z,
            z: self.y,
            w: self.y,
        }
    }

    /// Returns a swizzled vector.
    pub fn yzyz(&self) -> Float4 {
        Float4 {
            x: self.y,
            y: self.z,
            z: self.y,
            w: self.z,
        }
    }

    /// Returns a swizzled vector.
    pub fn yzzx(&self) -> Float4 {
        Float4 {
            x: self.y,
            y: self.z,
            z: self.z,
            w: self.x,
        }
    }

    /// Returns a swizzled vector.
    pub fn yzzy(&self) -> Float4 {
        Float4 {
            x: self.y,
            y: self.z,
            z: self.z,
            w: self.y,
        }
    }

    /// Returns a swizzled vector.
    pub fn yzzz(&self) -> Float4 {
        Float4 {
            x: self.y,
            y: self.z,
            z: self.z,
            w: self.z,
        }
    }

    /// Returns a swizzled vector.
    pub fn zxxx(&self) -> Float4 {
        Float4 {
            x: self.z,
            y: self.x,
            z: self.x,
            w: self.x,
        }
    }

    /// Returns a swizzled vector.
    pub fn zxxy(&self) -> Float4 {
        Float4 {
            x: self.z,
            y: self.x,
            z: self.x,
            w: self.y,
        }
    }

    /// Returns a swizzled vector.
    pub fn zxxz(&self) -> Float4 {
        Float4 {
            x: self.z,
            y: self.x,
            z: self.x,
            w: self.z,
        }
    }

    /// Returns a swizzled vector.
    pub fn zxyx(&self) -> Float4 {
        Float4 {
            x: self.z,
            y: self.x,
            z: self.y,
            w: self.x,
        }
    }

    /// Returns a swizzled vector.
    pub fn zxyy(&self) -> Float4 {
        Float4 {
            x: self.z,
            y: self.x,
            z: self.y,
            w: self.y,
        }
    }

    /// Returns a swizzled vector.
    pub fn zxyz(&self) -> Float4 {
        Float4 {
            x: self.z,
            y: self.x,
            z: self.y,
            w: self.z,
        }
    }

    /// Returns a swizzled vector.
    pub fn zxzx(&self) -> Float4 {
        Float4 {
            x: self.z,
            y: self.x,
            z: self.z,
            w: self.x,
        }
    }

    /// Returns a swizzled vector.
    pub fn zxzy(&self) -> Float4 {
        Float4 {
            x: self.z,
            y: self.x,
            z: self.z,
            w: self.y,
        }
    }

    /// Returns a swizzled vector.
    pub fn zxzz(&self) -> Float4 {
        Float4 {
            x: self.z,
            y: self.x,
            z: self.z,
            w: self.z,
        }
    }

    /// Returns a swizzled vector.
    pub fn zyxx(&self) -> Float4 {
        Float4 {
            x: self.z,
            y: self.y,
            z: self.x,
            w: self.x,
        }
    }

    /// Returns a swizzled vector.
    pub fn zyxy(&self) -> Float4 {
        Float4 {
            x: self.z,
            y: self.y,
            z: self.x,
            w: self.y,
        }
    }

    /// Returns a swizzled vector.
    pub fn zyxz(&self) -> Float4 {
        Float4 {
            x: self.z,
            y: self.y,
            z: self.x,
            w: self.z,
        }
    }

    /// Returns a swizzled vector.
    pub fn zyyx(&self) -> Float4 {
        Float4 {
            x: self.z,
            y: self.y,
            z: self.y,
            w: self.x,
        }
    }

    /// Returns a swizzled vector.
    pub fn zyyy(&self) -> Float4 {
        Float4 {
            x: self.z,
            y: self.y,
            z: self.y,
            w: self.y,
        }
    }

    /// Returns a swizzled vector.
    pub fn zyyz(&self) -> Float4 {
        Float4 {
            x: self.z,
            y: self.y,
            z: self.y,
            w: self.z,
        }
    }

    /// Returns a swizzled vector.
    pub fn zyzx(&self) -> Float4 {
        Float4 {
            x: self.z,
            y: self.y,
            z: self.z,
            w: self.x,
        }
    }

    /// Returns a swizzled vector.
    pub fn zyzy(&self) -> Float4 {
        Float4 {
            x: self.z,
            y: self.y,
            z: self.z,
            w: self.y,
        }
    }

    /// Returns a swizzled vector.
    pub fn zyzz(&self) -> Float4 {
        Float4 {
            x: self.z,
            y: self.y,
            z: self.z,
            w: self.z,
        }
    }

    /// Returns a swizzled vector.
    pub fn zzxx(&self) -> Float4 {
        Float4 {
            x: self.z,
            y: self.z,
            z: self.x,
            w: self.x,
        }
    }

    /// Returns a swizzled vector.
    pub fn zzxy(&self) -> Float4 {
        Float4 {
            x: self.z,
            y: self.z,
            z: self.x,
            w: self.y,
        }
    }

    /// Returns a swizzled vector.
    pub fn zzxz(&self) -> Float4 {
        Float4 {
            x: self.z,
            y: self.z,
            z: self.x,
            w: self.z,
        }
    }

    /// Returns a swizzled vector.
    pub fn zzyx(&self) -> Float4 {
        Float4 {
            x: self.z,
            y: self.z,
            z: self.y,
            w: self.x,
        }
    }

    /// Returns a swizzled vector.
    pub fn zzyy(&self) -> Float4 {
        Float4 {
            x: self.z,
            y: self.z,
            z: self.y,
            w: self.y,
        }
    }

    /// Returns a swizzled vector.
    pub fn zzyz(&self) -> Float4 {
        Float4 {
            x: self.z,
            y: self.z,
            z: self.y,
            w: self.z,
        }
    }

    /// Returns a swizzled vector.
    pub fn zzzx(&self) -> Float4 {
        Float4 {
            x: self.z,
            y: self.z,
            z: self.z,
            w: self.x,
        }
    }

    /// Returns a swizzled vector.
    pub fn zzzy(&self) -> Float4 {
        Float4 {
            x: self.z,
            y: self.z,
            z: self.z,
            w: self.y,
        }
    }

    /// Returns a swizzled vector.
    pub fn zzzz(&self) -> Float4 {
        Float4 {
            x: self.z,
            y: self.z,
            z: self.z,
            w: self.z,
        }
    }
}
