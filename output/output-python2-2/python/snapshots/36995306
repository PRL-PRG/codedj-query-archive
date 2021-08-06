import string
from glob import glob
from os.path import exists
from math import floor,log10,log,exp
from operator import add
from os import chdir,system,popen,getcwd
from sys import stderr,argv,exit
import sys
from whrandom import random
from math import sqrt
import re

#Need an error function to calculate maxsub crap.
sys.path.append('/users/rhiju/python')
from rhiju import *

# conversion
lb_id = {}
id_lb = {}
id8 = open( '/dump/pbradley/LB8/bioinfo/ids.txt' )
id9 = open( '/dump/pbradley/LB9/bioinfo/status.txt' )


lines = id8.readlines()
for line in lines :
     cols = string.split( line )
     lb_id[cols[3]] = cols[1]
     id_lb[cols[1]] = cols[3]
id8.close

lines = id9.readlines()
for line in lines :
     cols = string.split( line )
     lb_id[cols[3]] = cols[1]
     id_lb[cols[1]] = cols[3]
id9.close

args = argv[1:]
SERVER = 'none'
useSERVER = float(0)

if args.count( '-s' ):
    pos = args.index('-s')
    SERVER = args[pos+1]
    useSERVER = float(1)
    del args[pos]
    del args[pos]


targets = glob( '/data/xmas/js_bench/final_results/server_results/*.filter' )

print 'Best of top five 3DJury scores or cluster centers'
print ' pdb length     3dJ server   st   co   jj   sj'

for target in targets:

    pdb = target[-11:-7]
    #print pdb

    #LB8 max 21818
    lbid_num = float( id_lb[pdb] )
    lbid =  id_lb[pdb] 
    lb_num = 9
    if ( lbid_num < 21818 ):
        lb_num = 8

    # get 3djury information
    #
    # Modified by Rhiju, Aug. 28, 2005.
    #  I want to read out all 3Djury hits.  


    juryf = glob( '/dump/pbradley/LB%s/bioinfo/%s/3djuryA1.pdb'%( lb_num, lbid) )
    jury = open( juryf[0], 'r' )
    lines = jury.readlines()
    found_source = float(0)
    found_score = float(0)
    score = ''
    source = ''
    jur_num = float(0)

    allscore = {}
    allsource = []
    while ( found_score == 0 and found_source == 0 ):
        for line in lines:
            if ( re.search( 'SCORE', line ) and found_score == 0 ):
                cols = string.split( line )
                score = cols[2]
                found_score = float(1)
            if ( re.search( 'SOURCE', line ) and found_source == 0 ):
                cols = string.split( line )
                source = cols[2]
                found_source = float(1)

            if ( re.search( 'SOURCE', line )):
                cols = string.split( line )
                allsource.append(cols[2])
                lastsource = cols[2]
            if ( re.search( 'SCORE', line )):
                cols = string.split( line )
                allscore[lastsource] =  cols[2]

    #parse out scores from out file

    st = {}
    co = {}
    jj = {}
    sj = {}
    all = {}

    st_maxsub_tagkey = {}
    co_maxsub_tagkey = {}
    jj_maxsub_tagkey = {}
    sj_maxsub_tagkey = {}

#I need to get the protein length...
    bigoutfile = open( '/data/xmas/js_bench/final_results/decoys/st%s.out'%pdb )
    line = bigoutfile.readline();
    cols = string.split(line,': ')
    proteinlength = len(cols[1]);
    bigoutfile.close()

    st_f = open( '/data/xmas/js_bench/final_results/decoys/st%s.out.new'%pdb )
    co_f = open( '/data/xmas/js_bench/final_results/decoys/co%s.out.new'%pdb )
    jj_f = open( '/data/xmas/js_bench/final_results/decoys/jj%s.out.new'%pdb )
    sj_f = open( '/data/xmas/js_bench/final_results/decoys/sj%s.out.new'%pdb )

    lines = st_f.readlines()
    st_maxsub = float(0)
    for line in lines :
        if ( re.match( 'SCORE', line ) and not re.search( 'rms', line ) ):
             cols = string.split( line )
             st_maxsub_tagkey[ cols[-1]] = cols[17]
             if ( float( cols[17] ) > st_maxsub  ):
                 st_maxsub = float(cols[17])                 
    st_f.close()

    lines = co_f.readlines()
    co_maxsub = float(0)
    for line in lines :
        if ( re.match( 'SCORE', line ) and not re.search( 'rms', line )):
             cols = string.split( line )
             co_maxsub_tagkey[ cols[-1]] = cols[18]
             if ( float( cols[18] ) > co_maxsub  ):
                 co_maxsub = float(cols[18])
    co_f.close()

    lines = jj_f.readlines()
    jj_maxsub = float(0)
    for line in lines :
        if ( re.match( 'SCORE', line ) and not re.search( 'rms', line )):
             cols = string.split( line )
             jj_maxsub_tagkey[ cols[-1]] = cols[17]
             if ( float( cols[17] ) > jj_maxsub  ):
                 jj_maxsub = float(cols[17])
    jj_f.close()

    lines = sj_f.readlines()
    sj_maxsub = float(0)
    for line in lines :
        if ( re.match( 'SCORE', line ) and not re.search( 'rms', line )):
             cols = string.split( line )
             sj_maxsub_tagkey[ cols[-1]] = cols[18]
             if ( float( cols[18] ) > sj_maxsub  ):
                 sj_maxsub = float(cols[18])
    sj_f.close()

   
    #now parse the filtered server file
    #print target
    out = open( '%s'%target )
    lines = out.readlines()
    mx_num_maxsub = float(0)
    mx_server = ''

    alljur_num = {}

    for line in lines :

        cols = string.split( line )
         
        if ( re.match( 'target.0', line ) ):
            continue
         
        if ( re.match( 'dali', line ) or  re.match( '3dhit', line ) ):
            continue

        if ( useSERVER == 1 and not re.match( SERVER, line ) ) :
            continue


        if ( re.match( source, line ) ):
             jur_num = float( cols[2] )

