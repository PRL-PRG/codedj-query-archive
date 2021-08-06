#!/usr/bin/python

from os import popen
import sys
import string

outfile = sys.argv[1]
nativescorefile = sys.argv[2]


# Go through native score file, find which flavors gave which score.

lines = open(nativescorefile).readlines()

cols = string.split(lines[0])
scoreindex = cols.index('score')
flavorindex = cols.index('flavor')

score_flavor_list = []
for line in lines:
    cols = string.split(line)
    if not( cols[scoreindex] == 'score'):
        score_flavor_list.append( [ float(cols[scoreindex]), cols[flavorindex] ])

# Sort by native energy
score_flavor_list.sort()

# Make a mapping from flavor to how the native scores with that flavor.
flavor_to_nativescore = {}
flavor_to_reorder = {}
for i in range( len(score_flavor_list) ):
    entry = score_flavor_list[i]
    flavor_to_nativescore[ entry[1]] = entry[0]
    flavor_to_reorder[ entry[1]] = i

# Now go through outfile and put in two new columns, based on the flavor:
# the native energy for that flavor and a reordered flavor.
lines = open(outfile).readlines()

cols = string.split(lines[1])
flavorindex = cols.index('flavor')

for line in lines:
    cols = string.split(line)
    print line[:-1],  #No newline
    if cols[0] == 'SCORE:':
        if cols[flavorindex] == 'flavor':
            print '%s %s' % ('reorder','nativescore'),
        else:
            flavor = cols[flavorindex]
            print '%d %f' % (flavor_to_reorder[flavor],flavor_to_nativescore[flavor]),
    print

