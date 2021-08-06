# Calculate the sum of all the primes below two million.

def isprime(n):
  # range starts with 3 and only needs to go up the squareroot of n
  # for all odd numbers

  # It might be better to implement the Sieve of Eratosthenes since we know our
  # max value.
  for x in xrange(3, int(n ** 0.5) + 1, 2):
    if n % x == 0:
      return False
  return True

sum = 2

for n in xrange(3, 2000000, 2):
  if isprime(n):
    sum += n

print sum
