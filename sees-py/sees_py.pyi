from typing import Optional

from .sees_py import *


class FibSolver:
    places: int
    solution: Optional[int]

    def solve(self) -> int: ...
