// Compliance Note
// This project is based entirely on publicly available academic information and general knowledge of orbital mechanics. It contains no restricted, proprietary, or export-controlled information of any kind. This is a personal learning project only.

namespace DeltaWrecker.Orbital;

/// <summary>
/// Represents the Keplerian orbital elements that define an orbit.
/// Mirrors engine/src/orbital/keplerian_elements.rs.
///
/// For Phase 1 only the semi-major axis is required; all other elements are
/// nullable so the data structure stays minimal while remaining extensible.
/// </summary>
public sealed class KeplerianElements
{
    /// <summary>
    /// Semi-major axis of the orbit in kilometers (km).
    /// Average distance from the central body to the orbiting object.
    /// </summary>
    public double SemiMajorAxis { get; }

    /// <summary>
    /// Eccentricity of the orbit (dimensionless).
    /// 0.0 = perfectly circular, &lt; 1.0 = elliptical.
    /// Must be &lt; 0.05 for a near-circular orbit (Phase 1).
    /// </summary>
    public double? Eccentricity { get; init; }

    /// <summary>Inclination relative to the reference plane in radians.</summary>
    public double? Inclination { get; init; }

    /// <summary>Longitude of the ascending node in radians.</summary>
    public double? LongitudeOfAscendingNode { get; init; }

    /// <summary>Argument of periapsis in radians.</summary>
    public double? ArgumentOfPeriapsis { get; init; }

    /// <summary>Mean anomaly at a reference time in radians.</summary>
    public double? MeanAnomaly { get; init; }

    /// <param name="semiMajorAxis">Semi-major axis in km. All other elements default to null.</param>
    public KeplerianElements(double semiMajorAxis)
    {
        SemiMajorAxis = semiMajorAxis;
    }

    /// <summary>Returns true if the orbit is near-circular (eccentricity &lt; 0.05).</summary>
    public bool IsNearCircular() => (Eccentricity ?? 0.0) < 0.05;
}
