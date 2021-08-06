import time
tStart = time.time()

# Find the sum of all the numbers that can be written as the sum of fifth powers of their digits.

# The maximum value for one digit is 9^5 = 59049. We can find out the maximum possible 
# sum for a given number of digits by multiplying 59049 with the number of digits.
max = (9 ** 5) * 5

print filter(lambda n: n == sum([int(i)**5 for i in str(n)]), xrange(10, max))

print "Run Time = " + str(time.time() - tStart)
