**Compliance Note**  
This project is based entirely on publicly available academic information and general knowledge of orbital mechanics. It contains no restricted, proprietary, or export-controlled information of any kind. This is a personal learning project only.

# Requirements

The Delta Wrecker project must satisfy the following requirements:

### Core Technical Requirements

1. The orbital mechanics library must provide accurate bidirectional conversion between Cartesian state vectors (position and velocity) and Keplerian orbital elements.
2. The game must maintain a stable 60 FPS during Phase 1 gameplay.
3. The game must use a clean 2D top-down view for all of Phase 1.
4. The codebase must maintain a clear separation between the reusable orbital math library (Rust) and the game layer (Python + pygame).
5. All core orbital math functions must have unit tests using known reference values.
6. The project must include a working save/load system so progress is not lost between sessions.
7. The project must be able to be cloned, built, and run using only standard Rust tools (`cargo run`).

### Learning Requirements

8. The project must be structured so that I actively implement and understand significant portions of the orbital math myself.
9. The difficulty of orbital mechanics must increase progressively across the three phases (circular → eccentric → inclined orbits).

### Project Requirements

10. The final deliverable must be a complete, playable game with at least one full gameplay loop (intercept → grapple → tow).

**Compliance Note**  
This project is based entirely on publicly available academic information and general knowledge of orbital mechanics. It contains no restricted, proprietary, or export-controlled information of any kind. This is a personal learning project only.
