# Implementation Plan for Issue #34: Godot Baseline Setup

**Compliance Note**  
This project is based entirely on publicly available academic information and general knowledge of orbital mechanics. It contains no restricted, proprietary, or export-controlled information of any kind. This is a personal learning project only.

## Objective
Establish the foundational 3D Godot scene for Phase 0 by replacing the existing Python prototype with a Godot 4 C# project.

## Required Steps for Coder (`// TODO-Coder:`)

### 1. Clean Up Existing Prototype
- Delete the current Python `game/` directory and all its contents entirely.

### 2. Scaffold Godot 4 C# Project
- Create a new Godot 4 C# project in a new `game/` folder.
- Use the standard Godot C# template.
- Configure the project settings as necessary for a 3D desktop target.

### 3. Create Foundational 3D Scene
- **Scene Root:** Create a `Node3D` as the root of the main scene.
- **Camera:** Add a `Camera3D` configured as:
  - Fixed top-down orthographic projection.
  - Positioned to view the Earth and orbit area clearly.
- **Scale:** Use the scale of `1 Godot unit = 1 km`.
- **Earth:** 
  - Add a 3D sphere (e.g., `MeshInstance3D` with a `SphereMesh`).
  - Apply a basic Earth texture PNG.
  - Configure the material to use flat shading with no lighting (unlit).
- **Satellite:** 
  - Add a simple colored 3D sphere (e.g., `MeshInstance3D` with `SphereMesh` and a basic colored unlit material).
- **Reference Orbit:** 
  - Draw a faint, static white circle to represent the reference orbit (can use a `MeshInstance3D` with `TorusMesh` or immediate geometry node).
- **Environment (Star Map):**
  - Add a `WorldEnvironment` node.
  - Configure a Sky system using a `PanoramaSkyMaterial` with an equirectangular star map PNG.

### 4. Constraints
- **DO NOT** add any code for movement, controls, or UDP networking yet. This issue is strictly for static scene scaffolding.
