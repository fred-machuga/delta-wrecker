// **Compliance Note**
// This project is based entirely on publicly available academic information and general knowledge of orbital mechanics. It contains no restricted, proprietary, or export-controlled information of any kind. This is a personal learning project only.

/// Represents the Keplerian orbital elements that define an orbit.
///
/// Keplerian elements provide a compact representation of an orbit’s shape,
/// orientation, and position in space. For Sprint 1 we only require the
/// semi-major axis; the other elements are `Option<f64>` so we can keep the
/// data structure minimal while still being extensible later.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct KeplerianElements {
    /// Semi-major axis of the orbit in kilometers (km).
    /// This is the average distance from the central body to the orbiting object.
    pub semi_major_axis: f64,

    /// Eccentricity of the orbit (dimensionless).
    /// 0.0 = perfectly circular, < 1.0 = elliptical.
    /// Must be < 0.05 for a near-circular orbit in this sprint.
    pub eccentricity: Option<f64>,

    /// Inclination of the orbit relative to the reference plane in radians.
    pub inclination: Option<f64>,

    /// Longitude of the ascending node in radians.
    pub longitude_of_ascending_node: Option<f64>,

    /// Argument of periapsis in radians.
    pub argument_of_periapsis: Option<f64>,

    /// Mean anomaly at a reference time in radians.
    pub mean_anomaly: Option<f64>,
}

impl KeplerianElements {
    /// Creates a new `KeplerianElements` with the required semi-major axis.
    ///
    /// All other orbital elements default to `None`.  
    /// This is intentional for Sprint 1: it lets us build a minimal near-circular orbit
    /// (eccentricity < 0.05) and fill in the rest later when we add full Keplerian support.
    pub fn new(semi_major_axis: f64) -> Self {
        KeplerianElements {
            semi_major_axis,
            eccentricity: None,
            inclination: None,
            longitude_of_ascending_node: None,
            argument_of_periapsis: None,
            mean_anomaly: None,
        }
    }

    /// Validates that the orbit is near-circular (e < 0.05).
    pub fn is_near_circular(&self) -> bool {
        self.eccentricity.unwrap_or(0.0) < 0.05
    }
}

// **Compliance Note**
// This project is based entirely on publicly available academic information and general knowledge of orbital mechanics. It contains no restricted, proprietary, or export-controlled information of any kind. This is a personal learning project only.