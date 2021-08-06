#/usr/bin/python

from phil import *

def replacechain(actualpdbname, actualpdbname_new, chainid):
    lines = open(actualpdbname,'r').readlines()
    out = open(actualpdbname_new,'w')
    for i in range( len(lines)):
        line = lines[i]
        if line.count('ATOM'):
            line = line[0:21]+chainid+line[22:]
            out.write(line)
    out.close()

base_dir = argv[1]

id_list = map(lambda x:string.split(x,'/')[-2],
              glob('%s/?????/?????.fasta'%base_dir))
id_list.sort()

#s = 'abcdefghijklmnopqrstuvwxyz'
#assert len(s) == 26

old_frag_tag = "_05.200_v1_4"
new_frag_tag = "_05.200_v1_3"

#letters = {}
out = open('%s/mapping.txt'%base_dir,'w')
for i in range(len(id_list)):
    id = id_list[i]
#    out.write('%s %s\n'%(id,letters[id]))
    out.write('%s \n' % id)
out.close()


frag_dir = base_dir+'/frags/'
mkdir(frag_dir)

#id_list = ['1a19A']
nonative = argv.count('-nonative')

for id in id_list:

    dirs = glob('%s/%s/h???_/'%(base_dir,id))
    dirs.sort()

    for dir in dirs:
        hom_id = string.split(dir,'/')[-2]

        n = int(hom_id[1:4])
#        new_id = '%s%03d_'%(letter,n)
        prefix = 'hom%03d_' % n
        new_id = prefix + id

        ##
        for suffix in ['fasta','psipred_ss2']:
            file = '%s/%s.%s'%(dir,hom_id,suffix)
            assert exists(file)

            new_file = '%s/%s.%s'%(frag_dir,new_id,suffix)
            cmd = 'rsync -avz %s %s'%(file,new_file)
            print cmd
            system(cmd)

        if not nonative:
            for suffix in ['pdb']:
                file = '%s/%s.%s'%(dir,hom_id[:-1],suffix)
                assert exists(file)

                new_file = '%s/%s.%s'%(frag_dir,new_id[:-1],suffix)
                stupidchainid = id[4]
                if (stupidchainid == '_'):
                    stupidchainid = ' '

                replacechain(file, new_file, stupidchainid)

        ##
        for m in [3,9]:
            file = '%s/aa%s%02d%s'%(dir,hom_id,m,old_frag_tag)
            assert exists(file)

            new_file = '%s/%saa%s%02d%s'%(frag_dir,prefix,id,m,new_frag_tag)
            cmd = '/users/boinc/bin/reduce_fragment_library_size.pl %s > %s'%(file,new_file)
            print cmd
            system(cmd)