## #Added by rhiju -- spit out all 3Djury scores and stuff.
##     #
        for eachsource in allsource :
              if ( re.match( eachsource, line ) ):
                   alljur_num[eachsource] = float( cols[2] )

        mx_num = float(cols[2])
        
        if (  mx_num_maxsub  == float(cols[2]) ):
            t_s = mx_server
            mx_server = '%s %s'%(cols[0], t_s)
        
        if ( mx_num_maxsub  < float(cols[2]) ):
            mx_num_maxsub =  float(cols[2])
            mx_server = '%s'%(cols[0])

    
      
    out.close()

## Now go through cluster centers, which were created with Phil's C routine,
    

    st_clustercenter_maxsub = []
    co_clustercenter_maxsub = []
    jj_clustercenter_maxsub = []
    sj_clustercenter_maxsub = []

    clusterpath = '/data/rhiju/js_bench/final_results/decoys/cluster/'

    clusterinfo = open('%s/st%s/st%s.info' %
                          (clusterpath,pdb,pdb) )
    lines = clusterinfo.readlines()

    for line in lines :
        cols = string.split( line )
        if ( re.match('CLUSTER_INFO:', line) ) :
           subcols = string.split(cols[3],':')
           tagname = subcols[1]
           st_clustercenter_maxsub.append(float(st_maxsub_tagkey[tagname]))
           
    clusterinfo.close()



    clusterinfo = open( '%s/co%s/co%s.info' %
                          (clusterpath,pdb,pdb) )
    lines = clusterinfo.readlines()

    for line in lines :
        cols = string.split( line )
        if ( re.match('CLUSTER_INFO:', line) ) :
           subcols = string.split(cols[3],':')
           tagname = subcols[1]
           if co_maxsub_tagkey.has_key(tagname) :
              co_clustercenter_maxsub.append(float(co_maxsub_tagkey[tagname]))
           
    clusterinfo.close()


    clusterinfo = open( '%s/jj%s/jj%s.info' %
                          (clusterpath,pdb,pdb) )
    lines = clusterinfo.readlines()

    for line in lines :
        cols = string.split( line )
        if ( re.match('CLUSTER_INFO:', line) ) :
           subcols = string.split(cols[3],':')
           tagname = subcols[1]
           jj_clustercenter_maxsub.append(float(jj_maxsub_tagkey[tagname]))
           
    clusterinfo.close()

    clusterinfo = open( '%s/sj%s/sj%s.info' %
                          (clusterpath,pdb,pdb) )
    lines = clusterinfo.readlines()

    for line in lines :
        cols = string.split( line )
        if ( re.match('CLUSTER_INFO:', line) ) :
           subcols = string.split(cols[3],':')
           tagname = subcols[1]
           sj_clustercenter_maxsub.append(float(sj_maxsub_tagkey[tagname]))
           
    clusterinfo.close()


#    print '%s: '%pdb,
#    for i in range(5):
#         print '%4.0f' % float(allscore[ allsource[i]]),    
#    print '| Top 3Djury scores'

    print '%s: '%pdb,
    i=range(5);



    server_maxsub =[]
    juryscorewithmaxsub = {}
    for eachsource in allsource :
        server_maxsub.append( alljur_num[eachsource])
        juryscorewithmaxsub[ alljur_num[eachsource]] = allscore[eachsource]

    print '%4d   ' % proteinlength,    

    bestoffive = max(server_maxsub[0:4]);
    print '%4.1f' % float(juryscorewithmaxsub[bestoffive]),
    print '%4d  '   % int(bestoffive ),    
 
    
    print '%4d' % int(max(st_clustercenter_maxsub[0:4])),
    print '%4d' % int(max(co_clustercenter_maxsub[0:4])),
    print '%4d' % int(max(jj_clustercenter_maxsub[0:4])),
    print '%4d' % int(max(sj_clustercenter_maxsub[0:4]))
 
    

    
    
