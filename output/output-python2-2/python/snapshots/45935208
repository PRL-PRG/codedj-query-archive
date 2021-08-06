# Find the 10001st prime.

def isprime(n):
  # Not fully correct isprime implementation because it doesn't care about 
  # numbers smaller than 13, and only receives odd numbers

  # Range starts with 3 and only needs to go up the squareroot of n
  # for all odd numbers

  # It might be better to implement the sieve of Eratosthenes to solve this 
  # problem more effectively.
  for x in range(3, int(n ** 0.5) + 1, 2):
    if n % x == 0:
      return False
  return True

prime, n, value = 13, 6, 15
n = 6

while n < 10001:
  if isprime(value):
    prime = value
    n += 1
  value += 2

print prime

