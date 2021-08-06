#!/usr/bin/python

from os import popen,system
import sys
import string

pdbfile = sys.argv[1]
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


if pdbfile[-2:] == 'gz':
    lines = popen('zcat '+pdbfile).readlines()
else:
    lines = open(pdbfile).readlines()

i = 0
oldresidue = '   '
for line in lines:
    currentresidue = line[23:26]
    if not currentresidue == oldresidue:
        i += 1
    oldresidue = currentresidue

    if i >= startseq and i <= endseq:
        outid.write(line)

outid.close()

if gzipped:
    command = 'gzip -f '+outfile
    system(command)
