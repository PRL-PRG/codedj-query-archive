#!/usr/bin/python

from sys import argv,stdout,exit
from os import system

if len( argv) < 4:
    print
    print argv[0],' <phaser PDB> <ARPwARP PDB> <native>'
    print
    exit()

inputfiles = argv[1:]

pdbfiles = []
highlight_residues = []
for i in range( len(inputfiles) ):
    inputfile = inputfiles[i]
    try:
        highlight_residues.append( int( inputfile) )
    except:
        if (not inputfile.find("superposition") > 0):
            pdbfiles.append( inputfile)

pdbfiles.reverse()

#superimpose first!
command = "python ~rhiju/python/superimpose.py "
for pdbfile in pdbfiles: command += " "+pdbfile
prefix = pdbfiles[0][0:4]
command += " -R 1.0 > "+ prefix+"_superposition.pdb"
system(command)

#Extract models
command = "python ~rhiju/python/parse_NMR_models.py "+prefix+"_superposition.pdb"
system(command)

fid = open(prefix+'.pml','w')
#fid = stdout


fid.write('reinitialize\n')
count = 0
model_name = ['native','arpwarp_model','phaser_model'];
for pdbfile in pdbfiles:
    count += 1
    fid.write('load %s_superposition_%03d.pdb,%s\n' %
              (prefix,count, model_name[count-1]))


fid.write('hide everything,all\n')
fid.write('\n')
fid.write('bg_color white\n')
fid.write('\n')
fid.write('set backface_cull,0\n')
fid.write('set antialias,1\n')
fid.write('select hydro, elem H\n')
fid.write('select backbone, name o+c+n\n')
fid.write('select sc, not backbone and not hydro\n')
fid.write('select core_sc, sc and not hydro and buried \n')
fid.write('select Calpha, name ca+o+c+n\n')
fid.write('\n')
fid.write('set line_width=3.0\n')
#fid.write('show ribbon, Calpha\n')
#fid.write('set ribbon_samping, 1\n')
fid.write('show cartoon\n')
fid.write('color red,phaser_model\n')
fid.write('color blue,native\n')
fid.write('\n')
fid.write('color forest, arpwarp_model\n')
fid.write('set cartoon_rect_length,0.75\n')
fid.write('set cartoon_oval_width,0.5\n')
fid.write('set cartoon_oval_length, 0.5\n')
fid.write('set cartoon_transparency, 0.5,phaser_model\n')
fid.write('\n')
fid.write('select none\n')

fid.close()

print ' CHECK OUT: ',prefix+'.pml'
