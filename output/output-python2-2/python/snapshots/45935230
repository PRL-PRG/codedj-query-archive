import time
tStart = time.time()

# What is the total of all the name scores in the file of first names?

f = open('22.txt', 'r')
names = map(lambda x: x[1:-1], f.read().split(","));

names.sort()

total = 0
for n in range(len(names)):
  total += sum([ord(i) - 64 for i in names[n]]) * (n + 1)

print total

print "Run Time = " + str(time.time() - tStart)
