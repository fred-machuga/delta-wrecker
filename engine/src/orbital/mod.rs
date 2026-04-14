// **Compliance Note**
// This project is based entirely on publicly available academic information and general knowledge of orbital mechanics. It contains no restricted, proprietary, or export-controlled information of any kind. This is a personal learning project only.

//! # Orbital Mechanics Data Structures
//!
//! This module defines the core data structures for representing orbital states
//! and Keplerian orbital elements used in orbital mechanics calculations.

pub mod orbit_state;
pub mod keplerian_elements;
pub mod vec3;

// Re-export the main types for convenience
pub use orbit_state::OrbitState;
pub use keplerian_elements::KeplerianElements;
pub use vec3::Vec3;

// **Compliance Note**
// This project is based entirely on publicly available academic information and general knowledge of orbital mechanics. It contains no restricted, proprietary, or export-controlled information of any kind. This is a personal learning project only.