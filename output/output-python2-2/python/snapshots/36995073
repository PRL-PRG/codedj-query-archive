#!/usr/bin/python

from sys import argv
from os import popen
from os.path import basename,abspath,dirname,exists
import string

scorefile = argv[1]
sol_files = argv[2:]

if not exists( sol_files[0] ):
    sol_files = []

############################################
# info for scores.
lines = popen( 'grep "SCORE\|input\|description" '+scorefile ).readlines()
scoreline = {}
tags = []

tag_index = 0

for line in lines:
    line = line[:-1]

    cols = string.split( line )
    if (cols.count( 'description' ) ):
        if cols.count('filename'):
            tag_index = cols.index( 'filename' )
        else:
            tag_index = cols.index( 'description' )
        header_line = line
        continue

    else:
        tag = cols[ tag_index ]
        tag = tag.replace( '.pdb','')
        tag = tag.replace( '_noH','')
        tags.append( tag )
        scoreline[ tag ] = line
        line_length = len( line )


############################################
sol_scores = {}
for sol_file in sol_files:
    lines = open( sol_file).readlines()

    foundit = 0

    for line in lines[0:3]:
        cols = string.split( line )
        if len(cols)>=7:
            try:
                rfz = float( cols[-5][4:] )
                tfz = float( cols[-4][4:] )
                llg1 = float( cols[-2][4:] )
                llg2 = float( cols[-1][4:] )
                foundit = 1
            except:
                continue


    symm_dist_score = -2.0
    symm_dist_file = sol_file.replace( '.sol', '.1.pdb.symm_dist' )
    if exists( symm_dist_file ):
        symm_dist_score = float( open( symm_dist_file ).readlines()[0] )

    if foundit:
        tag = basename( sol_file )
        tag = tag.replace('.sol','')
        tag = tag.replace('_noH','')
        tag = tag.replace('_stripsidechain','')
        sol_scores[ tag ] = [rfz,tfz,llg1,llg2,symm_dist_score]


############################################
# Combined score line
score_index = header_line.find( 'SCORE ' )
if (score_index < 0): score_index = header_line.find( 'score ' )
score_index -= 2

header_line_new = header_line[: score_index]+'  RFZ  TFZ LLG1 LLG2 symdst  '+header_line[score_index:]
print header_line_new

for tag in tags:
    if tag in sol_scores.keys():
        scores = sol_scores[tag]
        print '%s %4.1f %4.1f %4d %4d %6.3f  %s' % ( scoreline[tag][:score_index],scores[0],scores[1],scores[2],scores[3],scores[4],scoreline[tag][score_index:])

############################################
# For MATLAB, a "text-free" file.
fid = open( dirname( abspath( scorefile) )+'/data_for_matlab.txt', 'w' )
fid_pdbnames = open( dirname( abspath( scorefile) )+'/pdbnames_for_matlab.txt', 'w' )

header_cols = string.split( header_line )
useful_cols = ['rms','mxn','mxrms','mm11','mm22','mm33','mm43','mm74','gdtmm','score']
useful_index = []
for useful_col in useful_cols:
    useful_index.append( header_cols.index( useful_col  ) )

for tag in tags:
    if tag in sol_scores.keys():

        scores = sol_scores[tag]

        pdbfile = tag.replace('.sol','.1.pdb');
        fid_pdbnames.write( pdbfile+'\n')

        fid.write( '%4.1f %4.1f %4d %4d %6.3f' % ( scores[0],scores[1],scores[2],scores[3],scores[4] ) )

        cols = string.split( scoreline[ tag ] )
        for i in useful_index:
            fid.write( '%8.3f ' % float( cols[i] ) )

        fid.write( '\n' )

        #description_pos = scoreline[ tag ].index('S_')
        #        fid.write( '%4.1f %4.1f %4d %4d %s\n' % \
        #           ( scores[0],scores[1],scores[2],scores[3],
        #             scoreline[ tag ][6:description_pos] ))


fid.close()
fid_pdbnames.close()


