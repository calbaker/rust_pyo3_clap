---
marp: true
paginate: true
---

# Writing Rust code as a blazing fast backend for python
Chad Baker

---

## The problems: 
python is slow! ([video explaining how and why](https://www.youtube.com/watch?v=vVUnCXKuNOg&t=56s))  
TLDW: interpreted languages have consistently been ok becuase of Moore's law, but Moore's law is no longer holding true.  
Python's slowness adds cost, [energy usage](https://www.researchgate.net/publication/320436353_Energy_efficiency_across_programming_languages_how_do_energy_time_and_memory_relate), and slowness to all projects!
[Maybe Python 3.14 will be faster than C++!?!?!!?](https://towardsdatascience.com/python-3-14-will-be-faster-than-c-a97edd01d65d)  (hint: only if you believe in reverse causality!)

## Myth (that has historically been _mostly_ justified)
Compiled languages are about as much fun as eating glass!

---

# What is Rust? 
Rust is a statically typed, compiled language with a mix of functional and object-oriented programming paradigms and a strong emphasis on performance, memory safety, and having readable syntax.  

# My background with Rust
To avoid having to learn C++, I started learning it in January 2022, and I've found it to be so incredibly useful that I am compelled to tell people about it!  At first, the idea of static typing seemed annoying to me, but now that I've gotten used to it, it's actually a huge time saver.  I no longer have to waste any energy figuring out the type of a variable during run time with a debugger like I would in Python.  

---

# Why Rust?
- Rust is fast, almost as fast as C and Fortran and sometimes faster (and ~100x faster than Python!).  Rust is a "bare metal" language.  
- Rust has a well established and growing package ecosystem built on Rust's Cargo package manager.
- Rust has a de facto build system and compiler so you don't have to mess with any of that ... ever!
- Rust has great learning resources and documentation:
    - [THE book](https://doc.rust-lang.org/stable/book/)
    - [rustlings](https://github.com/rust-lang/rustlings) -- interactive excercises that are about as much fun as NES Zelda
- Rust has powerful built-in tools to help you write idiomatic code

---

# Ok, if Rust is so great, should I stop using Python?
Heck no!  Python is a great "glue" language with a lot of rich features.  Also, if you write code that makes heavy use of numpy stuff, the Rust performance advantage is greatly reduced.  If you are developing a large package that does complicated things, needs to run fast, and will be used for several years, that's when you need to start mulling over the idea of doing it in Rust.  Lastly, since python allows [type hinting](https://docs.python.org/3/library/typing.html), you may find yourself writing better Python code after learning Rust.   

---

# How to build an easy example project
Follow this guide: 
https://pyo3.rs/v0.17.2/#using-rust-from-python  

Live demo: modify the pyo3 example to calculate the fibonacci series to `x` places.

For the final slide, I'll leave you with a fairly fleshed out polyglot (mixed language) Rust/Python project that demonstrates several useful Rust concepts:  
- separating crates (like Python packages) into coherent units with 
    - a core crate for core functionality
    - a python wrapper crate to expose the core crate to python
    - a Command-Line Interface (CLI) wrapper crate to expose the core crate as a CLI app
- `proc-macros` crate to optionally autogenerate API code
- how to create a CLI app 

---

# How to build and run the hard example project
1. Create and activate a python environment with `venv` or `conda` -- e.g. `python -m venv sees-venv`.
1. Run `pip install maturin` -- this is the package that builds Rust code as a python module
1. Build and test:
    - run `cargo test --release` to build CLI app and run tests
    - run `(cd fib-py/ && maturin develop --release)` to build Python module
1. Run:
    - `python demo.py` will run both the pure Python and Rust versions of the Fibonacci solver and compare performance.
    - in `target/release/`, run `./fib-cli.exe 42`





