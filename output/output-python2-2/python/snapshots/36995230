#!/usr/bin/python
import sys
import string
from os import system

outfiles = sys.argv[1:-1]
coordfile = sys.argv[-1]

for outfile in outfiles:
    command = '/users/rhiju/C/cluster_info_silent.out '+outfile+' '+coordfile+' '+string.split(outfile,'.')[0]+' -ms 0,0,0,0 0,0'
    print(command)
    system(command)
    command = '/users/rhiju/python/make_new_plot.py '+string.split(outfile,'.')[0]+'.contacts -e -DC BP -NC NBP'
    print(command)
    system(command)

    command = '/users/rhiju/C/cluster_info_silent.out '+outfile+' '+coordfile+' '+string.split(outfile,'.')[0]+'.lowE -ms 0,0,0,0 0,0 -1 0.1'
    print(command)
    system(command)
    command = '/users/rhiju/python/make_new_plot.py '+string.split(outfile,'.')[0]+'.lowE.contacts -e -DC BP -NC NBP'
    print(command)
    system(command)
