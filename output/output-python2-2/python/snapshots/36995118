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


replace_names = 1
if argv.count('-no_replace_names'):
    pos = argv.index('-no_replace_names')
    del( argv[pos] )
    replace_names = 0

try:
    NSTRUCT = int(argv[-1])
    del(argv[-1])
except:
    NSTRUCT = 2

scorecol_defined = 0
try:
    SCORECOL = int(argv[-1])
    del(argv[-1])
    scorecol_defined = 1
except:
    SCORECOL = -1

REVERSE = ''
if SCORECOL > 0:
    REVERSE = ' --reverse '

infiles = argv[1:]



for infile in infiles:
    tags = []
    assert(infile[-3:] == 'out')
    lines = popen('grep SCORE '+infile+' |  head -n %d' % (  NSTRUCT+1) ).readlines()

    scoretag=''
    if scorecol_defined:
        scoretags = string.split( popen('head -n 2 '+infile).readlines()[1] )
        scoretag = scoretags[ abs(SCORECOL) ]

    templist_name = 'temp.%s.list'% basename(infile)

    fid = open(templist_name,'w')
    count = 0
    for line in lines:
        cols = string.split(line)
        tag = cols[-1]
        if tag.find('desc') < 0:
            fid.write(tag+'\n')
            tags.append(tag)
            count = count+1
        if count >= NSTRUCT:
            break
    outfilename = infile

    fid.close()
    command = '/work/rhiju/rosetta++/rosetta.gcc -extract -l %s -paths /work/rhiju/paths.txt -s %s'% (templist_name,outfilename)

    # Check if this is an RNA run.
    fid = open( infile, 'r')
    line = fid.readline(); # Should be the sequence.
    if (line.count('a') or line.count('c') or
        line.count('g') or line.count('u')):
        command  += ' -enable_dna -enable_rna '

    lines = popen('head -n 8 '+outfilename).readlines()
    if len(string.split(lines[7])) > 10:
        command += ' -fa_input'

    print(command)
    system(command)

    if outfilename.find('t343')>0:
        command = '/users/rhiju/python/extract_t343.py %s %s' % (outfilename,
                                                                 string.join(tags,' '))
        print(command)
        system(command)


    count = 1
    if replace_names:
        for tag in tags:
            if scorecol_defined:
                command = 'mv %s.pdb %s.%s.%d.pdb' % (tag,infile,scoretag,count)
            else:
                command = 'mv %s.pdb %s.%d.pdb' % (tag,infile,count)
            print(command)
            system(command)
            count += 1

    command = 'rm '+templist_name
    print(command)
    system(command)


