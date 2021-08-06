#!/usr/bin/python

from sys import argv
import string

align_file = argv[1]
lines = open( align_file ).readlines()

tags = []
sequences = {}
for line in lines:
    cols = string.split(line)
    tag = cols[2]
    sequence = cols[1]
    tags.append(tag)
    sequences[tag] = sequence

for i in range( len(tags) ):
    for j in range( i+1, len(tags) ):
        tag1 = tags[i]
        tag2 = tags[j]
        sequence1 = sequences[tag1]
        sequence2 = sequences[tag2]

        sim = 0
        len1 = 0
        len2 = 0
        for c in range( len(sequence1) ):
            if (sequence1[c]!='-' and
                sequence1[c] == sequence2[c]):
                sim+=1
            if sequence1[c]!='-': len1+=1
            if sequence2[c]!='-': len2+=1

        print tag1, tag2, sim, len1, len2, sim/float(min(len1,len2))
