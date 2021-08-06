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
from sys import argv

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

#############################################################
if 1:
    p_file = argv[1]
    allfiles = argv[1:-1]

    R = float(argv[-1])

    good_numbers = {}
    translation = {}
    seq = {}
    numbers = {}
    matrix={p_file:[]}

#Define a list of all the files that need to be aligned
    e_files = []
    for file in allfiles[1:]:
        e_files.append(  file )

    Transform = {p_file:P_transform}

    for e_file in e_files:
       files = [p_file, e_file]
       Transform[e_file] = E_transform

       e_file_nopath = string.split(e_file,'/')[-1]
       system('~pbradley/maxsub/maxsub -e %s -p %s -R %f -o %s'\
              %(e_file,p_file,R,
                e_file_nopath+'.mammoth'))

    ## read the transformations
       file = 'maxsub_sup.pdb'
       matrix[e_file] = map(lambda x:map(float,string.split(x)[1:]),
                    popen('grep -A3 "Transformation Matrix" %s'%file).readlines()[1:])

       translation[p_file] = map(float,
                                 string.split(popen('grep -A1 "Translation vector (Pred" %s'%file).readlines()[1])[1:])

       translation[e_file] = map(float,
                                 string.split(popen('grep -A1 "Translation vector (Exp" %s'%file).readlines()[1])[1:])

       for file in files:
           lines = popen('grep "^ATOM.*CA" '+file).readlines()
           seq[file] = ''
           numbers[file] = []

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

       good_numbers[e_file] = {}
       good_numbers[p_file] = {}
       for i in align_E2P.keys():
           good_numbers[e_file][numbers[e_file][i]] = numbers[e_file][i]
           good_numbers[p_file][numbers[p_file][align_E2P[i]]] = numbers[e_file][i]

       chain = 'A'


    files = [p_file]+e_files
    for file in files:
        file_nopath = string.split(file,'/')[-1]
        out = open(file_nopath+'.sup','w')
        lines = popen('grep "^ATOM" '+file).readlines()
        for line in lines:
            number = int(line[22:26])
            #            if good_numbers[file].has_key(number):
            if 1:
                pos = map(float,[line[30:38],
                                 line[38:46],
                                 line[46:54]])

                pos = Transform[file](pos,matrix[file],translation[file])
                #                new_number = good_numbers[file][number]
                new_number = number

                out.write('%s%s%4d%s%8.3f%8.3f%8.3f%s'\
                          %(line[:21],chain,new_number,line[26:30],
                            pos[0],pos[1],pos[2],
                            line[54:]))
        out.close()

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
    fid.write('load %s.sup, native\n' % p_file );
    count = 0
    for e_file in e_files:
        count += 1
        fid.write('load %s.sup, prediction%d\n' % (e_file,count));
    fid.write('\n');
    fid.write('hide everything, all\n');
    fid.write('\n');

    fid.write('select hydro, elem H\n');
    fid.write('select backbone, name o+c+n\n');
    fid.write('select sc, not backbone and not hydro\n');
    fid.write('select core_sc, sc and not hydro and buried \n');
    fid.write('\n');
    fid.write('show cartoon, all\n');
#    fid.write('show sticks, core_sc\n');
    fid.write('\n');
    fid.write('bg_color white\n');
    fid.write('\n');
    fid.write('select native_align, native and aligned\n');
    fid.write('select prediction_align, prediction and aligned\n');
    fid.write('cmd.spectrum(selection = "native")\n');

    count = 0
    for e_file in e_files:
        count += 1
        fid.write('cmd.spectrum(selection = "prediction%d")\n' % count);
#    fid.write('color white, notalign\n');
    fid.write('\n');
    fid.write('\n');
    fid.close()



fid = open('outputgraphics1.pml','w')
fid.write('hide all\n')
fid.write('show cartoon,native\n')
fid.write('cmd.refresh()\n')
fid.write('ray 1200,800\n')
fid.write('save native.png\n')
fid.write('\n')


count = 0
for e_file in e_files:
    count += 1
    fid.write('hide all\n')
    fid.write('show cartoon,prediction%d\n' % count)
    fid.write('cmd.refresh()\n')
    fid.write('ray 1200,800\n')
    fid.write('save prediction%d.png\n' % count)



print 'Now run in pymol: TEST.pml'
print 'Rotate to desired view. Run outputgraphics1.pml.'
