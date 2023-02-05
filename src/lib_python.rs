// TODO: remove before 1.0.0
#![allow(dead_code)]
#![allow(unused_macros)]

use pyo3::prelude::*;

// mod core;
mod data;
mod error;
// mod error;
// mod python;

#[pymodule]
fn parside(_py: Python, _m: &PyModule) -> PyResult<()> {
    // m.add_class::<Matter>()?;
    Ok(())
}
