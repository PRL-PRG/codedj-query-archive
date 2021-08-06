#!/usr/bin/python

import string
from sys import argv,stderr,exit
from os import popen,system
from os.path import basename,exists
from glob import glob

def Help():
    print
    print 'Usage: ',argv[0],' <file1> <file2> '
    print '  outfiles must have a "flavor" column. '
    print
    exit()

if len(argv)<2:
    Help()



infiles_input = argv[1:]

infiles = []
for infile in infiles_input:
    if exists(infile):
        infiles.append(infile)
    else:
        glob_infiles = glob('/net/boinc/results/'+infile[:8]+'/'+infile)
        glob_infiles.sort()
        if len(glob_infiles) == 0:
            glob_infiles = glob('/net/boinc/results_ralph/'+infile[:8]+'/'+infile)
            glob_infiles.sort()

        infiles += glob_infiles


for infile in infiles:
    file_ids = {}
    count = {}

    if not exists(infile):
        Help()

    print 'Cleaning silent file ==> ',infile
    inp = open( infile, 'r' )

    headerlines = []
    line = inp.readline()
    headerlines += line
    line = inp.readline()
    headerlines += line

    cols = string.split( line )

    if cols.count( 'flavor' ) > 0:
        flavor_index = cols.index( 'flavor' )
    elif cols.count( 'FLAVOR' ) > 0:
        flavor_index = cols.index( 'FLAVOR' )
    else:
        Help()


    line = inp.readline()

    assert( line[0:6] == 'SCORE:' )

    while line:
        if line[0:6] == 'SCORE:':
            cols = string.split( line )
            flavor = cols[ flavor_index ]

            if not file_ids.has_key( flavor ):

                outfile = infile.replace('.out','.flavor'+flavor+'.out' )
                file_ids[ flavor ] = open( outfile, 'w')

                for headerline in headerlines:
                    file_ids[ flavor ].write(headerline)

                count[ flavor ] = 0

            count[flavor] += 1

        file_ids[flavor].write(line)

        line = inp.readline()


    flavors = file_ids.keys()
    flavors.sort()
    for flavor in flavors:
        print flavor ,': Found ', count[flavor], ' decoys.'
