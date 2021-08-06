#!/usr/bin/python

# Takes fragment file and output secondary structure probabilities
# in usual three state order.
#
from os import popen
import sys
import string
from operator import add

def In_range(angle):
    while angle>180: angle = angle-360
    while angle<=-180:angle = angle+360
    return angle

def pp_class(ppo): ## E G A B O
    ppo = ( In_range( ppo[0]), In_range(ppo[1]), In_range(ppo[2]))
    assert -180<=ppo[0]<=180 and -180<=ppo[1]<=180 and -180<=ppo[1]<=180

    if abs( ppo[2] ) < 90: return 'O'
    elif ppo[0]>=0:
        if -100< ppo[1] <= 100:return 'G'
        else: return 'E'
    elif -125 < ppo[1] <= 50: return 'A'
    else: return 'B'


def Read_fragments(fragment_file):
#As usual, stolen from a phil bradley script (make_new_plot.py)

# 1st five lines of frag file:
# position:            1 neighbors:          200
#
# 1di1 A   229 S L -147.015  136.320  177.891
# 1di1 A   230 A H  -61.103  -28.146  179.117
# 1di1 A   231 V H  -66.832  -46.265  179.757

    base = string.split(fragment_file,'/')[-1]
    if fragment_file[-3:] == '.gz':
        data = popen('zcat '+fragment_file)
        size = int(base[-17:-15])
    else:
        data = open(fragment_file,'r')
        size = int(base[-14:-12])
    sys.stderr.write('Reading fragment file: %s size= %d\n'\
                     %(fragment_file,size))

    line = data.readline()
    prev = (-1,-1)
    ss_count = {}
    ppo_count = {}
    while line:
        l = string.split(line)
        line = data.readline()
        assert len(l) == 4 and l[0] == 'position:'

        window = int(l[1])-1 ## numbering starts at 0
        nbrs = int(l[3])
        for i in range(size):
            pos = window + i
            if not ss_count.has_key( pos ):
                ss_count[ pos ] = {'H':0,'E':0,'L':0}
                ppo_count[ pos ] = {'A':0,'B':0,'G':0,'E':0,'O':0}

        for n in range(nbrs):
            for i in range(size):
                line = data.readline()
                l = string.split(line)
                pos = window + i
                ss = l[4]
                ppo = pp_class( map(float, l[5:8] ) )
                ss_count[pos][ss] += 1
                ppo_count[pos][ppo] += 1
            line = data.readline() ## extra blank line

        line = data.readline()
    data.close()

    L = len(ss_count.keys())

    l = []
    e = []
    h = []
    abgeo = ( [], [], [], [], [] )
    ABGEO = 'ABGEO'
    for i in range(L):
        total = reduce(add,ss_count[i].values())
        if total>0:
            e.append( float(ss_count[i]['E'])/total)
            h.append( float(ss_count[i]['H'])/total)
            l.append( float(ss_count[i]['L'])/total)
            for j in range(len(ABGEO)):
                count = ppo_count[i][ABGEO[j]]
                frac = float( count )/total
                if not (frac>= 0 and frac<=1):
                    print i,count,total,frac
                abgeo[j].append( frac )

    frag_ss = (e,h,l)

    return frag_ss,abgeo



fragment_file = sys.argv[1]

frag_ss,abgeo =  Read_fragments(fragment_file)

Cweight =  frag_ss[2]
Hweight =  frag_ss[1]
Eweight =  frag_ss[0]

for i in range( len(Cweight)):
    print "%4d   %4.3f  %4.3f  %4.3f" % (i+1, Cweight[i],Hweight[i],Eweight[i])
