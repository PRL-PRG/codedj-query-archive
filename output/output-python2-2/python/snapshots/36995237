#!/usr/bin/python

from sys import argv,exit
from os import popen,system
from os.path import exists
import string
from glob import glob

def Help():
    print
    print 'Usage: '+argv[0]+' <ralph submit file> [<batchnumber>] -queue <queue number>'
    print
    exit()


if len(argv)<2:
    Help()

if argv.count('-queue') < 1:
    Help()

pos = argv.index('-queue')
del( argv[pos] )
queue = int( argv[pos] )
del( argv[pos] )


submitfile = argv[1]
batchid = 0
if len(argv)>2:
    batchid = int( argv[2])

data = open(submitfile,'r')

if not submitfile.find('ralph.submit'):
    print 'ralph submit file must end in .ralph.submit!'
    exit()

newsubmitfile = submitfile.replace( 'ralph.submit','boinc.submit')

fid = open( newsubmitfile, 'w')

keeptesting = 1
count = 1
dirnames = []
line = data.readline()
while line and keeptesting:
    while line and line[:4] != 'name':
        fid.write(line)
        line = data.readline()

    if not line: break

    WUname = string.split(line,'=')[-1][1:-1]

    #Mirror the results from ralph
    dirname = '/net/boinc/results_ralph/'+WUname[:8]
    if dirname not in dirnames:
        dirnames.append(dirname)
        command = '/work/rhiju/casp7/python/update_results.py '+dirname
        print(command)
        system(command)

    relax_score_filter1 = []
    relax_score_filter1a= {}
    relax_score_filter2 = {}

    # Find the scorefile -- assume its the first scorefile available with the WU name.
    if batchid > 0:
        scorefiles = glob('%s/%s*%d*sc.bz2' % (dirname,WUname,batchid))
    else:
        scorefiles = glob('%s/%s*sc.bz2' % (dirname,WUname))
    assert( len(scorefiles)>0)
    scorefile = scorefiles[-1]

    # Figure out scorefilters.
    scorelines = popen('bzcat '+scorefile+ '| grep SCORE ').readlines()

    scoreline = scorelines[0]
    cols = string.split(scoreline)
    index_found = 0
    index1a_found = 0
    if cols.count('rlxfilt1'):
        index1 = cols.index('rlxfilt1')
        index2 = cols.index('rlxfilt2')
        index_found = 1
    elif cols.count('RLXSCOR1'):
        index1  = cols.index('RLXSCOR1')
        index2  = cols.index('RLXSCOR2')
        index_found = 1

        if cols.count('RLXSCOR1A'):
            index1a = cols.index('RLXSCOR1A')
            index1a_found = 1

    else:
        print scoreline
        print 'Hey! Outfiles must have columns with rlxfilt1 (or RLXSCORE1) and rlxfilt2 (or RLXSCORE2)'
#        exit()

    if index_found:
        index1 = index1 - len(cols) - 1
        index2 = index2 - len(cols) - 1

        if index1a_found:
            index1a = index1a - len(cols) - 1

        for scoreline in scorelines[1:]:
            cols = string.split(scoreline)
            try:
                relax_score_filter1.append( float(cols[index1]))
                relax_score_filter2[ float(cols[index1]) ] = float(cols[index2])
                if index1a_found:
                    relax_score_filter1a[ float(cols[index1]) ] = float(cols[index1a])
            except:
                print 'BAD SCORE? ', cols[index1], cols[index2]
                continue

        relax_score_filter1.sort()
        numdecoys = len( relax_score_filter1 )

        set_score_filter1 = relax_score_filter1[ int(numdecoys/ 2) ]


        relax_score_filter2_cull = []
        for x in relax_score_filter1[ :int(numdecoys/2)]:
            if x in relax_score_filter2.keys():
                relax_score_filter2_cull.append( relax_score_filter2[x] )

        relax_score_filter1a_cull = []
        for x in relax_score_filter1[ :int(numdecoys/2)]:
            if x in relax_score_filter1a.keys():
                relax_score_filter1a_cull.append( relax_score_filter1a[x] )

        relax_score_filter2_cull.sort()

        set_score_filter2 = relax_score_filter2_cull[ int(numdecoys/ 4) ]

        if index1a_found:
            set_score_filter1a = relax_score_filter1a_cull[ int(numdecoys/ 4) ]

    #Save a filter file.

    while line and line[:9] != 'arguments':
        fid.write( line )
        line = data.readline()
    if not line: break

    if index1a_found and 1:
        fid.write(line[:-1] + ' -relax_score_filter -filter1 %4.0d -filter1a %4.0d ' % (set_score_filter1, set_score_filter1a) + '\n' )
        print set_score_filter1, set_score_filter1a
    else:
        fid.write(line[:-1] + ' -relax_score_filter -filter1 %4.0d -filter2 %4.0d ' % (set_score_filter1, set_score_filter2) + '\n' )
        print set_score_filter1, set_score_filter2

    line = data.readline()
    while line and line[:5] != 'Queue':
        fid.write(line)
        line = data.readline()
    fid.write( 'Queue = %d' % queue )

    line = data.readline()

fid.close()

