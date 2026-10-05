from .stats import mean


def drop_outliers(values: list[float], limit: float) -> list[float]:
    """Keeps values within `limit` of the mean."""
    centre = mean(values)
    return [v for v in values if abs(v - centre) <= limit]


def round_all(values: list[float]) -> list[float]:
    return [round(v, 1) for v in values]
