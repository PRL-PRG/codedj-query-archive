import time
tStart = time.time()

# Find the first ten digits of the sum of one-hundred 50-digit numbers.

f = open('13.txt', 'r')
numbers = map(int, f.read().split("\n")[0:-1])

print str(sum(numbers))[0:10]

print "Run Time = " + str(time.time() - tStart)
