#!/usr/bin/python

import string
import re
from phil import *


def get3djuryScore ( pdb_name ):

    pdb_file = open( '/dump/pbradley/LB%d/bioinfo/%s/3djuryA1.pdb'%(lb_num, pdb_name ) )
    lines = pdb_file.readlines()
    for line in lines :
        if ( re.match( 'REMARK SCORE',line ) ):
            cols = string.split( line )
            return float(cols[2])

# end def get3djuryScore

def read_File ( top_file, output_dir ):

    # open the file
    file = open( top_file )


    # get all the lines
    lines = file.readlines()

    # this gets passed to phil's part of the script that
    # makes the actual pairing file
    good_features = []

    # this threshold will likely need to be adjusted
    #threshold = float(.2)
    threshold = float(0.0)

    totalLineCount = 0
    thresholdLineCount = 0
    nativeLineCount = 0


    for line in lines :

        if ( re.match( 'FEATURE',line ) ):
            # only consider the "FEATURE" lines

            # split the line for parsing
            cols = string.split( line )

            # exlcude BAB and HP
            #if ( len(cols) == 14 and  re.match('BAB|HP',cols[13]) ) :
            #    continue

            totalLineCount+=1

            score = float(cols[4])
            if ( score >= threshold ) :
                thresholdLineCount+=1
                good_features.append( line )

    print 'Making Pairing'
    Make_Pairing( top_file, good_features, output_dir )

# end def readFile


############## ARGS
#top_file = argv[1]
SCORE_FRACTION = float(1)


############## PARAMS

EXCLUDE_BAB = 1
EXCLUDE_HP = 1 ##!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!
EXCLUDE_LONG_HAIRPINS = 0 ## see function Long_hairpin() below

## EXCLUDE_HP = 1
## EXCLUDE_LONG_HAIRPINS = 1 ## see function Long_hairpin() below

## SCORE_FRACTION = 1.0 ## fraction of decoys to choose, by score
## SCORE_FRACTION = 0.25 ## fraction of decoys to choose, by score
MAX_FREQUENCY = 0.5 ## Exclude features with higher frequencies
## MAX_FREQUENCY = 0.2 ## Exclude features with higher frequencies
MIN_FREQUENCY = 0.01
## MIN_FREQUENCY = 0.01


## in this file: orientations and pleatings are 1 or 2 as in ROSETTA

def Show_feature(f):
    a = f[0]
    b = f[1]
    o = f[2]
    if o == 1: ## antiparallel
        i = (a+b)/2
        j = (b-a)/2
    else:
        i = (a+b)/2
        j = (a-b)/2

    return '%d,%d,%d'%(i-1,j-1,o-1)## return to input numbering !!! aargh

def Long_hairpin( E_sep, H_sep, L_sep): ## should already have checked if antiparallel
    total = E_sep + H_sep + L_sep
    return ( E_sep <1 and H_sep <1 and total < 15)

def Long_axis(i,j,o):
    if o==1: ## antiparallel
        return i-j
    else:
        return i+j

def Short_axis(i,j,o):
    if o==1: ## antiparallel
        return i+j
    else:
        return i-j


def Feature(i,j,o):
    return (Long_axis(i,j,o), Short_axis(i,j,o), o)


BIG_T = 6
SMALL_T = 4

def Get_closest_feature(i,j,o,features): ## f = (aa,bb,o)

    a = Long_axis(i,j,o)
    b = Short_axis(i,j,o)

    best = [1000]

    for f in features:
        if f[2] == o:
            adev = abs(a-f[0])
            bdev = abs(b-f[1])
            dev = adev+bdev
            if adev <= BIG_T and bdev <= SMALL_T and dev<best[0]:
                best = [dev, f]
    if best[0]<1000:
        return best[1]
    else:
        return ()


