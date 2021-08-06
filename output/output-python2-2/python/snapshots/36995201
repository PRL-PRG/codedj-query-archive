#!/usr/bin/python

from sys import argv
from os import system
from os.path import basename,exists
import string


pdb_files = argv[1:]

for pdb_file in pdb_files:

    mammoth_file = pdb_file+'.mammoth'

    if not exists( mammoth_file ):
        command = '/work/rhiju/mcm/mammoth -e /work/rhiju/mcm/mammothDbCEMS/all.list -p %s -o %s' % (pdb_file, mammoth_file)
        print(command)
        system(command)

    command = '/work/rhiju/python/mcm_scores.py %s' % mammoth_file
    print(command)
    system(command)
