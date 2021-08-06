#!/usr/bin/python

from sys import argv,exit
from os import popen, system
import string

def Help():
    print
    print 'Usage: '+argv[0]+' <.info file> <number of cluster centers> '
    print '  The .info file needs to be of the format created by the'
    print '  clustering program cluster_info_silent.out. '
    print '  This utility needs to be run from the same directory as where the'
    print '  clustering program was run. (Its a paths thing.)'
    print
    exit()


if len(argv)<2:
    Help()


try:
    NSTRUCT = int(argv[-1])
    del(argv[-1])
except:
    NSTRUCT = 9999999

infiles = argv[1]
clusternums = argv[2:]


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
        elif fulltag.find('F') > -1:
            tag = 'F'+string.join(string.split(fulltag,'F')[1:],'S')
        else:
            tag = fulltag

        taglist[outfilename].append(tag)
        whichcenter[tag] = count
        count = count+1
        if (count >= NSTRUCT):break

    for outfilename in taglist.keys():
        suboutfilename = outfilename.replace('.out','.sub.out')
        command = 'head -n 2 '+outfilename+' > '+suboutfilename
        print(command)
        system(command)
        for tag in taglist[outfilename]:
            fid.write(tag+'\n')
            command = 'grep '+tag+' '+outfilename+ ' >> '+ suboutfilename
            print(command)
            system(command)


