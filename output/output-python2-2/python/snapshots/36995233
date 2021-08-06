#!/usr/bin/python

import sys
import string
from os.path import exists
from os import system

pdbfile = sys.argv[1]

lines = open( pdbfile ).readlines()

count = 0
writeout = 0
outfiles = []
for line in lines:
    if line[0:6] == 'ENDMDL':
        fid.close()
        writeout = 0

    if writeout:
        fid.write( line )

    if line[0:5] == 'MODEL':
        count += 1
        outfile = pdbfile.replace( '.pdb', '_%03d.pdb' % count )
        outfiles.append(outfile)
        fid = open( outfile, 'w')
        writeout = 1


#        command = '/users/rhiju/python/replace_chain.py '+outfile+' A _ > temp'
#        system(command)
#        command = 'mv temp '+outfile
#        system(command)

