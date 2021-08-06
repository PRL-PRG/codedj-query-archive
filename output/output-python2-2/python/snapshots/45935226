import math

def primes(n): 
  if n == 2: 
    return [2]
  elif n<2: 
    return []
  
  s = range(3, n + 1, 2)
  mroot = n ** 0.5
  half = (n + 1) / 2 - 1
  i, m = 0, 3

  while m <= mroot:
    if s[i]:
      j = (m * m - 3) / 2
      s[j] = 0
      while j<half:
        s[j] = 0
        j += m
    i += 1
    m = 2 * i + 3
  return [2] + [x for x in s if x]

def isprime(n):
  if n < 2: 
    return False
  if n == 2: 
    return True
  if (n / 2) * 2 == n:
    return False

  for k in primes(int((n + 1) ** 0.5))[1:]:
    if (n / k) * k == n: 
      return False
  return True

largest = 0
ab = 0

for a in xrange(-999, 1000):
  for b in xrange(-999, 1000):
    n = 0
    while isprime(n ** 2 + a * n + b):
      n += 1
    if n > largest:
      largest = n
      ab = a * b

print ab
