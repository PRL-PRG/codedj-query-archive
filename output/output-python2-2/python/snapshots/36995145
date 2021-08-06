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
condorid.write('Executable   =  ./run_flex_warp.py\n')
condorid.write('\n')
condorid.write('GetEnv       = True\n')
condorid.write('\n')
condorid.write('Output = logtmp.out\n')
condorid.write('Error = logtmp.err\n')
condorid.write('\n')

wrapper_name = arpwarp_dir+'run_flex_warp.py'
silly_arpwarp_wrapper = open( wrapper_name,'w')
silly_arpwarp_wrapper.write('#!/usr/bin/python\n')
silly_arpwarp_wrapper.write('\n')
silly_arpwarp_wrapper.write('import sys\n')
silly_arpwarp_wrapper.write('import string\n')
silly_arpwarp_wrapper.write('from os import system\n')
silly_arpwarp_wrapper.write('\n')
silly_arpwarp_wrapper.write('system( \'source ~/.bashrc\' )\n')
silly_arpwarp_wrapper.write('\n')
silly_arpwarp_wrapper.write('args = sys.argv[1:]\n')
silly_arpwarp_wrapper.write('\n')
silly_arpwarp_wrapper.write('command = string.join( [\'CAutoPyWARP.py \'] + args )\n')
silly_arpwarp_wrapper.write('print command\n')
silly_arpwarp_wrapper.write('system( command )\n')
silly_arpwarp_wrapper.close()
system( 'chmod 777 ' + wrapper_name )

##############################################################
# OK, do the setup!!
##############################################################
for pdbfile in pdbfiles:

    print 'Setting up condor queue command for: ', pdbfile

    title = 'arpwarp autotracing'
    workdir = pdbfile.replace('.pdb','')+'/'

    condorid.write('arguments = --datafile=%s --modelin=%s  --seqin=%s:%d --title=%s --workdir %s --mode=PDBStart --NCS-cross-completion --New-chain-tracing --snow-docking TopologyVectorThenRotamerDensityDocker --fp=F --sigfp=SIGF \n' \
                   % (mtzfile, pdbfile, pirfile, N_ASU, title, workdir )  )
    condorid.write('Queue 1\n\n')



condorid.close()
