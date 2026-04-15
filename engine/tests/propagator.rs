// **Compliance Note**
// This project is based entirely on publicly available academic information and general knowledge of orbital mechanics. It contains no restricted, proprietary, or export-controlled information of any kind. This is a personal learning project only.

use delta_wrecker::orbital::OrbitState;
use delta_wrecker::orbital::propagator::propagate;
use std::f64::consts::PI;

#[test]
fn test_propagate_circular_quarter_orbit() {
    // A circular orbit with radius 7000 km, velocity 7.546 km/s
    let r = 7000.0;
    let v = 7.546;
    let state = OrbitState::new(r, 0.0, 0.0, 0.0, v, 0.0);
    
    let omega = v / r;
    let period = 2.0 * PI / omega;
    
    // Propagate by one quarter period
    let dt = period / 4.0;
    let new_state = propagate(&state, dt);
    
    assert!((new_state.x - 0.0).abs() < 1e-6);
    assert!((new_state.y - r).abs() < 1e-6);
    assert!((new_state.z - 0.0).abs() < 1e-6);
    
    assert!((new_state.vx - (-v)).abs() < 1e-6);
    assert!((new_state.vy - 0.0).abs() < 1e-6);
    assert!((new_state.vz - 0.0).abs() < 1e-6);
}

#[test]
fn test_propagate_circular_half_orbit() {
    let r = 7000.0;
    let v = 7.546;
    let state = OrbitState::new(r, 0.0, 0.0, 0.0, v, 0.0);
    
    let omega = v / r;
    let period = 2.0 * PI / omega;
    
    // Propagate by half period
    let dt = period / 2.0;
    let new_state = propagate(&state, dt);
    
    assert!((new_state.x - (-r)).abs() < 1e-6);
    assert!((new_state.y - 0.0).abs() < 1e-6);
    assert!((new_state.z - 0.0).abs() < 1e-6);
    
    assert!((new_state.vx - 0.0).abs() < 1e-6);
    assert!((new_state.vy - (-v)).abs() < 1e-6);
    assert!((new_state.vz - 0.0).abs() < 1e-6);
}

#[test]
fn test_propagate_circular_full_orbit() {
    let r = 7000.0;
    let v = 7.546;
    let state = OrbitState::new(r, 0.0, 0.0, 0.0, v, 0.0);
    
    let omega = v / r;
    let period = 2.0 * PI / omega;
    
    // Propagate by one full period
    let dt = period;
    let new_state = propagate(&state, dt);
    
    assert!((new_state.x - r).abs() < 1e-6);
    assert!((new_state.y - 0.0).abs() < 1e-6);
    assert!((new_state.z - 0.0).abs() < 1e-6);
    
    assert!((new_state.vx - 0.0).abs() < 1e-6);
    assert!((new_state.vy - v).abs() < 1e-6);
    assert!((new_state.vz - 0.0).abs() < 1e-6);
}

// **Compliance Note**
// This project is based entirely on publicly available academic information and general knowledge of orbital mechanics. It contains no restricted, proprietary, or export-controlled information of any kind. This is a personal learning project only.
