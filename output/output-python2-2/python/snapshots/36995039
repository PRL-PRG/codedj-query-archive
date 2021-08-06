#!/usr/bin/python

from sys import argv,exit
from os import system
from os.path import exists,basename
import string

outfiles = argv[1:]

num_models = 20

RMS_THRESHOLD = 2.0

for outfile in outfiles:
    if not exists( outfile ):
        print "Cannot find", outfile
        continue

    outfile_scorecut = outfile.replace('.out','.low%d.out' % num_models )
    if not exists( outfile_scorecut ):
        command = '~rhiju/python/extract_lowscore_decoys_outfile.py %s %d > %s '% (outfile, num_models, outfile_scorecut)
        #print( command )
        system( command )

    cluster_logfile = outfile.replace('.out','.cluster.log' )
    if not exists( cluster_logfile ):
        command = 'cluster.macosgccrelease -database ~/minirosetta_database  -in:file:silent %s -in:file:fullatom -score:weights rna_hires.wts -rescore:output_only  -radius %f > %s' % ( outfile_scorecut, RMS_THRESHOLD, cluster_logfile )
        #print( command )
        system( command )

    lines = open( cluster_logfile ).readlines()
    rmsds = []
    NUM_CLUSTERS = 5
    for i in range( len( lines) ):
        line = lines[i]
        cols =  string.split( line )
        if len( cols ) > 2:
            if cols[0] == 'Cluster:':
                cluster_num = int( cols[1] )
                if cluster_num < NUM_CLUSTERS:
                    rmsds.append( [string.split( lines[i+1] )[2], cluster_num+1] )

    for i in range( NUM_CLUSTERS ):
        clusterfile = 'c.%s.0.pdb' % i
        if exists( clusterfile ):
            command = 'mv %s %s.cluster%s.pdb' % (clusterfile, outfile.replace('.out',''),i+1 )
            system( command )
        #else:
        #    print clusterfile, 'missing!'

    command = 'rm -rf c.*pdb'
    system( command )

    rmsds.sort()
    best_cluster_file = '%s.cluster%s.pdb' % ( outfile.replace('.out',''),rmsds[0][1])
    print best_cluster_file, '==>', rmsds[0][0]
