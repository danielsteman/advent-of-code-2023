from dataclasses import dataclass


@dataclass
class Result:
    points: int
    count_winning_numbers: int


def get_points(s: str) -> Result:
    temp = s.split(":")[1].strip().split("|")

    my_numbers = temp[0].strip().split(" ")
    my_numbers = [i for i in my_numbers if i]

    winning_numbers = temp[1].strip().split(" ")
    winning_numbers = [i for i in winning_numbers if i]

    my_winning_numbers = set(my_numbers).intersection(set(winning_numbers))

    count_winning_numbers = len(my_winning_numbers)
    if count_winning_numbers == 0:
        return Result(0, 0)

    points = 2 ** (count_winning_numbers - 1)

    return Result(points, count_winning_numbers)


total_points = 0

with open("input.txt", "r") as file:
    data = file.readlines()
    for i in range(len(data)):
        try:
            result = get_points(data[i])
            total_points += result.points
        except Exception as e:
            print(f"error row: {data[i]}. With error: {e}")


print(total_points)
