#!/usr/bin/python

from sys import argv,exit
from os import popen, system
import string

def Help():
    print
    print 'Usage: '+argv[0]+' <.info file> <cluster number> <cluster number> ... '
    print ' Extracts cluster members from specified clusters (with 0 being'
    print '  the number of the first cluster.'
    print
    exit()


if len(argv)<2:
    Help()




only_hom001 = 0
if argv.count('-hom001'):
    pos = argv.index('-hom001')
    del(argv[pos])
    only_hom001 = 1

infile = argv[1]
clusternums = argv[2:]
NSTRUCT = 99999999

tags = []
tagfilename = {}
assert( infile[-4:] == 'info' ) # Its output from Phil's clustering program



lines = popen('grep CLUSTER_INFO %s' % infile).readlines()
fid = open('temp.list','w')
taglist = {}
whichcenter = {}
whichmember = {}
clustercount = 0
for clusternum in clusternums:
    line = lines[ int(clusternum) ]
    members = string.split(line,'Members:')[-1][:-1]
    clustermembers = string.split(members,',')[1:]
    count = 0
    for clustermember in clustermembers:
        cols = string.split(clustermember,':')
        outfilename = cols[0]

        if not outfilename in taglist.keys():
            taglist[ outfilename ] = []

        therest = string.split(cols[1],' ')[0]
        fulltag = therest
        if fulltag.find('S') > -1:
            #            tag = 'S'+string.join(string.split(fulltag,'S')[0:],'')
            S_index = fulltag.find('S')
            tag = 'S' + fulltag[S_index+1:]
        else:
            tag = 'F'+string.join(string.split(fulltag,'F')[0:],'')

        taglist[outfilename].append(tag)
        whichcenter[tag] = int( clusternum )
        count = count + 1
        whichmember[tag] = count
#        tagfilename[ tag ] = 'cluster%03d.%d' % (int(clusternum), count)
        if (count >= NSTRUCT): break
    clustercount += 1


outfilenames = taglist.keys()

print 'OUTFILENAMES: ',outfilenames


if only_hom001:
    outfilenames = []
    for outfilename in taglist.keys():
        if outfilename.find('hom001') >= 0:
            outfilenames = [outfilename]
            break

for outfilename in outfilenames:
    fid = open('temp.list','w')
    for tag in taglist[outfilename]:
        fid.write(tag+'\n')
    fid.close()
    command = '/users/rhiju/rosetta++/rosetta.mactelboincgraphics -extract -l temp.list -paths /users/rhiju/paths.txt -s '+outfilename

    # Check if this is an RNA run.
    fid = open( outfilename, 'r')
    line = fid.readline(); # Should be the sequence.
    if (line.count('a') or line.count('c') or
        line.count('g') or line.count('u')):
        command  += ' -enable_dna -enable_rna '


    lines = popen('head -n 8 '+outfilename).readlines()
    if len(string.split(lines[7])) > 10:
        command += ' -fa_input'

    print(command)
    system(command)

    for tag in taglist[outfilename]:
        command = 'mv %s.pdb %s.cluster%03d.%d.pdb' % (tag,infile,whichcenter[tag],whichmember[tag])
        print(command)
        system(command)

    command = 'rm temp.list'
    print(command)
    system(command)

