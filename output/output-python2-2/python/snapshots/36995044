#!/usr/bin/python

from sys import argv,exit
from os.path import exists
from os import system, popen
import string

##########################################################
TINKER_BIN = '~rhiju/src/tinker/bin/'
TINKER_PARAMS = '~rhiju/src/tinker/params/'
param_file = 'amber99'

CONSTRAIN = 0
if (argv.count( '-constrain' ) ):
    pos = argv.index( '-constrain' )
    del( argv[ pos ] )
    CONSTRAIN = 1

pdbfile = argv[1]

min_pdbfile = param_file + '_' + pdbfile
tag = min_pdbfile.replace('.pdb','')
xyzfile = min_pdbfile.replace('.pdb','.xyz')

###########################################################
# Convert pdb to xyz -- this will figure out chainbreaks too.
system( 'rm -rf %s*' % tag  ) # Get rid of anything from before
system( 'cp '+pdbfile+' '+min_pdbfile )
system( 'cp '+TINKER_PARAMS+param_file+'.prm .' )
system( '~rhiju/python/pdb2xyz.py '+min_pdbfile )

###########################################################
# Minimize it.
# Should we do one round with a constraint?
if (CONSTRAIN):
    lines = open( tag+".xyz" ).readlines()
    coords = []
    for line in lines[1:]:
        cols = string.split( line )
        coords.append( [ cols[2],cols[3],cols[4] ] )

    # First, need to make a "key" file, which will allow
    # for GB/SA calculation.
    fid = open( 'test.key', 'w' )
    fid.write( 'parameters amber99\n' )
    fid.write( '# Potential Function Parameter\n' )
    fid.write( 'SOLVATE            GBSA\n' )
    force_constant = 0.6 * ( 1/2.0) * (1/2.0) #In units of (kcal/mol)/Angstrom^2
    for i in range( len( coords) ):
        fid.write( 'RESTRAIN-POSITION %5d %12s %12s %12s %12.6f\n' % \
                       (i+1, coords[i][0],coords[i][1],coords[i][2],force_constant));
    fid.close()

    command = '%s/minimize %s 0.01 -k test.key' % ( TINKER_BIN, tag )
    print( command )
    system( command )
else:
    # First, need to make a "key" file, which will allow
    # for GB/SA calculation.
    fid = open( 'test.key', 'w' )
    fid.write( 'parameters amber99\n' )
    fid.write( '# Potential Function Parameter\n' )
    fid.write( 'SOLVATE            GBSA\n' )
    fid.close()
    command = '%s/minimize %s 0.01 -k test.key' % ( TINKER_BIN, tag )
    print( command )
    system( command )


#################################################################
# Score

command = '%s/analyze %s E -k test.key' % ( TINKER_BIN, tag )
print( command )
lines = popen( command ).readlines()

scorefile = 'minimize_'+pdbfile.replace('.pdb','.sc')
fid = open( scorefile, 'w' )
for line in lines:
    if ( len( line ) > 2 and not line[1]=='#' ): fid.write( line )
fid.close()

############################################################
# Convert back to PDB
system( '%s/xyzpdb %s %s.prm' % ( TINKER_BIN, tag, param_file ) )

############################################################
# Cleanup
system( 'mv %s.pdb_2 %s.pdb'  % (tag,tag.replace(param_file + '_','minimize_') ) )
system( 'rm -rf test.key %s* ' % (param_file ) )
