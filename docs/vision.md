**Compliance Note**  
This project is based entirely on publicly available academic information and general knowledge of orbital mechanics. It contains no restricted, proprietary, or export-controlled information of any kind. This is a personal learning project only.

# Vision

**Game Title:** Delta Wrecker

**One-Line Pitch**  
An independent orbital wrecker game where you rescue stranded satellites and clear dangerous debris using real orbital mechanics.

## Core Fantasy
You run a small one-ship salvage company. When satellites fail or debris threatens active orbits, you’re the one who answers the call. You plan the intercept, execute the burn, grapple the target, and tow it safely — all while making a profit and upgrading your ship.

## Core Gameplay
- Use real orbital mechanics to plan intercepts and burns
- Execute instantaneous or impulsive burns
- Switch to a placeholder grappling animation when close
- Deliver debris to an orbital station for payment and upgrades
- Progress by purchasing ship upgrades that unlock new capabilities

## Progression
The game progresses through increasing levels of orbital and technical complexity. Each phase adds new orbital math, new visualization features, and new player capabilities (unlocked via upgrades):

- **Phase 0** – Foundation: Basic circular orbits and top-down view
- **Phase 1** – First Player Interaction: Instantaneous burns and multiple camera views
- **Phase 2** – Eccentric Orbits: Proper Keplerian math and improved visualization
- **Phase 3** – Multi-Object & COLA: RSOs, 3-sigma zones, and basic salvage
- **Phase 4** – Salvage Loop & Progression: Full jobs, money, and first upgrades
- **Phase 5** – Inclined Orbits & 3D: 3D views and advanced towing

## Technical Vision
- **Rust** → Clean, reusable orbital math library
- **Godot 4 (C#)** → Game/client layer
- Rust engine exposed to Godot via local UDP sockets

Because we are using Godot 4, we have the flexibility to introduce multiple camera views and true 3D capability earlier than originally planned once the 2D viewer is stable.

## Goals
- Deeply learn Rust and functional programming
- Learn real orbital mechanics by implementing the math myself
- Build a clean, reusable Rust orbital math library
- Create something visually engaging and technically impressive that I’m proud to show others

**Compliance Note**  
This project is based entirely on publicly available academic information and general knowledge of orbital mechanics. It contains no restricted, proprietary, or export-controlled information of any kind. This is a personal learning project only.