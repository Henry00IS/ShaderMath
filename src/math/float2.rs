use core::fmt;
use core::ops;

use super::Float3;

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

impl ops::AddAssign<Self> for Float2 {
    fn add_assign(&mut self, rhs: Self) {
        self.x += rhs.x;
        self.y += rhs.y;
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

impl ops::SubAssign<Self> for Float2 {
    fn sub_assign(&mut self, rhs: Self) {
        self.x -= rhs.x;
        self.y -= rhs.y;
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

impl ops::MulAssign<Self> for Float2 {
    fn mul_assign(&mut self, rhs: Self) {
        self.x *= rhs.x;
        self.y *= rhs.y;
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

impl ops::DivAssign<Self> for Float2 {
    fn div_assign(&mut self, rhs: Self) {
        self.x /= rhs.x;
        self.y /= rhs.y;
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

    /// Computes the per-component cosine numbers (in radians).
    pub fn cos(&self) -> Self {
        Self {
            x: self.x.cos(),
            y: self.y.cos(),
        }
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

    /// Computes the per-component sine numbers (in radians).
    pub fn sin(&self) -> Self {
        Self {
            x: self.x.sin(),
            y: self.y.sin(),
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

    /// Returns a swizzled vector.
    pub fn xx(&self) -> Self {
        Self {
            x: self.x,
            y: self.x,
        }
    }

    /// Returns a swizzled vector.
    pub fn xy(&self) -> Self {
        Self {
            x: self.x,
            y: self.y,
        }
    }

    /// Returns a swizzled vector.
    pub fn yx(&self) -> Self {
        Self {
            x: self.y,
            y: self.x,
        }
    }

    /// Returns a swizzled vector.
    pub fn yy(&self) -> Self {
        Self {
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
}