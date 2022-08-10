//! This code is modified from
//! https://github.com/drujensen/fib/blob/master/fib.rs

pub mod sees_pyo3;
// #[cfg(feature = "pyo3")]
use sees_pyo3::*;

/// Stuct for sovling the fibonacci sequence
// #[cfg_attr(feature = "pyo3", pyo3_api)]
#[pyo3_api]
#[derive(Clone, Debug)]
pub struct FibSolver {
    /// number of positions to solve
    pub n: u64,
    pub sol: Option<u64>,
}

impl Default for FibSolver {
    fn default() -> Self {
        Self { n: 42, sol: None }
    }
}

impl FibSolver {
    pub fn solve(&mut self) {
        self.sol = Some(fib(self.n));
    }

    pub fn new(n: u64) -> Self {
        Self { n, sol: None }
    }
}

fn fib(n: u64) -> u64 {
    if n <= 1 {
        return n;
    }
    fib(n - 1) + fib(n - 2)
}

#[cfg(test)]
mod tests {
    use crate::FibSolver;

    #[test]
    fn test_zero() {
        // must be mutable
        let mut fib = FibSolver::new(0);
        fib.solve();
        assert_eq!(fib.sol, Some(0))
    }

    #[test]
    fn test_one() {
        // must be mutable
        let mut fib = FibSolver::new(1);
        fib.solve();
        assert_eq!(fib.sol, Some(1))
    }

    #[test]
    fn test_grtr_than_one() {
        // must be mutable
        let mut fib = FibSolver::new(42);
        fib.solve();
        assert_eq!(fib.sol, Some(267914296))
    }
}
