#!/usr/bin/python

from os import popen
import sys
import string

secstructprobfile = sys.argv[1]
lines = open(secstructprobfile,'r').readlines()

Cweight = []
Hweight = []
Eweight = []
for line in lines:
    cols = string.split(line)
    resnum = int( cols[0] )
    Cweight.append( float( cols[1] ) )
    Hweight.append( float( cols[2] ) )
    Eweight.append( float( cols[3] ) )

LOOP_CUTOFF = 0.5
for i in range(resnum-1):
    if Cweight[i]< LOOP_CUTOFF and Cweight[i+1]< LOOP_CUTOFF:
        break

startseq = i+1 #convert to standard numbering

for i in range(resnum-1):
    j = resnum - i -1
    if Cweight[j]< LOOP_CUTOFF and Cweight[j-1]< LOOP_CUTOFF:
        break

endseq = j+1

#print 'Original sequence: %d -> %d' % (1,resnum)
#print 'New sequence:      %d -> %d' % (startseq,endseq)

print '%d %d' % (startseq,endseq)




