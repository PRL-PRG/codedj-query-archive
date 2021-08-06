#!/usr/bin/python
##
## make boinc submit files for homologs
##

from phil import *
import sys
from os.path import dirname, basename, abspath

fasta_list = argv[1:]

for fasta in fasta_list:
    prefix_defined = 0
    gzipped = 0

    frag_dir = dirname(abspath(fasta))+'/'
    fasta = basename(fasta)

    fivelettercode = fasta[-11:-6]
    fourlettercode = fasta[-11:-7]
    chaincode = fasta[-7:-6]

    prefix = fasta[:-11]
    if len(prefix)>0:
       prefix_defined = 1

    print prefix


    oldfile = frag_dir + prefix + fivelettercode + '.fasta'
    assert( exists( oldfile ) )
    newfile = frag_dir + prefix + fivelettercode + '.fasta'
    command = 'cp ' + oldfile + ' ' + newfile
    print( command )
    system( command )
    command = 'gzip -f ' + newfile
    print( command )
    system( command )

    oldfile = frag_dir + fivelettercode + '.psipred_ss2'
    print oldfile
    assert( exists( oldfile ) )
    newfile = frag_dir + prefix + fivelettercode + '.psipred_ss2'
    command = 'cp ' + oldfile + ' ' + newfile
    print( command )
    system( command )
    command = 'gzip -f ' + newfile
    print( command )
    system( command )


    for fragsize in ['03','09']:
        fragfile = frag_dir + 'aa'+fivelettercode+fragsize+'_05.200_v1_3'
        assert( exists( fragfile ) )
        tempfile = frag_dir + 'temp'

        command = '/users/boinc/bin/reduce_fragment_library_size.pl '+ \
                  fragfile + ' > temp'
        print( command )
        system( command )

        command = 'gzip temp'
        print( command )
        system( command )

        newfragfile = frag_dir + prefix + 'aa'+fivelettercode+fragsize+'_05.200_v1_3.gz'
        command = 'mv temp.gz ' + newfragfile
        print( command )
        system( command )


    for fragsize in ['03','09']:
        fragfile = frag_dir + 'aa'+fivelettercode+fragsize+'_05.200_v1_3'
        assert( exists( fragfile ) )

        shrink_to_25 = ''
        if fragsize == '09':
            shrink_to_25 = '25'
        command = '/users/boinc/bin/reduce_fragment_library_size.pl '+ \
                  fragfile + ' ' + shrink_to_25 + ' > temp'
        print( command )
        system( command )


        command = 'gzip temp'
        print( command )
        system( command )

        newfragfile = frag_dir + 'boinc_'+prefix + 'aa'+fivelettercode+fragsize+'_05.200_v1_3.gz'
        command = 'mv temp.gz ' + newfragfile
        print( command )
        system( command )
