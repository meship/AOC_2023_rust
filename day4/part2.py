if __name__ == "__main__":
    wins = 0
    
    with open("input.txt", "r") as f:
        lines = f.readlines()
        lst = [1] * len(lines)
        idx = 0
        for line in lines:
            win_nums, my_card = line[:-1].split(":")[1].split("|")
            win_nums = [num if num else "," for num in win_nums.split(" ")]
            win_nums = set(win_nums) - set(",")
            my_card = [num if num else "," for num in my_card.split(" ")]
            my_card = set(my_card) - set(",")
            inter = win_nums.intersection(my_card)
            print(inter)
            wins = len(inter)
            for i in range(1, wins+1):
                lst[idx+i] += lst[idx]
            idx += 1          
    print(sum(lst))