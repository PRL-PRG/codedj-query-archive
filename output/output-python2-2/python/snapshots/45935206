"""
We can show that all triangle numbers are included within the hexagonal numbers. See ... for proof. 
This means that for a given hexagonal number we can find the triagonal number as:
  triagonal number = 2 * hexagonal number - 1
"""

pentagonal = lambda p: (3 * p ** 2 - p) / 2
hexagonal  = lambda h: h * (2 * h - 1)

p, h = 1, 1

while(True):
  number = pentagonal(p)
  diff = number - hexagonal(h)

  if (diff > 0): 
    # Pentagonal is larger than hexagonal, increase hexagonal
    h += 1
  elif (diff < 0):
    # Hexagonal is larger than pentagonal, increase pentagonal
    p += 1
  else:
    # Equal, print 'em
    print 2*h-1, p, h, '=>', number
    # ... and increase both to keep the chain going
    p += 1
    h += 1

  if number > 10000000000:
    break
