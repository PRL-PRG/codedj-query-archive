#!/usr/bin/python

## setup for frag-picking

from phil import *

base_dir = argv[1]
round = int(argv[2]) ## 1 or 2

master = argv.count('-master')

if master and round == 2:
    ## debug the vall files
    ## assert that they all have the same size
    for dir in glob('%s/?????/'%base_dir):
        lines = map(string.split,popen('wc %s/?????/?????.homolog_vall'\
                                       %dir).readlines())
        n = 0
        for line in lines[:-1]:
            if n:
                assert int(line[0]) == n
            else:
                n = int(line[0])
        print '[CONFIRMED] all homolog_vall files have same size:',\
              string.split(dir,'/')[-2]

if master:
    ## drive all the runs
    hosts = whips_by_usage(4.0)

    MOD = len(hosts)*2

    for i in range(len(hosts)):
        host = hosts[i]

        for mod in range(2*i,2*i+2):

            logfile = '%s/make_frags_round%d_%d.log'%(base_dir, round, mod)

            #In case we want to do another round without sam...
            samoption = ' '
            if argv.count('-nosam'):
                samoption = '-nosam'

            cmd = 'ssh %s "( nice -n +19 python python/make_fragments.py %s %d %d %d %s > %s 2> %s.err & )"'\
                  %( host, base_dir, round, mod, MOD, samoption, logfile, logfile )
            print cmd
            system(cmd)

else:
    ## running on a single whip
    #assert len(argv) == 5
    base_dir = argv[1]
    round = int(argv[2])
    mod = int(argv[3])
    MOD = int(argv[4])

    big_count = 0

#    MF_EXE = '/users/pbradley/nnmake/08.02.05/nnmake/make_fragments_whip.pl'
#    NN_EXE = '/users/pbradley/nnmake/08.02.05/nnmake/pNNMAKE.gnu'
    MF_EXE = '/work/tex/fragpicker/nnmake/make_fragments.pl'
    NN_EXE = '/work/tex/fragpicker/nnmake/pNNMAKE.gnu'

    dir_list = glob('%s/?????/'%base_dir)
    dir_list.sort()

    print dir_list

    for target_dir in dir_list:
        id = string.split(target_dir,'/')[-2]
        hom_fasta_files = glob('%s/?????.fasta'\
                               %(target_dir))
        hom_fasta_files.sort()

        print hom_fasta_files
        assert( len( hom_fasta_files) == 1)

        dir = target_dir
        for file in hom_fasta_files:
            big_count += 1

            hom_id = id

            if round == 1:
                frag_files = glob('%s/ch?????0?_05.200_v1_3'%dir)
                if len(frag_files) == 2:
                    print 'already done:',id,hom_id
                    continue
            else:
                assert round == 2
                frag_files = glob('%s/ch?????0?_05.200_v1_3'%dir)
                if len(frag_files) != 2:
                    print 'missing:',id,hom_id
                    continue

            if big_count % MOD == mod:
                chdir(dir)

                if round == 1:
                    if argv.count('-nosam'):
                        cmd = '%s -xx ch -verbose -id %s -nohoms -nocleanup -nosam %s.fasta > mf.out 2> mf.err'\
                              %(MF_EXE,hom_id,hom_id)
                    else:
                        cmd = '%s -xx aa -verbose -id %s -nohoms -nocleanup %s.fasta > mf.out 2> mf.err'\
                              %(MF_EXE,hom_id,hom_id)

                else:
                    frag_files = glob('%s/aa?????0?_05.200_v1_3'%dir)
                    if len(frag_files) == 2:
                      print 'already done:',id,hom_id
                      continue

                    cmd = '%s aa %s %s > mf.aa.out 2> mf.aa.err'\
                          %(NN_EXE,hom_id[:4],hom_id[4])

                print dir,big_count,cmd
                system(cmd)



