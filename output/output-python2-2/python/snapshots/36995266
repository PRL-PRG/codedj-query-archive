#!/usr/bin/python

from sys import argv
import string

fasta_files = argv[1:]

for fasta_file in fasta_files:
    lines = open( fasta_file ).readlines()

    pir_file = fasta_file.replace('.fasta.txt','.pir')
    pir_file = pir_file.replace('.fasta','.pir')
    pir_file = string.lower( pir_file )

    lines = map(  lambda x:x[:-1], lines )

    goodlines = []
    in_sequence = 0
    done = 0
    for line in lines:
        if line[:1] == '>':
            if in_sequence: #Next sequence.
                sequence = string.join( goodlines, '')
                if len( sequence ) > 30:
                    break
                else:
                    goodlines = []
            in_sequence = 0
            continue
        else:
            in_sequence = 1
            goodlines.append( line )


    sequence = string.join( goodlines, '')
    sequence = sequence.replace( 'X', 'M' ) ######SELENOMETHIONINE?
    print fasta_file, ' ==> ', sequence

    fid = open( pir_file, 'w')
    fid.write( lines[0][:-1] )
    fid.write( '  NRES: %d' % len( sequence ) )
    fid.write( '\n\n' )
    fid.write( '%s\n' % sequence )
    fid.close()
