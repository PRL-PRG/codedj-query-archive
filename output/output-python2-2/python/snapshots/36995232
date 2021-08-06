#!/usr/bin/python

from sys import argv,exit
from os import popen, system
from os.path import basename
import string



def Help():
    print
    print 'Usage: '+argv[0]+' <silent out file 1> < silent file 2> ... <N> '
    print '  Will extract N decoys with lowest score from each silent file.'
    print '  If you want to select based on another column, say 12 (Rg), the'
    print '    last arguments should be -12 <N>  (for lowest Rg) or +12 <N>'
    print '    (for highest Rg).'
    print

    exit()


if len(argv)<2:
    Help()


try:
    NSTRUCT = int(argv[-1])
    del(argv[-1])
except:
    NSTRUCT = 2

infiles = argv[1:]

for infile in infiles:
    tags = []
    #Figure out rms column...
    line = popen('head -n 2 ' + infile).readlines()[1]
    cols = string.split(line)
    SCORECOL = len(cols)-1

    print 'Found score column: %d '% SCORECOL

    assert(infile[-4:] == 'info')
    lines = popen('sort -k %d -n %s | head -n %d' % (abs(SCORECOL), infile, NSTRUCT+1) ).readlines()

    templist_name = 'temp.%s.list'% basename(infile)

    fid = open(templist_name,'w')
    count = 0
    for line in lines:
        if line.find('DECOY_SCORE') >= 0:
            cols = string.split(line)
            fulltag = cols[-1]

            tag =''
            if fulltag.find('S_') > -1:
                tag = 'S'+string.split(fulltag,'S')[-1]
            if fulltag.find('F_') > -1:
                tag = 'F'+string.split(fulltag,'F')[-1]
            assert( len(tag) > 0 )

            fid.write(tag+'\n')
            tags.append(tag)
            count = count+1
        if count >= NSTRUCT:
            break

    outfilename = string.split(fulltag,':')[0]

    fid.close()
    command = '/work/rhiju/rosetta++/rosetta.gcc -extract -l %s -paths /work/rhiju/paths.txt -s %s'% (templist_name,outfilename)

    lines = popen('head -n 8 '+outfilename).readlines()
    if len(string.split(lines[7])) > 10:
        command += ' -fa_input'

    print(command)
    system(command)

    count = 1
    for tag in tags:
        command = 'mv %s.pdb %s.%d.pdb' % (tag,infile,count)
        print(command)
        system(command)
        count += 1

    command = 'rm '+templist_name
    print(command)
    system(command)


