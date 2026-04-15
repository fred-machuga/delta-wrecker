**Compliance Note**  
This project is based entirely on publicly available academic information and general knowledge of orbital mechanics. It contains no restricted, proprietary, or export-controlled information of any kind. This is a personal learning project only.

# Delta Wrecker - Project Charter

**One-Line Description**  
A 2D orbital wrecker game where you operate an independent salvage tug, rescuing stranded satellites and clearing dangerous debris using real orbital mechanics.

## Purpose
This project serves as my primary learning vehicle to:
- Deeply improve my Rust skills and functional programming style
- Learn real orbital mechanics by implementing the math myself (propagation, burns, Kepler’s equation, eccentricity, inclination, COLA, etc.)
- Build a clean, reusable Rust orbital math library
- Create something visual, interactive, and technically impressive that I’m proud to show others

## Target Audience
- Primarily myself (for learning and personal satisfaction)
- Future employers or technical peers who want to see demonstrated skills in Rust and orbital mechanics

## Success Criteria
The project will be considered successful when:
- The game is playable and visually engaging (not just blinking dots)
- I can confidently explain the orbital mechanics I implemented
- The Rust code demonstrates clean architecture and good functional patterns
- I have a reusable orbital math crate that can be extended in future projects
- The overall project is well-structured and properly documented

## High-Level Approach
- **Rust** → Core orbital math library (propagation, burns, Kepler solver, etc.)
- **Python + Pygame** → Game layer, UI, and arcade-style grappling minigame (for Phases 1 & 2)
- **Architectural Checkpoint** → Due to the hard shift from a 2D top-down view to 3D in Phase 3, this requires a re-evaluation of the game engine (e.g., PyOpenGL or a complete shift to a Rust 3D framework like Bevy).
- Agile development with planning done here with Ara and implementation split between me and GitHub Copilot
- Three progressive phases that naturally increase in orbital and technical complexity

## Out of Scope (for now)
- Full n-body simulation
- Highly realistic RPO / proximity operations
- Multiplayer features
- High-fidelity perturbations (atmospheric drag, solar pressure, etc.)

**Compliance Note**  
This project is based entirely on publicly available academic information and general knowledge of orbital mechanics. It contains no restricted, proprietary, or export-controlled information of any kind. This is a personal learning project only.
