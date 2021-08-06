#!/usr/bin/python

import sys
import string
from math import floor
from os.path import basename
import re

###########################################################################
# Split strands into sheets. Each sheet is assumed to have at least
# two strands!
###########################################################################

###########################################################################
###########################################################################

def addsheetboundary(sheetboundarylist,totalstrands):
    newsheetboundarylist = []
    MINIMUM_SHEET_SIZE = 2

    for sheetboundary in sheetboundarylist:
	# Minimum sheet size
	minsheetsize = MINIMUM_SHEET_SIZE
	# To ensure uniqueness, all later sheets have to be at least as big
	# as the previous sheets
	if len(sheetboundary) > 1:
	    minsheetsize = sheetboundary[-1] - sheetboundary[-2]

	lastsheetboundary = sheetboundary[-1]
	maxsheetsize = totalstrands - lastsheetboundary - MINIMUM_SHEET_SIZE

	if maxsheetsize >= minsheetsize:
	    # There's room to sneak in another sheet boundary
	    for k in range(maxsheetsize-minsheetsize+1):
		nextsheetsize = minsheetsize + k
		newsheetboundary = sheetboundary+ [lastsheetboundary + nextsheetsize]
		newsheetboundarylist.append(newsheetboundary)
	    # Ah the magic of recursion:
	    newsheetboundarylist = addsheetboundary( newsheetboundarylist, totalstrands)


	# If we have some room, put the last sheetboundary at the end.
	if (totalstrands - lastsheetboundary) >= minsheetsize:
	    newsheetboundary = sheetboundary + [totalstrands]
	    newsheetboundarylist.append(newsheetboundary)

	# Done for now, this strand boundary definition is ready.
	if lastsheetboundary == totalstrands:
	    newsheetboundarylist.append(sheetboundary)

    return newsheetboundarylist

def generate_sheetboundarylist(totalstrands):
    # Assume at least two strands per sheet
    # Do one sheet, two sheets, ...  e.g., for 6 strands.
    sheetboundarylist = [ [0] ]
    sheetboundarylist = addsheetboundary(sheetboundarylist,totalstrands)

    return sheetboundarylist

def filter_num_sheet( sheetboundarylist, num_sheet):
    sheetboundarylist_filter = []
    for sheetboundary in sheetboundarylist:
        if len( sheetboundary ) == num_sheet + 1:
            sheetboundarylist_filter.append( sheetboundary )

    return sheetboundarylist_filter

###########################################################################
###########################################################################
def addorientationlist(orientationlist, antiparallel_only, parallel_only):
    neworientationlist = []
    for orientation in orientationlist:
        if not parallel_only:
            neworientation = orientation + ['A']
            neworientationlist.append(neworientation)
        if not antiparallel_only:
            neworientation = orientation + ['P']
            neworientationlist.append(neworientation)

    return neworientationlist

def generate_orientationlist(totalstrands, antiparallel_only, parallel_only):
    orientationlist = [['A'],['P']]
    if antiparallel_only:
        orientationlist = [['A']]
    if parallel_only:
        orientationlist = [['P']]
    for k in range(totalstrands-2):
	orientationlist = addorientationlist(orientationlist, antiparallel_only, parallel_only)
    return orientationlist



###########################################################################
###########################################################################
# Pretty silly, just figures out the next set of permutations based on the
# input list.
def addtopermutelist(currentlist,i):
    currentlistnew = []
    for permutation in currentlist:
        permutelength = len(permutation)
        for k in range( permutelength+1 ):
            permutationadd = permutation[0:k] + [i] + \
                             permutation[k:permutelength]
            currentlistnew.append(permutationadd)
    return currentlistnew

# Makes a list of possible permutations of strands
def makepermutelist(n):
    for k in range(n):
        if k==0:
            blah = [[1]]
        else:
            blah = addtopermutelist(blah,k+1)
    return blah

###########################################################################
###########################################################################
# Look through list of 2^N orientations -- if an orientation are defined across
# a strand boundary, replace it with an 'X'. And eliminate redundancy.
def filter_orientationlist( orientationlist, sheetboundary):
    new_orientationlist = []
    for orientation in orientationlist:
	numpairings = len(orientation)
	output_OK = 1
	for i in range( numpairings ):
	    if (i+1) in sheetboundary :
		if (orientation[i] == 'P'):
		    output_OK = 0
		    break
		else:
		    orientation = orientation[0:i] + ['X'] + orientation[(i+1):numpairings]
	if output_OK:
	    new_orientationlist.append( orientation )

    return new_orientationlist

