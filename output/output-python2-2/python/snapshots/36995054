#!/usr/bin/python

from os import popen, popen2,popen3
import string
from sys import argv

native = argv[1]
predictions = argv[2:]

for prediction in predictions:

    maxsubs = []
    for threshold in [0.5, 1, 2, 4, 8]:
        command = '~/mammoth2/mammoth_rna -e %s  -p %s  -R %5.2f' % \
            (native, prediction, threshold )
        #print command

        (w,r,e) = popen3( command )
        lines = r.readlines()

        tot_length = 0
        for line in lines:
            cols = string.split( line )
            #print cols[0]
            if len(cols) > 0 :
                if cols[0] == 'Number':
                    tot_length = max( tot_length, int(cols[-1] ))
                elif cols[0] == 'PSI(end)=':
                    nali = int(cols[3])
        #print nali, tot_length
        maxsubs.append( nali )

    gdt_ha = (0.25 *(maxsubs[0]+maxsubs[1]+maxsubs[2]+maxsubs[3]))/tot_length
    gdt_ts = (0.25 *(maxsubs[1]+maxsubs[2]+maxsubs[3]+maxsubs[4]))/tot_length

    print 'GDT_HA: %8.3f  GDT_TS: %8.3f ' % (gdt_ha, gdt_ts ),
    print ' MM0.5:%8.3f  MM1:%8.3f'  % ( 1.0*maxsubs[0]/tot_length, 1.0*maxsubs[1]/tot_length),
    print '%s' % ( prediction)


