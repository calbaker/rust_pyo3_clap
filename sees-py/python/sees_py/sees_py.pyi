"""
This file tells the python language server what objects (e.g. functions, classes)
are available to it from Rust.  
"""

from typing import Optional

from .sees_py import *


class FibSolver:
    places: int
    solution: Optional[int]

    def solve(self) -> int: ...
