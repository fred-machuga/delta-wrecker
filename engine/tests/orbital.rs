// **Compliance Note**
// This project is based entirely on publicly available academic information and general knowledge of orbital mechanics. It contains no restricted, proprietary, or export-controlled information of any kind. This is a personal learning project only.

use delta_wrecker::orbital::{OrbitState, KeplerianElements, Vec3, MU_EARTH, EARTH_RADIUS_KM};

#[test]
fn test_vec3_creation() {
    let v = Vec3::new(1.0, 2.0, 3.0);
    assert_eq!(v.x, 1.0);
    assert_eq!(v.y, 2.0);
    assert_eq!(v.z, 3.0);
}

#[test]
fn test_vec3_magnitude() {
    // 3-4-5 triangle
    let v = Vec3::new(3.0, 4.0, 0.0);
    assert_eq!(v.magnitude(), 5.0);

    // 1-2-2 vector gives a length of 3 (sqrt(1+4+4)=sqrt(9)=3)
    let v2 = Vec3::new(1.0, 2.0, 2.0);
    assert_eq!(v2.magnitude(), 3.0);
}

#[test]
fn test_orbit_state_creation() {
    let state = OrbitState::new(7000.0, 0.0, 0.0, 0.0, 7.5, 0.0);
    assert_eq!(state.x, 7000.0);
    assert_eq!(state.y, 0.0);
    assert_eq!(state.z, 0.0);
    assert_eq!(state.vx, 0.0);
    assert_eq!(state.vy, 7.5);
    assert_eq!(state.vz, 0.0);
}

#[test]
fn test_orbit_state_position() {
    let state = OrbitState::new(7000.0, 1000.0, 2000.0, 0.0, 7.5, 1.2);
    let pos = state.position();
    assert_eq!(pos, Vec3::new(7000.0, 1000.0, 2000.0));
}

#[test]
fn test_orbit_state_velocity() {
    let state = OrbitState::new(7000.0, 1000.0, 2000.0, 0.0, 7.5, 1.2);
    let vel = state.velocity();
    assert_eq!(vel, Vec3::new(0.0, 7.5, 1.2));
}

#[test]
fn test_orbit_state_derived_metrics() {
    // 7000 km distance on X axis. Velocity 7.5 km/s on Y axis.
    let state = OrbitState::new(7000.0, 0.0, 0.0, 0.0, 7.5, 0.0);
    
    // distance = sqrt(7000^2 + 0^2 + 0^2) = 7000.0
    assert_eq!(state.distance_km(), 7000.0);
    
    // altitude = 7000.0 - 6371.0 = 629.0
    assert_eq!(state.altitude_km(), 629.0);
    
    // speed = sqrt(0^2 + 7.5^2 + 0^2) = 7.5
    assert_eq!(state.speed_kms(), 7.5);
}

#[test]
fn test_keplerian_elements_creation() {
    let elements = KeplerianElements::new(7000.0);
    assert_eq!(elements.semi_major_axis, 7000.0);
    assert_eq!(elements.eccentricity, None);
    assert_eq!(elements.inclination, None);
    assert_eq!(elements.longitude_of_ascending_node, None);
    assert_eq!(elements.argument_of_periapsis, None);
    assert_eq!(elements.mean_anomaly, None);
}

#[test]
fn test_orbit_state_debug_clone_copy() {
    let state1 = OrbitState::new(7000.0, 0.0, 0.0, 0.0, 7.5, 0.0);
    let state2 = state1; // Copy
    let state3 = state1.clone(); // Clone

    assert_eq!(state1, state2);
    assert_eq!(state1, state3);
    assert_eq!(state2, state3);
}

#[test]
fn test_keplerian_elements_debug_clone_copy() {
    let elements1 = KeplerianElements::new(7000.0);
    let elements2 = elements1; // Copy
    let elements3 = elements1.clone(); // Clone

    assert_eq!(elements1, elements2);
    assert_eq!(elements1, elements3);
    assert_eq!(elements2, elements3);
}

#[test]
fn test_near_circular_validation() {
    // Default is None, which implies e = 0.0 (perfectly circular)
    let mut elements = KeplerianElements::new(7000.0);
    assert!(elements.is_near_circular());

    // Explicitly 0.0
    elements.eccentricity = Some(0.0);
    assert!(elements.is_near_circular());

    // Near circular (e.g. 0.02)
    elements.eccentricity = Some(0.02);
    assert!(elements.is_near_circular());

    // Right on the boundary 0.05 is NOT strictly less than 0.05
    elements.eccentricity = Some(0.05);
    assert!(!elements.is_near_circular());

    // Highly elliptical
    elements.eccentricity = Some(0.5);
    assert!(!elements.is_near_circular());
}

#[test]
fn test_vec3_debug_clone_copy() {
    let v1 = Vec3::new(1.0, 2.0, 3.0);
    let v2 = v1; // Copy
    let v3 = v1.clone(); // Clone

    assert_eq!(v1, v2);
    assert_eq!(v1, v3);
    assert_eq!(v2, v3);
}

#[test]
fn test_mu_earth_constant() {
    assert_eq!(MU_EARTH, 398600.4418);
}

#[test]
fn test_earth_radius_constant() {
    assert_eq!(EARTH_RADIUS_KM, 6371.0);
}

// **Compliance Note**
// This project is based entirely on publicly available academic information and general knowledge of orbital mechanics. It contains no restricted, proprietary, or export-controlled information of any kind. This is a personal learning project only.