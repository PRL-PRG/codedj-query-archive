#!/usr/bin/python

from sys import argv
from os import system,chdir,getcwd
from os.path import abspath
import string

QUEUENUM = 16
NSTRUCT = 1000
do_all = 1 # AS opposed to just the first job.



setup_directories = 1

submit_to_cluster = 0
if argv.count('-submit'):
    submit_to_cluster = 1
    setup_directories = 0

getresults_from_cluster = 0
if argv.count('-getresults'):
    getresults_from_cluster = 1
    setup_directories = 0

getfilters = 0
if argv.count('-getfilters'):
    getfilters = 1
    setup_directories = 0

clean_directories = 0
if argv.count('-clean'):
    clean_directories = 1
    setup_directories = 0


getcontacts = 0
if argv.count('-getcontacts'):
    getcontacts = 1
    setup_directories = 0

if argv.count('-queue'):
    pos = argv.index('-queue')
    del( argv[pos] )
    QUEUENUM = int( argv[pos] )
    del( argv[pos] )


if argv.count('-nstruct'):
    pos = argv.index('-nstruct')
    del( argv[pos] )
    NSTRUCT = int( argv[pos] )
    del( argv[pos] )

file = argv[1]

clusterlist = [ 'maat',  'bes', 'hapy', 'niau', 'seth',  'yah', 'gebb', 'ptah', 'apep',  'set',   'ra',  'dua', 'atum']

try:
    clustername = argv[2]
    assert( clusterlist.count( clustername ) )
except:
    foundcluster = 0
    for clustername in clusterlist:
        if file.find(clustername)>=0 :
            foundcluster = 1
            break
    if not foundcluster:
        print 'must specify cluster name in submit file name or separate'


condorfilename = 'jobTEST'+clustername


testdir = 'test_' + file.replace('.submit','')

if setup_directories:
    data = open(file,'r')

    command = 'mkdir '+testdir
    print command
    system(command)

    command = 'cp /users/rhiju/paths.txt '+testdir
    print command
    system(command)

    chdir(testdir)

    #Prepare a condor submit file
    condorid = open(condorfilename,'w')
    condorid.write('universe     = vanilla\n\n')
    condorid.write('Notify_user  = rhiju@u.washington.edu\n')
    condorid.write('notification = Error\n\n')

    condorid.write('Requirements = ( Memory > 248 )\n');
    condorid.write('Log          = condorscript.log\n');
    condorid.write('Executable   = /users/rhiju/rosetta++/rosetta.gcc\n\n')

    line = data.readline()

    keeptesting = 1
    count = 1
    while line and keeptesting:

        # Input files first
        while line and line[:10] != 'inputfiles': line = data.readline()
        if not line: break
        inputfiles_string = string.split(line,'=')[-1][:-1]
        inputfiles = string.split(inputfiles_string,';')

        for inputfile in inputfiles:
            command = 'rsync -avz '+inputfile+' .'
            print(command)
            system(command)


        line = data.readline()
        while line and line[:9] != 'arguments': line = data.readline()
        if not line: break

        args = string.split(line,'=')[-1][:-1]

        if do_all: args=args.replace(' xx',' %02d' % count)

        args = args.replace('-output_silent_gz','')
    #    args = args.replace('increase_cycles 10','increase_cycles 0.1')
        args = args.replace('nstruct 30','nstruct %d' % NSTRUCT)

    #    command = '/users/rhiju/rosetta++/rosetta.gcc '+args
    #    print(command)
    #    system(command)

        condorid.write('arguments = %s\n' % args)

        condorid.write('Queue %d\n\n' % QUEUENUM)

        if not do_all:
            keeptesting = 0
        count=count+1

    condorid.close()

    fullpath = abspath( getcwd())
    fullpath = fullpath.replace('/work/','/users/')
    command = 'ssh %s mkdir -p %s'  % (clustername,fullpath)
    print command
    system(command)

    command = 'rsync -avz . %s:%s'  % (clustername,fullpath)
    print command
    system(command)

if submit_to_cluster:
    chdir(testdir)
    fullpath = abspath( getcwd())
    fullpath = fullpath.replace('/work/','/users/')
    command = 'ssh %s "cd %s; condor_submit %s"'  % (clustername,fullpath,
                                                     condorfilename)
    print command
    system(command)


if getresults_from_cluster:
    chdir(testdir)
    fullpath = abspath( getcwd())
    fullpath = fullpath.replace('/work/','/users/')
    command = 'rsync -avz %s:%s/ .'  % (clustername,fullpath)
    print command
    system(command)



if clean_directories:
    command = 'rm -rf '+testdir
    print(command)
    system(command)

    fullpath = abspath( getcwd())
    fullpath = fullpath.replace('/work/','/users/')
    command = 'ssh %s rm -rf %s'  % (clustername,fullpath)
    print command
    system(command)


if getfilters:
    data = open(file,'r')

    #Prepare a condor submit file
    lines = open(testdir+'/'+condorfilename,'r').readlines()

    for line in lines:
        cols = string.split(line)
        if not cols.count('arguments'): continue

        prefixindex = cols.index('-protein_name_prefix')
        prefix = cols[prefixindex+1]

        fourlettercode = cols[3]
        fivelettercode = cols[3]+cols[4]
        twolettercode = cols[2]

        outfile = testdir+'/'+twolettercode+fourlettercode+'.out'
        filterfile = prefix+fivelettercode+'filters.txt'
        command = '/users/rhiju/python/figure_out_score_filters.py ' + outfile + ' > ' + filterfile
        print(command)
        system(command)



if getcontacts:
    data = open(file,'r')

    #Prepare a condor submit file
    lines = open(testdir+'/'+condorfilename,'r').readlines()

    chdir(testdir)

    for line in lines:
        cols = string.split(line)
        if not cols.count('arguments'): continue

        prefixindex = cols.index('-protein_name_prefix')
        prefix = cols[prefixindex+1]

        fourlettercode = cols[3]
        fivelettercode = cols[3]+cols[4]
        twolettercode = cols[2]

        outfile = twolettercode+fourlettercode+'.out'
        command = 'rm blah*; /users/rhiju/rosetta_centroidinformation/rosetta.gcc -s %s -score -scorefile blah -centroid_information -all -silent_input -new_reader -refold' % outfile
        print(command)
        system(command)

        command = 'mv CentroidInformation.txt '+prefix+fivelettercode+'CentroidInformation.txt'
        print(command)
        system(command)



