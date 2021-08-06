#! /usr/bin/python

from phil import *
import sys
from os import system
from os.path import basename,abspath,exists
from glob import glob

def Help():
    print '\n This file reads in a list of PDB files and generates the phaser script for batch PHASER runs on the clusters\n'
    print '\nUsage: <pdbs> <input mtz file> \n'
    exit()

if len(argv) < 2:
    Help()

files = sys.argv[1:-1]
mtzfile =  sys.argv[-1]

if not mtzfile[-3:] == 'mtz':
    print
    print ' Must give mtz file at end!'
    Help()

condor_file = open('jobPHASER','w')

condor_file.write('universe     = vanilla\n')
condor_file.write('\n')
condor_file.write('Notify_user  = rhiju@u.washington.edu\n')
condor_file.write('notification = Error\n')
condor_file.write('\n')
condor_file.write('Log          = condorscript.log\n')
condor_file.write('\n')
condor_file.write('Executable   = ./run_phaser.py\n')
condor_file.write('\n')
condor_file.write('GetEnv       = True\n')
condor_file.write('\n')
condor_file.write('Error   = logerr\n')
condor_file.write('Output  = logout\n')
condor_file.write('\n')

command = 'cp ~/python/run_phaser.py .'
system(command)

command = 'chmod 777 run_phaser.py .'
system(command)

for file in files:
    if not exists(file): # Need to use glob
        print 'Using glob... '
        globfiles = glob( file )
    else:
        globfiles = [file]

    for globfile in globfiles:
        print 'Doing ... '+globfile

        data_file = globfile[:-4]+".script"
        data = open(data_file,'w')


        outdir = globfile[:-4]+"_out/"
        command = "mkdir -p "+outdir
        system(command)

        file = abspath( globfile ).replace('/work/','/users/')
        mtzfile = abspath( mtzfile ).replace('/work/','/users/')
        outdir = abspath( outdir ).replace('/work/','/users/')
        data_file = abspath( data_file ).replace('/work/','/users/')

        file = abspath( globfile ).replace('/Users/','/users/')
        mtzfile = abspath( mtzfile ).replace('/Users/','/users/')
        outdir = abspath( outdir ).replace('/Users/','/users/')
        data_file = abspath( data_file ).replace('/Users/','/users/')

        data.write('MODE MR_AUTO\n')
        data.write('HKLIN "%s"\n'%(mtzfile))
        data.write('LABIN  F=F_pk SIGF=SIGF_pk\n')
        data.write('TITLE [No title given]\n')
        data.write('COMPOSITION PROTEIN MW 9200 NUMBER 2\n')
        data.write('TOPFILES 1\n')
        data.write('ENSEMBLE ensemble1 &\n')
        data.write('    PDBFILE "%s" &\n'%(file))
        data.write('    RMS 1.50\n')
        data.write('SEARCH ENSEMBLE ensemble1 &\n')
        data.write('    NUMBER 2\n')
    #    data.write('PACK 5\n')
        data.write('ROOT "%s"\n'%( outdir + '/'+basename(globfile[:-4])))
        data.write('END\n')
        data.close()

        log_file = outdir + '/' + basename(globfile)[:-4] + '.log'
        condor_file.write(' Arguments = %s %s\n' % (data_file,log_file))
        condor_file.write('Queue 1 \n\n')

condor_file.close()
