import functools
import time

def timeit(limit: float = 5):
    def decorator(func):
        @functools.wraps(func)
        def wrapper(*args, **kwargs):
            start = time.perf_counter()
            end = 0.0
            repetitions = 0
            while end - start < limit:
                repetitions += 1
                result = func(*args, **kwargs)
                end = time.perf_counter()
            avg_time = (end - start) / repetitions
            avg_time_ms = avg_time * 1000
            print(f"\t{func.__name__} took {avg_time_ms:.6f} ms")
            return result
        return wrapper
    return decorator
