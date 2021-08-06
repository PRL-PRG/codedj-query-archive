#!/usr/bin/python

from sys import argv, exit, stdout
import string
from os.path import dirname, basename, exists



alignfile = argv[1]
startfile = argv[2]

lines = open( alignfile, 'r').readlines()
startlines = open( startfile, 'r').readlines()

def figureoutnewnum(targetsequence,sequence,startnum):
    targetseqpos = 0
    seqpos = 0
    for k in range( len(targetsequence)):
        alignpos = k+1
        if targetsequence[k] != '-':
            targetseqpos += 1
        if sequence[k] != '-':
            seqpos += 1
        if (targetseqpos >= startnum):
            break

    if seqpos == 0:
        seqpos = 1
    return (seqpos, alignpos)

line = lines[0]
targetsequence = string.split(line)[1]

for i in range(1, len(lines)):
    newfile = startfile.replace('001','%03d' % (i+1) )
    fid = open(newfile,'w')

    line = lines[i]
    sequence = string.split(line)[1]

    for startline in startlines:
        cols = string.split(startline)
        try:
            assert( len(cols) == 7)
            blah = int(cols[0])
            blah = int(cols[2])
            for j in [0,2]:
                col = cols[j]
                startnum = int( col )
                (startnum_map, startnum_in_alignfile) = figureoutnewnum(targetsequence,sequence,startnum)
                cols[j] = '%4d' % startnum_map
            newline = ' %6s %2s %6s %2s %12s %10s %10s\n' % (cols[0],cols[1],cols[2],cols[3],cols[4],cols[5],cols[6])
        except:
            newline = startline
        fid.write(newline)

    fid.close()
