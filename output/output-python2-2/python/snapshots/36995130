#!/usr/bin/python

## used to be ~/python/2002/scratch11_29.py

## run maxsub, get superpositions

## Edited by Rhiju from Phil's script to include pymol output.

import string
from glob import glob
from os import system, popen
from os.path import exists
from amino_acids import extra_longer_names
from popen2 import popen2
from blast import NBAlign
from sys import argv,exit

def Rclean(rin,rout):
    rin.write('select none\n')
    rin.flush()
    line = rout.readline()
    while line and (len(line)<10 or line[:10] != 'No atoms s'):
        #print line[:-1]
        line = rout.readline()
    return

def Mammoth_align(data): ## this version returns empty if z<MIN_Z_SCORE

    BLOCK = 10
    NBLOCK = 5
    MIN_Z_SCORE = -20

    line = data.readline()
    while line and line[:4] not in ['Z-sc',' Z-s']:
        line = data.readline()

    if len(string.split(line))<2:
        data.close()
        log('Bad mammoth data\n')
        return 0.0,'','','','',{}

    z = float(string.split(line)[1])

    if z<MIN_Z_SCORE:
        data.close()
        return z,'','','','',{}

    while line and line[0:4] != 'Pred':line = data.readline()
    al1 = ''
    al2 = ''
    bars = ''
    stars = ''
    ss1 = ''
    ss2 = ''
    align = []
    while line and line[0:4] == 'Pred':
        #print line[:-1]
        line = line[11:-1] + ' '*(NBLOCK*BLOCK+NBLOCK+10 - len(line) +1)
        #print line
        for i in range(NBLOCK):
            for j in range(BLOCK):
                #print j+i*BLOCK+i,len(line)
                al1 = al1 + line[j+i*BLOCK+i]
        line = data.readline()
        line = line[11:-1] + ' '*(NBLOCK*BLOCK+NBLOCK+10 - len(line) +1)
        for i in range(NBLOCK):
            for j in range(BLOCK):
                #print j+i*BLOCK+i,len(line)
                ss1 = ss1 + line[j+i*BLOCK+i]
        line = data.readline()
        line = line[11:-1] + ' '*(NBLOCK*BLOCK+NBLOCK+10 - len(line) +1)
        for i in range(NBLOCK):
            for j in range(BLOCK):
                bars = bars + line[j+i*BLOCK+i]
        line = data.readline()
        line = line[11:-1] + ' '*(NBLOCK*BLOCK+NBLOCK+10 - len(line) +1)
        for i in range(NBLOCK):
            for j in range(BLOCK):
                #print j+i*BLOCK+i,len(line)
                ss2 = ss2 + line[j+i*BLOCK+i]
        line = data.readline()
        line = line[11:-1] + ' '*(NBLOCK*BLOCK+NBLOCK+10 - len(line) +1)
        for i in range(NBLOCK):
            for j in range(BLOCK):
                al2 = al2 + line[j+i*BLOCK+i]
        line = data.readline()
        line = line[11:-1] + ' '*(NBLOCK*BLOCK+NBLOCK+10 - len(line) +1)
        for i in range(NBLOCK):
            for j in range(BLOCK):
                stars = stars + line[j+i*BLOCK+i]
        line = data.readline()
        line = data.readline()
        line = data.readline()
    data.readlines()
    data.close()

    assert string.count(al1,'-')==0
    assert len(al1) == len(al2) == len(stars) == len(bars) == len(ss1) == len(ss2)

    align = []
    for i in range(len(al1)):
        if stars[i] == '*':
#        if bars[i] == '|':
            ## numbering starts at 0
            pos1 = i - string.count(al1[:i],'.')
            pos2 = i - string.count(al2[:i],'.')
            align.append([pos1,pos2])

    seq1 = string.join(string.split(string.join(string.split(al1,'.'),''),' '),'')
    ss1 = string.join(string.split(string.join(string.split(ss1,'.'),''),' '),'')
    seq2 = string.join(string.split(string.join(string.split(al2,'.'),''),' '),'')
    ss2 = string.join(string.split(string.join(string.split(ss2,'.'),''),' '),'')
    return z,seq1,seq2,ss1,ss2,align
################################################################

