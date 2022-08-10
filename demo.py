# Before running, see README.md

import time

import sees_py

rust_fib = sees_py.FibSolver(42)
t0 = time.time()
rust_fib.solve()
t1 = time.time()

print(f"Rust solution: {rust_fib.solution}")
print(f"Elapsed time to solve in rust: {t1-t0:.3g} s")


def fib(n: int) -> int:
    if n <= 1:
        return n
    return fib(n - 1) + fib(n - 2)


t0 = time.time()
py_fib = fib(42)
t1 = time.time()

print(f"Python solution: {py_fib}")
print(f"Elapsed time to solve in python: {t1-t0:.3g} s")

# rust should be about 100x faster than python for this!
