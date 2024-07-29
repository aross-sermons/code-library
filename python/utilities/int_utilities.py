def get_first_digit(num:'int') -> int:
    while num > 10:
        num //= 10
    return num

def get_last_digit(num:'int') -> int:
    return num % 10

def get_power_ten(num:'int') -> int:
    counter = 0
    while num >= 10:
        num = num / 10
        counter += 1
    return counter

def split_into_list(num:'int') -> list[int]:
    int_list = [ ]
    while num > 0:
        int_list.append(get_last_digit(num))
        num //= 10
    int_list.reverse()
    return int_list

def concatenate_int(num_one:'int', num_two:'int') -> int:
    if num_two > 9:
        num_two_list = split_into_list(num_two)
        for i in num_two_list:
            num_one = concatenate_int(num_one, i)
        return num_one
    return((num_one*10) + num_two)