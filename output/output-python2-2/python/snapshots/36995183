#!/usr/bin/python

from sys import argv
from os import system,getcwd
from os.path import basename,dirname,abspath

pdbfiles = argv[1:]


#########################
# Condor id file
#########################
condorid = open('jobDESIGN','w')
condorid.write('universe     = vanilla\n')
condorid.write('\n')
condorid.write('Notify_user  = rhiju@u.washington.edu\n')
condorid.write('notification = Error\n')
condorid.write('\n')
condorid.write('Log          = condorscript.log\n')
condorid.write('\n')
condorid.write('Executable   = /users/rhiju/rosetta++/rosetta.gcc\n')
condorid.write('\n')
condorid.write('GetEnv       = True\n')
condorid.write('\n')
condorid.write('Error   = logerr\n')
condorid.write('Output  = logout\n')
condorid.write('\n')

############################
# Setup working directories
############################
cwd = getcwd()
for pdbfile in pdbfiles:
    working_dir = cwd + '/designs/'+ basename(pdbfile).replace('.pdb','')

    command = 'mkdir -p ' + working_dir
    print command
    system(command)

    command = 'cp %s %s' % (pdbfile, working_dir)
    print command
    system(command)

    command = 'cp /work/rhiju/paths.txt '+working_dir
    print command
    system(command)

    cluster_working_dir = abspath(working_dir).replace('/work/','/users/')
    condorid.write( 'Initialdir     = %s\n' % cluster_working_dir )
    condorid.write( 'arguments =  -design -s %s -fixbb -ndruns 40 -pdbout %s -ex1 -ex2 -extrachi_cutoff 12\n' %\
                    (basename(pdbfile),basename(pdbfile)) )
    condorid.write( 'Queue 1\n\n' )


condorid.close()

