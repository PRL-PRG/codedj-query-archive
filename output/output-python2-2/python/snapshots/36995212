#!/usr/bin/python

from os import popen
import sys
import string

outfile = sys.argv[1]
barcodefile = sys.argv[2]
fractionkeep = float( sys.argv[3] )

# Go through native score file, find which flavors gave which score.

lines = open(outfile).readlines()

cols = string.split(lines[1])
scoreindex = cols.index('score')
flavorindex = cols.index('flavor')
score_flavor_list = []
for line in lines:
    cols = string.split(line)
    if cols.count('SCORE:'):
        if not( cols[scoreindex] == 'score'):
            score_flavor_list.append( [ float(cols[scoreindex]), cols[flavorindex] ])

# Sort by native energy
score_flavor_list.sort()
numdecoys = len(score_flavor_list)
num_flavor = {}

if fractionkeep < 1.0: # USer has specified percentile cutoff for score.
    numkeep = int(numdecoys * fractionkeep)
    for i in range(numkeep):
        flavor = int( score_flavor_list[i][1])
        if flavor in num_flavor.keys():
            num_flavor[flavor] += 1
        else:
            num_flavor[flavor] = 1

    num_notopcode = 0
    if 0 in num_flavor.keys():
        num_notopcode = num_flavor[0]

    numtot = numkeep - num_notopcode
else:
    numtot = fractionkeep
    flavors_to_keep = []
    for i in range(numdecoys):
        flavor = int( score_flavor_list[i][1])
        if (flavor not in flavors_to_keep) and not (flavor == 0):
            flavors_to_keep.append(flavor)
            num_flavor[flavor] = 1
        if len(flavors_to_keep) >= numtot:
            break




# Now go through outfile and put in two new columns, based on the flavor:
# the native energy for that flavor and a reordered flavor.
lines = open(barcodefile).readlines()

totfrequency = 0.0
for i in range( len(lines)):
    line = lines[i]
    barcodestuff = string.join(string.split(line)[2:]) # Everything after barcode frequency

    frequency = 0.0
    if (i+1) in num_flavor.keys():
        frequency = num_flavor[(i+1)] / float(numtot)

    print 'PERMUTE %f %s' % (frequency, barcodestuff)
    totfrequency += frequency

#print 'TOT FREQUENCY: %f' % totfrequency
#print num_flavor
