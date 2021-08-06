# Evaluate the sum of all amicable pairs under 10000.

import time
tStart = time.time()

def factorize(n):
  factors, max, i = [1], n, 2
  while i <= max:
    if n % i == 0:
      factors.append(i)
      factors.append(n / i)
    i += 1
    max = int((n / i) + 1)
  return factors

pair = {}
ambicle = []

for i in xrange(2, 10000):
  factor_sum = sum(factorize(i))
  pair[i] = factor_sum

  # Check if it is an ambicle pair
  if pair.has_key(factor_sum) and pair[factor_sum] == i and factor_sum != i:
    ambicle.append(factor_sum)
    ambicle.append(i)

print sum(ambicle)

print "Run Time = " + str(time.time() - tStart)
