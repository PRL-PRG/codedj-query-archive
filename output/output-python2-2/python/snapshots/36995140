#!/usr/bin/python


from sys import argv,exit
import string
from os import system
from os.path import exists



def Help():
    print '\n This file reads in a list of PDB files and generates a script for batch ARPWARP runs on SYD'
    print '\nUsage: '+argv[0]+' <input mtz file> <input pir file> <N in ASU> <pdbs> \n'
    print ' Note that you need to have a bunch of paths setup right, a good .bashrc, etc...!'
    print
    exit()

if len(argv) < 2:
    Help()

mtzfile = argv[1]
pirfile = argv[2]

try:
    N_ASU = int( argv[3] )
except:
    Help()

pdbfiles = argv[4:]


arpwarp_dir = './'
condorid = open( arpwarp_dir+'jobARPWARP', 'w' )
condorid.write('universe     = vanilla\n')
condorid.write('\n')
condorid.write('Notify_user  = rhiju@u.washington.edu\n')
condorid.write('notification = Error\n')
condorid.write('\n')
condorid.write('Executable   =  ./run_classic_arp_warp.py\n')
condorid.write('\n')
condorid.write('GetEnv       = True\n')
condorid.write('\n')
condorid.write('Output = logtmp.out\n')
condorid.write('Error = logtmp.err\n')
condorid.write('\n')


system( 'cp  ~rhiju/python/run_classic_arp_warp.py .' )
system( 'chmod 777 run_classic_arp_warp.py' )

##############################################################
# OK, do the setup!!
##############################################################
for pdbfile in pdbfiles:

    print 'Setting up condor queue command for: ', pdbfile

    title = 'arpwarp autotracing'
    workdir = pdbfile.replace('.pdb','')+'/'

    condorid.write('arguments = %s %s %s %d \n' \
                   % (mtzfile, pirfile, pdbfile, N_ASU )  )
    condorid.write('Queue 1\n\n')



condorid.close()
