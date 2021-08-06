#!/usr/bin/python

## trees from distances

import string
from os import popen,system
from os.path import exists
import sys
from whrandom import random
from math import floor
from popen2 import popen2
import score_trees_devel
from sys import stderr


def Print_help_message():
    print '\n'+'-'*75
    print 'Usage: %s  <prefix> <score-index1> <score-index2> <P>'%sys.argv[0]
    print 'Loads decoy scores from the <score-index> column of the '
    print '  DECOY_SCORE lines in <prefix>.info\n'
    print 'Score assigned to a cluster is the Pth percentile score. (0<=P<=100)'

if len(sys.argv) <3:
    Print_help_message()
    assert len(sys.argv)>=3

args=sys.argv[1:]
if '-max' in args:
    USE_MAX = 1
    pos = args.index('-max')
    max_score = float(args[pos+1])
    print 'max_score=', max_score
    del args[pos]
    del args[pos]
else:
    USE_MAX = 0
    max_score = 0


if '-min' in args:
    USE_MIN = 1
    pos = args.index('-min')
    min_score = float(args[pos+1])
    del args[pos]
    del args[pos]
else:
    USE_MIN = 0
    min_score = 0

prefix = args[0]
score_index1 = int(args[1])
score_index2 = int(args[2])
P = int(args[3])

if not exists(prefix+'.info'):
    sys.stderr.write('WARNING: couldnt open info_file: %s\n'%prefix+'.info')
    Print_help_message()
    assert (exists(prefix+'.info'))

if 1:

    ## parse .info file
    file = prefix+'.info'
    data = open(file,'r')
    line = string.split(data.readline())

    sizes = []
    cluster_members = {}
    decoy_score1 = {}
    decoy_score2 = {}
    distance = {}

    sys.stderr.write('parsing .info file: %s\n'%file)
    while line:
        if line[0] == 'CLUSTER_RMSDS':
            cluster = int(line[1])
            assert cluster == len(sizes) and len(line) == 10+cluster
            sizes.append(int(line[2]))
            for i in range(cluster+1):
                distance[(i,cluster)] = float(line[9+i])
                distance[(cluster,i)] = float(line[9+i])
        elif line[0] == 'CLUSTER_INFO:':
            cluster = int(line[1])
            cluster_members[cluster] = map(lambda x:string.split(x,',')[1],
                                           line[9:])
        elif line[0] == 'DECOY_SCORE:':
            if score_index1 < 0:
                score_index1 = len(line) + score_index1
            score = float(line[score_index1])
            if USE_MAX:
                score = min( max_score, score)
            if USE_MIN:
                score = max( min_score, score)
            decoy_score1[line[-1]] = score

            if score_index2 < 0:
                score_index2 = len(line) + score_index2
            score = float(line[score_index2])
            if USE_MAX:
                score = min( max_score, score)
            if USE_MIN:
                score = max( min_score, score)
            decoy_score2[line[-1]] = score
        line = string.split(data.readline())
    data.close()

    N = len(sizes)

    cluster_scores1 = {}
    cluster_scores2 = {}
    print '              SIZE  RANK  SCORE1  SCORE2 '
    for cluster in range(N):
##         print sizes[cluster],len(cluster_members[cluster])
        assert sizes[cluster] == len(cluster_members[cluster])
        scores1 = []
        scores2 = []
        for decoy in cluster_members[cluster]:
            scores1.append(decoy_score1[decoy])
            scores2.append(decoy_score2[decoy])
        cluster_scores1[cluster] = scores1
        cluster_scores2[cluster] = scores2

        scores1.sort()
        scores2.sort()
        pos = (P * len(scores1) ) / 100
        if pos==len(scores1): pos = len(scores1)-1
        print 'CLUSTER:  %d  %d   %d   %s   %s'%(cluster,sizes[cluster],pos, scores1[pos],scores2[pos])
