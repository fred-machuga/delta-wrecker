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

10. The final deliverable must be a complete, playable game with at least one full gameplay loop (intercept → grapple → tow).**Compliance Note**  
This project is based entirely on publicly available academic information and general knowledge of orbital mechanics. It contains no restricted, proprietary, or export-controlled information of any kind. This is a personal learning project only.

# Requirements

The Delta Wrecker project has the following mandatory requirements. These requirements are measurable and will be used to determine project completion.

### Technical Requirements

1. The Rust orbital mechanics library **must** provide accurate, bidirectional conversion between Cartesian state vectors (position and velocity) and Keplerian orbital elements (semi-major axis, eccentricity, inclination, etc.).
2. The game **must** maintain a stable 60 FPS during normal gameplay in Phase 1.
3. The project **must** enforce a strict architectural separation between the reusable Rust orbital math library and the Python game/UI layer.
4. All core orbital math functions **must** be covered by unit tests that validate results against known reference solutions from standard astrodynamics literature.
5. The game **must** implement a complete, end-to-end gameplay loop consisting of target acquisition, orbital intercept, grappling, and towing under realistic mass and thrust constraints.
6. The project **must** include a working save/load system so player progress is preserved between sessions.
7. The entire codebase **must** build and run cleanly on a standard Rust toolchain using only `cargo run`.

### Learning Requirements

8. The developer **must** personally implement a significant portion of the orbital math (including propagation, burn calculations, and Kepler’s equation) to ensure deep learning of both Rust and orbital mechanics.
9. The orbital mechanics library **must** be written with clean, well-documented, reusable APIs so that it can be extracted and used in future independent projects.

### Project Requirements

10. The project **must** be structured in three progressive phases with increasing orbital complexity (near-circular → eccentric → inclined) as defined in the roadmap.

**Compliance Note**  
This project is based entirely on publicly available academic information and general knowledge of orbital mechanics. It contains no restricted, proprietary, or export-controlled information of any kind. This is a personal learning project only.
