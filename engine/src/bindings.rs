// Compliance Note
// This project is based entirely on publicly available academic information and general knowledge of orbital mechanics. It contains no restricted, proprietary, or export-controlled information of any kind. This is a personal learning project only.

use pyo3::prelude::*;

/// A simple function that adds two numbers, exposed to Python.
#[pyfunction]
fn add_numbers(a: i32, b: i32) -> PyResult<i32> {
    Ok(a + b)
}

/// A function that returns a greeting from Rust.
#[pyfunction]
fn hello_from_rust() -> PyResult<String> {

// TODO-Coder: Expose propagate_circular to Python and write Rust-Python interop test

    Ok("Hello from Rust!".to_string())
}

/// The Python module definition.
#[pymodule]
fn delta_wrecker(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(add_numbers, m)?)?;
    m.add_function(wrap_pyfunction!(hello_from_rust, m)?)?;
    Ok(())
}

// Compliance Note
// This project is based entirely on publicly available academic information and general knowledge of orbital mechanics. It contains no restricted, proprietary, or export-controlled information of any kind. This is a personal learning project only.
