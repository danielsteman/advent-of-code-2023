from collections import namedtuple
from typing import List

Cube = namedtuple("Cube", ["quantity", "color"])


class Game:
    def __init__(self, s):
        s = s.strip("\n")
        self.id = self.get_id(s)
        self.rules = {
            "red": 12,
            "green": 13,
            "blue": 14,
        }
        self.points = self.parse_and_count(s)

    @staticmethod
    def get_id(s: str) -> List[str]:
        return s.split(":")[0].split(" ")[-1]

    def parse_and_count(self, s: str) -> int:
        # count = {key: 0 for key in self.rules}
        for str_set in s.split(": ")[1].split("; "):
            for cubes in str_set.split(", "):
                unpacked_cubes = Cube(*cubes.split(" "))
                # count[unpacked_cubes.color] += int(unpacked_cubes.quantity)
                if int(unpacked_cubes.quantity) > self.rules[unpacked_cubes.color]:
                    return 0

        return int(self.id)


total_points = 0

with open("input.txt", "r") as f:
    for line in f.readlines():
        game = Game(line)
        total_points += game.points

print(total_points)
