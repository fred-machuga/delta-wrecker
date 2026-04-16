**Compliance Note**  
This project is based entirely on publicly available academic information and general knowledge of orbital mechanics. It contains no restricted, proprietary, or export-controlled information of any kind. This is a personal learning project only.

# Roadmap

**Overall Goal**  
Build *Delta Wrecker* while systematically increasing both **orbital mathematics complexity** and **game/visualization capabilities** in parallel. Each phase ends with a releasable demo and may contain multiple sprints. The number of phases is intentionally open-ended.

## Phase 0 – Foundation
**Orbital Learning:** Simple near-circular propagation (e < 0.05) using simplified circular math.  
**Game / Visualization Features:** Basic 2D top-down view.  
**User Interaction Capabilities:** Start/stop simulation, adjust time multiplier, basic camera zoom.

**Key Deliverables:** Godot 4 (C#) project setup, Rust engine integration via local UDP sockets, clean game loop, Earth + orbiting dot + faint full orbit circle, basic orbit information display.

**Releasable Demo:** A simple orbiting satellite viewer where the player can watch a satellite move around Earth in real time and control simulation speed.

## Phase 1 – First Player Interaction
**Orbital Learning:** Instantaneous delta-V burns while keeping orbits near-circular (e < 0.05).  
**Game / Visualization Features:** Multiple camera views and basic orbit preview.  
**User Interaction Capabilities:** 
- Plan and execute instantaneous burns (choose apoapsis/perigee/prograde/retrograde + delta-V amount)
- See preview of the new orbit before executing
- Switch between top-down and side/tilted orbit views
- Adjust time multiplier

**Key Deliverables:** Burn planning interface, old/new orbit visualization, improved camera system.

**Releasable Demo:** The player can plan and execute their first burns and see the orbit change in real time.

## Phase 2 – Eccentric Orbits & Improved Visualization
**Orbital Learning:** Eccentric orbits, introduction of Kepler’s equation and true anomaly.  
**Game / Visualization Features:** History-based fading orbital trail and refined burn planning.  
**User Interaction Capabilities:** 
- Plan burns on eccentric orbits
- See preview of the new orbit before executing
- See both old and new orbit simultaneously until the burn occurs

**Key Deliverables:** Proper eccentric propagation, fading trail system, improved burn planning.

**Releasable Demo:** The player can create and fly eccentric orbits with a nice visual trail.

## Phase 3 – Multi-Object System & Basic COLA
**Orbital Learning:** Multi-object propagation and basic covariance / COLA (Conjunction Analysis).  
**Game / Visualization Features:** Introduction of RSO (debris) objects and 3-sigma zone visualization.  
**User Interaction Capabilities:** 
- Intercept moving RSOs using burns
- See 3-sigma zone around both player and target
- Trigger placeholder grappling animation when inside 3-sigma zone
- Simple mass update when towing

**Key Deliverables:** Multi-object system, basic COLA, placeholder grappling animation.

**Releasable Demo:** The player can intercept a piece of debris and “grapple” it (placeholder animation).

## Phase 4 – Salvage Loop & Progression
**Orbital Learning:** Advanced COLA usage and basic rendezvous mechanics.  
**Game / Visualization Features:** Full salvage loop with delivery to station.  
**User Interaction Capabilities:** 
- Deliver debris to an orbital station (placeholder animation)
- Earn credits from completed salvage jobs
- Purchase basic ship upgrades

**Key Deliverables:** Delivery system, basic money system, first upgrade system.

**Releasable Demo:** The player can complete full salvage jobs, earn money, and buy upgrades.

## Phase 5 – Inclined Orbits & 3D
**Orbital Learning:** Inclination and full 3D orbital elements.  
**Game / Visualization Features:** True 3D capability.  
**User Interaction Capabilities:** 
- Operate in inclined orbits
- Use 3D vehicle-follow camera
- Advanced towing with attached mass and thrust limits

**Key Deliverables:** Inclined orbit support, 3D camera system, advanced towing.

**Releasable Demo:** The player can operate in inclined orbits with full 3D views.

## Future Phases (Open-Ended)
Additional phases will be added based on progress and interest. Possible future topics include:
- Advanced RPO / proximity operations
- Burn optimization and planning tools
- Multi-body effects and high-fidelity perturbations
- Free camera / manual camera control

## Technical Approach
- **Rust** → Clean, reusable orbital math library
- **Godot 4 (C#)** → Game/client layer
- Rust engine exposed to Godot via local UDP sockets

**Compliance Note**  
This project is based entirely on publicly available academic information and general knowledge of orbital mechanics. It contains no restricted, proprietary, or export-controlled information of any kind. This is a personal learning project only.