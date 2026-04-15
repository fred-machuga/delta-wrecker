// Compliance Note
// This project is based entirely on publicly available academic information and general knowledge of orbital mechanics. It contains no restricted, proprietary, or export-controlled information of any kind. This is a personal learning project only.

namespace DeltaWrecker.Orbital;

/// <summary>
/// A simple 3D vector for Cartesian positions and velocities.
/// Mirrors engine/src/orbital/vec3.rs.
/// All values are in kilometers (position) or km/s (velocity).
/// </summary>
public readonly struct Vec3
{
    /// <summary>X component.</summary>
    public double X { get; }

    /// <summary>Y component.</summary>
    public double Y { get; }

    /// <summary>Z component.</summary>
    public double Z { get; }

    public Vec3(double x, double y, double z)
    {
        X = x;
        Y = y;
        Z = z;
    }

    /// <summary>
    /// Returns the magnitude (length) of the vector.
    /// For position vectors this is in km; for velocity vectors in km/s.
    /// </summary>
    public double Magnitude() => Math.Sqrt(X * X + Y * Y + Z * Z);
}
