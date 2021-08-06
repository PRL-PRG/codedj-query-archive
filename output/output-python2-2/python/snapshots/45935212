# Find the largest palindrome made from the product of two 3-digit numbers.

def isPalindrome(int):
  return (str(int) == str(int)[::-1])

largest = 0
for i in xrange(100, 1000):
  for j in xrange(100, 1000):
    if isPalindrome(i * j) and i * j > largest:
      largest = i * j

print largest
