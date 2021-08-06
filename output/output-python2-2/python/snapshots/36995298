## get .outfiles from hosts

from phil import *



base_dir = argv[1]

lines = map(lambda x: string.split(x)[0],open('%s/mapping.txt'%base_dir,
                              'r').readlines())

for id in lines:
    align_file = '%s/align_files/%s.align_extended'%(base_dir,id)
    coords_file = '-'
    #assert exists(coords_file)

    log_file = glob('%s/%s/%s.fasta.?.blast.pick_homs_real.log'\
                    %(base_dir,id,id))


    assert len(log_file) == 1
    blast_file = log_file[0][:-19]
    assert exists(blast_file)

    if not exists(align_file):
        cmd = 'mkdir %s/align_files' % base_dir
        system(cmd)
        outfiles = glob('%s/%s/h???_/h???_.fasta'\
                        %(base_dir,id))
        outfiles.sort()
        print id,len(outfiles)
        cmd = '/users/rhiju/python/blast2align.py -extend %s %s %s > %s 2> %s.err'\
              %(blast_file, coords_file, string.join(outfiles), align_file,
                align_file)
        print cmd
        system(cmd)
