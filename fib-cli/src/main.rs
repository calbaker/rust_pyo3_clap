use clap::Parser;

use fib_core::FibSolver;

/// Wrapper for FibSolver.
#[derive(Parser)]
#[clap(author, version, about, long_about = None)]
struct FibSolverAPI {
    // `conflicts_with` tells rust one or the other, not both, can be provided
    /// Cycle as json string
    #[clap(value_parser)]
    places: u64,
}

fn main() {
    let fib_api = FibSolverAPI::parse();
    let mut fib = FibSolver::new(fib_api.places);
    fib.solve();
    println!("{}", fib.solution.unwrap());
}