if 1:
    p_file = argv[1]
    e_file = argv[2]

    files = [p_file,e_file]


    R = float(argv[3])

    system('/work/pbradley/maxsub/maxsub -e %s -p %s -R %f -o %s'\
           %(e_file,p_file,R,
             p_file+'.mammoth'))

    ## read the transformations
    file = 'maxsub_sup.pdb'
    matrix = map(lambda x:map(float,string.split(x)[1:]),
                 popen('grep -A3 "Transformation Matrix" %s'%file).readlines()[1:])

    matrix_identity = [[1,0,0],[0,1,0],[0,0,1]]

    translation = {}
    translation[p_file] = map(float,
             string.split(popen('grep -A1 "Translation vector (Pred" %s'%file).readlines()[1])[1:])

    translation[e_file] = map(float,
             string.split(popen('grep -A1 "Translation vector (Exp" %s'%file).readlines()[1])[1:])


    translation_back = map(lambda x:-x, translation[p_file])

    def P_transform(v,matrix,t):
        ans = [0.0]*3
        for i in range(3):
            ans[i] = v[i] - t[i]
        return ans

    def E_transform(v,matrix,t):
        ans = [0.0]*3
        for i in range(3):
            for j in range(3):
                ans[i] = ans[i] + matrix[i][j]*(v[j]-t[j])


        return ans

    Transform = {p_file:P_transform,
                 e_file:E_transform}


    seq = {}
    numbers = {}
    for file in files:
        lines = popen('grep "^ATOM.*CA" '+file).readlines()
        seq[file] = ''
        numbers[file] = []

        number_old = '   '
        for line in lines:
            rsd = line[17:20]
            number = int(line[22:26])
            if rsd in extra_longer_names.keys():
                seq[file] = seq[file] + extra_longer_names[rsd]
            else:
                seq[file] = seq[file] + 'X'
            numbers[file].append(number)

    s1 = seq[e_file]
    s2 = seq[p_file]

    align_E2P = NBAlign(s1,s2)
    print 'len(e_seq) %d len(p_seq) %d aligned: %d'\
          %(len(s1),len(s2),len(align_E2P.keys()))

    good_numbers = {}
    good_numbers[e_file] = {}
    good_numbers[p_file] = {}
    for i in align_E2P.keys():
        good_numbers[e_file][numbers[e_file][i]] = numbers[e_file][i]
        good_numbers[p_file][numbers[p_file][align_E2P[i]]] = numbers[e_file][i]



    chain = 'A'

    for file in files:

        out = open(file+'.sup','w')
        lines = popen('grep "^ATOM" '+file).readlines()
        for line in lines:
            number = int(line[22:26])
            if good_numbers[file].has_key(number):
                pos = map(float,[line[30:38],
                                 line[38:46],
                                 line[46:54]])

                pos = Transform[file](pos,matrix,translation[file])
