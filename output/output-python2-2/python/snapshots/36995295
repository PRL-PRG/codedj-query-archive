##

from phil import *
from os.path import dirname

scop_file = '/userspbradley/scop/dir.cla.scop.txt'
vall_scop_file = '/users/pbradley/scop/vall.scop.1.65.2001-02-02'


base_dir = argv[1]
id_list = map(lambda x:string.split(x,'/')[-2],
              glob('%s/?????/?????.fasta'%base_dir))
id_list.sort()

#id_list = ['t197_']

for id in id_list:
    pdbcode = id[0:4]
    chain = id[4]

    # First the natives.
    pdbfile       = base_dir+'natives/'+pdbcode+'.pdb'
    secstructfile = base_dir+'natives/'+id+'.secstruct'

    cmd = '/users/rhiju/python/pdb_to_secstruct.py %s %s > %s' % (pdbfile, chain, secstructfile)
    print(cmd)
    system(cmd)

    psipredfile = base_dir+'natives/'+id+'.psipred_ss2.perfect'
    cmd = 'python /users/rhiju/python/pdb_to_psipred_ss2.py %s %s > %s' % (pdbfile, chain, psipredfile)
    print(cmd)
    system(cmd)


    #Now any available homolog fragment files (do 3mers).
    fragfiles = glob('%s/%s/????_/aa?????03_05.200_v1_4'% (base_dir,id) )
    fragfiles.sort()
    for fragfile in fragfiles:
        secstructprobfile = fragfile+'.secstructprob'

        cmd = 'python /users/rhiju/python/fragfile_to_secstructprob.py %s %s > %s' % (fragfile, '_', secstructprobfile)
        print(cmd)
        if not exists( secstructprobfile):
            system(cmd)



    #Use the global alignment to then combine the secondary structure probabilities into one big file, useful for plotting.
    outputfile = base_dir+'/align_files/'+id+'.secstructprob.align'
    fid = open(outputfile, 'w')
    alignfile = base_dir+'/align_files/'+id+'.align_extended'
    print alignfile
    lines = open( alignfile, 'r').readlines()

    def outputcrap(fid,sequence,secstructprobfile):
        if exists(secstructprobfile):
            problines = open( secstructprobfile).readlines()
            probpos = 0
            for k in range( len(sequence)):
                alignpos = k+1
                if sequence[k] == '-' or probpos >= len(problines):
                    fid.write( '%d 0.000 0.000 0.000\n' % alignpos)
                else:
                    fid.write( '%d %s\n' % (alignpos, string.join( string.split(problines[probpos])[-3:] ))); #Need to cut out last three numbers.
                    probpos += 1

        else:
            for k in range( len(sequence)):
                alignpos = k+1
                fid.write( '%d 0.000 0.000 0.000\n' % alignpos)


    #Again put the native in there first. NOTE: Assumes native corresponds to first sequence.
    sequence = string.split(lines[0])[1]
    print fid, sequence, psipredfile
    outputcrap(fid, sequence, psipredfile)

    #Now the fragment secondary structure fractions.
    for line in lines:
        sequence = string.split(line)[1]
        fastafile = string.split(line)[2]
        secstructprobfiles = glob('%s/aa?????03_05.200_v1_4.secstructprob'% dirname(fastafile))
        #        assert( len(secstructprobfiles)> 0)
        if len(secstructprobfiles)>0:
            secstructprobfile = secstructprobfiles[0]
            outputcrap( fid, sequence, secstructprobfile)
        else:
            print "NO SECSTRUCTPROB FILE!!!!!"
            outputcrap( fid, sequence, 'xxx')

