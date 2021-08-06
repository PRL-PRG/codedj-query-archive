#!/usr/bin/python

from sys import argv

pdbs = argv[1:]

count = 0
for pdb in pdbs:
    print 'MODEL       ', count

    lines = open(pdb).readlines()
    for line in lines:
        if line[:4]=='ATOM':
            print line[:-1]

    print 'ENDMDL'
    count += 1
