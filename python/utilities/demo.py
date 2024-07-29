import int_utilities

def int_utilities_demo():
    int_one = 37604
    print(f'int_one = {int_one}')
    int_two = 21958
    print(f'int_two = {int_two}')

    #print(f'concatenate_int(int_one, int_two) = {concatenate_int(int_one, int_two)}')

print(int_utilities.concatenate_int(37, 604))
print(int_utilities.concatenate_int(1, 23456))
print(int_utilities.concatenate_int(0, 1001))
