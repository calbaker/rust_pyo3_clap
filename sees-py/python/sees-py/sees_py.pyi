from typing import Optional

from .sees_py import *

class FibSolver:
    n: int
    sol: Optional[int]

    def solve(self) -> int: ...