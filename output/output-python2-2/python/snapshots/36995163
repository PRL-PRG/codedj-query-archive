#!/usr/bin/python

import sys
from os import popen
import string

infiles = sys.argv[1:]

for infile in infiles:

    assert( infile[-4:] == '.pdb' )
    sasafile = infile[:-4] + '.sasa'
    fid = open( sasafile, 'w' )

    command = '~rhiju/src/rosetta++/rosetta.mactel  -s %s  -rna_sasa -prna -paths ~rhiju/paths.txt -rna_sasa_file temp.sasa' % infile
#    command = 'rm stdout.txt; ~rhiju/rosetta++/rosetta.mactelboincgraphics  -s %s  -rna_sasa -prna -paths ~rhiju/paths.txt -rna_sasa_file ~rhiju/projects/rna_tests/1gid/1gid_footprint.sasa' % infile
    print command
    lines = popen(command).readlines()
#    lines = open( 'stdout.txt').readlines()

    for line in lines:
        cols = string.split( line )

        if len(cols)==0: continue

        if cols[0] != 'ACTUAL_RNA_SASA': continue

        fid.write( '%s %s\n' % (cols[1],cols[2]) )

    fid.close()
