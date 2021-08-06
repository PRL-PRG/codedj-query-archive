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
    print 'Usage: %s {-gs} {-cartoons <N>} <prefix> <score-index> <P>'%sys.argv[0]
    print '-gs gives grey-scale colors, default is rainbow'
    print '-cartoons gives little cartoon plots for N of the clusters (thanks jens meiler)'
    print '\nLoads cluster distances from <prefix>.info'
    print 'Loads decoy scores from the <score-index> column of the '
    print '  DECOY_SCORE lines in <prefix>.info\n'
    print 'Score assigned to a cluster is the Pth percentile score. (0<=P<=100)'
    print 'Makes two trees:\n 1) average linkage: "<prefix>.average_linkage_cluster_tree.ps"'
    print ' 2) single linkage: "<prefix>.single_linkage_cluster_tree.ps"\n'+'-'*75+'\n'



if len(sys.argv) <4:
    Print_help_message()
    assert len(sys.argv)>=4

args = sys.argv[1:]
if '-gs' in args:
    GREY_SCALE = 1
    del args[args.index('-gs')]
else:
    GREY_SCALE = 0

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

num_cartoons = 0
if '-cartoons' in args:
    pos = args.index('-cartoons')
    num_cartoons = int(args[pos+1])
    del args[pos]
    del args[pos]

prefix = args[0]
score_index = int(args[1])
P = int(args[2])

if not exists(prefix+'.info') and not exists(prefix):
    sys.stderr.write('WARNING: couldnt open info_file: %s\n'%prefix+'.info')
    Print_help_message()
    assert (exists(prefix+'.info'))

if 1:

    ## parse .info file
    if prefix[-5:] == '.info':
        prefix = prefix[:-5]

    file = prefix+'.info'

    data = open(file,'r')
    line = string.split(data.readline())

    sizes = []
    cluster_members = {}
    decoy_score = {}
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
                                           line[17:])
        elif line[0] == 'DECOY_SCORE:':
            if score_index < 0:
                score_index = len(line) + score_index
            score = float(line[score_index])
            if USE_MAX:
                score = min( max_score, score)
            if USE_MIN:
                score = max( min_score, score)
            decoy_score[line[-1]] = score
        line = string.split(data.readline())
    data.close()

    N = len(sizes)

    cluster_scores = {}
    for cluster in range(N):
##         print sizes[cluster],len(cluster_members[cluster])
        assert sizes[cluster] == len(cluster_members[cluster])
        scores = []
        for decoy in cluster_members[cluster]:
            scores.append(decoy_score[decoy])
        cluster_scores[cluster] = scores


    names = []
    for i in range(N):
        names.append( `i`+'_'+`sizes[i]`)

## def Make_tree(distance,num_leaves,Update_distance_matrix,leaf_scores,percentile):

    if num_cartoons:
        ps_file = prefix+'.average_linkage_cluster_tree.I%02d.P%03d.C%d.ps'\
                  %(score_index,P,num_cartoons)
    else:
        ps_file = prefix+'.average_linkage_cluster_tree.I%02d.P%03d.ps'%(score_index,P)

    score_tree = score_trees_devel.Make_tree(distance, N,
                                             score_trees_devel.Update_distance_matrix_AL,
                                             cluster_scores,P)
    stderr.write('making %s\n'%ps_file)
    score_trees_devel.Plot_tree( score_tree, names, sizes, ps_file, GREY_SCALE,
                                 num_cartoons, prefix)


    if num_cartoons:
        ps_file = prefix+'.single_linkage_cluster_tree.I%02d.P%03d.C%d.ps'\
                  %(score_index,P,num_cartoons)
    else:
        ps_file = prefix+'.single_linkage_cluster_tree.I%02d.P%03d.ps'%(score_index,P)

    score_tree = score_trees_devel.Make_tree(distance, N,
                                             score_trees_devel.Update_distance_matrix_SL,
                                       cluster_scores,P)

    stderr.write('making %s\n'%ps_file)
    score_trees_devel.Plot_tree( score_tree, names, sizes, ps_file, GREY_SCALE,
                                 num_cartoons,prefix)
