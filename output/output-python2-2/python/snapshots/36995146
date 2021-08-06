#!/usr/bin/python

from sys import argv
import string
from os import popen

barcode_file = argv[1]
fasta_file = argv[2]

jump_file = "/work/rhiju/rosetta_database/jump_templates_RNA_basepairs_v2.dat"
if len(argv) > 3:
    jump_file = argv[3]

if fasta_file[-3:]=='.gz':
    sequence = popen( 'zcat '+fasta_file ).readlines()[-1][:-1]
else:
    sequence = open( fasta_file ).readlines()[-1][:-1]


#print sequence

jump_lines = open( jump_file ).readlines()
pair_types = []
for line in jump_lines:
    cols = string.split( line )
    pair_type = (cols[2],cols[4],cols[5],cols[6],cols[7])
    if pair_type not in pair_types:
        pair_types.append( pair_type )

    pair_type = (cols[4],cols[2],cols[5],cols[7],cols[6])
    if pair_type not in pair_types:
        pair_types.append( pair_type )


lines = open( barcode_file ).readlines()
goodlines = []
for line in lines:
    cols = string.split( line )
    pair_type = ( cols[6],cols[7],cols[8], sequence[ int(cols[3])-1] ,sequence[ int(cols[4])-1] )

    if pair_type in pair_types:
        goodlines.append( line )
    else:
        #print 'Bad barcode?', pair_type
        continue

numlines = len( goodlines )
freq = 1.0/numlines
for line in goodlines:
    cols = string.split( line )
    cols[1] = '%8.6f' % freq
    print( string.join( cols) )
