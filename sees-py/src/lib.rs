use pyo3::prelude::*;

use sees_core::FibSolver;

#[pymodule]
fn sees_py(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_class::<FibSolver>()?;
    Ok(())
}
