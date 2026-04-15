// **Compliance Note**
// This project is based entirely on publicly available academic information and general knowledge of orbital mechanics. It contains no restricted, proprietary, or export-controlled information of any kind. This is a personal learning project only.

use super::Vec3;

/// Represents the instantaneous Cartesian state of an orbiting object.
///
/// For Sprint 1 this is the minimal 3D position + velocity state used by the
/// propagator. It is intentionally simple (no time stamp yet) so we can focus
/// on clean data structures before adding propagation logic.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct OrbitState {
    /// X position component in kilometers (km)
    pub x: f64,
    /// Y position component in kilometers (km)
    pub y: f64,
    /// Z position component in kilometers (km)
    pub z: f64,
    /// X velocity component in kilometers per second (km/s)
    pub vx: f64,
    /// Y velocity component in kilometers per second (km/s)
    pub vy: f64,
    /// Z velocity component in kilometers per second (km/s)
    pub vz: f64,
}

impl OrbitState {
    /// Creates a new `OrbitState` with the given position and velocity.
    pub fn new(x: f64, y: f64, z: f64, vx: f64, vy: f64, vz: f64) -> Self {
        OrbitState { x, y, z, vx, vy, vz }
    }

    /// Returns the position as a `Vec3`.
    pub fn get_position(&self) -> Vec3 {
        Vec3::new(self.x, self.y, self.z)
    }

    /// Returns the velocity as a `Vec3`.
    pub fn get_velocity(&self) -> Vec3 {
        Vec3::new(self.vx, self.vy, self.vz)
    }
}

// **Compliance Note**
// This project is based entirely on publicly available academic information and general knowledge of orbital mechanics. It contains no restricted, proprietary, or export-controlled information of any kind. This is a personal learning project only.