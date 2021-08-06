def sequence_length(number):
  length, n = 0, number

  while(n > 1):
    if n % 2 == 0:
      n = n / 2
      if already.has_key(n):
        already[number] = length + already[n] + 1
        return length + already[n] + 1
    else:
      n = 3 * n + 1
    length += 1

  already[number] = length + 1
  return length + 1

already = {}
terms = 0
longest = 0
seqlen = 0

for i in xrange(1, 1000000):
  seqlen = sequence_length(i)

  if seqlen > terms:
    terms = seqlen
    longest = i

print longest
