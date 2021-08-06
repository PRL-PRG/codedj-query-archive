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
        addon = []
        if (len(cols) == 4) and (cols[-1] == '1' or cols[-1] == '2') and (cols[-2] == '1' or cols[-2] == '2'):
            addon = cols[-2:]   #For Jumping pairing files.
            del(cols[-2:])
        for j in range(len(cols)):
            col = cols[j]
            try:
                startnum = int( col )
                (startnum_map, startnum_in_alignfile) = figureoutnewnum(targetsequence,sequence,startnum)
                cols[j] = '%d' % startnum_map
            except:
                continue
        fid.write( string.join(cols+addon,' ')+'\n' )


    fid.close()
