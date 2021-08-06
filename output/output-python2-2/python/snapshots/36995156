#!/usr/bin/python

from sys import argv
import string
from glob import glob
from os import popen,system

indirs = argv[1:]

for indir in indirs:

    print

    ##############################################
    # Get secondary structure of native pdb
    ##############################################
    pdb_files = glob( indir+'/*.pdb.gz' )
    pdb_file = pdb_files[0]

    command = "~rhiju/python/pdb_to_secstruct.py %s _ 2> /dev/null" % pdb_file
    ss = popen(command).readlines()[0][:-1]+'L' # End with loop to force a strand end there.

    print ss

    ##############################################
    # Where are the native strands?
    ##############################################
    in_strand = 0
    strand_segments = []

    for i in range( len(ss) ):

        if ss[i] == 'E' and not in_strand:
            in_strand = 1
            strand_start = i+1

        if ss[i] != 'E' and in_strand:
            in_strand = 0
            strand_end = i
            strand_segments.append( [strand_start, strand_end] )


    strand_centers = map( lambda x: (x[0]+x[1])/2, strand_segments )

    #print strand_centers


    if len( strand_centers ) > 0:

        ##############################################
        # Check strand probability in fragments. OK?
        ##############################################
        frag_file = glob( indir+'/boinc*03_05*3.gz' )[0]
        frag_ss_lines = popen( "~rhiju/python/fragfile_to_secstructprob.py "+frag_file ).readlines()

        for i in strand_centers:
            if ( i < len(frag_ss_lines) ):
                ss_prob = float ( string.split( frag_ss_lines[i-1] ) [-1] )
                print "Checking E probability in fragments at position %3d : %f" % (i,ss_prob)

        ##############################################
        # Prepare barcode file.
        ##############################################
        fasta_file = glob( indir+'/*fasta.gz' )[0]
        barcode_file = fasta_file.replace(".fasta.gz","_forcestrand.bar")

        fid = open(barcode_file,'w')
        fid.write( "FORCESTRAND 1.0 " );
        for i in strand_centers:
            if ( i < len( frag_ss_lines) ):    # Only need to force in first monomer, right?
                fid.write( " SS %d 10.0 E " % i )
        fid.write( "\n" )

        fid.close()

        system( "gzip -rf "+barcode_file )
        print "Prepared barcode file ",barcode_file+".gz"
        print

    else:

        print "No strands in native for ",pdb_file
