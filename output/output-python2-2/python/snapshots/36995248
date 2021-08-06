#!/usr/bin/python

from sys import argv, exit, stdout
import string
from os.path import dirname,basename,exists
from os import system



#Use the global alignment to then combine the secondary structure probabilities into one big file, useful for plotting.
#outputfile = stdout
#fid = open(outputfile, 'w')
fid = stdout
alignfile = argv[1]

lines = open( alignfile, 'r').readlines()

def outputcrap(fid,sequence,secstructprobfile):
    if exists(secstructprobfile):
        problines = open( secstructprobfile).readlines()
        probpos = 0
        for k in range( len(sequence)):
            alignpos = k+1
            if sequence[k] == '-' or probpos >= len(problines):
                fid.write( '%d 0.000 0.000 0.000\n' % alignpos)
            else:
                fid.write( '%d %s\n' % (alignpos, string.join( string.split(problines[probpos])[-3:] ))); #Need to cut out last three numbers.
                probpos += 1

    else:
        for k in range( len(sequence)):
            alignpos = k+1
            fid.write( '%d 0.000 0.000 0.000\n' % alignpos)



#The fragment secondary structure fractions.
secstructprobfiles = argv[2:]
if (len(secstructprobfiles) < len(lines)):
    print 'HEY! Must give the same number of secstructprob files as lines in the alignfile.'
    print 'And the order should match.'
    exit()

for i in range( len(lines)):
    line = lines[i]
    sequence = string.split(line)[1]

    secstructprobfile = secstructprobfiles[i]

    if secstructprobfile.find('secstructprob') < 0: # Assume its a fragment file.
        fragfile = secstructprobfile
        fragfile_unzip = basename(fragfile)[:-3]
        secstructprobfile = fragfile_unzip +'.secstructprob'

        if not exists(secstructprobfile):
            command = 'rsync -avz ' + fragfile + ' .'
            print(command)
            system(command)
            command = 'gunzip -f ' + basename(fragfile)
            print(command)
            system(command)
            command = '/users/rhiju/python/fragfile_to_secstructprob.py '+ fragfile_unzip + ' > ' + secstructprobfile
            print(command)
            system(command)


    outputcrap( fid, sequence, secstructprobfile)

fid.close()
