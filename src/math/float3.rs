use core::fmt;

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

impl PartialEq for Float3 {
    fn eq(&self, rhs: &Self) -> bool {
        self.x == rhs.x && self.y == rhs.y && self.z == rhs.z
    }
}

impl Float3 {
    /// Creates a vector from 3 floating point values.
    pub fn new(x: f32, y: f32, z: f32) -> Self {
        Self { x, y, z }
    }
}