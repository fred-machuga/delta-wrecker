using Godot;
using System;

public partial class Main : Node3D
{
    private const float EarthRadiusKm = 6371.0f;
    private const float OrbitRadiusKm = 6771.0f;

    private MeshInstance3D _satellite;
    private float _time = 0;

    public override void _Ready()
    {
        _satellite = GetNode<MeshInstance3D>("Satellite");
        GD.Print("Main Ready! Satellite loaded: ", _satellite != null);
    }

    public override void _Process(double delta)
    {
        _time += (float)delta;
        float speed = 5.0f; 
        float angle = _time * speed;
        
        float x = Mathf.Cos(angle) * OrbitRadiusKm;
        float z = Mathf.Sin(angle) * OrbitRadiusKm;
        
        if (_satellite != null)
        {
            _satellite.Position = new Vector3(x, 0, z);
        }
    }
}
