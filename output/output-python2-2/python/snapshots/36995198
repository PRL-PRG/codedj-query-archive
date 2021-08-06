#!/usr/bin/python

from sys import argv
import string

cutpoint = int( argv[1] )
residues = map( lambda x:int(x), argv[2:]  )

edges = ['W','H','S']
orientations = ['A','P']

frequency = 1.0; #Barcode machinery is smart enough to normalize?

num_jumps = 0
for i in residues:
    for j in residues:
        if (i > cutpoint or j <= cutpoint): continue
        num_jumps += 1

frequency = 1.0/( 3 * 3 * 2 * num_jumps )


for i in residues:
    for j in residues:
        if (i > cutpoint or j <= cutpoint): continue
        for k in edges:
            for m in edges:
                for o in orientations:
                    print 'INTERMOLECULAR_JUMP %f RNA_BP_JUMP %d %d %d %s %s %s' % \
                        (frequency,i,j,cutpoint,k,m,o)


