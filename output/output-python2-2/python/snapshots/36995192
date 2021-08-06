##

from phil import *


SLEEP = 5*60
base_dir = argv[1]

if argv.count('-final'):
    ## final pass, after running w/o -final arg once

    id_list = map(lambda x:string.split(x,'/')[-2],
                  glob('%s/?????/?????.fasta'%base_dir))

    ## figure out which blast files to use for each id
    MAX_THRESHOLD = 0.80
    MIN_THRESHOLD = 0.4

    for id in id_list:
        #if id != '1fna_':continue ### CHANGE ME

        cmd = "awk '(/filter_threshold/ && $4<= 60){print FILENAME,$0}' %s/%s//?????.fasta.?.blast.pick_homs.log | sort -n +2 -r"\
              %(base_dir,id)
        #print cmd
        lines = map(string.split, popen(cmd).readlines())
        if not lines:
            file = '%s/%s/%s.fasta.7.blast.pick_homs.log'%(base_dir,id,id)
            threshold = 0
        else:
            file = lines[0][0]
            threshold = float(lines[0][2])
        for line in lines:
            #print line
            t = float(line[2])
            if t < MIN_THRESHOLD:
                if threshold > MAX_THRESHOLD:
                    print 'WARNING: big gap between blast files!'
                else:
                    break
            file = line[0]
            threshold = t
        blast_file = file[:-14]
        assert exists(blast_file)
        print id,threshold,blast_file

        fasta_file = '%s/%s/%s.fasta'%(base_dir,id,id)
        assert exists(fasta_file)

        logfile = blast_file+'.pick_homs_real.log'


        cmd = 'nice -n +19 /users/pbradley/python/pick_homs.py %s %s > %s 2> %s'\
              %( fasta_file, blast_file, logfile, logfile )
        print cmd
        system(cmd)


elif argv.count('-master'):
    ## run on pick_homs on all the blast files

    id_list = map(lambda x:string.split(x,'/')[-2],
                  glob('%s/?????/?????.fasta'%base_dir))

    while id_list:
        hosts = whips_by_usage(2.0)

        for host in hosts:
            log(`id_list`)
            if not id_list:break
            id = id_list[0]
            del id_list[0]

            logfile = '%s/pick_homs_wrapper_%s.log'%(base_dir,id)

            cmd = 'ssh %s "(nice -n +19 python python/pick_homs_wrapper.py %s %s > %s 2> %s.err &)"'\
                  %(host, base_dir, id, logfile, logfile )
            print cmd
            system(cmd)

        sleep(SLEEP)

else:

    base_dir= argv[1]
    id = argv[2]

    dir = base_dir+'/'+id+'/'
    fasta_file = dir+id+'.fasta'
    assert exists(fasta_file)
    blast_files = glob('%s/%s.fasta.?.blast'%(dir,id))
    print len(blast_files)

    for file in blast_files:
        logfile = file+'.pick_homs.log'
        cmd = 'nice /users/pbradley/python/pick_homs.py %s %s -NOPICK > %s 2> %s.err'\
              %(fasta_file,file,logfile,logfile)
        print cmd
        system(cmd)



