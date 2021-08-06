#!/usr/bin/python

from sys import argv
import string

infile = argv[1]

lines = open(infile,'r').readlines();

tagline = lines[0]
if len(string.split(tagline)) < 5:
    tagline = lines[1]
tags = string.split(tagline)
filterindex = tags.index('filter')

filter_frequency = {}
total_decoys = 0
filter_frequency["pass"] = 0.0
for line in lines:
    scores = string.split(line)
    if len(scores) > filterindex and not scores.count('filter'):
        filter = scores[filterindex]
        if filter not in filter_frequency.keys():
            filter_frequency[filter] = 0
        filter_frequency[filter] += 1
        total_decoys += 1

filters = filter_frequency.keys()
filters.sort()

for filter in filters:
    filter_frequency[filter] /= (1.0*total_decoys)

filter = "pass"
print "%14s %4.3f" % (filter, filter_frequency[filter])

for filter in filters:
    if not filter == "pass":
        print "%14s %4.3f" % (filter, filter_frequency[filter])



