#!/usr/bin/python

import sys
import string
from math import floor
from os.path import basename
from os import popen,system

pairing_file = sys.argv[1]
barcode_file = sys.argv[2]
output_file = sys.argv[3]

pairlines = open(pairing_file,'r').readlines()
out = open(output_file,'w')

# Parse barcode file
barcodelines = open(barcode_file,'r').readlines()
barcodeline = barcodelines[0]

barcodes = string.split(barcodeline,'SSPAIR')
barcodes = barcodes[1:]

score = {}
direction = {}
startblock1 = {}
endblock1 = {}
startblock2 = {}
endblock2 = {}
bonusexist = 0
for barcode in barcodes:
    cols =string.split(barcode)

    score[barcode] = float(cols[0])
    direction[barcode] = cols[1]
    startblock1[barcode] = int(cols[2])
    endblock1[barcode]   = int(cols[3])
    startblock2[barcode] = int(cols[4])
    endblock2[barcode]   = int(cols[5])
    if score[barcode]<0:
        bonusexist = 1

if bonusexist:
    print 'Only allowing pairs that satisfy a bonus bar code'

# Go through each pair and check if its allowed.
pairlines_allowed = []
for pairline in pairlines:
    cols_pairline = string.split(pairline)
    cols_pairline = map( lambda x: int(x), cols_pairline)
    if len(cols_pairline) < 3:
        continue
    allowed = 1

    # This could be much much faster, but it is conceptually simple.
    for barcode in barcodes:
        if (score[barcode] >= 0.0):
        # Penalties
            if  (((cols_pairline[2] == 2) & (direction[barcode]=='P')) |
                 ((cols_pairline[2] == 1) & (direction[barcode]=='A')) ):
                if ((cols_pairline[0] >= startblock1[barcode]) & \
                    (cols_pairline[0] <=   endblock1[barcode]) & \
                    (cols_pairline[1] >= startblock2[barcode]) & \
                    (cols_pairline[1] <=   endblock2[barcode])):
                    allowed = 0
                    continue
                if ((cols_pairline[1] >= startblock1[barcode]) & \
                    (cols_pairline[1] <=   endblock1[barcode]) & \
                    (cols_pairline[0] >= startblock2[barcode]) & \
                    (cols_pairline[0] <=   endblock2[barcode])):
                    allowed = 0
                continue

    if bonusexist:
        foundabonus = 0
        for barcode in barcodes:
            if (score[barcode] < 0.0):
                # Bonuses
                if  (((cols_pairline[2] == 2) & (direction[barcode]=='P')) |
                     ((cols_pairline[2] == 1) & (direction[barcode]=='A')) ):
                    if ((cols_pairline[0] >= startblock1[barcode]) & \
                        (cols_pairline[0] <=   endblock1[barcode]) & \
                        (cols_pairline[1] >= startblock2[barcode]) & \
                        (cols_pairline[1] <=   endblock2[barcode])):
                        foundabonus = 1
                    if ((cols_pairline[1] >= startblock1[barcode]) & \
                        (cols_pairline[1] <=   endblock1[barcode]) & \
                        (cols_pairline[0] >= startblock2[barcode]) & \
                        (cols_pairline[0] <=   endblock2[barcode])):
                        foundabonus = 1

    if (bonusexist and not foundabonus):
        allowed = 0

    if (allowed):
        pairlines_allowed.append(pairline)


#out.write('%d\n' % len( pairlines_allowed))
for pairline in pairlines_allowed:
    out.write(pairline)

out.close()
