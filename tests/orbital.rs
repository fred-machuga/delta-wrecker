// **Compliance Note**
// This project is based entirely on publicly available academic information and general knowledge of orbital mechanics. It contains no restricted, proprietary, or export-controlled information of any kind. This is a personal learning project only.

use delta_wrecker::orbital::{OrbitState, KeplerianElements, Vec3};

#[test]
fn test_vec3_creation() {
    let v = Vec3::new(1.0, 2.0, 3.0);
    assert_eq!(v.x, 1.0);
    assert_eq!(v.y, 2.0);
    assert_eq!(v.z, 3.0);
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
fn test_orbit_state_get_position() {
    let state = OrbitState::new(7000.0, 1000.0, 2000.0, 0.0, 7.5, 1.2);
    let pos = state.get_position();
    assert_eq!(pos, Vec3::new(7000.0, 1000.0, 2000.0));
}

#[test]
fn test_orbit_state_get_velocity() {
    let state = OrbitState::new(7000.0, 1000.0, 2000.0, 0.0, 7.5, 1.2);
    let vel = state.get_velocity();
    assert_eq!(vel, Vec3::new(0.0, 7.5, 1.2));
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
fn test_vec3_debug_clone_copy() {
    let v1 = Vec3::new(1.0, 2.0, 3.0);
    let v2 = v1; // Copy
    let v3 = v1.clone(); // Clone

    assert_eq!(v1, v2);
    assert_eq!(v1, v3);
    assert_eq!(v2, v3);
}

// **Compliance Note**
// This project is based entirely on publicly available academic information and general knowledge of orbital mechanics. It contains no restricted, proprietary, or export-controlled information of any kind. This is a personal learning project only.