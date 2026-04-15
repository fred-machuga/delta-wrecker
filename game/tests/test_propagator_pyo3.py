# Compliance Note
# This project is based entirely on publicly available academic information and general knowledge of orbital mechanics. It contains no restricted, proprietary, or export-controlled information of any kind. This is a personal learning project only.

import pytest
import delta_wrecker

def test_propagator_init():
    # TODO-Coder: Test instantiating the propagator via PyO3
    prop = delta_wrecker.PyPropagator(7000.0, 0.0, 0.0, 0.0, 7.5, 0.0, 0.0)
    assert prop is not None

def test_propagator_step():
    # TODO-Coder: Test the step(dt) method
    prop = delta_wrecker.PyPropagator(7000.0, 0.0, 0.0, 0.0, 7.5, 0.0, 0.0)
    prop.step(10.0)
    state = prop.state
    assert state.time == 10.0

def test_propagator_state_dataclass():
    # TODO-Coder: Test that we can read the current ship state as a clean Python dataclass
    # (pos, vel, time, orbital elements)
    prop = delta_wrecker.PyPropagator(7000.0, 0.0, 0.0, 0.0, 7.5, 0.0, 0.0)
    state = prop.state
    
    assert hasattr(state, "pos")
    assert hasattr(state, "vel")
    assert hasattr(state, "time")
    assert hasattr(state, "orbital_elements")

    assert state.pos == (7000.0, 0.0, 0.0)
    assert state.vel == (0.0, 7.5, 0.0)
    assert state.time == 0.0
    
    elements = state.orbital_elements
    assert hasattr(elements, "semi_major_axis")
    # For a circular orbit near Earth with v=7.5 and r=7000, 
    # we expect the semi_major_axis to be computed.
    assert elements.semi_major_axis > 0

# Compliance Note
# This project is based entirely on publicly available academic information and general knowledge of orbital mechanics. It contains no restricted, proprietary, or export-controlled information of any kind. This is a personal learning project only.
# This project is based entirely on publicly available academic information and general knowledge of orbital mechanics. It contains no restricted, proprietary, or export-controlled information of any kind. This is a personal learning project only.
