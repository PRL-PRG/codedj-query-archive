#!/usr/bin/python

#
# May 18, 2006
# Bzip old outfiles, and prepare
# for concatenating to new outfiles.
#

from glob import glob
from sys import argv
from os import getcwd,popen,system
from os.path import basename,dirname,exists
import string

if len(argv) > 1:
    currentdir = argv[1:]
else:
    currentdir = getcwd()

filelist = []
for dir in currentdir:
    filelist += glob( dir+'/*.out' )
    filelist += glob( dir+'/*.sc' )


for file in filelist:
    filepath = dirname(file)
    filebase = basename(file)

    bzipfilename = file+'.bz2'

    if exists(bzipfilename): #Prepare older file for catting.
        command = 'wc ' + file
        wclines = popen(command).readlines()
        numlines = int( string.split( wclines[0] )[0] )
        print filebase, numlines
        num_real_lines = numlines - 2  #I.e., without two-line header

        if (numlines > 3):
            addendumfile = file+'.addendum'
            command = 'tail -n %d %s > %s' % (num_real_lines, file, addendumfile)
            print(command)
            system(command)

            command = 'bzip2 %s' % (addendumfile)
            print 'Bzipping: ',addendumfile
            system(command)
        else:
            print 'Nothing in '+file+'?'

    else: # Just bzip the damn thing
        command = 'bzip2 %s' % (file)
        print 'Bzipping: ',file
        system(command)

