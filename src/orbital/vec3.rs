// **Compliance Note**
// This project is based entirely on publicly available academic information and general knowledge of orbital mechanics. It contains no restricted, proprietary, or export-controlled information of any kind. This is a personal learning project only.

/// A 3D vector for representing positions and velocities.
///
/// This is a simple 3D vector implementation for Cartesian coordinates.
/// Components are stored as f64 for high precision calculations.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Vec3 {
    /// X component
    pub x: f64,
    /// Y component
    pub y: f64,
    /// Z component
    pub z: f64,
}

impl Vec3 {
    /// Creates a new Vec3 with the given components.
    ///
    /// # Arguments
    ///
    /// * `x` - The x component
    /// * `y` - The y component
    /// * `z` - The z component
    ///
    /// # Example
    ///
    /// ```
    /// use delta_wrecker::orbital::Vec3;
    /// let v = Vec3::new(1.0, 2.0, 3.0);
    /// assert_eq!(v.x, 1.0);
    /// assert_eq!(v.y, 2.0);
    /// assert_eq!(v.z, 3.0);
    /// ```
    pub fn new(x: f64, y: f64, z: f64) -> Self {
        Vec3 { x, y, z }
    }
}

// **Compliance Note**
// This project is based entirely on publicly available academic information and general knowledge of orbital mechanics. It contains no restricted, proprietary, or export-controlled information of any kind. This is a personal learning project only.