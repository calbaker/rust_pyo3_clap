use pyo3::prelude::*;

use fib_core::FibSolver;

#[pymodule]
fn fib_py(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_class::<FibSolver>()?;
    Ok(())
}
