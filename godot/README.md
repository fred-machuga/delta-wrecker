# Delta Wrecker – Godot Client

**Compliance Note**  
This project is based entirely on publicly available academic information and general knowledge of orbital mechanics. It contains no restricted, proprietary, or export-controlled information of any kind. This is a personal learning project only.

## Overview

This folder contains the **Godot 4 / C#** implementation of the Delta Wrecker game client.  
It is a direct port of the Python/Pygame client in `game/` and uses the same orbital mechanics
concepts implemented in the Rust engine (`engine/`).

## Folder Structure

```
godot/
├── project.godot              # Godot 4 project configuration (800×600, C#)
├── DeltaWrecker.csproj        # C# project file (Godot.NET.Sdk)
├── scenes/
│   └── Main.tscn              # Main scene (Node2D root)
├── scripts/
│   ├── Main.cs                # Game loop, input handling, rendering
│   └── orbital/
│       ├── OrbitalConstants.cs  # MU_EARTH, EARTH_RADIUS_KM
│       ├── Vec3.cs              # 3D vector (mirrors Rust Vec3)
│       ├── OrbitState.cs        # Position + velocity state (mirrors Rust OrbitState)
│       ├── KeplerianElements.cs # Orbital elements (mirrors Rust KeplerianElements)
│       └── Propagator.cs        # Circular orbit propagator (mirrors Rust propagator.rs)
└── tests/
    ├── DeltaWrecker.Tests.csproj
    ├── orbital/
    │   ├── OrbitStateTests.cs   # Tests for OrbitState (mirrors test_propagator_pyo3.py)
    │   └── PropagatorTests.cs   # Tests for Propagator
    └── rendering/
        └── RenderingTests.cs    # Tests for rendering math (mirrors test_rendering.py)
```

## Prerequisites

- [Godot 4.3+](https://godotengine.org/) with .NET / C# support enabled
- [.NET 8 SDK](https://dotnet.microsoft.com/)

## Running the Game

1. Open Godot 4, click **Import** and select `godot/project.godot`.
2. Press **F5** (or the Play button) to run.

## Running Tests

The test project uses NUnit and references the orbital C# scripts directly
(no Godot engine required at test time).

```bash
cd godot/tests
dotnet test
```

## Target Visual Output

Matches `game/main.py` exactly:

| Element | Description |
|---|---|
| Window | 800 × 600, title "Delta Wrecker" |
| Background | Dark grey `(30, 30, 30)` |
| Planet | Blue filled circle, radius 20 px, centered |
| Orbit path | Grey ring, radius 150 px, 1 px thick |
| Spacecraft | Yellow filled circle, radius 5 px, animated along orbit |
| Frame rate | 60 FPS |

## Relationship to Other Components

| Component | Language | Role |
|---|---|---|
| `engine/` | Rust | Core orbital math library (authoritative) |
| `game/` | Python / Pygame | Original game client |
| `godot/` | C# / Godot 4 | Godot game client (this folder) |

The C# orbital math classes in `scripts/orbital/` mirror the Rust engine so
that the Godot client can run standalone without the PyO3 bindings.

**Compliance Note**  
This project is based entirely on publicly available academic information and general knowledge of orbital mechanics. It contains no restricted, proprietary, or export-controlled information of any kind. This is a personal learning project only.
