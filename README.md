# Description
This crate, which I’ll use for the complex example in SEES (after providing a much simpler, less intimidating example), demonstrates:
- How to make a core crate which conditionally compiles pyo3 interface stuff
- How to make a pyo3 api crate that exposes the core structs to python
- How to make a crate that uses `clap` to make a CLI app using the core crate
- How to use procedural macros to eliminate boiler plate
- That rust is ~100x faster than python in solving the Fibonacci sequence to 42 places
- How to use a cargo workspace to organize a collection of co-dependent crates
- How to use pyi … but I’m pretty sure I screwed this up and would appreciate another set of eyes on it

# How to make it run
## Compiling and such
1. [Install rust](https://www.rust-lang.org/tools/install)
1. At the top level, run `cargo test --release`, and this will test the code and generate the CLI app in targe/release.  
1. Create and activate a suitable python environment, e.g.
    1. `python3.10 -m venv .venv`
    1. Activate:
        - linux/unix: `source /.venv/bin/activate`
        - windows: `.\.venv\Scripts\activate.ps1` or `.\.venv\Scripts\activate.bat`, depending on your terminal environment
1. run `maturin develop --release` within sees-py to build the python package.  

## Stuff you can run
- [demo.py](demo.py) -- show that rust+python is 100x faster than pure python
- `target/release/sees-cli 42` -- demonstrate how to run directly as a CLI app