def Make_Pairing(top_file, feature_lines, output_dir ):## make re-sampled pairing files from .top files

        score = {} ## decoy total score (column1)
        pairings = {}
        topology = {}
        info = {} ## feature info, eg: ['NAT','HP','BAB']

        #log('reading top_file %s\n'%top_file)

        data = open(top_file,'r')
        lline = data.readline()
        while lline:
            line = string.split(lline)
            if line[0] == 'PAIRS':
                npairs = (len(line)-3)/4
                if len(line) != 3 + 4*npairs:
                    #log('bad line: %s\n'%(`line`))
                    lline = data.readline()
                    continue
                name = line[1]
                if name == 'NATIVE':
                    lline = data.readline()
                    continue
                if score.has_key(name):
                    #log('repeated ID, skipping: %s\n'%name)
                    lline = data.readline()
                    continue
                score[name] = float(line[2])
                pairings[name] = []
                for p in range(npairs):
                    i = int(line[3+4*p]) ## ROSETTA numbering!! see cluster...c
                    j = int(line[4+4*p])
                    orientation = float(line[5+4*p])
                    pleating = float(line[6+4*p])
                    if orientation < 0: o = 1
                    else: o = 2
                    if pleating < 0: p = 1
                    else: p = 2
                    pairings[name].append((i,j,o,p))

            elif line[0] == 'TOP':
                name = line[1]
                if not pairings.has_key(name):
                    #log('missing pairings:%s\n'%name)
                    lline = data.readline()
                    continue
                topology[name] = []
                if abs(float(line[2])-score[name])>0.5:
                    #log('score mismatch: repeat??: PAIR vs TOP lines!! %s %f %f\n'\
                    #    %(name,score[name],float(line[2])))
                    lline = data.readline()
                    continue
                ntop = (len(line)-5)/3
                for t in range(ntop):
                    i = int(line[3*t+5]) + 1 ## stupid: convert from C numbers
                    j = int(line[3*t+6]) + 1
                    o = int(line[3*t+7]) + 1 ## go from C:0,1 to ROS:1,2
                    f = Feature(i,j,o)
                    topology[name].append( f )


            lline = data.readline()
        data.close()


        for feature_line in feature_lines:
            line = string.split(feature_line)
            i = int(line[1]) + 1
            j = int(line[2]) + 1
            o = int(line[3]) + 1
            f = Feature(i,j,o)
            info[f] = line[13:]
            if EXCLUDE_LONG_HAIRPINS and o == 1:
                assert line[5] == 'E:' and \
                       line[7] == 'H:' and \
                       line[9] == 'L:'
                if Long_hairpin( float(line[6]), float(line[8]), float(line[10])) and \
                       'HP' not in info[f]:
                    #log('Long Hairpin: %s\n'%(string.join(line)))
                    info[f] .append('HP')

        features = info.keys()

        ## choose top SCORE_FRACTION by total score:
        l = map(lambda x:[score[x],x],score.keys())
        l.sort()
        N = int(floor( SCORE_FRACTION * len(l)))
        #log('total decoys %d choosing the top %d decoys: scores (min,N,max): %f %f %f\n'\
        #    %(len(l),N,l[0][0],l[N-1][0],l[-1][0]))

        decoy_count = {}
        pairing_list = {}
        for f in features:
            decoy_count[f] = 0
            pairing_list[f] = []

        for ll in l[:N]:
            name = ll[1]
            match = {}
            for f in features: match[f] = 0
            for p in pairings[name]:
                f = Get_closest_feature(p[0],p[1],p[2],features)
                if f:
                    match[f] = 1
                    pairing_list[f].append(p)

            for f in features: decoy_count[f] = decoy_count[f]+match[f]

        ##
        total = 0.0
        good_features = []
        for f in features:
            frequency = float(decoy_count[f])/ N
            if ( EXCLUDE_BAB and info[f].count('BAB')) or \
               ( EXCLUDE_HP and info[f].count('HP')) or \
               frequency < MIN_FREQUENCY or \
               frequency > MAX_FREQUENCY:
                #log('skipping: %s %f %s\n'%(`f`,frequency,`info[f]`))
                continue

            good_features.append(f)
            #log('keeping feature %s decoy_count= %d\n'%(Show_feature(f),decoy_count[f]))
            total = total + decoy_count[f]

        p_factor = 2000.0 / total

        ## make the big list with pairings from the non-local features
        big_pairing_list = []
        for f in good_features:
            npairings = int (floor(0.5 + p_factor * decoy_count[f]))
           # log('%s %d %d %s'%(Show_feature(f),decoy_count[f],npairings,info[f]))
            #print f,float(decoy_count[f])/N,npairings,info[f]
            l = []
            for p in pairing_list[f]:
                l.append( [random(), p])
            l.sort()
            for ll in l[:npairings]:
                big_pairing_list.append(ll[1])

        #print id,len(big_pairing_list)

        #out = open(pairing_file,'w')
        #out.write('%d\n'%(len(big_pairing_list)))
        #for p in big_pairing_list:
        #    out.write('%d %d %d %d\n'%(p[0],p[1],p[2],p[3]))
        #out.close()

        pdat = open( '%s/%s.pdat'%(output_dir, top_file),'w' )
        ## make the pairing file
        #print len(big_pairing_list)
        length = len(big_pairing_list)
        pdat.write('%s'%length )
        pdat.write('\n')
        for p in big_pairing_list:
            #print p[0],p[1],p[2],p[3]
            pdat.write('%s %s %s %s'%(p[0],p[1],p[2],p[3]))
            pdat.write('\n')
        pdat.close()
#end make pairings


##############################
##############################
# Program Starts From Here


def Help():
    print '\nUsage: %s <Top File> <output directory>'%(argv[0])
    exit()


top_file = argv[1]
if len(argv)>2:
    output_dir = argv[2]
else:
    output_dir = './'

read_File( top_file, output_dir )


