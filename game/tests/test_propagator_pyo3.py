# Compliance Note
# This project is based entirely on publicly available academic information and general knowledge of orbital mechanics. It contains no restricted, proprietary, or export-controlled information of any kind. This is a personal learning project only.

import pytest
import delta_wrecker

def test_propagator_init():
    # TODO-Coder: Test instantiating the propagator via PyO3
    prop = delta_wrecker.PyOrbitState(7000.0, 0.0, 0.0, 0.0, 7.5, 0.0)
    assert prop is not None

def test_propagator_step():
    # TODO-Coder: Test the step(dt) method
    prop = delta_wrecker.PyOrbitState(7000.0, 0.0, 0.0, 0.0, 7.5, 0.0)
    next_state = prop.propagate(10.0)
    # the new state should have updated pos and vel
    assert next_state is not None
    assert next_state.x != 7000.0  # it moved!

def test_propagator_state_dataclass():
    # TODO-Coder: Test that we can read the current ship state as a clean Python dataclass
    # (pos, vel, time, orbital elements)
    state = delta_wrecker.PyOrbitState(7000.0, 0.0, 0.0, 0.0, 7.5, 0.0)
    
    assert hasattr(state, "pos")
    assert hasattr(state, "vel")
    assert hasattr(state, "x")
    assert hasattr(state, "y")
    assert hasattr(state, "z")
    assert hasattr(state, "vx")
    assert hasattr(state, "vy")
    assert hasattr(state, "vz")

    assert state.pos == (7000.0, 0.0, 0.0)
    assert state.vel == (0.0, 7.5, 0.0)
    
    assert state.distance_km() == 7000.0
    assert state.speed_kms() == 7.5

# Compliance Note
# This project is based entirely on publicly available academic information and general knowledge of orbital mechanics. It contains no restricted, proprietary, or export-controlled information of any kind. This is a personal learning project only.
# This project is based entirely on publicly available academic information and general knowledge of orbital mechanics. It contains no restricted, proprietary, or export-controlled information of any kind. This is a personal learning project only.
