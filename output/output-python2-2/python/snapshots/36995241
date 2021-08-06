#!/usr/bin/python

from sys import argv,stderr,exit

def Help():
    print argv[0]+' pdb1 pdb2 ... [-n native_pdb] > yourscript.pml'
    print
    exit()


if len(argv) < 2:
    Help()

pdbfiles = argv[1:]

print 'reinitialize'

if pdbfiles.count( "-n"):
    pos = pdbfiles.index("-n")
    native_pdbfile = pdbfiles[pos+1]
    del( argv[pos] )
    del( argv[pos] )
    print 'load %s,native' % (native_pdbfile)


count = 0
for pdbfile in pdbfiles:
    if not pdbfile == native_pdbfile:
        count += 1
        print 'load %s,mov,%d' % (pdbfile,count)

print 'hide everything,all'
print 'set cartoon_fancy_helices,1'
print 'select hydro, elem H'
print 'select backbone, name o+c+n'
print 'select sc, not backbone and not hydro'
print 'show sticks,sc'
print 'show cartoon,all'
print 'bg_color white'
print 'cmd.spectrum( selection = "mov" )'
print 'color white,native'
print 'set cartoon_transparency,0.5,native'
