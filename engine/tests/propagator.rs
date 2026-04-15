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
#[test]
fn test_propagate_zero_delta_time() {
    let r = 7000.0;
    let v = 7.546;
    let state = OrbitState::new(r, 0.0, 0.0, 0.0, v, 0.0);
    let new_state = propagate(&state, 0.0);
    assert_eq!(new_state, state);
}

#[test]
fn test_propagate_negative_delta_time() {
    let r = 7000.0;
    let v = 7.546;
    let state = OrbitState::new(r, 0.0, 0.0, 0.0, v, 0.0);
    let omega = v / r;
    let period = 2.0 * PI / omega;
    let dt = period / 4.0;
    let new_state_neg = propagate(&state, -dt);
    // After -quarter: position (0, -r, 0), velocity (v, 0, 0)
    assert!((new_state_neg.x - 0.0).abs() < 1e-6);
    assert!((new_state_neg.y - (-r)).abs() < 1e-6);
    assert!((new_state_neg.z - 0.0).abs() < 1e-6);
    assert!((new_state_neg.vx - v).abs() < 1e-6);
    assert!((new_state_neg.vy - 0.0).abs() < 1e-6);
    assert!((new_state_neg.vz - 0.0).abs() < 1e-6);
}

#[test]
fn test_propagate_non_zero_z() {
    let r = 7000.0;
    let v = 7.546;
    let state = OrbitState::new(r, 0.0, 100.0, 0.0, v, 5.0);
    let omega = v / r;
    let period = 2.0 * PI / omega;
    let dt = period / 4.0;
    let new_state = propagate(&state, dt);
    assert!((new_state.z - 100.0).abs() < 1e-6);
    assert!((new_state.vz - 5.0).abs() < 1e-6);
}

#[test]
fn test_propagate_different_initial_orientation() {
    let r = 7000.0;
    let v = 7.546;
    let state = OrbitState::new(0.0, r, 0.0, -v, 0.0, 0.0);
    let omega = v / r;
    let period = 2.0 * PI / omega;
    let dt = period / 4.0;
    let new_state = propagate(&state, dt);
    // After quarter: position (-r, 0, 0), velocity (0, -v, 0)
    assert!((new_state.x - (-r)).abs() < 1e-6);
    assert!((new_state.y - 0.0).abs() < 1e-6);
    assert!((new_state.z - 0.0).abs() < 1e-6);
    assert!((new_state.vx - 0.0).abs() < 1e-6);
    assert!((new_state.vy - (-v)).abs() < 1e-6);
    assert!((new_state.vz - 0.0).abs() < 1e-6);
}

#[test]
fn test_frame_rate_independence() {
    let r = 7000.0;
    let v = 7.546;
    let state = OrbitState::new(r, 0.0, 0.0, 0.0, v, 0.0);
    let omega = v / r;
    let period = 2.0 * PI / omega;
    let dt = period / 4.0;
    // One large step
    let single_step = propagate(&state, dt);
    // Multiple small steps (10 steps)
    let mut multi_step = state;
    let small_dt = dt / 10.0;
    for _ in 0..10 {
        multi_step = propagate(&multi_step, small_dt);
    }
    // Compare
    assert!((single_step.x - multi_step.x).abs() < 1e-6);
    assert!((single_step.y - multi_step.y).abs() < 1e-6);
    assert!((single_step.z - multi_step.z).abs() < 1e-6);
    assert!((single_step.vx - multi_step.vx).abs() < 1e-6);
    assert!((single_step.vy - multi_step.vy).abs() < 1e-6);
    assert!((single_step.vz - multi_step.vz).abs() < 1e-6);
}
// **Compliance Note**
// This project is based entirely on publicly available academic information and general knowledge of orbital mechanics. It contains no restricted, proprietary, or export-controlled information of any kind. This is a personal learning project only.
