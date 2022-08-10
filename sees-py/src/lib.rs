use pyo3::prelude::*;

use sees_core::FibSolver;

#[pyclass] // provided by pyo3::prelude
pub struct PyFibSolver(FibSolver);

#[pymethods(FibSolver)]
impl PyFibSolver {
    #[new]
    pub fn __new__(n: u64) -> Self {
        Self(FibSolver::new(n))
    }

    pub fn solve(&mut self) {
        self.0.solve()
    }

    #[getter]
    pub fn get_n(&self) -> u64 {
        self.0.n
    }

    #[setter]
    pub fn set_n(&mut self, n: u64) {
        self.0.n = n;
    }

    #[getter]
    pub fn get_sol(&self) -> Option<u64> {
        self.0.sol
    }
}

#[pymodule]
fn sees_py(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_class::<PyFibSolver>()?;
    Ok(())
}
