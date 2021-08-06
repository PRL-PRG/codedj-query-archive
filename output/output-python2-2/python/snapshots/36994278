#!/usr/bin/python

from sys import argv
from os import popen
import string

logfiles = argv[1:]

relax_score_filter1 = []
relax_score_filter2 = {}
for logfile in logfiles:

    this_is_a_log_file = 0
    if logfile[0:3] == 'log':
        this_is_a_log_file = 1

    if (this_is_a_log_file):
        data = popen('grep score_filter '+logfile)
        line = data.readline()
        while line and line[:5] != 'score': line = data.readline()
        while line:
            while line and not line.count('relax_score_filter1'): line = data.readline()
            if not line: break
            cols = string.split(line)
            score1 = float(cols[4])
            relax_score_filter1.append(score1)

            while line and not line.count('relax_score_filter2'): line = data.readline()
            if not line: break
            cols = string.split(line)
            score2 = float(cols[4])

            relax_score_filter2[ score1 ] = score2
    else:
        lines = popen('grep SCORE '+logfile).readlines()
        # I'm assuming that the first line has the tags.
        line = lines[0]
        cols = string.split(line)
        if not cols.count('rlxfilt1'):
            print 'Hey! Outfiles must have columns with rlxfilt1 and rlxfilt2'

        index1 = cols.index('rlxfilt1')
        index2 = cols.index('rlxfilt2')

        for line in lines[1:]:
            cols = string.split(line)
            relax_score_filter1.append( float(cols[index1]))
            relax_score_filter2[ float(cols[index1]) ] = float(cols[index2])

relax_score_filter1.sort()
numdecoys = len( relax_score_filter1 )

set_score_filter1 = relax_score_filter1[ int(numdecoys/ 2) ]


relax_score_filter2_cull = []
for x in relax_score_filter1[ :int(numdecoys/2)]:
    if x in relax_score_filter2.keys():
        relax_score_filter2_cull.append( relax_score_filter2[x] )

relax_score_filter2_cull.sort()

set_score_filter2 = relax_score_filter2_cull[ int(numdecoys/ 4) ]

print set_score_filter1, set_score_filter2


