#!/usr/bin/python
# From phil and rob, edited by rhiju to grep for scop ID.

from phil import *
from os.path import dirname

scop_file = '/work/pbradley/scop/dir.cla.scop.txt'
VALL_SCOP_FILE = '/work/pbradley/scop/vall.scop.1.65.2001-02-02'
#VALL_SCOP_FILE = '/data/pbradley/scop/vall.scop.1.65.2001-02-02'
OVERWRITE = 1

def Help():
    print '\nUsage: %s <fasta-file1> \n'%(argv[0])
    print 'This script will read and move the existing *.homolog_vall files,'
    print 'replacing them with a new file that contains the same homologs plus'
    print 'any new ones in the desired scop group\n\n'
    exit()

if len(argv) <2:
    Help()

## parse args
fasta_file = argv[1]

target_id = string.split(fasta_file,'/')[-1][:5]

id = target_id
scop = {}
scop[id] = []
chain = target_id[4]

lines = popen('grep "%s" %s'%(id[:4],scop_file)).readlines()
for line in lines:
    scop_id = string.split(line)[3]
    if scop_id not in scop[id]:
        if string.split(line)[2][0] == chain :
            scop[id].append(scop_id)

if argv.count('-switch21') and len(scop[id]) == 0 and \
   id[0] == '2':
    lines = popen('grep "1%s" %s'%(id[1:4],scop_file)).readlines()
    for line in lines:
        scop_id = string.split(line)[3]
        if scop_id not in scop[id]:
            scop[id].append(scop_id)

print id, scop[id]
#assert len( scop[id] ) == 1 ## this will fail if multi-domain chain, or
## if no scop match
if len( scop[id]) < 1:
    ## if no scop match
    scop_id = string.split(lines[0])[3]
    scop[id].append(scop_id)


## get all vall ids that match this scop_id
l = string.split(scop_id,'.')
if len(l) not in range(1,5) and l[0] in ['a','b','c','d','e','f','g','h','i']:
    print '****  WEIRD SCOP ID! *****'
    exit(0)


grepper = string.join(l,'\\.')
if len(l)<4:
    grepper = grepper + '\\.'

command = 'grep "%s" %s'%(grepper,VALL_SCOP_FILE)
stderr.write(command+'\n')

id_list = map(lambda x:string.split(x)[0],popen(command).readlines())



target_id = string.split(fasta_file,'/')[-1][:5]

## now read existing file
homolog_file = '%s.homolog_vall'%(fasta_file[:-6])
if not exists(homolog_file):
    print 'missing homolog-file!!! skipping:',homolog_file

lines = map(string.split, open(homolog_file,'r').readlines())
hom_list = []
for line in lines:
    for id in line:
        if id not in hom_list:
            hom_list.append(id)

command = 'cp %s %s.old'%(homolog_file, homolog_file)
print command
system(command)

new_homolog_file = '%s.scop'%homolog_file
out = open(new_homolog_file,'w')
done = []
for id in id_list + hom_list:
    if id not in done:
        done.append(id)
        out.write('%s %s\n'%(target_id,id))
out.close()

if OVERWRITE:
    command = 'cp %s %s'%(new_homolog_file,homolog_file)
    print command
    system(command)

