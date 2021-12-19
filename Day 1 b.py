path = '--<insert file location here>--'
input_file = open(path, 'r')

count = 0
i = 0
num1 = input_file.readline()
num2 = input_file.readline()
num3 = input_file.readline()
sum2 = 0

while (num3 != ""):
    num1 = int(num1)
    num2 = int(num2)
    num3 = int(num3)
    sum1 = num1+num2+num3

    if i == 0:
        print(sum1, " - no previous values")
        i += 1
    else:
        if sum2 < sum1:
           print(sum1, " - increased")
           count += 1
        else:
            print(sum1, " - decreased")
    sum2 = sum1
    num1 = num2
    num2 = num3
    num3 = input_file.readline()

print()
print("The required value: ", count)
     


    
