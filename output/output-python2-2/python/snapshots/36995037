#!/usr/bin/python

from sys import argv,exit
from os.path import exists,basename,dirname,abspath
from os import system, popen, chdir, getcwd
import string
from random import random

##########################################################
TINKER_BIN = '~rhiju/src/tinker/bin/'
TINKER_PARAMS = '~rhiju/src/tinker/params/'
param_file = 'amber99'

CONSTRAIN = 0
if (argv.count( '-constrain' ) ):
    pos = argv.index( '-constrain' )
    del( argv[ pos ] )
    CONSTRAIN = 1

pdbfile = abspath( argv[1] )

WORKDIR = dirname( pdbfile )
if exists( '/scratch/USERS' ):
    WORKDIR = '/scratch/USERS/rhiju/'
    system( 'mkdir -p '+WORKDIR )
    assert( exists( WORKDIR) )

random_tag = '%06d' % int( 1000000 * random() )

min_pdbfile = param_file + '_' + random_tag + '_' + basename(pdbfile)
tag = min_pdbfile.replace('.pdb','')
xyzfile = min_pdbfile.replace('.pdb','.xyz')

CWD = getcwd()

# Let's get over to our working directory...
system( 'cp '+pdbfile+' '+WORKDIR+min_pdbfile )
chdir( WORKDIR )
system( 'cp '+TINKER_PARAMS+param_file+'.prm .' )

###########################################################
# Convert pdb to xyz -- this will figure out chainbreaks too.
PDBXYZ = '~rhiju/python/pdb2xyz.py'
#assert( exists( PDBXYZ ) )
system( PDBXYZ+' '+min_pdbfile )

############################################################
# How about superimposing?
#Is there an obvious native?
found_native = 0
pos = pdbfile.index( 'chunk' )
if (pos > 0 ):
    rna_name = pdbfile[pos:(pos+13)]
    native_pdb = '/work/rhiju/projects/rna_new_benchmark/bench_final/%s_RNA.pdb' % rna_name
    if exists( native_pdb ):
        found_native = 1
        native_xyz = native_pdb.replace('.pdb','.xyz')
        if not exists( native_xyz ):
            system( '~rhiju/python/pdb2xyz.py '+native_pdb )

if found_native:
    rms_file = pdbfile.replace( '.pdb', '.rms.txt' )
    command = TINKER_BIN+'superpose '+native_xyz+' '+TINKER_PARAMS+param_file+'.prm'+' '+tag+' '+TINKER_PARAMS+param_file+' 1,0,0 n u n 0.0 > '+rms_file+' 2> /dev/null'
    print( command )
    system( command )

    rms_line = popen( 'tail -n 1 '+rms_file ).readlines()[0]
    print string.split(rms_line)[-1], ' ==> ', file

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

scorefile = dirname(abspath(pdbfile))+'/minimize_'+basename(pdbfile).replace('.pdb','.sc')
fid = open( scorefile, 'w' )
for line in lines:
    if ( len( line ) > 2 and not line[1]=='#' ): fid.write( line )
fid.close()

# superimpose minimized pdb
if found_native:
    rms_file = dirname( pdbfile ) +  '/' + 'minimize_'+ basename(pdbfile).replace( '.pdb', '.rms.txt' )
    command = TINKER_BIN+'superpose '+native_xyz+' '+TINKER_PARAMS+param_file+'.prm'+' '+tag+' '+TINKER_PARAMS+param_file+' 1,0,0 n u n 0.0 > '+rms_file+' 2> /dev/null'
    print( command )
    system( command )

    rms_line = popen( 'tail -n 1 '+rms_file ).readlines()[0]
    print string.split(rms_line)[-1], ' ==> ', file


############################################################
# Convert back to PDB
system( '%s/xyzpdb %s %s.prm' % ( TINKER_BIN, tag, param_file ) )

############################################################
# Cleanup
system( 'mv %s.pdb_2 %s/%s'  % (tag,dirname(pdbfile),'minimize_'+basename( pdbfile ) ) )
system( 'rm -rf test.key %s* ' % ( random_tag ) )

chdir( CWD )

