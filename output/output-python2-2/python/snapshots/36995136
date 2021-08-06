#!/usr/bin/python

from sys import argv
from os import popen, system
from glob import glob
from os.path import exists,basename

pdbfiles = argv[1:]

if not exists(pdbfiles[0]): # Need to use glob
    print 'Using glob... '
    pdbfiles = glob( pdbfiles[0] )

    pdbfiles.sort()

for pdbfile in pdbfiles:

    lines = popen('zcat '+pdbfile).readlines()

    print pdbfile,

    xray = 1
    chain_lengths = [];
    old_chain = ''
    chain_init = 0
    old_res = '   '

    count = 0

    for line in lines:

        end_of_chain = 0

        if len(line) < 7:
            continue

        if line[:6] == 'EXPDTA':
            if ( line.find( 'X-RAY' ) < 0 ):
                xray = 0
                break

        if line[:6] == 'ENDMDL':
            end_of_chain = 1

        if line[:4] == 'ATOM':
            chain = line[21:22]

            if not chain_init:
                old_chain = chain # initialize
                chain_init = 1

            if not (chain == old_chain):
                end_of_chain = 1
            old_chain = chain

            res = line[22:26]
            if not (res == old_res) :
                count += 1
            old_res = res

            elem = line[13:14]
            if elem == 'P' : #prob nucleic acid
                count = 0

        if end_of_chain:
            if ( count > 5):
                chain_lengths.append( count )
            count = 0

    if (count > 1 ):
        chain_lengths.append( count )

    if ( not xray ):
        print 'Not x-ray!'
        continue
    else:
        print '',




    mean_length = 0
    tot_length = 0
    if len( chain_lengths ) > 0:
        for i in range( len( chain_lengths ) ):
            tot_length += chain_lengths[i]
        mean_length = tot_length / len( chain_lengths )

        multimer = 1
        N = 0
        for i in range( len( chain_lengths ) ):
            length_diff = abs( (chain_lengths[i] - mean_length)/(1.0 * mean_length) )
            if length_diff <= 0.1:
                N += 1
            else:
                multimer = 0
                break

        if (N <= 1): multimer = 0

    else:
        multimer = 0

    if multimer:
        print 'MULTIMER ',N,
    else:
        print 'NOTMULT  ',0,


    if multimer:
        helix = 1

        command = 'python ~rhiju/python/pdb_to_secstruct.py  '+pdbfile+ ' 2> /dev/null '
        secstruct_line = popen( command ).readlines()
        #print secstruct_line[0]
        #print secstruct_line[0].count('E')

        if (secstruct_line[0].count('E') > 0 ):
            helix = 0
    else:
        helix = 0

    if helix:
        print ' HELIX ',
    else:
        print ' NOTHLX',

    print ('%4d' % mean_length), ('%4d' % tot_length),
    print chain_lengths
