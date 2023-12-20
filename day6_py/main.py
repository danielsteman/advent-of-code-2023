# Time:        52     94     75     94
# Distance:   426   1374   1279   1216

from collections import namedtuple


class Race:
    def __init__(self, time: int, distance: int) -> None:
        self.time = time
        self.distance = distance
        self.optimum_dist = self.get_optimum(time)
        self.optimum_hold = self.time / 2

    @staticmethod
    def get_optimum(t: int) -> float:
        return (t - t / 2) * t / 2

    @staticmethod
    def get_dist(t: int, t_hold: int) -> int:
        return (t - t_hold) * t_hold

    def get_n_strategies(self):
        if self.optimum_dist <= self.distance:
            return 0

        n_strategies = 1
        position = 0
        t = self.optimum_hold

        left = self.get_dist(self.time, t - 1)
        if left > self.distance:
            n_strategies += 1

        right = self.get_dist(self.time, t + 1)
        if right > self.distance:
            n_strategies += 1

        return n_strategies


races = [
    Race(time=52, distance=426),  # optimum: hold 26 seconds and reach 676
    Race(time=94, distance=1374),
    Race(time=75, distance=1279),
    Race(time=94, distance=1216),
]


for race in races:
    print(f"optimum dist: {race.optimum_dist}")
    print(f"n_strategies: {race.get_n_strategies()}")
