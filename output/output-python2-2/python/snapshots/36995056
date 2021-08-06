#!/usr/bin/python

from sys import argv,exit
from os.path import exists
from os import system, popen

##########################################################
# NOTE: WILL NEED TO FIX THIS FOR CLUSTER USE!
TINKER_BIN = '~rhiju/src/tinker/bin/'
TINKER_PARAMS = '~rhiju/src/tinker/params/'
param_file = 'amber99'

pdbfile = argv[1]

min_pdbfile = 'min_' + pdbfile
tag = min_pdbfile.replace('.pdb','')
xyzfile = min_pdbfile.replace('.pdb','.xyz')
scorefile = '%s.sc' % tag

###########################################################
# Convert pdb to xyz
system( 'rm -rf %s*' % tag  ) # Get rid of anything from before

#First need to remove r from rG, rA, rC, rU...
lines = open( pdbfile ).readlines()
fid = open( min_pdbfile, 'w')
for line in lines:
    if line[:4] == 'ATOM':
        fid.write( line[:16] + '   '+line[19:] )
fid.close()

# We'll need this params file.
system( 'cp %s/%s.prm .' % (TINKER_PARAMS, param_file ) )

# OK, convert
system( '%s/pdbxyz %s %s.prm' % ( TINKER_BIN, min_pdbfile, param_file ) )
assert( exists( xyzfile) )

###########################################################
# Minimize it.
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

############################################################
# analyze score components
command = '%s/analyze %s E  -k test.key' % ( TINKER_BIN, tag )
print( command )
lines = popen( command ).readlines()
fid = open( scorefile, 'w' )
for line in lines:
    if ( len( line ) > 2 and not line[1]=='#' ): fid.write( line )
fid.close()

############################################################
# Convert back to PDB
system( '%s/xyzpdb %s %s.prm' % ( TINKER_BIN, tag, param_file ) )

############################################################
# Cleanup
system( 'mv %s.pdb_2 %s.pdb'  % (tag,tag) )
system( 'rm -rf %s.seq* %s.xyz* test.key %s.prm ' % (tag,tag,param_file ) )
