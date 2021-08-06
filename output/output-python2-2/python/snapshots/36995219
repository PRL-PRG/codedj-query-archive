##!/usr/bin/python
##
##

from phil import *
from time import sleep
from os.path import abspath

ej_list = [ [1e-12, 1], ## 0
            [1e-9, 1],  ## 1
            [1e-6, 1],  ## 2
            [1e-3, 1],  ## 3
            [1e-9, 2],  ## 4
            [1e-6, 2],  ## 5
            [1e-3, 2],  ## 6
            [1e-6, 3],  ## 7
            [1e-3, 3],  ## 8
            [1e-3, 5] ] ## 9

BLAST = '/users/pbradley/shareware/new_blast/blast-2.2.11/bin/blastpgp'
DB = '/scratch/Phil/genomes/nr'

MAX_LOAD = 2.0
SLEEP = 5*60


if argv[-1] == '-master':
    base_dir = abspath( argv[1] )
    to_do = map(lambda x:string.split(x,'/')[-2],
              glob('%s/?????/?????.fasta'%base_dir))

    to_do.sort()

    while to_do:
        log(`to_do`)
        hosts = whips_by_usage( MAX_LOAD )

        for host in hosts:
            if not to_do:break

            id = to_do[0]
            del to_do[0]

            logfile = '%s/blast_log_%s.log'%(base_dir,id)
            cmd = 'ssh %s "(nice -n +19 python python/pick_homs_blast.py %s %s  > %s 2> %s.err &)"'\
                  %(host,base_dir,id,logfile,logfile)
            print cmd
            system(cmd)


        sleep( SLEEP )


else:
    base_dir = argv[1]
    id = argv[2]

    assert len(id) == 5

    ## get fasta file
    dir = '%s/%s/'%(base_dir,id)

    fasta_file = '%s/%s.fasta'%(dir,id)
    assert exists(fasta_file)

    ## blast fasta
    for ej in ej_list:
        round = ej_list.index(ej)
        print id,round

        e = ej[0]
        j = ej[1]

        blast_file = '%s.%d.blast'%(fasta_file,round)
        check_file = '%s.%d.check'%(fasta_file,round)
        pssm_file = '%s.%d.pssm'%(fasta_file,round)

        command = '%s -i %s -d %s -e %g -a 2 -I T -j %d -v 10000 -b 10000 -C %s -Q %s -o %s'\
              %( BLAST, fasta_file, DB, e, j, check_file, pssm_file, blast_file )
        print command
        system(command)

