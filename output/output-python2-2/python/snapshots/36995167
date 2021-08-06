#!/usr/bin/python
## make mammoth structure alignments


import string
from glob import glob
from sys import argv,stderr,exit
from os import popen,system
from os.path import exists
from operator import add

pdb_files = argv[1:-1]
prefix = argv[-1]

command = '/work/rhiju/python/superimpose.py  '
for pdb_file in pdb_files:
    command += ' '+pdb_file
command += ' -copy_resnum -copy_hetatm  -renumber_atoms > '+prefix+'.pdb'
print(command)
system(command)

command = '/work/rhiju/python/parse_NMR_models.py  '+prefix+'.pdb'
print(command)
system(command)

