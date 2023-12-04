if __name__ == "__main__":
    sum = 0
    with open("input.txt", "r") as f:
        for line in f:
            win_nums, my_card = line[:-1].split(":")[1].split("|")
            win_nums = [num if num else "," for num in win_nums.split(" ")]
            win_nums = set(win_nums) - set(",")
            my_card = [num if num else "," for num in my_card.split(" ")]
            my_card = set(my_card) - set(",")
            inter = win_nums.intersection(my_card)
            sum += 2**(len(inter)-1) if inter else 0
    print(sum)
            