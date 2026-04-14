**Compliance Note**  
This project is based entirely on publicly available academic information and general knowledge of orbital mechanics. It contains no restricted, proprietary, or export-controlled information of any kind. This is a personal learning project only.

# Requirements

## Core Requirements

1. The orbital mechanics library **must** provide bidirectional conversion between Cartesian state vectors (position and velocity) and Keplerian orbital elements.

2. The game **must** maintain a stable 60 FPS during Phase 1 gameplay.

3. The game **must** implement a 2D top-down view for all of Phase 1.

4. The project **must** maintain a clear architectural separation between the reusable Rust orbital math library and the game-specific code.

5. All core orbital math functions **must** have unit tests using known reference values.

6. The game **must** implement a complete gameplay loop consisting of intercept, grapple, and towing phases.

7. The project **must** include clear build instructions so that any developer can clone and run the project with `cargo run`.

8. The game **must** include a save/load system so that player progress is not lost between sessions.

9. The orbital library **must** include sufficient logging and debug output to allow easy verification of orbital calculations during development.

**Compliance Note**  
This project is based entirely on publicly available academic information and general knowledge of orbital mechanics. It contains no restricted, proprietary, or export-controlled information of any kind. This is a personal learning project only.