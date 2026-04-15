// Compliance Note
// This project is based entirely on publicly available academic information and general knowledge of orbital mechanics. It contains no restricted, proprietary, or export-controlled information of any kind. This is a personal learning project only.

using NUnit.Framework;
using DeltaWrecker.Orbital;

namespace DeltaWrecker.Tests.Orbital;

/// <summary>
/// Unit tests for <see cref="Propagator"/>.
/// Mirrors game/tests/test_propagator_pyo3.py and engine Rust propagator tests.
/// </summary>
[TestFixture]
public class PropagatorTests
{
    [Test]
    public void Propagate_MovesSpacecraft()
    {
        // TODO-Coder: After one step the position should have changed.
        //   var state = new OrbitState(7000.0, 0.0, 0.0, 0.0, 7.5, 0.0);
        //   var next  = state.Propagate(10.0);
        //   Assert.That(next.X, Is.Not.EqualTo(7000.0));  // it moved!
        Assert.Ignore("Not yet implemented");
    }

    [Test]
    public void Propagate_PreservesDistance()
    {
        // TODO-Coder: For a circular orbit the distance from Earth's center must
        // remain constant after propagation (within floating-point tolerance 1e-6).
        //   var state = new OrbitState(7000.0, 0.0, 0.0, 0.0, 7.5, 0.0);
        //   var next  = state.Propagate(60.0);
        //   Assert.That(next.DistanceKm(), Is.EqualTo(state.DistanceKm()).Within(1e-6));
        Assert.Ignore("Not yet implemented");
    }

    [Test]
    public void Propagate_PreservesSpeed()
    {
        // TODO-Coder: For a circular orbit the speed must remain constant
        // after propagation (within floating-point tolerance 1e-6).
        //   var state = new OrbitState(7000.0, 0.0, 0.0, 0.0, 7.5, 0.0);
        //   var next  = state.Propagate(60.0);
        //   Assert.That(next.SpeedKms(), Is.EqualTo(state.SpeedKms()).Within(1e-6));
        Assert.Ignore("Not yet implemented");
    }

    [Test]
    public void Propagate_MatchesAnalyticalCircularOrbit()
    {
        // TODO-Coder: Propagate by a known angle and verify the result matches
        // the analytical formula within 1e-6 (same tolerance as the Rust test).
        // Example: propagate so θ = π/2 and check x ≈ 0, y ≈ r.
        //   double r     = 7000.0;
        //   double v     = 7.5;
        //   double omega = v / r;
        //   double dt    = (Math.PI / 2.0) / omega;
        //   var state    = new OrbitState(r, 0.0, 0.0, 0.0, v, 0.0);
        //   var next     = state.Propagate(dt);
        //   Assert.That(next.X, Is.EqualTo(0.0).Within(1e-6));
        //   Assert.That(next.Y, Is.EqualTo(r).Within(1e-6));
        Assert.Ignore("Not yet implemented");
    }

    [Test]
    public void Propagate_IsFrameRateIndependent()
    {
        // TODO-Coder: One large step should give the same result as many small steps
        // (within floating-point tolerance 1e-9).
        //   var state = new OrbitState(7000.0, 0.0, 0.0, 0.0, 7.5, 0.0);
        //   var oneBig  = state.Propagate(60.0);
        //   var manySmall = state;
        //   for (int i = 0; i < 60; i++) manySmall = manySmall.Propagate(1.0);
        //   Assert.That(oneBig.X, Is.EqualTo(manySmall.X).Within(1e-9));
        //   Assert.That(oneBig.Y, Is.EqualTo(manySmall.Y).Within(1e-9));
        Assert.Ignore("Not yet implemented");
    }
}
