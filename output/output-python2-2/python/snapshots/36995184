#!/usr/bin/python

import string
from sys import argv,stderr,stdout
from os import popen,system
from os.path import exists
from amino_acids import longer_names

assert( len(argv)>1)
pdbname = argv[1]

lines = open(pdbname,'r').readlines()


oldresnum = '   '
count = 0;

outid  = stdout

for line in lines:
        line_edit = line
        if line[0:3] == 'TER':
            continue

        if line_edit[0:4] == 'ATOM' or line_edit[0:6] == 'HETATM':

	    if not (line[16]==' ' or line[16]=='A'): continue

            resnum = line_edit[23:26]
            if not resnum == oldresnum:
                count = count + 1
            oldresnum = resnum

            newnum = '%3d' % count
            line_edit = line_edit[0:23] + newnum + line_edit[26:]

            outid.write(line_edit)

outid.close()
