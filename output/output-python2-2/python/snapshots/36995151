#!/usr/bin/python

from sys import argv
import string
from os import popen,system


pdb_files = argv[1:]

for pdb_file in pdb_files:
    command = '~rhiju/python/pdb_to_secstruct.py '+pdb_file
    line = popen( command ).readlines()[0][:-1]

    print line

    goodres = []
    goodres_pad = []
    in_loop =1
    for i in range( len(line) ):
        if not line[i]=='L':

            goodres.append( i+1 )
            goodres_pad.append( i+1 )

            if in_loop and i > 0:
                goodres_pad.append( i )
            in_loop = 0
        else:
            if not in_loop:
                goodres_pad.append( i + 1)
            in_loop = 1
        #Hmm, cutting out DSSP loops doesn't leave a whole lot.
        # keep one residue into each loop?

    command = '~rhiju/python/pdbslice.py '+pdb_file+' -subset '
    for i in goodres: command += ' %d' % i
    command += ' noloop_ '
    print( command )
    system( command )


    command = '~rhiju/python/pdbslice.py '+pdb_file+' -subset '
    for i in goodres_pad: command += ' %d' % i
    command += ' noloop_pad_ '
    print( command )
    system( command )
