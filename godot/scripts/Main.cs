// Compliance Note
// This project is based entirely on publicly available academic information and general knowledge of orbital mechanics. It contains no restricted, proprietary, or export-controlled information of any kind. This is a personal learning project only.

using Godot;
using DeltaWrecker.Orbital;

namespace DeltaWrecker;

/// <summary>
/// Main game scene controller. Mirrors game/main.py (Python/Pygame implementation).
///
/// Responsibilities:
///   - Initialize the orbital state for the player spacecraft
///   - Drive the game loop (_Process)
///   - Render the central body, orbit path, and spacecraft (_Draw)
///   - Handle quit / escape input
///
/// Visual output must match game/main.py:
///   - 800 × 600 viewport, dark-grey background (30, 30, 30)
///   - Blue filled circle at center: planet, radius 20 px
///   - Grey orbit ring: radius 150 px, 1 px thick
///   - Yellow filled circle: spacecraft, radius 5 px, animated along orbit
/// </summary>
public partial class Main : Node2D
{
    // ── Display constants (match Python game) ──────────────────────────────
    private const int ScreenWidth  = 800;
    private const int ScreenHeight = 600;
    private const int OrbitRadius  = 150;
    private const int PlanetRadius = 20;
    private const int ShipRadius   = 5;

    private static readonly Color BackgroundColor  = new Color(30  / 255f, 30  / 255f, 30  / 255f);
    private static readonly Color PlanetColor      = new Color(0   / 255f, 100 / 255f, 255 / 255f);
    private static readonly Color OrbitPathColor   = new Color(100 / 255f, 100 / 255f, 100 / 255f);
    private static readonly Color SpacecraftColor  = new Color(255 / 255f, 255 / 255f, 0   / 255f);

    // ── Orbital state ──────────────────────────────────────────────────────
    // TODO-Coder: Initialize _orbitState using a real OrbitState value that
    // places the spacecraft at (OrbitRadius, 0, 0) with appropriate circular
    // orbit velocity. Use OrbitalConstants.MuEarth as needed.
    private OrbitState _orbitState;

    // Running simulation time in seconds (accumulates delta each frame).
    private double _elapsedSeconds;

    // ── Godot lifecycle ────────────────────────────────────────────────────

    public override void _Ready()
    {
        // TODO-Coder: Set the window title to "Delta Wrecker".
        // In Godot 4: DisplayServer.WindowSetTitle("Delta Wrecker");

        GD.Print("Delta Wrecker – Godot initialised");

        // TODO-Coder: Initialise _orbitState with starting position and velocity
        // that matches the Python game's initial spacecraft position:
        //   center_x + orbit_radius at angle 0  →  (OrbitRadius, 0, 0)
        // Compute circular orbit velocity: v = sqrt(MuEarth / r)
        // _orbitState = new OrbitState(OrbitRadius, 0, 0, 0, v, 0);
    }

    public override void _Process(double delta)
    {
        // TODO-Coder: Handle quit/escape input equivalent to Pygame QUIT event:
        //   if (Input.IsActionPressed("ui_cancel")) GetTree().Quit();

        // TODO-Coder: Advance the orbital state each frame using the propagator:
        //   _orbitState = _orbitState.Propagate(delta);
        //   _elapsedSeconds += delta;

        // Trigger a redraw every frame (equivalent to pygame.display.flip()).
        QueueRedraw();
    }

    public override void _Draw()
    {
        // TODO-Coder: Fill background with BackgroundColor.
        // In Godot 4 Node2D you can draw a filled rect covering the viewport:
        //   DrawRect(new Rect2(0, 0, ScreenWidth, ScreenHeight), BackgroundColor);

        Vector2 center = new Vector2(ScreenWidth / 2f, ScreenHeight / 2f);

        // TODO-Coder: Draw the central planet (blue filled circle).
        //   DrawCircle(center, PlanetRadius, PlanetColor);

        // TODO-Coder: Draw the circular orbit path (grey ring, 1 px thick).
        //   DrawArc(center, OrbitRadius, 0, Mathf.Tau, 128, OrbitPathColor, 1f);

        // TODO-Coder: Compute the spacecraft screen position from _orbitState
        // and draw the yellow spacecraft dot.
        // The Python game uses a simple time-based formula:
        //   angle = time * 0.5
        //   ship_x = center_x + orbit_radius * cos(angle)
        //   ship_y = center_y + orbit_radius * sin(angle)
        // In the Godot version, derive position from _orbitState.X / _orbitState.Y
        // (which are already in "simulation km" — you will need a px-per-km scale
        // factor, or keep coordinates in screen-pixels for Phase 1).
        //   Vector2 shipPos = new Vector2(center.X + (float)_orbitState.X, center.Y + (float)_orbitState.Y);
        //   DrawCircle(shipPos, ShipRadius, SpacecraftColor);
    }
}
