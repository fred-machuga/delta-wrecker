// **Compliance Note**
// This project is based entirely on publicly available academic information and general knowledge of orbital mechanics. It contains no restricted, proprietary, or export-controlled information of any kind. This is a personal learning project only.

/// A simple 3D vector for Cartesian positions and velocities.
///
/// Used throughout the orbital math library for position and velocity components.
/// All values are in kilometers (position) or km/s (velocity).
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
    /// Creates a new `Vec3` with the given components.
    pub fn new(x: f64, y: f64, z: f64) -> Self {
        Vec3 { x, y, z }
    }
    
    /// Returns the magnitude (length) of the vector.
    ///
    /// For position vectors this is in km, for velocity vectors in km/s.
    pub fn magnitude(&self) -> f64 {
        (self.x * self.x + self.y * self.y + self.z * self.z).sqrt()
    }
}

// **Compliance Note**
// This project is based entirely on publicly available academic information and general knowledge of orbital mechanics. It contains no restricted, proprietary, or export-controlled information of any kind. This is a personal learning project only.