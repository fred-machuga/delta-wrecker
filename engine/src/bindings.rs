// Compliance Note
// This project is based entirely on publicly available academic information and general knowledge of orbital mechanics. It contains no restricted, proprietary, or export-controlled information of any kind. This is a personal learning project only.

use pyo3::prelude::*;
use crate::orbital::OrbitState;

#[pyclass(get_all)]
#[derive(Clone, Debug)]
pub struct PyOrbitalElements {
    pub semi_major_axis: f64,
    pub eccentricity: Option<f64>,
    pub inclination: Option<f64>,
    pub longitude_of_ascending_node: Option<f64>,
    pub argument_of_periapsis: Option<f64>,
    pub mean_anomaly: Option<f64>,
}

#[pyclass(module = "delta_wrecker.orbital")]
pub struct PyOrbitState {
    state: OrbitState,
    #[pyo3(get)]
    pub time: f64,
}

#[pymethods]
impl PyOrbitState {
    #[new]
    pub fn new(x: f64, y: f64, z: f64, vx: f64, vy: f64, vz: f64, time: f64) -> Self {
        Self {
            state: OrbitState::new(x, y, z, vx, vy, vz),
            time,
        }
    }

    pub fn propagate(&self, dt_s: f64) -> PyOrbitState {
        PyOrbitState {
            state: crate::orbital::propagate(&self.state, dt_s),
            time: self.time + dt_s,
        }
    }

    #[getter]
    pub fn pos(&self) -> (f64, f64, f64) {
        (self.state.x, self.state.y, self.state.z)
    }

    #[getter]
    pub fn vel(&self) -> (f64, f64, f64) {
        (self.state.vx, self.state.vy, self.state.vz)
    }

    #[getter]
    pub fn orbital_elements(&self) -> PyOrbitalElements {
        let r = self.state.distance_km();
        let v_mag = self.state.speed_kms();
        let v2 = v_mag * v_mag;
        let gm = crate::orbital::MU_EARTH;
        let a = 1.0 / (2.0 / r - v2 / gm);

        PyOrbitalElements {
            semi_major_axis: a,
            eccentricity: None,
            inclination: None,
            longitude_of_ascending_node: None,
            argument_of_periapsis: None,
            mean_anomaly: None,
        }
    }
}

/// A simple function that adds two numbers, exposed to Python.
#[pyfunction]
fn add_numbers(a: i32, b: i32) -> PyResult<i32> {
    Ok(a + b)
}

/// A function that returns a greeting from Rust.
#[pyfunction]
fn hello_from_rust() -> PyResult<String> {
    Ok("Hello from Rust!".to_string())
}

/// The Python module definition.
#[pymodule]
fn delta_wrecker(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(add_numbers, m)?)?;
    m.add_function(wrap_pyfunction!(hello_from_rust, m)?)?;
    m.add_class::<PyOrbitState>()?;
    Ok(())
}

// Compliance Note
// This project is based entirely on publicly available academic information and general knowledge of orbital mechanics. It contains no restricted, proprietary, or export-controlled information of any kind. This is a personal learning project only.
