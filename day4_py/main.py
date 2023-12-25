def get_points(s: str) -> int:
    temp = s.split(":")[1].strip().split("|")

    my_numbers = temp[0].strip().split(" ")
    my_numbers = [i for i in my_numbers if i]

    winning_numbers = temp[1].strip().split(" ")
    winning_numbers = [i for i in winning_numbers if i]

    my_winning_numbers = set(my_numbers).intersection(set(winning_numbers))

    count_winning_numbers = len(my_winning_numbers)
    if count_winning_numbers == 0:
        return 0

    points = 2 ** (count_winning_numbers - 1)

    return points


total_points = 0

with open("input.txt", "r") as file:
    data = file.readlines()
    for i in range(len(data)):
        try:
            points = get_points(data[i])
            total_points += points
        except Exception as e:
            print(f"error row: {data[i]}. With error: {e}")


print(total_points)
