#!/usr/bin/python
# Uses tinker superimpose to get all-atom rmsd
# between two pdbs.


from sys import argv,exit
from os.path import exists,basename,dirname
from os import system, popen, getcwd, chdir
import string

pdb_files_in = argv[1:]

TINKER_BIN = '~rhiju/src/tinker/bin/'
TINKER_PARAMS = '~rhiju/src/tinker/params/'
param_file = 'amber99'

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

CWD = getcwd()

for file in pdb_files:


    chdir( dirname( file ) )

    # We'll need this params file.
    system( 'cp %s/%s.prm .' % (TINKER_PARAMS, param_file ) )

    command = '~rhiju/python/pdb2xyz.py '+basename(file)
    #print( command )
    system( command )

    fid = open( 'test.key', 'w' )
    fid.write( 'parameters amber99\n' )
    fid.write( '# Potential Function Parameter\n' )
    fid.write( 'SOLVATE            GBSA\n' )
    fid.close()

    tag = basename( file ).replace( '.pdb','')
    command = '%s/analyze %s E -k test.key' % ( TINKER_BIN, tag )
    print( command )
    lines = popen( command ).readlines()

    scorefile = basename(file).replace('.pdb','.sc')
    fid2 = open( scorefile, 'w' )
    for line in lines:
        if ( len( line ) > 2 and not line[1]=='#' ): fid2.write( line )
    fid2.close()

    command =  'rm '+tag+'.xyz* test.key '+param_file+'.prm'
    system( command )

    chdir( CWD )
