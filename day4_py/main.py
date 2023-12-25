with open("input.txt") as file:
    data = file.read()
    numbers = data.split("|")

example = "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53"
temp = example.split(":")
card_id = temp[0]
my_numbers = temp[1].strip().split("|")[0].strip().split(" ")
winning_numbers = temp[1].strip().split("|")[1].strip().split(" ")
print(my_numbers)
print(winning_numbers)
