#!/usr/bin/python

from sys import argv,exit
from os import popen, system
from os.path import basename,exists
import string
import commands
from glob import glob

def Help():
    print
    print 'Usage: '+argv[0]+' <silent out file 1> < silent file 2> ... <N> '
    print '  Will extract N decoys with lowest score from each silent file.'
    print '  If you want to select based on another column, say 12 (Rg), the'
    print '    last arguments should be -12 <N>  (for lowest Rg) or +12 <N>'
    print '    (for highest Rg).'
    print

    exit()


if len(argv)<2:
    Help()

replace_names = 1
if argv.count('-no_replace_names'):
    pos = argv.index('-no_replace_names')
    del( argv[pos] )
    replace_names = 0

use_start_pdb = 0
if argv.count('-start_pdb'):
    pos = argv.index('-start_pdb')
    del( argv[pos] )
    start_pdb_file = argv[ pos ]
    del( argv[pos] )
    use_start_pdb = 1

try:
    NSTRUCT = int(argv[-1])
    del(argv[-1])
except:
    NSTRUCT = 2



#Another possibility... user supplies -rms or +rms
scorecol_name = argv[-1]
if scorecol_name[0] == '-':
    REVERSE = ''
if scorecol_name[0] == '+':
    scorecol_name = scorecol_name[1:]
    REVERSE = '-r'

scorecol_name = scorecol_name[1:]
del( argv[-1] )

infiles = argv[1:]

tags = []

lines = popen( 'grep SCORE '+infiles[0]+'| head -n 50').readlines()
cols = string.split( lines[-1] )
scoreindex = cols.index( scorecol_name ) + 2

command = 'grep SCORE %s | grep -v is_reference_pose | sort %s -nk%d | head -n %d' % \
          (string.join( infiles ), REVERSE, scoreindex, NSTRUCT )
lines = popen( command ).readlines()

tags = {}
for N in range( NSTRUCT ):
    line = lines[ N ]
    cols = string.split( line )
    outfile = string.split( cols[0], ':' )[0]
    if not outfile  in tags.keys():
        tags[ outfile ] = []
    tags[ outfile ].append( ( cols[1], N )   )


EXTRA_PARAMS_TAG = ''
extra_params = glob( '*.params' )
if len( extra_params ) > 0:
    EXTRA_PARAMS_TAG = ' -extra_res_fa '+string.join( extra_params )


for outfile in tags.keys():
    tags_to_extract= []
    for tag in tags[outfile]:
        tags_to_extract.append( tag[0] )

    command = '~rhiju/src/mini/bin/extract_atomtree_diffs.linuxgccrelease  -database ~rhiju/minirosetta_database/   '+EXTRA_PARAMS_TAG + ' -s '+outfile+' -tags '+string.join( tags_to_extract )
    print command
    system( command )

    for tag in tags[ outfile ]:
        new_tag = '%s.%s.%d.pdb' % (infiles[0],scorecol_name, (tag[1]+1) )
        command = 'mv %s.pdb %s' % (tag[0], new_tag )
        print( command )
        system( command )


