//! This code is modified from
//! https://github.com/drujensen/fib/blob/master/fib.rs

#[cfg(feature = "pyo3")]
mod pyo3_imports;
#[cfg(feature = "pyo3")]
use pyo3_imports::*;

use proc_macros::pyo3_api;

/// Stuct for sovling the fibonacci sequence
#[pyo3_api(
    #[new]
    pub fn __new__(n: u64) -> Self {
        Self::new(n)
    }

    #[pyo3(name = "solve")]
    pub fn solve_py(&mut self) {
        self.solve()
    }
)]
#[derive(Clone, Debug)]
pub struct FibSolver {
    /// number of places to solve
    pub places: u64,
    #[api(skip_set)]
    pub solution: Option<u64>,
}

impl Default for FibSolver {
    fn default() -> Self {
        Self {
            places: 42,
            solution: None,
        }
    }
}

impl FibSolver {
    pub fn solve(&mut self) {
        self.solution = Some(fib(self.places));
    }

    pub fn new(places: u64) -> Self {
        Self {
            places,
            solution: None,
        }
    }
}

fn fib(n: u64) -> u64 {
    if n <= 1 {
        n
    } else {
        fib(n - 1) + fib(n - 2)
    }
}

#[cfg(test)]
mod tests {
    use crate::FibSolver;

    #[test]
    fn test_zero() {
        // must be mutable
        let mut fib = FibSolver::new(0);
        fib.solve();
        assert_eq!(fib.solution, Some(0))
    }

    #[test]
    fn test_one() {
        // must be mutable
        let mut fib = FibSolver::new(1);
        fib.solve();
        assert_eq!(fib.solution, Some(1))
    }

    #[test]
    fn test_grtr_than_one() {
        // must be mutable
        let mut fib = FibSolver::new(42);
        fib.solve();
        assert_eq!(fib.solution, Some(267914296))
    }
}
