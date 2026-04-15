// Compliance Note
// This project is based entirely on publicly available academic information and general knowledge of orbital mechanics. It contains no restricted, proprietary, or export-controlled information of any kind. This is a personal learning project only.

namespace DeltaWrecker.Orbital;

/// <summary>
/// Physical constants used in orbital mechanics calculations.
/// Mirrors engine/src/orbital/constants.rs.
/// </summary>
public static class OrbitalConstants
{
    /// <summary>Standard gravitational parameter for Earth (km³/s²).</summary>
    public const double MuEarth = 398600.4418;

    /// <summary>Mean radius of the Earth in kilometers (km).</summary>
    public const double EarthRadiusKm = 6371.0;
}
