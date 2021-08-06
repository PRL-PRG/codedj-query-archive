def isprime(n):
  # range starts with 3 and only needs to go up the squareroot of n
  # for all odd numbers
  for x in range(3, int(n ** 0.5) + 1, 2):
    if n % x == 0:
      return False
  return True

sum = 2

for n in range(3, 2000000, 2):
  if isprime(n):
    sum += n

print sum
