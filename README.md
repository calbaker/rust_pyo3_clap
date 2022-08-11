This crate, which I’ll use for the complex example in SEES (after providing a much simpler, less intimidating example), demonstrates:
- How to make a core crate which conditionally compiles pyo3 interface stuff
- How to make a pyo3 api crate that exposes the core structs to python
- How to make a crate that uses `clap` to make a CLI app using the core crate
- How to use procedural macros to eliminate boiler plate
- That rust is ~100x faster than python in solving the Fibonacci sequence to 42 places
- How to use a cargo workspace to organize a collection of co-dependent crates
- How to use pyi … but I’m pretty sure I screwed this up and would appreciate another set of eyes on it

At the top level, you can run `cargo test --release`, and this will test the code and generate the CLI app in targe/release.  Assuming you have an appropriate python environment created, you can run `maturin develop --release` within sees-py to build the python package.  You can then run [demo.py](demo.py).  
