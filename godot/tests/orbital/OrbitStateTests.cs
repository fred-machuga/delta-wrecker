// Compliance Note
// This project is based entirely on publicly available academic information and general knowledge of orbital mechanics. It contains no restricted, proprietary, or export-controlled information of any kind. This is a personal learning project only.

using NUnit.Framework;
using DeltaWrecker.Orbital;

namespace DeltaWrecker.Tests.Orbital;

/// <summary>
/// Unit tests for <see cref="OrbitState"/>.
/// Mirrors game/tests/test_propagator_pyo3.py.
/// </summary>
[TestFixture]
public class OrbitStateTests
{
    [Test]
    public void Constructor_SetsAllComponents()
    {
        // TODO-Coder: Verify that the OrbitState constructor correctly assigns
        // x, y, z, vx, vy, vz.
        // Example:
        //   var state = new OrbitState(7000.0, 0.0, 0.0, 0.0, 7.5, 0.0);
        //   Assert.That(state.X,  Is.EqualTo(7000.0));
        //   Assert.That(state.Y,  Is.EqualTo(0.0));
        //   Assert.That(state.Z,  Is.EqualTo(0.0));
        //   Assert.That(state.Vx, Is.EqualTo(0.0));
        //   Assert.That(state.Vy, Is.EqualTo(7.5));
        //   Assert.That(state.Vz, Is.EqualTo(0.0));
        Assert.Ignore("Not yet implemented");
    }

    [Test]
    public void Position_ReturnsCorrectVec3()
    {
        // TODO-Coder: Verify Position() returns Vec3(X, Y, Z).
        //   var state = new OrbitState(7000.0, 1.0, 2.0, 0.0, 7.5, 0.0);
        //   var pos = state.Position();
        //   Assert.That(pos.X, Is.EqualTo(7000.0));
        //   Assert.That(pos.Y, Is.EqualTo(1.0));
        //   Assert.That(pos.Z, Is.EqualTo(2.0));
        Assert.Ignore("Not yet implemented");
    }

    [Test]
    public void Velocity_ReturnsCorrectVec3()
    {
        // TODO-Coder: Verify Velocity() returns Vec3(Vx, Vy, Vz).
        Assert.Ignore("Not yet implemented");
    }

    [Test]
    public void DistanceKm_ReturnsPositionMagnitude()
    {
        // TODO-Coder: For a state at (7000, 0, 0) distance should be 7000 km.
        //   var state = new OrbitState(7000.0, 0.0, 0.0, 0.0, 7.5, 0.0);
        //   Assert.That(state.DistanceKm(), Is.EqualTo(7000.0).Within(1e-9));
        Assert.Ignore("Not yet implemented");
    }

    [Test]
    public void SpeedKms_ReturnsVelocityMagnitude()
    {
        // TODO-Coder: For a state with velocity (0, 7.5, 0) speed should be 7.5 km/s.
        //   var state = new OrbitState(7000.0, 0.0, 0.0, 0.0, 7.5, 0.0);
        //   Assert.That(state.SpeedKms(), Is.EqualTo(7.5).Within(1e-9));
        Assert.Ignore("Not yet implemented");
    }

    [Test]
    public void AltitudeKm_ReturnsDistanceMinusEarthRadius()
    {
        // TODO-Coder: AltitudeKm should equal DistanceKm - EarthRadiusKm.
        //   var state = new OrbitState(7000.0, 0.0, 0.0, 0.0, 7.5, 0.0);
        //   double expected = 7000.0 - OrbitalConstants.EarthRadiusKm;
        //   Assert.That(state.AltitudeKm(), Is.EqualTo(expected).Within(1e-9));
        Assert.Ignore("Not yet implemented");
    }
}
