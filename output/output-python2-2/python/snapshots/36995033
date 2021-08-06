#!/usr/bin/python

import sys, string
from os import popen,system

try:
	NUM_DECOYS = int( sys.argv[-1] )
	del( sys.argv[ -1 ] )
except:
	NUM_DECOYS = 5

files = sys.argv[1:]

energies = []
energy_to_pdb = {}

lines =  popen( 'grep total '+string.join( files) ).readlines()

for line in lines:

        cols = string.split(line)
        pdb = string.split( cols[0],':' )[0]

        energy = float( cols[1] )
        energy_to_pdb[ energy ] = pdb
        energies.append( energy )

energies.sort()

tag = string.split( files[0], '_' )[0]
for i in range( NUM_DECOYS ):
    pdb = energy_to_pdb[ energies[i] ]

    outpdb = '%s_%04d.pdb' % ( tag, i+1 )
    print '%4d %8.3f :  %s --> %s ' % (i, energies[ i ], pdb, outpdb)
    system( 'cp '+pdb+' '+outpdb )



