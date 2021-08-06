#!/usr/bin/python
#Currently only setup for RNA

from sys import argv
from os import system
import string

files = argv[1:]

convert = 0
if files.count( '-convert'):
    convert = 1
    del( files[ files.index('-convert') ] )

tinker_name_convert = { '  A':'  A',                        '  C':'  C',
                        '  G':'  G',                        '  U':'  U',
                        ' rA':'  A',                        ' rC':'  C',
                        ' rG':'  G',                        ' rU':'  U',
                        'ADE':'  A',                        'CYT':'  C',
                        'GUA':'  G',                        'URA':'  U' }

for file in files:
    new_file = file.replace( '.pdb', '_temp.pdb')

    lines = open( file ).readlines()
    fid = open( new_file, 'w' )

    # For keeping track of chainbreak
    coords_p = {}
    coords_o3star = {}
    multiple_chains = 0

    chainletters = 'ABCDEFGHIJKLMNOPQRSTUVWXYZ'

    chain = 0

    oldresnum = -1
    for line in lines:
        if line[:4] == 'ATOM':
            resname = line[17:20]

            if convert:
                if resname in tinker_name_convert:
                    tinker_name = tinker_name_convert[ resname ]
                else:
                #print "Unknown RNA Residue Name? ", resname
                    tinker_name = resname
                line =  line[:17] +tinker_name +line[20:]

            # need to be very smart about chainbreaks...
            resnum = int( line[22:26] )
            atomname = line[12:16]
            x = float( line[30:38] )
            y = float( line[38:46] )
            z = float( line[46:54] )
            if (atomname == ' O3*'): coords_o3star[resnum] = [x,y,z]
            if (atomname == ' P  '):
                coords_p[resnum] = [x,y,z]
                if (resnum-1) in coords_o3star.keys():
                    c1 = coords_o3star[resnum-1]
                    c2 = coords_p[ resnum ]
                    dist2 = \
                        (c1[0]-c2[0])*(c1[0]-c2[0]) + \
                        (c1[1]-c2[1])*(c1[1]-c2[1]) + \
                        (c1[2]-c2[2])*(c1[2]-c2[2])

                    if (dist2 > 9.0 ):
                        chain = chain+1

            if ( resnum < oldresnum ):
                chain = chain+1
            oldresnum = resnum

            line = line[:21] + chainletters[chain] + line[22:]
            fid.write( line )

    fid.close()

    system( 'mv -f '+new_file+' '+file )
