// **Compliance Note**
// This project is based entirely on publicly available academic information and general knowledge of orbital mechanics. It contains no restricted, proprietary, or export-controlled information of any kind. This is a personal learning project only.

use super::Vec3;

/// Represents the instantaneous state of an orbiting object in Cartesian coordinates.
///
/// This struct contains the position and velocity components of an object at a specific
/// point in time, typically used for numerical integration and state propagation.
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
    /// Creates a new OrbitState with the given position and velocity components.
    ///
    /// # Arguments
    ///
    /// * `x`, `y`, `z` - Position components in km
    /// * `vx`, `vy`, `vz` - Velocity components in km/s
    ///
    /// # Example
    ///
    /// ```
    /// use delta_wrecker::orbital::OrbitState;
    /// let state = OrbitState::new(7000.0, 0.0, 0.0, 0.0, 7.5, 0.0);
    /// assert_eq!(state.x, 7000.0);
    /// assert_eq!(state.vy, 7.5);
    /// ```
    pub fn new(x: f64, y: f64, z: f64, vx: f64, vy: f64, vz: f64) -> Self {
        OrbitState { x, y, z, vx, vy, vz }
    }

    /// Returns the position as a Vec3.
    ///
    /// # Example
    ///
    /// ```
    /// use delta_wrecker::orbital::{OrbitState, Vec3};
    /// let state = OrbitState::new(7000.0, 0.0, 0.0, 0.0, 7.5, 0.0);
    /// let pos = state.get_position();
    /// assert_eq!(pos, Vec3::new(7000.0, 0.0, 0.0));
    /// ```
    pub fn get_position(&self) -> Vec3 {
        Vec3::new(self.x, self.y, self.z)
    }

    /// Returns the velocity as a Vec3.
    ///
    /// # Example
    ///
    /// ```
    /// use delta_wrecker::orbital::{OrbitState, Vec3};
    /// let state = OrbitState::new(7000.0, 0.0, 0.0, 0.0, 7.5, 0.0);
    /// let vel = state.get_velocity();
    /// assert_eq!(vel, Vec3::new(0.0, 7.5, 0.0));
    /// ```
    pub fn get_velocity(&self) -> Vec3 {
        Vec3::new(self.vx, self.vy, self.vz)
    }
}

// **Compliance Note**
// This project is based entirely on publicly available academic information and general knowledge of orbital mechanics. It contains no restricted, proprietary, or export-controlled information of any kind. This is a personal learning project only.