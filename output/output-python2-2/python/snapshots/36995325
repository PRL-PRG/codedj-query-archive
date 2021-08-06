#!/usr/bin/python

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

print id_list

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

    dir = '%s/' % (id)

    for suffix in ['fasta','psipred_ss2']:
        file = '%s/%s.%s'%(dir,id,suffix)
        print file
        assert exists(file)

        new_file = '%s/%s.%s'%(frag_dir,id,suffix)
        cmd = 'rsync -avz %s %s'%(file,new_file)
        print cmd
        system(cmd)

    if not nonative:
        for suffix in ['pdb']:
            file = '%s/%s.%s'%(dir,id,suffix)
            if exists(file):
                new_file = '%s/%s.%s'%(frag_dir,id,suffix)
                stupidchainid = id[4]
                if (stupidchainid == '_'):
                    stupidchainid = ' '

                replacechain(file, new_file, stupidchainid)

    ##
    for m in [3,9]:
        file = '%s/ch%s%02d%s'%(dir,id,m,old_frag_tag)
        print file
        assert exists(file)

        new_file = '%s/aa%s%02d%s'%(frag_dir,id,m,new_frag_tag)
        cmd = '/users/boinc/bin/reduce_fragment_library_size.pl %s > %s'%(file,new_file)
        print cmd
        system(cmd)