#Translate back
                pos = Transform[file](pos,matrix_identity,translation_back)
                new_number = good_numbers[file][number]

                out.write('%s%s%4d%s%8.3f%8.3f%8.3f%s'\
                          %(line[:21],chain,new_number,line[26:30],
                            pos[0],pos[1],pos[2],
                            line[54:]))
        out.close()

    ## read the mammoth alignment, map back to seq[e_file]
    z,seq1,seq2,ss1,ss2,align = Mammoth_align(open(p_file+'.mammoth','r'))


    assert string.count(seq[e_file],seq2)
    offset = string.find(seq[e_file],seq2)

    aligned_numbers = []
    for pos in map(lambda x:x[1]+offset,align): ## starts at 0
        if pos in align_E2P.keys():
            aligned_numbers.append( numbers[e_file][pos] )



    #Also would like an estimate of what residues are buried... check out the natives
    rescored_native = string.split(e_file,'.pdb')[0] + '_0001.pdb'
    if not exists(rescored_native):
        command = '~rhiju/src/rosetta_scale_hessian/rosetta.gcc -read_all_chains -score -fa_input -s %s -nstruct 1 -scorefile blah -decoyfeatures -paths ~rhiju/paths.txt' % e_file
        print(command)
        system(command)

    #In the output pdb there's a useful column...

    DFlines = popen('grep "DF  " '+rescored_native).readlines()
    buried = []
    for line in DFlines:
        cols = string.split(line)
        SASAfrac = float(  cols[5] )
        if (SASAfrac < 0.2):
            buried.append( int( cols[1] ))

    #Create a pymol file too.
    fid = open('TEST.pml','w')
    fid.write('delete *\n');
    fid.write('set cartoon_round_helices,1\n');
    fid.write('set cartoon_fancy_helices,1\n');
    fid.write('set cartoon_fancy_sheets,1\n');
    fid.write('set cartoon_discrete_colors,0\n');
    fid.write('set backface_cull,0\n');
    fid.write('set antialias,1\n');
    fid.write('\n');
    fid.write('load %s.sup, native\n' % e_file );
    fid.write('load %s.sup, prediction\n' % p_file);
    fid.write('\n');
    fid.write('hide everything, all\n');
    fid.write('\n');

    notalign = []
    align = []
    for pos in good_numbers[p_file].values():
        if pos not in aligned_numbers:
            notalign.append(pos)
        else:
            align.append(pos)
    fid.write('\n')

    fid.write('select notaligned, resi ')
    for pos in notalign:
        fid.write('%d' % pos);
        if pos == notalign[-1]:
            fid.write('\n')
        else:
            fid.write('+')
    fid.write('\n')

    fid.write('select aligned, resi ')
    for pos in align:
        fid.write('%d' % pos);
        if pos == align[-1]:
            fid.write('\n')
        else:
            fid.write('+')

    #    print 'e_file ', numbers[e_file], len( numbers[e_file] )
    #    print 'p_file ', numbers[p_file], len( numbers[p_file] )

    fid.write('select buried, resi ')
    for pos in buried:
        print pos, numbers[e_file][pos-1]
        fid.write('%d' % numbers[e_file][pos-1] )
        if pos == buried[-1]:
            fid.write('\n')
        else:
            fid.write('+')



    fid.write('select continuous_aligned, resi %d-%d\n' %  (min(align),max(align)))
    fid.write('select hydro, elem H\n');
    fid.write('select backbone, name o+c+n\n');
    fid.write('select sc, not backbone and not hydro or (name n and resn pro)\n');
    fid.write('select core_sc, sc and not hydro and buried and aligned\n');
    fid.write('\n');
    fid.write('show cartoon, all\n');
#    fid.write('show sticks, core_sc\n');
    fid.write('\n');
    fid.write('bg_color white\n');
    fid.write('\n');
    fid.write('select native_align, native and aligned\n');
    fid.write('select prediction_align, prediction and aligned\n');
    fid.write('cmd.spectrum(selection = "native_align")\n');
    fid.write('cmd.spectrum(selection = "prediction_align")\n');
    fid.write('color white, notaligned\n');
    fid.write('\n');
    fid.write('\n');
    fid.close()



fid = open('outputgraphics1.pml','w')
fid.write('hide all\n')
fid.write('show cartoon,native\n')
fid.write('cmd.refresh()\n')
fid.write('ray 800,1200\n')
fid.write('png native.png,dpi=300\n')
fid.write('\n')
fid.write('hide all\n')
fid.write('show cartoon,prediction\n')
fid.write('cmd.refresh()\n')
fid.write('ray 800,1200\n')
fid.write('png prediction.png,dpi=300\n')
fid.write('save prediction.jpg\n')
fid.write('\n')
fid.write('hide all\n')
fid.write('show cartoon,native\n')
fid.write('show cartoon,prediction\n')
fid.write('cmd.refresh()\n')
fid.write('ray 800,1200\n')
fid.write('png superposition.png,dpi=300\n')
fid.close()


fid = open('TEST2.pml','w')
fid.write('hide all\n')
fid.write('show cartoon, native_align\n')
fid.write('show cartoon, prediction_align\n')
fid.write('cmd.color(2, "native_align")\n')
fid.write('color red, "prediction_align"\n')
fid.write('show sticks, core_sc\n')
fid.close()

fid = open('outputgraphics2.pml','w')
fid.write('cmd.refresh()\n')
fid.write('ray 800,1200 \n')
fid.write('save superposition_sidechains.png\n')
fid.write('save superposition_sidechains.jpg\n')
fid.close()

print 'Now run in pymol: TEST.pml'
print 'Rotate to desired view. Run outputgraphics1.pml.'
print 'Then run TEST2.pml.'
print 'Rotate to desired view. Run outputgraphics2.pml.'

