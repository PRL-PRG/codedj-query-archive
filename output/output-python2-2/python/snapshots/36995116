#!/usr/bin/python

from os import popen,system
from os.path import exists,basename
import sys
import string

def Help():
    print sys.argv[0]," <pdb1> <pdb2> ... "
    print '  Removes hydrogens from pdb file.'


pdbfiles = sys.argv[1:]
suffix = '_noH'

if len(pdbfiles) < 1:
    Help()

if not exists(pdbfiles[-1]):
    suffix = pdbfiles[-1]
    pdbfiles = pdbfiles[:-1]


for pdbfile in pdbfiles:
    gzipped = 0
    outfile  = basename(pdbfile).replace( '.pdb', suffix+'.pdb')
    outid = open( outfile ,'w')

    lines = open(pdbfile).readlines()

    i = 0
    oldresidue = '   '
    for line in lines:
        if line[0:4] == 'ATOM':
            currentelem = line[13]
            if not currentelem == 'H':
                outid.write(line)

    outid.close()

    print 'Stripped: ', outfile
