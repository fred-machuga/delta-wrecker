// Compliance Note
// This project is based entirely on publicly available academic information and general knowledge of orbital mechanics. It contains no restricted, proprietary, or export-controlled information of any kind. This is a personal learning project only.

namespace DeltaWrecker.Orbital;

/// <summary>
/// Represents the instantaneous Cartesian state of an orbiting object.
/// Mirrors engine/src/orbital/orbit_state.rs.
///
/// Stores 3D position (km) and velocity (km/s). Used by the propagator to
/// advance the orbit forward in time.
/// </summary>
public readonly struct OrbitState
{
    /// <summary>X position component in kilometers (km).</summary>
    public double X { get; }

    /// <summary>Y position component in kilometers (km).</summary>
    public double Y { get; }

    /// <summary>Z position component in kilometers (km).</summary>
    public double Z { get; }

    /// <summary>X velocity component in kilometers per second (km/s).</summary>
    public double Vx { get; }

    /// <summary>Y velocity component in kilometers per second (km/s).</summary>
    public double Vy { get; }

    /// <summary>Z velocity component in kilometers per second (km/s).</summary>
    public double Vz { get; }

    public OrbitState(double x, double y, double z, double vx, double vy, double vz)
    {
        X = x; Y = y; Z = z;
        Vx = vx; Vy = vy; Vz = vz;
    }

    /// <summary>Returns the position as a Vec3.</summary>
    public Vec3 Position() => new Vec3(X, Y, Z);

    /// <summary>Returns the velocity as a Vec3.</summary>
    public Vec3 Velocity() => new Vec3(Vx, Vy, Vz);

    /// <summary>Returns the distance from the center of the Earth in kilometers (km).</summary>
    public double DistanceKm() => Position().Magnitude();

    /// <summary>Returns the speed (magnitude of velocity vector) in km/s.</summary>
    public double SpeedKms() => Velocity().Magnitude();

    /// <summary>Returns the altitude above Earth's mean radius in kilometers (km).</summary>
    public double AltitudeKm() => DistanceKm() - OrbitalConstants.EarthRadiusKm;

    /// <summary>
    /// Propagates this orbit state forward by <paramref name="dtSeconds"/> seconds.
    /// </summary>
    public OrbitState Propagate(double dtSeconds) => Propagator.Propagate(this, dtSeconds);
}
