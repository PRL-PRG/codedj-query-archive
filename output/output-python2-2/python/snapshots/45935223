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

number = 1
count = 1

while len(factorize(number)) < 500:
  count += 1
  number += count

print number

print "Run Time = " + str(time.time() - tStart)
