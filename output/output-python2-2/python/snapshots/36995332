#!/usr/bin/python

from sys import argv
from os import system
from os.path import basename,exists
import string

mammoth_file = argv[1]

pdb_file_exists = 0
if len(argv) > 2:
    pdb_file = argv[2]
else:
    pdb_file = string.split( mammoth_file,'.mammoth')[0]

if exists( pdb_file ):
    pdb_file_exists = 1




lines = open(mammoth_file).readlines()

filename = ''
maxsub_forfilename = {}
maxsubs = []
filenames = []
look_for_EXPERIMENT = 1
for line in lines:
    if line.find('EXPERIMENT:'):
        look_for_EXPERIMENT = 0

    if line.find('Filename:') >  0  and not look_for_EXPERIMENT:
        filename = string.split(line)[1]
        look_for_EXPERIMENT = 1

    if line.find('-ln(E)=') > 0:
        maxsub_num = float( string.split(line)[3] )
        filenames.append(filename)
        maxsub_forfilename[filename] = maxsub_num
        maxsubs.append(maxsub_num)

maxsubs.sort()

numfiles = len(maxsubs)

NUMDECOYS = 3
maxsubs_best =  maxsubs[numfiles-NUMDECOYS:numfiles]
maxsubs_best.reverse()



fid = open(mammoth_file+'.txt','w')
count = 0
for maxsub in maxsubs_best:
    for i in range( numfiles ):
        if maxsub_forfilename[ filenames[i] ] == maxsub:
            count += 1
            fid.write( '%5.3f: %s\n' % (maxsub,filenames[i]) )

            if pdb_file_exists:
                superimpose_file = \
                                 string.split( basename(pdb_file),'.pdb')[0] \
                                 + '_MCM' + '%d'%count +\
                                 '.'+basename(filenames[i])

                command = '/work/rhiju/python/superimpose.py %s %s > %s' % (filenames[i],pdb_file,superimpose_file)
                print(command)
            system(command)

            del(filenames[i])
            break

fid.close()




