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

            logfile = '%s/make_pseudonatives_%s.log'%(base_dir, host)

            cmd = 'ssh %s "( nice -n +19 python python/pick_homs_make_pseudonatives.py %s %d %d > %s 2> %s.err & )"'\
                  %( host, base_dir, mod, MOD, logfile, logfile )
            print cmd
            system(cmd)

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


    for id in lines:
        align_file = '%s/align_files/%s.align_extended'%(base_dir,id)
        coords_file = '-'

#        print base_dir+id+'/*/*.fasta'
        fasta_list = glob(base_dir+id+'/*/*.fasta');
        #        pdb_list = map(lambda x: string.split(x,'/')[-1], pdb_list);
        fasta_list.sort()

        for fastafile in fasta_list:
            big_count += 1

            if big_count % MOD == mod :
                wheretorun = dirname(fastafile);
                chdir( wheretorun )
                namefile = string.split( basename(fastafile),'.')[0]   # E.g., h001_

                new_pdb_name = wheretorun+'/'+namefile[0:4]+'.pdb'
                stupidchainid = id[4]

                actualpdbname = base_dir+'natives/'+id[0:4]+'.pdb'
                actualpdbname_nochain = base_dir+'natives/'+id[0:4]+'.nochain.pdb'
                if not exists(actualpdbname):
                    print 'no ',actualpdbname
                    continue

                stripchain(actualpdbname, actualpdbname_nochain)

                if exists(new_pdb_name) :
                    numlines = int(string.split(popen('wc '+ new_pdb_name).readlines()[0])[0])

                    if numlines>0:
                        print 'Already exists: %s' % new_pdb_name
                        continue
                    else:
                        print 'Empty file: %s' % new_pdb_name
                        cmd = 'rm '+new_pdb_name
                        print(cmd)
                        system(cmd)

                cmd = EXE+' xx '+namefile[0:4]+' _  -score -nstruct 1 -s '+actualpdbname_nochain+' -map_sequence '+align_file+' -paths /users/rhiju/python/paths_for_homologs.txt'
                print(cmd)
                system(cmd)

                mapped_pdb_name = wheretorun+'/xx'+id[0:4]+'.nochain_0001.pdb'
                cmd = 'mv  '+mapped_pdb_name+' '+new_pdb_name
                print(cmd)
                system(cmd)

        chdir(startingdirectory)
