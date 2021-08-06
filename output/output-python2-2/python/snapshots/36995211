#!/usr/bin/python

from os import popen,system
import sys
import string

fragfilefile = sys.argv[1]
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

if fragfilefile[-2:] == 'gz':
    lines = popen('zcat '+fragfilefile).readlines()
    fragfilefile = fragfilefile[:-3]
else:
    lines = open(fragfilefile).readlines()

startoutput = 0
fragmentsize = int( fragfilefile[-14:-12])
for line in lines:
    cols = string.split(line)

    if cols.count('position:'):
        i = int(cols[1])
        if i >= startseq:
            startoutput = 1
        if i > endseq - fragmentsize + 1:
            break
        newresnum = '%4d' % (i - startseq + 1)
        line = line[:19] + newresnum + line[23:]

    if startoutput:
        outid.write(line)

outid.close()

if gzipped:
    command = 'gzip -f '+outfile
    system(command)
