import statistics


def mean(values: list[float]) -> float:
    return statistics.fmean(values)


def median(values: list[float]) -> float:
    ordered = sorted(values)
    return statistics.median(ordered)
