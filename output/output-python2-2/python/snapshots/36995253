#!/usr/bin/python

from sys import argv,exit
from os import popen, system
import string

try:
    NSTRUCT = int(argv[-1])
    del(argv[-1])
except:
    NSTRUCT = 9999999

infiles = argv[1:]

for infile in infiles:
    assert( infile[-4:] == 'info') # Its output from Phil's clustering program

    lines = popen('grep CLUSTER_RMSDS '+infile).readlines()

    count = 0
    taglist = {}
    whichcenter = {}
    for line in lines:
        cols = string.split(line)
        clustercenter = cols[3]
        cols = string.split(clustercenter,':')
        outfilename = cols[0]

        if not outfilename in taglist.keys():
            taglist[ outfilename ] = []

        fulltag = cols[1]
        if fulltag.find('S') > -1:
            tag = 'S'+string.join(string.split(fulltag,'S')[1:],'S')
        else:
            tag = 'F'+string.join(string.split(fulltag,'F')[1:],'S')

        taglist[outfilename].append(tag)
        whichcenter[tag] = count
        count = count+1
        if (count >= NSTRUCT):break

    for outfilename in taglist.keys():
        templist = outfilename+'.list'
        fid = open(templist,'w')
        for tag in taglist[outfilename]:
            fid.write(tag+'\n')
        fid.close()
        command = '/users/rhiju/rosetta++/rosetta.gcc -extract -l '+templist+' -paths /users/rhiju/paths.txt -s '+outfilename

        lines = popen('head -n 8 '+outfilename).readlines()
        if len(string.split(lines[7])) > 10:
            command += ' -fa_input'

        print(command)
        system(command)

        for tag in taglist[outfilename]:
            command = 'mv %s.pdb %s.cluster%03d.1.pdb' % (tag,infile,whichcenter[tag])
            print(command)
            system(command)

        command = 'rm '+templist
        print(command)
        system(command)


