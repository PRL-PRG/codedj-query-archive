#!/usr/bin/python

#
# May 18, 2006
# Concatenate old bzipped outfiles to new bzipped outfiles.
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


if len(argv) > 1:
    currentdir = argv[1:]
else:
    currentdir = getcwd()

filelist = []
for dir in currentdir:
    filelist += glob( dir+'/*.out.addendum.bz2' )
    filelist += glob( dir+'/*.sc.addendum.bz2' )


for file in filelist:
    filepath = dirname(file)
    filebase = basename(file)

    newbzipfilename = file[:-13]+'.bz2'

    assert(exists(newbzipfilename))

    command = 'cat ' + file+ ' >> '+newbzipfilename
    print 'Catting ',file,' and ',newbzipfilename
    system(command)

    command = 'rm ' + file
    system(command)
