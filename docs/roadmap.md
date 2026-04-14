**Compliance Note**  
This project is based entirely on publicly available academic information and general knowledge of orbital mechanics. It contains no restricted, proprietary, or export-controlled information of any kind. This is a personal learning project only.

# Roadmap

**Overall Goal**  
Build Delta Wrecker while systematically learning Rust and real orbital mechanics through three progressive phases.

## Phase 1 – Near-Circular Operations (Foundation)
- 2D top-down view
- Very low eccentricity orbits only (e < 0.05)
- Simple approximations (no full Kepler’s equation yet)
- Basic Hohmann transfers
- Simple instantaneous burns
- Basic grappling minigame
- Deliver debris to the Pre-Graveyard Belt

**Learning Focus:** Rust + PyO3 setup, basic orbital elements, simple propagation

## Phase 2 – Eccentric Operations (Core Mechanics)
- Increase allowed eccentricity
- Implement full Kepler’s equation solver
- Proper eccentric orbit propagation
- Introduce COLA detection and warnings
- Salvage runs inside the dangerous Pre-Graveyard Belt
- Docking at salvage shops for upgrades

**Learning Focus:** Kepler’s equation, true anomaly, better burn planning

## Phase 3 – Inclined & Advanced Operations (Mastery)
- Add inclination (move to 3D view)
- Advanced COLA analysis
- Towing with attached mass and strict thrust limits
- High-value recovery missions
- Basic burn optimizer

**Learning Focus:** 3D math, inclined orbits, precision maneuvering with constraints
**Note:** Pushing from 2D continuous rendering (Pygame) to 3D will require a deliberate pivot and engine decision (e.g., PyOpenGL, moving entirely to Bevy/Macroquad).

**Compliance Note**  
This project is based entirely on publicly available academic information and general knowledge of orbital mechanics. It contains no restricted, proprietary, or export-controlled information of any kind. This is a personal learning project only.