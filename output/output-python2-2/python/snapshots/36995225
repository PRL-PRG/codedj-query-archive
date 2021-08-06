#!/usr/bin/python

from sys import argv
import string
from os.path import basename

fastafiles = argv[1:]

for fastafile in fastafiles:
    alignfile = fastafile+'.align_extended'
    fid = open(alignfile,'w')

    data = open(fastafile,'r')
    line = data.readline()

    while line:
        if not line[0] == '>': line = data.readline()
        if not line: break
        name = line[1:-1]

        sequence = ''
        line = data.readline()
        while line and not line[0] == '>':
            sequence = sequence+line[:-1]
            line = data.readline()

        fid.write( 'ALIGN %s %s\n' % (sequence,name))

    fid.close()

