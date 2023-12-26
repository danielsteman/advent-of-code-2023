from collections import defaultdict
from dataclasses import dataclass


@dataclass
class Result:
    points: int
    count_winning_numbers: int
    card_number: int

    def __repr__(self):
        return f"card_number: {self.card_number}, winning_numbers: {self.count_winning_numbers}"


def get_points(s: str) -> Result:
    temp = s.split(":")[1].strip().split("|")

    card_number = int(s.split(":")[0].split(" ")[-1])

    my_numbers = temp[0].strip().split(" ")
    my_numbers = [i for i in my_numbers if i]

    winning_numbers = temp[1].strip().split(" ")
    winning_numbers = [i for i in winning_numbers if i]

    my_winning_numbers = set(my_numbers).intersection(set(winning_numbers))

    count_winning_numbers = len(my_winning_numbers)
    if count_winning_numbers == 0:
        return Result(0, 0, card_number)

    points = 2 ** (count_winning_numbers - 1)

    return Result(points, count_winning_numbers, card_number)


total_points = 0

card_counter = defaultdict(int)
for i in range(1, 200):
    card_counter[i] = 1

with open("input.txt", "r") as file:
    data = file.readlines()
    for i in range(len(data)):
        try:
            result = get_points(data[i])
            print(result)

            total_points += result.points
            if result.count_winning_numbers != 0:
                for i in range(
                    result.card_number + 1,
                    result.card_number + 1 + result.count_winning_numbers + 1,
                ):
                    print(i)
                    card_counter[i] += 1
        except Exception as e:
            print(f"error row: {data[i]}. With error: {e}")

print(card_counter)
print(total_points)
