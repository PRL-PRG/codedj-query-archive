#!/usr/bin/python

import sys
import string

infile = sys.argv[1]
thecolumn =int( sys.argv[2] )

lines = open(infile).readlines()
thedata = []
for line in lines:
    try:
        cols = string.split(line)
        thedata.append( float(cols[thecolumn-1]))
    except:
        continue

FIGURE_OUT_BINWIDTH = 0
try:
    binwidth = float(sys.argv[3])
    del( sys.argv[3] )
except:
    FIGURE_OUT_BINWIDTH = 1

try:
    mindata = float( sys.argv[3] )
    maxdata = float( sys.argv[4] )
except:
    mindata = min(thedata)
    maxdata = max(thedata)

if FIGURE_OUT_BINWIDTH:
    binwidth = (maxdata - mindata) / 50.0;


histogram = []
bincenter = []
currentbincenter = mindata + binwidth/2.0;
numbins =  int((maxdata-mindata)/binwidth)
for bin in range(numbins):
    histogram.append( 0.0 )
    bincenter.append( currentbincenter )
    currentbincenter += binwidth

for datum in thedata:
    bin = int( (datum - mindata)/binwidth )
    if bin<0       : bin = 0
    if bin>=numbins: bin = numbins - 1
    histogram[bin] += 1

for bin in range(numbins):
    print bincenter[bin], histogram[bin]
