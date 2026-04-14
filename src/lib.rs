//! # Delta Wrecker Orbital Library
//!
//! This crate provides utilities for orbital mechanics calculations.

/// A placeholder module for vector mathematics.
/// This will be expanded in future stories.
pub mod vectors {
    /// A simple 2D vector struct.
    #[derive(Debug, Clone, PartialEq)]
    pub struct Vec2 {
        pub x: f64,
        pub y: f64,
    }

    impl Vec2 {
        /// Creates a new 2D vector.
        pub fn new(x: f64, y: f64) -> Self {
            Self { x, y }
        }
    }
}

/// A placeholder module for orbital elements.
/// This will be expanded in future stories.
pub mod orbits {
    /// A placeholder struct for Keplerian elements.
    #[derive(Debug, Clone)]
    pub struct KeplerianElements {
        pub semi_major_axis: f64,
        pub eccentricity: f64,
        // Add more fields as needed
    }

    impl KeplerianElements {
        /// Creates new Keplerian elements.
        pub fn new(semi_major_axis: f64, eccentricity: f64) -> Self {
            Self {
                semi_major_axis,
                eccentricity,
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_vec2_creation() {
        let v = vectors::Vec2::new(1.0, 2.0);
        assert_eq!(v.x, 1.0);
        assert_eq!(v.y, 2.0);
    }

    #[test]
    fn test_keplerian_creation() {
        let k = orbits::KeplerianElements::new(7000.0, 0.1);
        assert_eq!(k.semi_major_axis, 7000.0);
        assert_eq!(k.eccentricity, 0.1);
    }
}