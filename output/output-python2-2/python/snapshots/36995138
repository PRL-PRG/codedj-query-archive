#!/usr/bin/python

from sys import argv
import string
import math

outfiles = argv[1:]

for outfile in outfiles:
    lines = open(outfile).readlines()

    rgs = []
    for line in lines:
        if line[:6] == 'SCORE:':
            try:
                cols = string.split( line )
                rgs.append( float( cols[12] ) )
            except:
                continue

    rms_rg = 0.0
    for rg in rgs:
        rms_rg += rg*rg

    rms_rg /= len(rgs)
    rms_rg = pow( rms_rg, 0.5 )
    print outfile, '%5.2f' % rms_rg
