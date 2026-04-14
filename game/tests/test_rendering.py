# Compliance Note
# This project is based entirely on publicly available academic information and general knowledge of orbital mechanics. It contains no restricted, proprietary, or export-controlled information of any kind. This is a personal learning project only.

import math
import pytest


def test_spacecraft_position_calculation():
    """Test basic circular orbit position calculation."""
    center_x, center_y = 400, 300
    orbit_radius = 150

    # Test at time 0 (should be at (center_x + radius, center_y))
    time = 0.0
    angle = time * 0.5
    ship_x = center_x + orbit_radius * math.cos(angle)
    ship_y = center_y + orbit_radius * math.sin(angle)

    assert ship_x == pytest.approx(400 + 150)  # 550
    assert ship_y == pytest.approx(300)  # 300

    # Test at time for 90 degrees (pi/2)
    time = math.pi  # since angle = time * 0.5, so time = pi for angle = pi/2
    angle = time * 0.5
    ship_x = center_x + orbit_radius * math.cos(angle)
    ship_y = center_y + orbit_radius * math.sin(angle)

    assert ship_x == pytest.approx(400)  # back to center_x
    assert ship_y == pytest.approx(300 + 150)  # 450


def test_orbit_parameters():
    """Test that orbit parameters are reasonable."""
    orbit_radius = 150
    assert orbit_radius > 0

    center_x, center_y = 400, 300
    assert isinstance(center_x, int)
    assert isinstance(center_y, int)


# Compliance Note
# This project is based entirely on publicly available academic information and general knowledge of orbital mechanics. It contains no restricted, proprietary, or export-controlled information of any kind. This is a personal learning project only.