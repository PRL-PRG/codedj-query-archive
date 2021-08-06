#!/usr/bin/python

from sys import argv
from os.path import exists
from os import system

infiles = argv[1:]

fid = open( 'jobMINI', 'w' )

fid.write('universe     = vanilla\n')
fid.write('\n')
fid.write('Notify_user  = rhiju@u.washington.edu\n')
fid.write('notification = Error\n')
fid.write('\n')
fid.write('Executable   = easy_target_test.linuxgccrelease\n')
fid.write('\n')
fid.write('GetEnv       = True\n')
fid.write('\n')
fid.write('JOB_START_DELAY = 10\n');

process_num  = -1
for file in infiles:

    native_file = 't311_native.pdb'
    assert( exists( native_file ) )
    constraints_file = 't0311_tether.constraints'
    assert( exists( constraints_file ) )


    OUTFILE_DIR = file+'_OUT'

    QUEUE_NUM = 20
    NSTRUCT = 50

    for queue in range( QUEUE_NUM):
        process_num += 1
        system( 'mkdir -p %s/%d' % (OUTFILE_DIR, process_num ) )

        silent_file = file.replace( '.pdb', '_relax.out' )

        command = 'arguments =  -database /scratch/USERS/rhiju/minirosetta_database /users/rhiju/minirosetta_database /work/rhiju/minirosetta_database -s %s -native %s -ex1 -ex2 -cst_relax -score::weights score13_env_hb.wts -nstruct %d -out:path %s/$(Process)/ -out::file::silent %s -cst_file %s ' %\
                  (  file, native_file, NSTRUCT, OUTFILE_DIR,silent_file, constraints_file )

        fid.write( command +'\n' )
        fid.write( 'Queue %d\n' % QUEUE_NUM )
        fid.write( '\n')

fid.close()
