#!/usr/bin/python

## used to be ~/python/2002/scratch11_29.py

## run maxsub, get superpositions

## Edited by Rhiju from Phil's script to include pymol output.

import string
from os import system, popen
from sys import argv,exit
from os.path import exists

e_files = argv[1:]

fid = open( 'list','w')

for e_file in e_files:
    rescored_native = string.split(e_file,'.pdb')[0] + '_0001.pdb'
    if not exists(rescored_native):
        fid.write( e_file+'\n')

fid.close()

command = '~rhiju/src/rosetta_scale_hessian/rosetta.gcc -score -fa_input -l list -nstruct 1 -scorefile blah -decoyfeatures -paths ~rhiju/paths.txt'
print(command)
system(command)

#In the output pdb there's a useful column...

for e_file in e_files:
    DFlines = popen('grep "DF  " '+rescored_native).readlines()
    buried = []
    for line in DFlines:
        cols = string.split(line)
        SASAfrac = float(  cols[5] )
        if (SASAfrac < 0.3):
            buried.append( int( cols[1] ))

    print buried


    command = '/work/rhiju/python/strip_sidechain_MR.py %s -subset ' % e_file
    for i in buried:
        command += ' %d' % i
    print command
    system( command )

