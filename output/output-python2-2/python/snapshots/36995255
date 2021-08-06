#!/usr/bin/python

from sys import argv
import string
from os import popen

files = argv[1:]

for file in files:
    if file[-3:]=='.gz':
        lines = popen(  'gzcat '+file ).readlines()
    else:
        lines = open( file ).readlines()

    pdblines = {}
    scores = {}
    count = -1

    for line in lines:
        if len(line) > 6 and line[:6]=='HEADER':
            count += 1
            pdblines[ count ] = []

        pdblines[ count ].append( line )

        if len(line) > 15 and line[ 9:15] == 'score:':
           scores[ count ] = float( string.split( line )[-1])

    bestcount = 0
    bestscore = scores[0]
    for i in range( count ):
        if scores[i] < bestscore:
            bestcount = i
            bestscore = scores[i]

    print 'Found best score: ', scores[ bestcount ]

    newfile = file.replace('.pdb.concat','.lowscore.pdb')
    newfile = newfile.replace('.gz','')

    fid = open( newfile, 'w' )
    for line in pdblines[ bestcount ]:
        fid.write( line )
    fid.close()
