#!/usr/bin/python

## make pseudonatives for each homolog
##
## rhiju, feb 2006 .. quick and dirty.
## It would be much smarter to distribute this onto the whips a la phil's other routines
##

from phil import *
from os import chdir,getcwd
from os.path import dirname, basename

base_dir = argv[1]
master = argv.count('-master')

def stripchain(actualpdbname, actualpdbname_nochain):
    lines = open(actualpdbname,'r').readlines()
    out = open(actualpdbname_nochain,'w')
    for i in range( len(lines)):
        line = lines[i]
        if line.count('ATOM'):
            line = line[0:21]+' '+line[22:]
            out.write(line)
    out.close()

if master:
    ## drive all the runs
    hosts = whips_by_usage(4.0)

    MOD = len(hosts)*2

    for i in range(len(hosts)):
        host = hosts[i]

        for mod in range(2*i,2*i+2):

            logfile = '%s/quickrosettatest_%s.log'%(base_dir, host)

            cmd = 'ssh %s "( nice -n +19 python python/pick_homs_quickrosettatest.py %s %d %d > %s 2> %s.err & )"'\
                  %( host, base_dir, mod, MOD, logfile, logfile )
            print cmd
            system(cmd)

            #cmd = 'ssh %s "killall python"' % host
            #system(cmd)
            #cmd = 'ssh %s "killall rosetta"' % host
            #system(cmd)

else:
    ## running on a single whip
    #assert len(argv) == 5
    base_dir = argv[1]
    mod = int(argv[2])
    MOD = int(argv[3])
    big_count = 0

    lines = map(lambda x: string.split(x)[0],open('%s/mapping.txt'%base_dir,
                                                  'r').readlines())

    frags_dir = base_dir+'/frags/'
    startingdirectory = getcwd()

    EXE = '/users/rhiju/rosetta++/rosetta.gcc'

#    lines = ['1a19A','1a32_']

    for id in lines:
        align_file = '%s/align_files/%s.align_extended'%(base_dir,id)
        coords_file = '-'

        fasta_list = glob(frags_dir+'/*'+id+'*.fasta.gz');
#        pdb_list = map(lambda x: string.split(x,'/')[-1], pdb_list);
        fasta_list.sort()

        for fastafile in fasta_list:
            big_count += 1

            if big_count % MOD == mod :
                wheretorun = base_dir+'/test/';
                chdir( wheretorun )
                namefile = string.split( basename(fastafile),'.')[0]   # E.g., h001_

                pdbcode = id[0:4]
                stupidchainid = id[4]
                prefix = namefile[0:7]
                twolettercode = prefix[-3:-1]

                cmd = 'nice -n +19 '+EXE+' '+twolettercode+' '+pdbcode+' '+stupidchainid+\
                      ' -nstruct 1 -increase_cycles 0.1 -no_filters -protein_name_prefix '+prefix+\
                      ' -frags_name_prefix '+prefix
                print(cmd)
                system(cmd)

        chdir(startingdirectory)
