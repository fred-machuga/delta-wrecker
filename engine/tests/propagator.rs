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

#[test]
fn test_velocity_perpendicular_to_position() {
    // Test that after propagation, the velocity vector remains roughly perpendicular to the position vector
    // - Compute dot product of position and velocity vectors
    // - Assert dot product is close to zero (within tolerance, e.g., 1e-6)
    let r = 7000.0;
    let v = 7.546;
    let state = OrbitState::new(r, 0.0, 0.0, 0.0, v, 0.0);
    let new_state = propagate(&state, 1.0); // Propagate by 1 second
    let pos = new_state.position();
    let vel = new_state.velocity();
    let dot_product = pos.x * vel.x + pos.y * vel.y + pos.z * vel.z;
    assert!(dot_product.abs() < 1e-6);
}

#[test]
fn test_altitude_stability_over_multiple_steps() {
    // Test that altitude stays roughly stable after multiple propagation steps
    // - Propagate over several steps (e.g., 100 small steps)
    // - Compute altitude at each step
    // - Assert that altitude variation is minimal (doesn't spiral in or out, within tolerance)
    let r = 7000.0;
    let v = 7.546;
    let mut state = OrbitState::new(r, 0.0, 0.0, 0.0, v, 0.0);
    let initial_alt = state.altitude_km();
    let dt = 1.0;
    for _ in 0..100 {
        state = propagate(&state, dt);
        let alt = state.altitude_km();
        assert!((alt - initial_alt).abs() < 0.01); // Very tight tolerance for stability
    }
}

#[test]
fn test_realistic_iss_like_orbit() {
    // Test using a realistic circular orbit (~6778 km radius / ~400 km altitude, like ISS)
    // - Use approximate ISS orbital parameters: radius ~6778 km, velocity ~7.66 km/s
    // - Propagate over one orbital period
    // - Assert final state matches initial state within tolerance
    let r = 6778.0; // km
    let v = 7.66; // km/s
    let state = OrbitState::new(r, 0.0, 0.0, 0.0, v, 0.0);
    let omega = v / r;
    let period = 2.0 * PI / omega;
    let new_state = propagate(&state, period);
    // Should return to initial state
    assert!((new_state.x - r).abs() < 1e-3);
    assert!((new_state.y - 0.0).abs() < 1e-3);
    assert!((new_state.z - 0.0).abs() < 1e-3);
    assert!((new_state.vx - 0.0).abs() < 1e-3);
    assert!((new_state.vy - v).abs() < 1e-3);
    assert!((new_state.vz - 0.0).abs() < 1e-3);
}

#[test]
fn test_small_vs_large_dt_consistency() {
    // Test that propagating with a very small dt_s vs a larger dt_s produces reasonably similar results
    // - Choose a total time interval
    // - Propagate with small dt (many steps) and large dt (few steps)
    // - Compare final states
    // - Assert positions and velocities match within tolerance
    let r = 7000.0;
    let v = 7.546;
    let state = OrbitState::new(r, 0.0, 0.0, 0.0, v, 0.0);
    let total_time = 10.0; // seconds
    // Small dt: 100 steps
    let mut small_step = state;
    let small_dt = total_time / 100.0;
    for _ in 0..100 {
        small_step = propagate(&small_step, small_dt);
    }
    // Large dt: 1 step
    let large_step = propagate(&state, total_time);
    // Compare
    assert!((small_step.x - large_step.x).abs() < 1e-4);
    assert!((small_step.y - large_step.y).abs() < 1e-4);
    assert!((small_step.z - large_step.z).abs() < 1e-4);
    assert!((small_step.vx - large_step.vx).abs() < 1e-4);
    assert!((small_step.vy - large_step.vy).abs() < 1e-4);
    assert!((small_step.vz - large_step.vz).abs() < 1e-4);
}

// **Compliance Note**
// This project is based entirely on publicly available academic information and general knowledge of orbital mechanics. It contains no restricted, proprietary, or export-controlled information of any kind. This is a personal learning project only.