###########################################################################
###########################################################################
# Look through list of N! permutations and remove redundancy due to symmetry:
#
#  1. Keep sheets with the first strand having lower number than last strand in sheet.
#  2. If there are strand arrangements with two strands with the same number of sheets,
#       keep the one whose first strand is lower in number than the second strand.
#
def filter_permutelist( permutelist, sheetboundary):
    new_permutelist = []

# Condition 1:
    numsheets = len(sheetboundary) - 1
    assert(numsheets > 0)
    for permute in permutelist:
	output_OK = 1
	for sheet in range( numsheets ):
	   startsheet = sheetboundary[sheet]
	   endsheet   = sheetboundary[sheet+1] - 1
	   if permute[startsheet] > permute[endsheet]:
	       output_OK = 0
	if output_OK:
	    new_permutelist.append( permute )


    new_permutelist2 = []
    for permute in new_permutelist:
	output_OK = 1
	prevsheetsize = 0
	prev_startsheet = 0
	if (numsheets > 1):
	    for sheet in range( numsheets ):
		startsheet = sheetboundary[sheet]
		endsheet   = sheetboundary[sheet+1] - 1
		sheetsize = endsheet - startsheet + 1
		if sheetsize == prevsheetsize: # Potential for symmetry
		    if permute[startsheet] > permute[prev_startsheet]:
			output_OK = 0
		prevsheetsize = sheetsize
		prev_startsheet = startsheet
	if output_OK:
	    new_permutelist2.append( permute )

    return new_permutelist2

###########################################################################
###########################################################################
#
# Dude, strands that are close in sequence can't be parallel.
#
def parallelhairpin_filter(SSpairs, permute, orientation):
    HAIRPIN_SEPARATION = 6
    OK = 1
    numstrands = len( permute )
    for i in range( numstrands - 1 ):
        if orientation[i] == 'P':
            firststrand  = min( permute[i]-1, permute[i+1]-1)
            secondstrand = max( permute[i]-1, permute[i+1]-1)
            endfirststrand    =  int(string.split(SSpairs[firststrand])[1])
            startsecondstrand =  int(string.split(SSpairs[secondstrand])[0])
            sequence_separation = startsecondstrand - endfirststrand
            if sequence_separation <= HAIRPIN_SEPARATION:
                OK = 0
                break

    return OK
###########################################################################
###########################################################################
#
# Look for interlock (a.k.a, cross-beta, interleaved beta strands).
#  Antiparallel pairing of (j, k)   and  of (j+1, k+1).
#
def interlock_filter(SSpairs, permute, orientation):
    HAIRPIN_SEPARATION = 6
    OK = 0
    numstrands = len( permute )
    for i in range( numstrands - 1 ):
        if orientation[i] == 'A':
            j = permute[i]
            k = permute[i+1]
            for h in range( numstrands - 1):
                if orientation[h] == 'A':
                    if (permute[h] == j+1) and (permute[h+1] == k+1):
                        OK = 1
                        break
                    if (permute[h+1] == j+1) and (permute[h] == k+1):
                        OK = 1
                        break
            if OK == 1:
                break

    return OK
###########################################################################
###########################################################################
#
# Look for greek keys:
#  Antiparallel pairing of (j, j+3)   and  of (j+1, j+2).
#
def greek_key_filter(SSpairs, permute, orientation):
    HAIRPIN_SEPARATION = 6
    OK = 0
    numstrands = len( permute )
    for i in range( numstrands - 1 ):
        if orientation[i] == 'A':
            firststrand  = min( permute[i]-1, permute[i+1]-1)
            secondstrand = max( permute[i]-1, permute[i+1]-1)
            if ( secondstrand - firststrand) == 1:
                for h in range( numstrands - 1):
                    if orientation[h] == 'A':
                        if (permute[h] == firststrand-1) and (permute[h+1] == secondstrand+1):
                            OK = 1
                            break
                        if (permute[h+1] == firststrand-1) and (permute[h] == secondstrand+1):
                            OK = 1
                            break
            if OK == 1:
                break

    return OK
###########################################################################
###########################################################################
#
# Look for greek keys, looser definition:
#  Antiparallel pairing of (j, k)   and  of (j-1, k+1).
#
def loose_greek_key_filter(SSpairs, permute, orientation):
    HAIRPIN_SEPARATION = 6
    OK = 0
    numstrands = len( permute )
    for i in range( numstrands - 1 ):
        if orientation[i] == 'A':
            firststrand  = min( permute[i]-1, permute[i+1]-1)
            secondstrand = max( permute[i]-1, permute[i+1]-1)
            for h in range( numstrands - 1):
                if orientation[h] == 'A':
                    if (permute[h] == firststrand-1) and (permute[h+1] == secondstrand+1):
                        OK = 1
                        break
                    if (permute[h+1] == firststrand-1) and (permute[h] == secondstrand+1):
                        OK = 1
                        break
            if OK == 1:
                break

    return OK
