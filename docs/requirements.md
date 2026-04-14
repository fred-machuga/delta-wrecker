**Compliance Note**  
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
7. The entire codebase **must** build through a standardized Rust-to-Python interoperability pipeline (e.g. `maturin develop`), where the Python interpreter runs the game loop while offloading math calculation to a `cdylib` compiled from the Rust `src` folder.

### Learning Requirements

8. The developer **must** personally implement a significant portion of the orbital math (including propagation, burn calculations, and Kepler’s equation) to ensure deep learning of both Rust and orbital mechanics.
9. The orbital mechanics library **must** be written with clean, well-documented, reusable APIs so that it can be extracted and used in future independent projects.

### Project Requirements

10. The project **must** be structured in three progressive phases with increasing orbital complexity (near-circular → eccentric → inclined) as defined in the roadmap.

**Compliance Note**  
This project is based entirely on publicly available academic information and general knowledge of orbital mechanics. It contains no restricted, proprietary, or export-controlled information of any kind. This is a personal learning project only.
