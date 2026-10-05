from .stats import mean, median


def summarize(values: list[float]) -> str:
    """One line with the mean and median of `values`."""
    return f"mean={mean(values):.2f} median={median(values):.2f}"
