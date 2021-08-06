# What is the smallest number that is evenly divisible by all the numbers from 1 to 20?

def divisable(n):
  return bool(sum([n % i for i in range(1,21)]) == 0)

n = 2520
while not divisable(n):
  n += 20

print n
