// **Compliance Note**
// This project is based entirely on publicly available academic information and general knowledge of orbital mechanics. It contains no restricted, proprietary, or export-controlled information of any kind. This is a personal learning project only.

//! Simplified 2D Keplerian circular orbit propagator.
//!
//! Propagates a given initial state and Δt to compute a new position and velocity
//! for a stable near-circular orbit. No burns or maneuvers are simulated.

use super::OrbitState;

/// Propagates a circular orbit by a given time delta dt (in seconds).
/// 
/// Note: Pure time-based propagation. Given the initial state and Δt, 
/// computes new position + velocity for a stable near-circular orbit.
pub fn propagate(state: &OrbitState, dt_s: f64) -> OrbitState {
    // - Matches analytical circular orbit math within 1e-6 error
    // - Frame-rate independent
    
    let r = state.distance_km(); // distance from center of Earth to the orbiting body
    let v: f64 = state.speed_kms(); // speed of the orbiting body in km/s
    let omega = v / r; // angular velocity of the orbiting body in rad/s
    let theta = omega * dt_s; // number of radians the spacecraft moves along its circular orbit over dt_s seconds
    let cos_theta = theta.cos(); 
    let sin_theta = theta.sin();

    let x = state.position().x * cos_theta - state.position().y * sin_theta;
    let y = state.position().x * sin_theta + state.position().y * cos_theta;
    let z = state.position().z;
    
    let vx = -v * y / r;
    let vy = v * x / r;
    let vz = state.velocity().z;

    OrbitState::new(x, y, z, vx, vy, vz)
}

// **Compliance Note**
// This project is based entirely on publicly available academic information and general knowledge of orbital mechanics. It contains no restricted, proprietary, or export-controlled information of any kind. This is a personal learning project only.
