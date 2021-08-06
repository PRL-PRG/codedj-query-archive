#!/usr/bin/python

from os import popen,system
import sys
import string

fastafile = sys.argv[1]
startseq = int( sys.argv[2])
endseq = int( sys.argv[3])

gzipped = 0
if len(sys.argv)>4:
   outfile = sys.argv[4]
   if outfile[-2:] == 'gz':
       outfile = outfile[:-3]
       gzipped = 1
   outid = open(outfile,'w')
else:
    outid = sys.stdout

if fastafile[-2:] == 'gz':
    lines = popen('zcat '+fastafile).readlines()
else:
    lines = open(fastafile).readlines()

outid.write(lines[0])

sequencelines = lines[1:]
sequencelines = map( lambda x:x[:-1], sequencelines)
sequence = string.join(sequencelines,'')

for i in range(len(sequence)):
    if i+1 >= startseq and i+1 <= endseq:
        outid.write('%s' % sequence[i])

outid.write('\n')

outid.close()

if gzipped:
    command = 'gzip -f '+outfile
    system(command)
