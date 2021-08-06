#!/usr/bin/python


import sys
import string
from os import system,popen
from os.path import basename
from glob import glob

outfiles = sys.argv[1:]

which_files_to_cat = {}

for outfile in outfiles:
    if (outfile[-4:] == '.out' ):
        #Old style, user specified a bunch of outfiles.
        tag = string.join( string.split( outfile,'_' )[:-2] , '_')
        if tag not in which_files_to_cat.keys():
            which_files_to_cat[tag] = []
        which_files_to_cat[tag].append( outfile )

    else: #New style -- look inside a directory!

        globfiles = glob( outfile+'/*/*out' )
        for file in globfiles:
            tag = basename( file ).replace('.out','')
            if tag not in which_files_to_cat.keys():
                which_files_to_cat[tag] = []
            which_files_to_cat[tag].append( file )


for tag in which_files_to_cat.keys():
    cat_file = "cat_"+tag+".out"
    print "Catting into: ",cat_file,
    command = '~rhiju/python/cat_outfiles.py %s >  %s ' % \
                                   (string.join( which_files_to_cat[tag] ) ,
                                    cat_file )
    #print command
    system( command )

    lines = popen( 'grep SCORE '+cat_file).readlines()
    print '... Will remove %d primary files. Found %d  decoys.' % (len( which_files_to_cat[tag] ),len(lines)-1)

for outfile in outfiles:
    command = 'rm -rf '+outfile
    print command
    system( command )
