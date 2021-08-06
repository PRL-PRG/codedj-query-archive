#!/usr/bin/python

import sys
import string
from math import floor
from os.path import basename
import re

###########################################################################
# Pretty silly, just figures out the next set of permutations based on the
# input list.
###########################################################################
def addtopermutelist(currentlist,i):
    currentlistnew = []
    for permutation in currentlist:
        permutelength = len(permutation)
        for k in range( permutelength):
            permutationadd = permutation[0:k+1] + [i] + \
                             permutation[k+1:permutelength]
            currentlistnew.append(permutationadd)
    return currentlistnew

###########################################################################
# Removes half of the permutations -- the ones where the second member
# is less than the last member. Getting rid of redundant permutations
# related by parity.
###########################################################################
def filterlist(currentlist):
    newlist = []
    for permutation in currentlist:
        permutelength = len(permutation)
        if (permutation[1] < permutation[ permutelength-1]) :
            newlist.append(permutation)
    return newlist

###########################################################################
# Makes a list of possible permutations for n dinner guests at a circular
# dinner table.
###########################################################################
def makepermutelist(n):
    for k in range(n):
        if k==0:
            blah = [[0]]
        else:
            blah = addtopermutelist(blah,k)
    newlist = filterlist(blah)
    return newlist

###########################################################################
# Start the real stuff!
###########################################################################

SSlist = sys.argv[1]
SSoutname = sys.argv[2]


SSpairs = open(SSlist,'r').readlines()
SSpairs = map(lambda x: x[:-1], SSpairs) #Get rid of newlines

#print SSpairs

numSSpairs = len(SSpairs)
if (numSSpairs < 4):
    sys.exit()

permutelist = makepermutelist(numSSpairs)

totpermutations = len(permutelist)


makethepermutelist = 0

find_force_pairs = 0
if sys.argv.count('-forcepairs'):
    find_force_pairs = 1
    forceindex = sys.argv.index('-forcepairs')
    forcepairs = sys.argv[forceindex+1: ]
    forcepairs= map(lambda x:int(x)-1, forcepairs) #Back to python numbering.

try_all_orientations = 0
if sys.argv.count('-orientations'):
    try_all_orientations = 1

separate_topologycode_files = 0
if sys.argv.count('-separate'):
    separate_topologycode_files = 1

count = 0
bigoutlines =[]
if (makethepermutelist):
    letters1 = 'abcdefghijklmnopqrstuvwxyz0123456789'
    letters2 = '0123456789'
    assert( len(letters1) == 36)
 #   fid_condorstuff = open( SSoutname+'condorstuff.txt','w')
    fid_mapping = open('%smapping.txt' % SSoutname, 'w')

    # Do all permutations!
    for i in range(totpermutations):
        #If user has specified any pairings to force, make sure this permutation has them.
        permutation = permutelist[i]
        permutation.append( permutation[0] ) # Make it a circle!

        acceptpermute = 1
        if find_force_pairs:
            for j in range (len(forcepairs) - 1):
                foundpairing = 0
                for m in range(numSSpairs):
                    if ( forcepairs[j] == permutation[m]) & (forcepairs[j+1] == permutation[m+1]):
                        foundpairing = 1
                    if ( forcepairs[j] == permutation[m+1]) & (forcepairs[j+1] == permutation[m]):
                        foundpairing = 1
                if not foundpairing:
                    acceptpermute = 0
                    break

        if acceptpermute:
            print permutation
            lineout = ''

            #Mapping
            outfilename = '%s%d.bar' % (SSoutname, count)
            fid_mapping.write(outfilename+' ')
            for m in range(numSSpairs):
                num = permutation[m]+1
                fid_mapping.write( '%d '%num)
            fid_mapping.write( '\n')

            #Stuff useful in condor file
#            twolettercode = letters1[ int(floor(count/10))]+letters2[ count % 10]
#            outfilename = '%s%d.bar' % ( basename(SSoutname), count)
#            fid_condorstuff.write('arguments = '+twolettercode+' 1oey J -nstruct 40 -silent -accept_all -output_all -new_centroid_packing -barcode_mode 3 -increase_cycles 40 -barcode_file '+ outfilename+'\n');
#            fid_condorstuff.write('Queue 2\n');

            count = count+1

            allowedpairings = []
            for j in range(numSSpairs):
                for k in range(numSSpairs):
                    if j<=k:
                        allowpairing = 0
                        for m in range(numSSpairs):
                            if (j==permutation[m])   & (k==permutation[m+1]):
                                allowpairing = 1
                            if (j==permutation[m+1]) & (k==permutation[m] ):
                                allowpairing = 1

                        if not allowpairing :
                            lineout += ' SSPAIR 4.0 A '+SSpairs[j]+' '+SSpairs[k]
                            lineout += ' SSPAIR 4.0 P '+SSpairs[j]+' '+SSpairs[k]
                        else:
                            allowedpairings.append( [j,k] )
                            continue

            if try_all_orientations:
                # 2^n possible codes corresponding to possible parallel/antiparallel combinations.
                num_allowed_pairings = len(allowedpairings)
                num_orientations = pow(2,num_allowed_pairings)
                lineout_start = lineout
                for m in range(num_orientations):
                    lineout = lineout_start
                    for n in range(num_allowed_pairings):
                        binarydigit = m/pow(2,n) % 2
                        if binarydigit:
                            lineout += ' SSPAIR 3.0 A '
                        else:
                            lineout += ' SSPAIR 3.0 P '
                        j = allowedpairings[n][0]
                        k = allowedpairings[n][1]
                        lineout += SSpairs[j] + ' ' + SSpairs[k]
                    bigoutlines.append(lineout)
            else:
                bigoutlines.append(lineout)

            if (separate_topologycode_files):
                fid = open(outfilename,'w')
                lineout = 'PERMUTE%d 1.0 %s \n' % (count,lineout)
                fid.write(lineout)
                fid.close()

    fid_mapping.close()
    #fid_condorstuff.close()


    bigoutfilename = '%s' % (SSoutname)
    bigoutid = open(bigoutfilename,'w')
    numlines = len(bigoutlines)
    print 'Found %d topology codes' % numlines
    frequency = 1.0/(numlines+1)
    for line in bigoutlines:
        lineout = 'PERMUTE %f %s \n' % (frequency, line)
        bigoutid.write(lineout)
    bigoutid.close()

else: #User has specified a desired permutation

    userpermute = sys.argv[2:]

#		outfilename = '%s.bar' % (SSoutname)    
#    fid = open( outfilename,'w')
    fid = sys.	stdout
    fid.write('PERMUTE  1.0  ');

    permutation = map(lambda x:int(x)-1, userpermute) #Back to python numbering.
    #permutation.append( permutation[0] ) # Make it a circle!

    nummembers = len(permutation)
    numbadpairs = 0
    for j in permutation:
        for k in permutation:
            if j<=k:
                allowpairing = 0
                for m in range( nummembers - 1):
                    if (j==permutation[m])   & (k==permutation[m+1]):
                        allowpairing = 1
                    if (j==permutation[m+1]) & (k==permutation[m] ):
                        allowpairing = 1

                if not allowpairing :
                    fid.write(' SSPAIR 4.0 A '+SSpairs[j]+' '+SSpairs[k])
                    fid.write(' SSPAIR 4.0 P '+SSpairs[j]+' '+SSpairs[k])
                    numbadpairs += 1
                else:
                    #print j+1,k+1
                    continue
      #print numbadpairs
    print
