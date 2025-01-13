use core::fmt;
use core::ops;

use super::Float3;
use super::Float4;

/// Vector containing 2 floating point values.
#[derive(Copy, Clone, Debug)]
pub struct Float2 {
    /// The x-component of the vector.
    pub x: f32,
    /// The y-component of the vector.
    pub y: f32,
}

impl fmt::Display for Float2 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Float2 ({}, {})", self.x, self.y)
    }
}

impl ops::Add<Self> for Float2 {
    type Output = Self;

    fn add(self, rhs: Self) -> Self {
        Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

impl ops::Add<f32> for Float2 {
    type Output = Self;

    fn add(self, rhs: f32) -> Self {
        Self {
            x: self.x + rhs,
            y: self.y + rhs,
        }
    }
}

impl ops::AddAssign<Self> for Float2 {
    fn add_assign(&mut self, rhs: Self) {
        self.x += rhs.x;
        self.y += rhs.y;
    }
}

impl ops::AddAssign<f32> for Float2 {
    fn add_assign(&mut self, rhs: f32) {
        self.x += rhs;
        self.y += rhs;
    }
}

impl ops::Sub<Self> for Float2 {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self {
        Self {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
        }
    }
}

impl ops::Sub<f32> for Float2 {
    type Output = Self;

    fn sub(self, rhs: f32) -> Self {
        Self {
            x: self.x - rhs,
            y: self.y - rhs,
        }
    }
}

impl ops::SubAssign<Self> for Float2 {
    fn sub_assign(&mut self, rhs: Self) {
        self.x -= rhs.x;
        self.y -= rhs.y;
    }
}

impl ops::SubAssign<f32> for Float2 {
    fn sub_assign(&mut self, rhs: f32) {
        self.x -= rhs;
        self.y -= rhs;
    }
}

impl ops::Mul<Self> for Float2 {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self {
        Self {
            x: self.x * rhs.x,
            y: self.y * rhs.y,
        }
    }
}

impl ops::Mul<f32> for Float2 {
    type Output = Self;

    fn mul(self, rhs: f32) -> Self {
        Self {
            x: self.x * rhs,
            y: self.y * rhs,
        }
    }
}

impl ops::MulAssign<Self> for Float2 {
    fn mul_assign(&mut self, rhs: Self) {
        self.x *= rhs.x;
        self.y *= rhs.y;
    }
}

impl ops::MulAssign<f32> for Float2 {
    fn mul_assign(&mut self, rhs: f32) {
        self.x *= rhs;
        self.y *= rhs;
    }
}

impl ops::Div<Self> for Float2 {
    type Output = Self;

    fn div(self, rhs: Self) -> Self {
        Self {
            x: self.x / rhs.x,
            y: self.y / rhs.y,
        }
    }
}

impl ops::Div<f32> for Float2 {
    type Output = Self;

    fn div(self, rhs: f32) -> Self {
        Self {
            x: self.x / rhs,
            y: self.y / rhs,
        }
    }
}

impl ops::DivAssign<Self> for Float2 {
    fn div_assign(&mut self, rhs: Self) {
        self.x /= rhs.x;
        self.y /= rhs.y;
    }
}

impl ops::DivAssign<f32> for Float2 {
    fn div_assign(&mut self, rhs: f32) {
        self.x /= rhs;
        self.y /= rhs;
    }
}

impl ops::Neg for Float2 {
    type Output = Self;

    fn neg(self) -> Self {
        Self {
            x: -self.x,
            y: -self.y,
        }
    }
}

impl PartialEq for Float2 {
    fn eq(&self, rhs: &Self) -> bool {
        self.x == rhs.x && self.y == rhs.y
    }
}

impl From<f32> for Float2 {
    fn from(lhs: f32) -> Self {
        Self { x: lhs, y: lhs }
    }
}

impl From<(f32, f32)> for Float2 {
    fn from(lhs: (f32, f32)) -> Self {
        Self { x: lhs.0, y: lhs.1 }
    }
}

impl Float2 {
    /// Creates a vector from 2 floating point values.
    pub fn new(x: f32, y: f32) -> Self {
        Self { x, y }
    }

    /// Computes the per-component absolute numbers.
    pub fn abs(&self) -> Self {
        Self {
            x: self.x.abs(),
            y: self.y.abs(),
        }
    }

    /// Computes the per-component arccosine numbers (in radians).
    /// Each component should be a value within the range of -1 to 1.
    /// Return values are in the range 0, pi or NaN if the number is outside the range -1 to 1.
    pub fn acos(&self) -> Self {
        Self {
            x: self.x.acos(),
            y: self.y.acos(),
        }
    }

    /// Determines if all components of the vector are non-zero.
    pub fn all(&self) -> bool {
        self.x != 0.0 && self.y != 0.0
    }

    /// Determines if any components of the vector are non-zero.
    pub fn any(&self) -> bool {
        self.x != 0.0 || self.y != 0.0
    }

    /// Computes the per-component arcsine numbers (in radians).
    /// Each component should be a value within the range of -1 to 1.
    /// Return values are in the range -pi/2 to pi/2 or NaN if the number is outside the range -1 to 1.
    pub fn asin(&self) -> Self {
        Self {
            x: self.x.asin(),
            y: self.y.asin(),
        }
    }

    /// Computes the per-component arctangent numbers (in radians).
    /// Return values are in the range -pi/2 to pi/2.
    pub fn atan(&self) -> Self {
        Self {
            x: self.x.atan(),
            y: self.y.atan(),
        }
    }

    /// Computes the four quadrant arctangent of y and x (in radians).
    pub fn atan2(&self) -> f32 {
        self.y.atan2(self.x)
    }

    /// Computes the per-component smallest integers greater than or equal to `self.x` and `self.y` respectively.
    pub fn ceil(&self) -> Self {
        Self {
            x: self.x.ceil(),
            y: self.y.ceil(),
        }
    }

    /// Computes the per-component clamped numbers between `min` and `max`.
    pub fn clamp(&self, min: f32, max: f32) -> Self {
        Self {
            x: self.x.clamp(min, max),
            y: self.y.clamp(min, max),
        }
    }

    /// Computes the per-component cosine numbers (in radians).
    pub fn cos(&self) -> Self {
        Self {
            x: self.x.cos(),
            y: self.y.cos(),
        }
    }

    /// Computes the per-component hyperbolic cosine numbers.
    pub fn cosh(&self) -> Self {
        Self {
            x: self.x.cosh(),
            y: self.y.cosh(),
        }
    }

    /// Converts the per-component numbers from radians to degrees.
    pub fn degrees(&self) -> Self {
        Self {
            x: self.x.to_degrees(),
            y: self.y.to_degrees(),
        }
    }

    /// Computes the distance scalar between two vectors.
    pub fn distance(&self, rhs: &Self) -> f32 {
        let dx = self.x - rhs.x;
        let dy = self.y - rhs.y;
        (dx * dx + dy * dy).sqrt()
    }

    /// Computes the dot product of two vectors.
    pub fn dot(&self, rhs: &Self) -> f32 {
        self.x * rhs.x + self.y * rhs.y
    }

    /// Computes the per-component e^(self), the exponential function.
    pub fn exp(&self) -> Self {
        Self {
            x: self.x.exp(),
            y: self.y.exp(),
        }
    }

    /// Computes the per-component 2^(self).
    pub fn exp2(&self) -> Self {
        Self {
            x: self.x.exp2(),
            y: self.y.exp2(),
        }
    }

    /// Computes the per-component largest integers less than or equal to `self.x` and `self.y` respectively.
    pub fn floor(&self) -> Self {
        Self {
            x: self.x.floor(),
            y: self.y.floor(),
        }
    }

    /// Computes the floating-point remainder of division for each component.
    pub fn fmod(&self, rhs: &Self) -> Self {
        Self {
            x: self.x % rhs.x,
            y: self.y % rhs.y,
        }
    }

    /// Computes the fractional (or decimal) part of each component; which is greater than or equal to 0 and less than 1.
    pub fn frac(&self) -> Self {
        Self {
            x: self.x.fract(),
            y: self.y.fract(),
        }
    }

    /// Computes `value * 2^exponent` for each component of the vector.
    pub fn ldexp(&self, exponent: &Self) -> Self {
        Self {
            x: self.x * (2.0f32).powi(exponent.x as i32),
            y: self.y * (2.0f32).powi(exponent.y as i32),
        }
    }

    /// Computes the length scalar between two vectors.
    pub fn length(&self) -> f32 {
        (self.x * self.x + self.y * self.y).sqrt()
    }

    /// Computes a linear interpolation between two vectors.
    pub fn lerp(&self, rhs: &Self, t: f32) -> Self {
        Self {
            x: self.x + t * (rhs.x - self.x),
            y: self.y + t * (rhs.y - self.y),
        }
    }

    /// Computes the natural logarithm (base e) of each component of the vector.
    pub fn log(&self) -> Self {
        Self {
            x: self.x.ln(),
            y: self.y.ln(),
        }
    }

    /// Computes the natural logarithm (base 10) of each component of the vector.
    pub fn log10(&self) -> Self {
        Self {
            x: self.x.log10(),
            y: self.y.log10(),
        }
    }

    /// Computes the natural logarithm (base 2) of each component of the vector.
    pub fn log2(&self) -> Self {
        Self {
            x: self.x.log2(),
            y: self.y.log2(),
        }
    }

    /// Computes the multiply-add operation: (self * b) + c.
    pub fn mad(&self, b: &Self, c: &Self) -> Self {
        Self {
            x: self.x * b.x + c.x,
            y: self.y * b.y + c.y,
        }
    }

    /// Computes the component-wise maximum of two vectors.
    pub fn max(&self, rhs: &Self) -> Self {
        Self {
            x: self.x.max(rhs.x),
            y: self.y.max(rhs.y),
        }
    }

    /// Computes the component-wise minimum of two vectors.
    pub fn min(&self, rhs: &Self) -> Self {
        Self {
            x: self.x.min(rhs.x),
            y: self.y.min(rhs.y),
        }
    }

    /// Computes the normalized vector (unit vector) of `self`.
    pub fn normalize(&self) -> Self {
        let length = self.length();
        if length != 0.0 {
            Self {
                x: self.x / length,
                y: self.y / length,
            }
        } else {
            // return the zero vector if the input vector has zero length.
            Self { x: 0.0, y: 0.0 }
        }
    }

    /// Computes the component-wise power: `self^exponent`.
    pub fn pow(&self, exponent: f32) -> Self {
        Self {
            x: self.x.powf(exponent),
            y: self.y.powf(exponent),
        }
    }

    /// Converts the per-component numbers from degrees to radians.
    pub fn radians(&self) -> Self {
        Self {
            x: self.x.to_radians(),
            y: self.y.to_radians(),
        }
    }

    /// Computes the reciprocal of each component of the vector.
    /// Equivalent to `1 / self`.
    pub fn rcp(&self) -> Self {
        Self {
            x: 1.0 / self.x,
            y: 1.0 / self.y,
        }
    }

    /// Computes the reciprocal of each component of the vector.
    /// Equivalent to `1 / self` and returning 0 when self is 0.
    pub fn rcp_safe(&self) -> Self {
        Self {
            x: if self.x != 0.0 { 1.0 / self.x } else { 0.0 },
            y: if self.y != 0.0 { 1.0 / self.y } else { 0.0 },
        }
    }

    /// Computes the reflection of an incident vector `self` about a normal vector `normal`.
    pub fn reflect(&self, normal: &Self) -> Self {
        let dot = self.dot(normal);
        Self {
            x: self.x - 2.0 * dot * normal.x,
            y: self.y - 2.0 * dot * normal.y,
        }
    }

    /// Computes the refraction vector for the given incident vector, normal, and refraction index.
    pub fn refract(&self, normal: &Self, eta: f32) -> Self {
        let dot_n_i = self.dot(normal);
        let k = 1.0 - eta * eta * (1.0 - dot_n_i * dot_n_i);
        if k < 0.0 {
            Self { x: 0.0, y: 0.0 }
        } else {
            let scale_i = eta;
            let scale_n = eta * dot_n_i + k.sqrt();
            Self {
                x: scale_i * self.x - scale_n * normal.x,
                y: scale_i * self.y - scale_n * normal.y,
            }
        }
    }

    /// Rounds each component of the vector to the nearest integer.
    pub fn round(&self) -> Self {
        Self {
            x: self.x.round(),
            y: self.y.round(),
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
        }
    }

    /// Clamps each component of the vector to the range [0, 1].
    pub fn saturate(&self) -> Self {
        Self {
            x: self.x.clamp(0.0, 1.0),
            y: self.y.clamp(0.0, 1.0),
        }
    }

    /// Computes the sign of each component of the vector.
    pub fn sign(&self) -> Self {
        Self {
            x: self.x.signum(),
            y: self.y.signum(),
        }
    }

    /// Computes the per-component sine numbers (in radians).
    pub fn sin(&self) -> Self {
        Self {
            x: self.x.sin(),
            y: self.y.sin(),
        }
    }

    /// Computes the per-component hyperbolic sine numbers.
    pub fn sinh(&self) -> Self {
        Self {
            x: self.x.sinh(),
            y: self.y.sinh(),
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
        }
    }

    /// Computes the square root of each component of the vector.
    pub fn sqrt(&self) -> Self {
        Self {
            x: self.x.sqrt(),
            y: self.y.sqrt(),
        }
    }

    /// Computes the component-wise step function.
    /// For each component: returns 0.0 if `self < edge`, else returns 1.0.
    pub fn step(&self, edge: &Self) -> Self {
        Self {
            x: if self.x < edge.x { 0.0 } else { 1.0 },
            y: if self.y < edge.y { 0.0 } else { 1.0 },
        }
    }

    /// Computes the per-component tangent numbers (in radians).
    pub fn tan(&self) -> Self {
        Self {
            x: self.x.tan(),
            y: self.y.tan(),
        }
    }

    /// Computes the per-component hyperbolic tangent numbers.
    pub fn tanh(&self) -> Self {
        Self {
            x: self.x.tanh(),
            y: self.y.tanh(),
        }
    }

    /// Truncates each component of the vector to its integer portion.
    pub fn trunc(&self) -> Self {
        Self {
            x: self.x.trunc(),
            y: self.y.trunc(),
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
}
