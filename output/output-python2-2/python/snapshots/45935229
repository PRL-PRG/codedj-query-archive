import time
tStart = time.time()

# Find the maximum sum travelling from the top of the triangle to the base.

f = open('67.txt', 'r')
triangle = []
for row in f.read().split("\n"):
  triangle.append(map(int, row.split()))
del triangle[-1]

for i in xrange(1, len(triangle)):
  triangle[i][0] += triangle[i - 1][0]
  triangle[i][len(triangle[i]) - 1] += triangle[i - 1][len(triangle[i]) - 2]

  for j in xrange(1, len(triangle[i]) - 1):
    triangle[i][j] += max(triangle[i - 1][j], triangle[i - 1][j - 1])

print max(triangle[-1])

print "Run Time = " + str(time.time() - tStart)

