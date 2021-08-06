#!/usr/bin/python

import string
from sys import argv,stderr
from os import popen
from whrandom import random

def Help():
    print '\n'+'-'*75
    print 'usage: %s <silent-file> <out-file> <score-index> <number-to-choose>\n'%argv[0]
    print 'negative score-index means choose lowest'
    print 'score-index is as in sort; 1=total_score, etc'
    print 'score-index = 0 will choose a random subset'
    print '-'*75
    print '\n'
    assert 0==1
    return

if len(argv) <4:
    Help()

in_files = argv[1:-2]
index = int(argv[-2])
choose_input = float(argv[-1])

#########################

for in_file in in_files:
    out_file = string.split(in_file,'.out')[0] + '.sub.out'


    lines = map(string.split,
                popen('grep "SCORE" '+in_file).readlines()[1:]) ## skip header

    N = len(lines)

    numbers = map(str,range(10))+['-']

    if index:
        scores = []
        for line in lines:
            if len(line) >= 15:
                if line[abs(index)][0] not in numbers:
                    print 'funny lline?',line
                else:
                    score = float(line[abs(index)])
                    scores.append(score)

        N = len(scores)

        scores.sort()
        print len(scores)
        stderr.write('score_index: %d min: %f 1st: %f 10th: %f 90th: %f 99th: %f max: %f\n'\
                         %(index,
                           scores[0],scores[N/100],scores[N/10],
                           scores[(90*N)/100],scores[(99*N)/100],scores[N-1]))

        if index<0:
            multiplier = -1
            index = -1*index
        else:
            multiplier = 1
            scores.reverse()

        if choose_input<1:
            choose = int( choose_input* N)
        else:
            choose = int( choose_input )

        threshold = multiplier * scores[choose]
        stderr.write('threshold: %f\n'%threshold)

    else:
        threshold = float(choose)/N


    data = open(in_file,'r')
    out = open(out_file,'w')

    out.write(data.readline())
    line = data.readline()
    out.write(line)
    S_index = line.find('desc')

    line = data.readline()
    counter = 0
    while line:
        if line[:5] != 'SCORE' or line[S_index] != 'S':
            line = data.readline()
            continue

        counter = counter+1
        if not counter%1000:stderr.write('%d of %d\n'%(counter,N))

        l = string.split(line)
        if len(line)<15 or l[-1] == 'description':
            line = data.readline()
            continue
        write = 0

        if index:
            try:
                score = float(l[index])
                if multiplier * float(l[index]) > threshold:
                    write = 1
            except:
                line = data.readline()
                continue

        elif random() < threshold:
            write = 1

        if write:
            out.write(line)
        line = data.readline()
        while line and line[:5] != 'SCORE':
            if write:
                out.write(line)
            line = data.readline()
    data.close()
    out.close()
