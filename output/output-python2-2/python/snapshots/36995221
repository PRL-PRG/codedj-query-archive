#!/usr/bin/python

import string
import sys
from os import popen,system
import pdb
from blast import NBAlign

fasta_file = sys.argv[1]

print "# PSIPRED VFORMAT (PSIPRED V2.5 by David Jones)"
print

silent_seq = open(fasta_file).readlines()[1][:-1]

for i in range(len(silent_seq)):
    print "%4d %s %s   %5.3f  %5.3f  %5.3f" % ( i+1, silent_seq[i], 'C', 0.334,0.333,0.333)
