use pyo3::prelude::*;
use crate::orbital::OrbitState;

#[pyclass(module = "delta_wrecker.orbital")]
pub struct PyOrbitState {
    state: OrbitState,
}

#[pymethods]
impl PyOrbitState {
    #[new]
    pub fn new(x: f64, y: f64, z: f64, vx: f64, vy: f64, vz: f64) -> Self {
        Self {
            state: OrbitState::new(x, y, z, vx, vy, vz),
        }
    }

    #[getter]
    pub fn x(&self) -> f64 {
        self.state.x
    }

    #[getter]
    pub fn y(&self) -> f64 {
        self.state.y
    }

    #[getter]
    pub fn z(&self) -> f64 {
        self.state.z
    }

    #[getter]
    pub fn vx(&self) -> f64 {
        self.state.vx
    }

    #[getter]
    pub fn vy(&self) -> f64 {
        self.state.vy
    }

    #[getter]
    pub fn vz(&self) -> f64 {
        self.state.vz
    }

    #[getter]
    pub fn pos(&self) -> (f64, f64, f64) {
        (self.state.x, self.state.y, self.state.z)
    }

    #[getter]
    pub fn vel(&self) -> (f64, f64, f64) {
        (self.state.vx, self.state.vy, self.state.vz)
    }

    pub fn altitude_km(&self) -> f64 {
        self.state.altitude_km()
    }

    pub fn speed_kms(&self) -> f64 {
        self.state.speed_kms()
    }

    pub fn distance_km(&self) -> f64 {
        self.state.distance_km()
    }
    
    pub fn propagate(&self, dt_s: f64) -> PyOrbitState {
        PyOrbitState {
            state: self.state.propagate(dt_s)
        }
    }
}