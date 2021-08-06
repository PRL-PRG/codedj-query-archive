#!/usr/bin/python

from sys import argv,stdout
import string

infiles = argv[1:]

twolettercode =['A','C','D','E','F','G','H','I','K','L','M','N','P','Q','R','S','T','V','W','Y']

assert(len(twolettercode) == 20)

for infile in infiles:
    lines = open(infile,'r').readlines()

    fid = open(string.split(infile,'.')[0] + '.burialcode','w')

    checkit = 0
    for line in lines:
        cols = string.split(line)
        if len(cols)>0 and checkit:
            burialcode = cols[4]

            assert( cols[1] in twolettercode)
            fid.write('%3s %s %s\n' % (cols[0],burialcode,cols[1]))

        if len(cols)>0 and cols[0] == 'POS':
            checkit = 1

    fid.close()



