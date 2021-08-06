#!/usr/bin/python

from sys import argv
from os import system,popen
from os.path import exists
import string

infofile = argv[1]

arguments = ''
if len(argv) > 2:
    arguments = string.join(argv[2:],' ')


if infofile[-5:] == '.info':
    infofile = infofile[:-5]


suffix=''
if argv.count('-DC') and argv.count('BP'):
    suffix = 'DC:BP.'

command = '/work/casp7/python/make_new_plot.py -e %s.contacts %s' % (infofile,arguments)
print(command)
system(command)

contactmapfiles = []
contactmapfiles.append( '%s.contacts.eps' % infofile)

lines = popen('grep CLUSTER_INFO %s.info' % infofile).readlines()
clusternames = [ string.split(x)[2] for x in lines]
clusternames = clusternames[:11]

for clustername in clusternames:
    command = '/work/casp7/python/make_new_plot.py -e %s.%s.contacts %s' % (infofile,clustername,arguments)
    print(command)
    system(command)
    contactmapfile =  '%s.%s.contacts.%seps' % (infofile,clustername,suffix)
    assert( exists(contactmapfile) )
    contactmapfiles.append(contactmapfile)


command = 'rm -f %s.contactmaps.ps' % (infofile)
print command
system(command)

contactmapfiles = string.join(contactmapfiles,' ')

command = '/work/casp7/python/combine_new_plots.py %s.contactmaps.ps 4 3 %s -l' % (infofile,contactmapfiles)
print command
system(command)

command = 'rm -rf %s' % (contactmapfiles)
print command
system(command)
