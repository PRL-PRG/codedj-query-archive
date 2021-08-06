#!/usr/bin/python

from sys import argv
from os import system

tinker_name_convert = { '  A':'  A',                        '  C':'  C',
                        '  G':'  G',                        '  U':'  U',
                        ' rA':'  A',                        ' rC':'  C',
                        ' rG':'  G',                        ' rU':'  U',
                        'ADE':'  A',                        'CYT':'  C',
                        'GUA':'  G',                        'URA':'  U' }

for file in argv[1:]:

    assert( file.count( '.pdb' ) )

    new_file = file.replace( '.pdb', '_temp.pdb')
    fid = open( new_file, 'w' )

    lines = open( file ).readlines()

    for line in lines:
        if line[:4] == 'ATOM':
            resname = line[17:20]
            if resname in tinker_name_convert:
                tinker_name = tinker_name_convert[ resname ]
            else:
                #print "Unknown RNA Residue Name? ", resname
                tinker_name = resname
            fid.write( line[:17] +tinker_name +line[20:])
    fid.close()

    command = '~rhiju/src/tinker/bin/pdbxyz '+new_file+' ~rhiju/src/tinker/params/amber99.prm > /dev/null 2> /dev/null'
    #print command
    system( command )

    command = 'mv '+new_file.replace('.pdb','.xyz')+' '+file.replace('.pdb','.xyz')
    #print command
    system( command )

    command = 'rm '+new_file.replace('.pdb','*')
    #print command
    system( command )