###########################################################################
###########################################################################
def output_barcode_penalty( SSpairs, permute, orientation):
    lineout = ''
    numstrands = len( permute )
#Go through potential strand pairs:
    for j in range(numstrands):
	for k in range(j,numstrands):
	    for o in ['A','P']:
		#Look through list
		OK = 0
		for i in range( numstrands - 1 ):
		    if orientation[i] == o:
			if (permute[i]   == j+1 and permute[i+1]==k+1):
			    OK = 1
			    break
			if (permute[i+1] == j+1 and permute[i]==k+1):
			    OK = 1
			    break
		if not OK:
		    lineout += ' SSPAIR 4.0 ' + o + ' ' + SSpairs[j] + ' ' + SSpairs[k]
    return lineout

###########################################################################
###########################################################################

def output_barcode( SSpairs, permute, orientation):
    lineout = ''
    numstrands = len( permute )
    for i in range( numstrands - 1 ):
        if not orientation[i] == 'X':
            lineout += ' SSPAIR -1.0 ' + orientation[i] + ' ' + SSpairs[permute[i]-1] + ' ' + SSpairs[permute[i+1]-1]
    return lineout

###########################################################################
###########################################################################


#############
#############
#   MAIN    #
#############
#############

SSlist = sys.argv[1]
SSoutname = sys.argv[2]

no_filters = 0
if sys.argv.count('-no_filters'):
    no_filters = 1

antiparallel_only = 0
if sys.argv.count('-antiparallel'):
    antiparallel_only = 1

parallel_only = 0
if sys.argv.count('-parallel'):
    parallel_only = 1

force_interlock = 0
if sys.argv.count('-interlock'):
    force_interlock = 1

force_greek_key = 0
if sys.argv.count('-greek_key'):
    force_greek_key = 1

force_loose_greek_key = 0
if sys.argv.count('-loose_greek_key'):
    force_loose_greek_key = 1

num_sheet = 0
if sys.argv.count('-num_sheet'):
    pos = sys.argv.index('-num_sheet')
    del(sys.argv[pos])
    num_sheet = int( sys.argv[pos])
    del(sys.argv[pos])

penalty = 0
if sys.argv.count('-penalty'):
    penalty = 1

SSpairs = open(SSlist,'r').readlines()
SSpairs = map(lambda x: x[:-1], SSpairs) #Get rid of newlines

print SSpairs

numSSpairs = len(SSpairs)
#if (numSSpairs < 4):
#    sys.exit()

sheetboundarylist = generate_sheetboundarylist(numSSpairs)
if num_sheet:
    sheetboundarylist = filter_num_sheet( sheetboundarylist, num_sheet)

print sheetboundarylist


orientationlist = generate_orientationlist(numSSpairs, antiparallel_only, parallel_only)
permutelist = makepermutelist(numSSpairs)


numtopologycodes = []
numtopologycodes_total = 0

lines_out = []
for sheetboundary in sheetboundarylist:
    orientationlist_filter = filter_orientationlist( orientationlist, sheetboundary)
    permutelist_filter     = filter_permutelist( permutelist, sheetboundary)

#    print sheetboundary
#    print orientationlist_filter, len( orientationlist_filter)
#    print permutelist_filter, len(permutelist_filter)
#    print

    numtopologycodes.append( len(orientationlist_filter) * len(permutelist_filter) )
    numtopologycodes_total += len(orientationlist_filter) * len(permutelist_filter)

    # Each combination of strand permutation and orientation defines a barcode.
    for permute in permutelist_filter:
        for orientation in orientationlist_filter:
            if not (parallelhairpin_filter(SSpairs, permute, orientation) or no_filters):
                continue
            if force_interlock and not (interlock_filter(SSpairs, permute, orientation) or no_filters):
                continue
            if force_greek_key and not (greek_key_filter(SSpairs, permute, orientation) or no_filters):
                continue
            if force_loose_greek_key and not (loose_greek_key_filter(SSpairs, permute, orientation) or no_filters):
                continue

	    if penalty:
		line_out = output_barcode_penalty( SSpairs, permute, orientation)
	    else:
		line_out = output_barcode( SSpairs, permute, orientation)
            lines_out.append(line_out)

# Output to file
frequency = 1.0 / len(lines_out)
SSoutid = open(SSoutname,'w')
for line_out in lines_out:
    SSoutid.write( 'PERMUTE %f %s\n' % (frequency, line_out))

print
print numtopologycodes
print numtopologycodes_total
print 'After filters: ', len(lines_out)

