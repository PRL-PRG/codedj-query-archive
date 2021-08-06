#!/usr/bin/python

from sys import argv
import string

scorefiles = argv[1:]

RMS_CUTOFF = 3.0;

for scorefile in scorefiles:
    lines = open( scorefile ).readlines()

    try:
        cols = string.split( lines[1] )
        rms_index = cols.index("rms")
    except:
        cols = string.split( lines[0] )
        rms_index = cols.index("rms")


    count = 0
    count_lowrms = 0

    for line in lines[2:]:
        try:
            cols = string.split( line )
            rms = float( cols[ rms_index ] )
            count += 1
            if (rms < RMS_CUTOFF):
                count_lowrms += 1
        except:
            continue

    nearnative_freq =  1.0 * count_lowrms / count
    print scorefile, nearnative_freq

