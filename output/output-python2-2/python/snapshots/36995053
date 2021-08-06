#!/usr/bin/python
# Uses tinker superimpose to get all-atom rmsd
# between two pdbs.


from sys import argv,exit
from os.path import exists
from os import system, popen
import string

native_file = argv[1]
pdb_files_in = argv[2:]

TINKER_BIN = '~rhiju/src/tinker/bin/'
TINKER_PARAMS = '~rhiju/src/tinker/params/'
param_file = 'amber99'

#Get xyz of native file.
assert( exists( native_file ))

command = '~rhiju/python/pdb2xyz.py '+native_file
#print( command )
system( command )
native_xyz  = native_file.replace('.pdb','.xyz')
assert( exists( native_xyz) )

pdb_files = []
for file in pdb_files_in:
    if file[-4:]=='.pdb':
        pdb_files.append( file )
    elif file[-4:] == '.txt': # Its a list?
        lines = open( file ).readlines()
        for line in lines:
            pdb_files.append( line[:-1] )
    else:
        lines = popen( 'find '+file+' -name "*.pdb"' ).readlines()
        for line in lines:
            pdb_files.append( line[:-1] )


#print pdb_files


for file in pdb_files:

    command = '~rhiju/python/pdb2xyz.py '+file
    #print( command )
    system( command )

    xyz  = file.replace('.pdb','.xyz')
    assert( exists( xyz) )

    rms_file = xyz.replace( '.xyz', '.rms.txt' )

    command = TINKER_BIN+'superpose '+native_xyz+' '+TINKER_PARAMS+param_file+'.prm'+' '+xyz+' '+TINKER_PARAMS+param_file+' 1,0,0 n u n 0.0 > '+rms_file+' 2> /dev/null'
    #print( command )
    system( command )

    rms_line = popen( 'tail -n 1 '+rms_file ).readlines()[0]
    print string.split(rms_line)[-1], ' ==> ', file


for file in pdb_files:
    xyz  = file.replace('.pdb','.xyz')
    command = 'rm -rf '+xyz+'*'
    #print( command )
    system( command )


command = 'rm -rf '+native_xyz+'*'
#print( command )
system( command )
