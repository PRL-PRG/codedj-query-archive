import operator

def factorial(x):
  if x == 0 or x == 1: 
    return x
  return reduce(operator.mul, xrange(2, x + 1))

print filter(lambda n: n == sum([factorial(int(i)) for i in str(n)]), xrange(3, 10000))
