# Delta Wrecker

**Compliance Note**  
This project is based entirely on publicly available academic information and general knowledge of orbital mechanics. It contains no restricted, proprietary, or export-controlled information of any kind. This is a personal learning project only.

## Overview

Delta Wrecker is a 2D orbital wrecker game where you operate an independent salvage tug, rescuing stranded satellites and clearing dangerous debris using real orbital mechanics.

This project is built across two distinct layers:
1. **Rust Library (`src/`)**: A clean, reusable core framework handling orbital mechanics formulas, propagations, and math objects (Cartesian vectors, Keplerian elements).
2. **Python UI (`main.py`)**: A game client built using Pygame that calls the Rust library via PyO3 bindings for an arcade-style visual and control loop.

## Documentation Overview

Our documentation is structured sequentially in the `docs/` folder.  
Please consult these files in order to understand the project:

- [Vision](docs/vision.md): The overall direction, core fantasy, and design philosophy.
- [Project Charter](docs/project-charter.md): The purpose, target audience, and success criteria.
- [Roadmap](docs/roadmap.md): The three-phase implementation approach.
- [Requirements](docs/requirements.md): Actionable requirements and limits.
- [About Me](docs/about-me.md): Background on my goals, skills, and areas of focus.

**Compliance Note**  
This project is based entirely on publicly available academic information and general knowledge of orbital mechanics. It contains no restricted, proprietary, or export-controlled information of any kind. This is a personal learning project only.