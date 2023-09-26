import example
from example import imports
from example import types

class Example(example.Example):
    def run(self) -> types.Result:
        time = example.imports.runtime.get_current_time()
        example.imports.runtime.print(f"time: {time.seconds}.{time.milliseconds}")
        f = fibonacci(32)
        time = example.imports.runtime.get_current_time()
        example.imports.runtime.print(f"fibonacci: {f}, time: {time.seconds}.{time.milliseconds}")

def fibonacci(n: int) -> int:
    match n:
        case 0 | 1:
            return 1
        case other:
            return fibonacci(n - 1) + fibonacci(n - 2)
