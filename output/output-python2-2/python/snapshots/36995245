#!/usr/bin/python

from sys import argv,stdout,stderr
from os import system
import string
infilelist = argv[1]


rmsd_threshold = 8.0
if argv.count('-R'):
    pos = argv.index('-R')
    rmsd_threshold = float( argv[pos+1])
    del(argv[pos])
    del(argv[pos])

lines = open(infilelist,'r').readlines()
lines = [x[:-1] for x in lines]

maxsub_save = {}
for line in lines:
    maxsub_save[line] = {}


for i in range(len(lines)):
    line1 = lines[i]
    for j in range(i, len(lines)):
        line2 = lines[j]

        command = '/work/rhiju/python/superimpose.py %s %s  -R %d > blah.pdb 2> blah.err' % (line1,line2,rmsd_threshold)
        #print(command)
        system(command)

        superimposeline = open( 'blah.err', 'r').readlines()[-1]
        fit_threshold = float( string.split(superimposeline)[3] )
        maxsub = int( string.split(superimposeline)[5] )
        if fit_threshold > rmsd_threshold:
            maxsub = 0 # Failure!

        stderr.write('%s %s %d\n' % (line1, line2, maxsub))

        maxsub_save[line1][line2] = maxsub
        maxsub_save[line2][line1] = maxsub

print
print


maxlen = max( [len(x) for x in lines] )
blanks = ' '*200
for line1 in lines:
    print '%s' % line1+blanks[len(line1):maxlen],
    for line2 in lines:
        print '%03d' % maxsub_save[line1][line2],
    print

command = 'rm maxsub*pdb blah*pdb'
system(command)
