using Godot;
using System;

public partial class Main : Node3D
{
    // Scale: 1 Godot unit = 1 km
    private const float EarthRadiusKm = 6371.0f;
    private const float OrbitRadiusKm = 6771.0f; // 400km altitude LEO

    public override void _Ready()
    {
        // Nodes are now loaded from Main.tscn
    }
}
