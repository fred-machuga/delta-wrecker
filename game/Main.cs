/*
 * TODO-Fred: Download or provide an Earth texture (earth.png) and an equirectangular star map (starmap.png). Place them in game/assets/.
 */
/*
 * TODO-Coder: Fix the following issues from Tech Lead review:
 * 1. The scene hierarchy (Camera, Earth, Satellite, Orbit, Sky) should be instantiated inside Main.tscn using the Godot Editor or by writing the raw TSCN file, NOT procedurally via C# script in _Ready().
 * 2. Connect a basic Earth texture PNG to the Earth mesh.
 * 3. Add an equirectangular star map PNG for the PanoramaSkyMaterial as requested.
 * 4. Remove all procedural generation of nodes inside the _Ready() function once they are replicated in Main.tscn.
 */
using Godot;
using System;

public partial class Main : Node3D
{
    // Scale: 1 Godot unit = 1 km
    private const float EarthRadiusKm = 6371.0f;
    private const float OrbitRadiusKm = 6771.0f; // 400km altitude LEO

    public override void _Ready()
    {
        // 1. Camera3D (Orthographic)
        var camera = new Camera3D
        {
            Name = "MainCamera",
            Projection = Camera3D.ProjectionType.Orthogonal,
            Position = new Vector3(0, 20000, 0), // Positioned top-down
            Size = 25000, // View width
            Far = 50000
        };
        camera.LookAt(Vector3.Zero);
        AddChild(camera);

        // 2. Earth Sphere (Flat shaded)
        var earthMesh = new SphereMesh
        {
            Radius = EarthRadiusKm,
            Height = EarthRadiusKm * 2,
            RadialSegments = 64,
            Rings = 32
        };
        var earthMaterial = new StandardMaterial3D
        {
            AlbedoColor = new Color(0.1f, 0.4f, 0.8f), // Blue placeholder
            ShadingMode = BaseMaterial3D.ShadingModeEnum.Unshaded // Flat shaded/no lighting
        };
        var earthNode = new MeshInstance3D
        {
            Name = "Earth",
            Mesh = earthMesh,
            MaterialOverride = earthMaterial
        };
        AddChild(earthNode);

        // 3. Satellite Sphere
        var satMesh = new SphereMesh
        {
            Radius = 200.0f, // Exaggerated size to be visible
            Height = 400.0f
        };
        var satMaterial = new StandardMaterial3D
        {
            AlbedoColor = new Color(1.0f, 0.0f, 0.0f), // Red satellite
            ShadingMode = BaseMaterial3D.ShadingModeEnum.Unshaded
        };
        var satNode = new MeshInstance3D
        {
            Name = "Satellite",
            Mesh = satMesh,
            MaterialOverride = satMaterial,
            Position = new Vector3(OrbitRadiusKm, 0, 0)
        };
        AddChild(satNode);

        // 4. Reference Orbit
        var orbitMesh = new TorusMesh
        {
            InnerRadius = OrbitRadiusKm - 20.0f,
            OuterRadius = OrbitRadiusKm + 20.0f,
            Rings = 64,
            TubeSegments = 8
        };
        var orbitMaterial = new StandardMaterial3D
        {
            AlbedoColor = new Color(1.0f, 1.0f, 1.0f, 0.3f), // Transparent white
            Transparency = BaseMaterial3D.TransparencyEnum.Alpha,
            ShadingMode = BaseMaterial3D.ShadingModeEnum.Unshaded
        };
        var orbitNode = new MeshInstance3D
        {
            Name = "ReferenceOrbit",
            Mesh = orbitMesh,
            MaterialOverride = orbitMaterial
        };
        AddChild(orbitNode);

        // 5. Star Map Background (Placeholder with a dark gray sky)
        var skyMaterial = new PanoramaSkyMaterial(); // Awaiting actual texture
        var sky = new Sky { SkyMaterial = skyMaterial };
        var environment = new Environment
        {
            BackgroundMode = Environment.BGMode.SolidColor, // Solid dark background for now
            BackgroundColor = new Color(0.02f, 0.02f, 0.05f) // Dark space color
        };
        var worldEnv = new WorldEnvironment
        {
            Name = "SpaceEnvironment",
            Environment = environment
        };
        AddChild(worldEnv);
    }
}
