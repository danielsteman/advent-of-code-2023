s = "Game 1: 3 blue, 4 red; 19 red, 2 green, 6 blue; 2 green"

game = s.split(":")[0].split(" ")[-1]

sets = []
for str_set in s.split(": ")[1].split("; "):
    set = {}
    for cubes in str_set.split(", "):
        unpacked_cubes = cubes.split(" ")
        set[unpacked_cubes[1]] = int(unpacked_cubes[0])
    sets.append(set)

print(sets)

rules = {
    "red": 12,
    "green": 13,
    "blue": 14,
}

counter = {
    "red": 0,
    "green": 0,
    "blue": 0,
}

for set in sets:
    print(set)
    for color, value in set.items():
        counter[color] += value
        if counter[color] > rules[color]:
            print("impossible")
            break

print(counter)


class Game:
    def __init__(self, s):
        self.id = self.get_id(s)
        self.sets = self.get_sets(s)
        self.rules = {
            "red": 12,
            "green": 13,
            "blue": 14,
        }

    @staticmethod
    def get_id(self, s):
        return s.split(":")[0].split(" ")[-1]

    @staticmethod
    def get_sets(self, s):
        sets = []
        for str_set in s.split(": ")[1].split("; "):
            set = {}
            for cubes in str_set.split(", "):
                unpacked_cubes = cubes.split(" ")
                set[unpacked_cubes[1]] = int(unpacked_cubes[0])
            sets.append(set)
        return sets

    def is_possible(self):
        return
