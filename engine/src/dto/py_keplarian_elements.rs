// Compliance Note
// This project is based entirely on publicly available academic information and general knowledge of orbital mechanics. It contains no restricted, proprietary, or export-controlled information of any kind. This is a personal learning project only.

use pyo3::prelude::*;
use crate::orbital::KeplerianElements;

#[pyclass(module = "delta_wrecker.orbital")]
pub struct PyKeplerianElements {
    elements: KeplerianElements,
}

#[pymethods]
impl PyKeplerianElements {
    #[new]
    pub fn new(semi_major_axis: f64) -> Self {
        Self {
            elements: KeplerianElements::new(semi_major_axis),
        }
    }

    #[getter]
    pub fn semi_major_axis(&self) -> f64 {
        self.elements.semi_major_axis
    }
}

// Compliance Note
// This project is based entirely on publicly available academic information and general knowledge of orbital mechanics. It contains no restricted, proprietary, or export-controlled information of any kind. This is a personal learning project only.
