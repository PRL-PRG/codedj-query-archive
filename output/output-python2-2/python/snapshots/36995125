#! /usr/bin/python2

from phil import *
import sys
from os import system
from os.path import basename,abspath,exists
from glob import glob

def Help():
    print '\n This file reads in a list of PDB files and generates the phaser script for batch PHASER runs on the clusters\n'
    print '\nUsage: '+argv[0]+' <starting phaser script> <input mtz file> <pdbs> [<N>]\n'
    print '   if you specify an integer N, then that is how many phaser jobs are bundled into'
    print '   one condor job.'
    exit()

if len(argv) < 2:
    Help()

for pos in range( len(argv) ):
    try:
        NUMJOBS = int( argv[pos] )
        del( argv[pos] )
        break
    except:
        NUMJOBS = 1
        continue

phaser_append = 0
if argv.count( '-append' ):
    pos = argv.index( '-append' )
    del( argv[pos] )
    phaser_append = 1
    # print 'APPENDING TO PHASER SCRIPT!'

phaser_script = sys.argv[1]
mtzfile =  sys.argv[2]
files = sys.argv[3:]

if not phaser_script[-6:] == 'script':
    print
    print ' Must give a starting phaser script as first argument!'
    Help()

if not mtzfile[-3:] == 'mtz':
    print
    print ' Must give mtz file as second argument!'
    Help()

if phaser_append:
    condor_file = open('jobPHASER','a')
else:
    condor_file = open('jobPHASER','w')

count = 0
if not phaser_append:
    condor_file.write('universe     = vanilla\n')
    condor_file.write('\n')
    condor_file.write('Notify_user  = rhiju@u.washington.edu\n')
    condor_file.write('notification = Error\n')
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

    command = 'rsync -az ~/phaser . '
    system(command)

    command = 'chmod 777 run_phaser.py .'
    system(command)

condor_file.write('Arguments = ')

lines = open( phaser_script ).readlines();

finished = 0

globfiles = []
for file in files:
    if not exists(file): # Need to use glob
        print 'Using glob... '
        globfiles += glob( file )
    else:
        globfiles += [file]




for globfile in globfiles:

    outdir = globfile[:-4]+"_out/"
    command = "mkdir -p "+outdir
    system(command)

    log_file = outdir + '/' + basename(globfile)[:-4] + '.log'
    sol_file = outdir + '/' + basename(globfile)[:-4] + '.sol'

    if exists( log_file ):
        if exists( sol_file ):
            print 'NOT DOING ',globfile,'. (Already seeing log file ',log_file,'.)'
            continue
        else:
            system( 'rm '+log_file)
    else:
        print 'Doing ... '+globfile

    data_file = globfile[:-4]+".script"
    data = open(data_file,'w')

    # Don't want absolute paths...
    #        file = abspath( globfile ).replace('/work/','/users/')
    #        mtzfile = abspath( mtzfile ).replace('/work/','/users/')
    #        outdir = abspath( outdir ).replace('/work/','/users/')
    #        data_file = abspath( data_file ).replace('/work/','/users/')

    for line in lines:
        if line.count('HKLIN'):
            line = 'HKLIN "%s"\n'% mtzfile

        if line.count('PDBFILE'):
            line = '    PDBFILE "%s" &\n'%(globfile)

        if line.count('ROOT'):
            line = 'ROOT "%s"\n'%( outdir + '/'+basename(globfile[:-4]))

        line = line.replace('/work/','/users/')

        data.write( line )


    count += 1
    condor_file.write(' %s %s ' % (data_file,log_file))
    if ( count % NUMJOBS == 0 ):
        condor_file.write('\nQueue 1\n')
        if ( globfile == globfiles[-1] ):
            finished = 1
        else:
            condor_file.write( '\nArguments = ')


if not finished and (count > 0):
    condor_file.write('\nQueue 1\n')
condor_file.write('\n')
condor_file.close()
