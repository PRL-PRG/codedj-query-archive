#!/usr/bin/python

from sys import argv,exit
from os import popen, system
from os.path import basename
import string



def Help():
    print
    print 'Usage: '+argv[0]+' <silent out file 1> < silent file 2> ... <N> '
    print '  Will extract first N decoys to a new outfile'
    print

    exit()


if len(argv)<2:
    Help()

try:
    NSTRUCT = int(argv[-1])
    del(argv[-1])
except:
    NSTRUCT = 2

infiles = argv[1:]

for infile in infiles:
    assert(infile[-3:] == 'out')

    fid = open( infile )

    outfile = infile.replace('.out','.%ddecoys.out' % NSTRUCT )
    outid = open( outfile, 'w')

    print 'Extracting %d decoys into %s' % (NSTRUCT, outfile)

    line = fid.readline()
    count = 0
    while (count <= NSTRUCT and line):

        if line[0:5] == 'SCORE':
            count += 1

        outid.write( line )

        line = fid.readline()

    outid.close()
    fid.close()

