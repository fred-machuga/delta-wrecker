**Compliance Note**  
This project is based entirely on publicly available academic information and general knowledge of orbital mechanics. It contains no restricted, proprietary, or export-controlled information of any kind. This is a personal learning project only.

# Requirements

## Functional Requirements

The game **MUST**:
- Allow the player to control an orbital tug using real orbital mechanics
- Require the player to calculate and execute burns to intercept stranded satellites or debris
- Support establishing a perch orbit before closing distance
- Include a fun, arcade-style grappling minigame once the tug is within three sigma of the target
- Allow the player to tow the payload to either a graveyard orbit or a repair station
- Enforce realistic thrust limits when towing (especially with attached mass)
- Progress through three distinct phases with increasing orbital complexity

## Phase Requirements

**Phase 1 (Near-Circular Operations) MUST:**
- Use only very low eccentricity orbits (e < 0.05)
- Support basic Hohmann-style transfers
- Use simple instantaneous burns
- Not require full Kepler’s equation

**Phase 2 (Eccentric Operations) MUST:**
- Support higher eccentricity orbits
- Implement and use a full Kepler’s equation solver
- Include COLA detection and warnings

**Phase 3 (Inclined Operations) MUST:**
- Support inclination (transition to 3D view)
- Handle towing with attached mass and strict thrust limits
- Include advanced COLA analysis

## Technical Requirements

The Rust orbital library **MUST**:
- Be clean and written in functional style
- Provide PyO3 bindings for use from Python
- Support orbit propagation for both circular and eccentric orbits
- Include Kepler’s equation solver
- Be designed as a reusable crate

## Non-Functional Requirements

The project **MUST**:
- Be visually engaging and fun to play (not just blinking dots)
- Include the compliance note at the top and bottom of every markdown and source file
- Follow clean architecture with clear separation between Rust (math) and Python (game/UI)
- Be suitable for demonstrating Rust and orbital mechanics skills

**Compliance Note**  
This project is based entirely on publicly available academic information and general knowledge of orbital mechanics. It contains no restricted, proprietary, or export-controlled information of any kind. This is a personal learning project only.
