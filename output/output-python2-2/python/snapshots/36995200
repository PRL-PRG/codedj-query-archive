#!/usr/bin/python

from os import popen
import sys
import string

secstructprobfiles = sys.argv[1:]

for secstructprobfile in secstructprobfiles:
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


    in_strand = 0

    outfile = secstructprobfile + '.strandpred.txt'
    out = open(outfile,'w');

    E_BEGIN_CUTOFF = 0.25
    E_END_CUTOFF = 0.5

    MIN_STRAND_LENGTH = 4

    E_BEGIN_CUTOFF = 0.1
    E_END_CUTOFF = 0.5
    MIN_STRAND_LENGTH = 3

    E_BEGIN_CUTOFF = 0.1
    E_END_CUTOFF = 0.1
    MIN_STRAND_LENGTH = 3

    for i in range(resnum):
        if not in_strand and (Eweight[i] > E_BEGIN_CUTOFF):
            in_strand = 1
            startnum = i+1

        if in_strand and (Eweight[i] < E_END_CUTOFF):
            in_strand = 0
            endnum = i+1
            if endnum-startnum+1 > MIN_STRAND_LENGTH:
                out.write('%3d  %3d\n' % (startnum,endnum))



