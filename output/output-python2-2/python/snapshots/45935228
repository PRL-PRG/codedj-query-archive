def isPalindrome(int):
  return (str(int) == str(int)[::-1])

print max([i for i in xrange(100*100, 999*999) if isPalindrome(i)])

largest = 0
for i in xrange(100, 1000):
  for j in xrange(100, 1000):
    if isPalindrome(i * j) and i * j > largest:
      largest = i * j

print largest
