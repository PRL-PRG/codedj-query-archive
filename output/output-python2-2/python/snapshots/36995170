#!/usr/bin/python

from os import popen, system
import sys
import string

psipred_ss2file = sys.argv[1]
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

if psipred_ss2file[-2:] == 'gz':
    lines = popen('gzcat '+psipred_ss2file).readlines()
else:
    lines = open(psipred_ss2file).readlines()


for i in range( len(lines)):
    line = lines[i]
    if i+1 >= startseq and i+1 <= endseq:
        outid.write(line)

outid.close()

if gzipped:
    command = 'gzip -f '+outfile
    system(command)
