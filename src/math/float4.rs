use core::fmt;

/// Vector containing 4 floating point values.
#[derive(Copy, Clone, Debug)]
pub struct Float4 {
    /// The x-component of the vector.
    pub x: f32,
    /// The y-component of the vector.
    pub y: f32,
    /// The z-component of the vector.
    pub z: f32,
    /// The w-component of the vector.
    pub w: f32,
}

impl fmt::Display for Float4 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Float4 ({}, {}, {}, {})", self.x, self.y, self.z, self.w)
    }
}

impl PartialEq for Float4 {
    fn eq(&self, rhs: &Self) -> bool {
        self.x == rhs.x && self.y == rhs.y && self.z == rhs.z && self.w == rhs.w
    }
}

impl Float4 {
    /// Creates a vector from 4 floating point values.
    pub fn new(x: f32, y: f32, z: f32, w: f32) -> Self {
        Self { x, y, z, w }
    }
}