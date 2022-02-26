# Advent Of Code 2021 - Day 3 , Puzzle 1
# This program might not make sense to you the first time you read it. If so, please head on to adventofcode.com/2021 and read the challenge of Day 3 first

import math

path = "--<text file location>--"   
gamma_rate = ""

def onesComp (num):     #just a standalone function to find the 1's complement of a decimal
    number_of_bits = (int)(math.floor(math.log(num) / math.log(2))) + 1
    return ((1 << number_of_bits) - 1) ^ num

for val in range(0,12):     #the complete input block
    input_file = open(path, 'r')    #opens the input stored in a text file into input_file
    input_line = input_file.readline()
    count = 0

    while input_line != "" :     #while loop till end of file is reached, count is used to find the most common digit in the required line
        extract = list(input_line)
        if extract[val] == '0' :
            count -= 1
        else:
            count += 1
        input_line = input_file.readline()
    
    if count > 0 :    #appends 1 to gamma_rate if 1 is the most common digit in the required line, else appends 0
        gamma_rate = gamma_rate + "1"
    else :
        gamma_rate = gamma_rate + "0"

gamma = int(gamma_rate, 2)
print("The gamma rate is:", gamma)
epsilon = onesComp(gamma)    
print("The epsilon rate is:", epsilon)
print()
print("The required value:", gamma*epsilon)
