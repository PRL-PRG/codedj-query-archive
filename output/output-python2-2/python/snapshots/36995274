#!/usr/bin/python

import string
from sys import argv
from amino_acids import extra_longer_names
import sys
from os.path import exists,basename
from math import floor
from whrandom import random

score_line_fields = ['score','env','pair','vdw','hs',
                     'ss','sheet','cb','rsigma','hb',
                     'rg','co','rama']

proper_label = {'r-sigma':'rsigma'}
for s in score_line_fields:
    proper_label[s] = s

args = argv

FOUND_POSE_LINE = 0

def Help():
    print '\n'+'-'*75
    print 'this program composes ab-initio pdbs into a silent file'
    print 'current score-line is composed of the following fields:'
    print string.join(score_line_fields)+'\n'
    print 'usage: %s <out_file> <pdb_list_file> {-new_tag <tag> {-new_tag <tag>}...}'%args[0]
    print '\n'+'-'*75+'\n'
    sys.exit()

if len(args) < 3:
    Help()

while args.count('-new_tag'):
    pos = args.index('-new_tag')
    new_tag = args[pos+1]
    score_line_fields.append(new_tag)
    proper_label[new_tag] = new_tag
    del args[pos]
    del args[pos]

residue_list = []
while args.count('-r'):
    pos = args.index('-r')
    start = int(string.split(args[pos+1],'-')[0])-1 ## command line uses rosetta nbrs
    stop = int(string.split(args[pos+1],'-')[1])-1
    residue_list = residue_list + range(start,stop+1)
    del args[pos]
    del args[pos]

if args.count('-no_seq_check'):
    pos = args.index('-no_seq_check')
    del args[pos]
    SEQ_CHECK = 0 ## only check that lengths are the same
else:
    SEQ_CHECK = 1

if args.count('-fullname'):
    pos = args.index('-fullname')
    del args[pos]
    FULLNAME = 1
else:
    FULLNAME = 0

if args.count('-noname'):
    pos = args.index('-noname')
    del args[pos]
    NONAME = 1
else:
    NONAME = 0


if args.count('-justname'):
    pos = args.index('-justname')
    del args[pos]
    JUSTNAME = 1
else:
    JUSTNAME = 0

if args.count('-extra_suffix'):
    pos = args.index('-extra_suffix')
    del args[pos]
    extra_suffix = args[pos]
    del args[pos]
else:
    extra_suffix = ''



def Read_pdb_file(filename):
    global FOUND_POSE_LINE,score_line_fields,proper_label
    print 'reading',filename
    coords = []
    sequence = ''
    phi = {}
    psi = {}
    omega = {}
    ss = {}
    problemo = 0
    score = {}

    data = open(filename,'r')
    line = data.readline()
    numbers = map(str,range(10))
    while line:
        l = string.split(line)
        if not l: ## empty line
            pass
        elif l[0] == 'POSE_SCORES:':
            #try:
            if 1:
                if not FOUND_POSE_LINE: ## setup the fields the first time through
                    score_line_fields = []
                    proper_label = {}
                if len(l)%2 == 1:
                    nscores = (len(l)-1)/2
                    for i in range(nscores):
                        tag = string.lower(l[2*i+1])
                        tag_score = float( l[2*i+2] )
                        if not FOUND_POSE_LINE:
                            score_line_fields.append( tag )
                            proper_label[tag] = tag

                        if proper_label.has_key( tag ):
                            score[proper_label[tag]] = tag_score

                FOUND_POSE_LINE = 1

            #except:
            else:
                print 'funny line:',line[:-1]
        elif line[:4] == 'ATOM' and line[13:15] == 'CA':
            sequence = sequence + extra_longer_names[line[17:20]]
            coords.append(map(float,[line[30:38],line[38:46],line[46:54]]))
        elif len(l)==2 and l[0][-1] == ':':
            tag = l[0][:-1]
            if proper_label.has_key(tag):
                score[proper_label[tag]] = float(l[1])
        elif l[0] == 'maxsub:':
            tag = string.split(line)[0][:-1]
            if proper_label.has_key(tag):
                score[proper_label[tag]] = float(l[-1])

        elif line[:8] == 'complete':
            for i in range(len(sequence)):
                line = string.split(data.readline())
                if len(line)<6 or line[0][0] not in numbers or int(line[0]) != i+1:
                    problemo = 1
                    break
                try:
                    phi[i] = float(line[2])
                    psi[i] = float(line[3])
                    omega[i] = float(line[4])
                    ss[i] = line[1]
                except:
                    problemo = 1
                    break
        line = data.readline()
    data.close()

    if problemo:
        sys.stderr.write('problem reading phi,psi,omega for '+filename+'\n')
        #sequence = ''
        ss = {}

    return sequence,score,ss,phi,psi,omega,coords

def Write(out,name,score,ss,phi,psi,omega,coords,residue_list):
    out.write('SCORE:   ')
    for tag in score_line_fields:
        if tag in score.keys():
            out.write(' %f'%score[tag])
        else:
            out.write(' %f'%0.0)
    out.write(' '+name+'\n')

    for i in range(len(residue_list)):
        pos = residue_list[i]
        out.write("%4d %s %9.3f%9.3f%9.3f%9.3f%9.3f%9.3f %s\n" \
                  %(i+1,ss[pos],phi[pos],psi[pos],omega[pos],
                    coords[pos][0],coords[pos][1],coords[pos][2],
                    name))
    return



out_file = args[1]
list_file = args[2]

#if exists(out_file):
#    lines = open(out_file,'r').readlines()
#    if lines and ( len(lines)>1 or len(lines[0])>1 ):
#        print '\n\nout_file: %s already exists!'%out_file
#        Help()

if not exists(list_file):
    print '\n\ncant open list_file:'.list_file
    Help()

pdb_files = map(lambda x:string.split(x)[0],open(list_file,'r').readlines())

base_sequence = ''
out = open(out_file,'w')

if NONAME:
    name_file = open(out_file+'.names','w')

counter = 1
for file in pdb_files:
    if not counter%100:print counter
    sequence,score,ss,phi,psi,omega,coords = Read_pdb_file(file)
    if not sequence:
        sys.stderr.write('empty sequence: %s\n'%file)
        continue

    if not base_sequence:
        ## setup residue_list
        if not residue_list:
            residue_list = range(len(sequence))

        silent_sequence = ''
        for pos in residue_list:
            silent_sequence = silent_sequence + sequence[pos]

        out.write('SEQUENCE: '+silent_sequence+'\n')
        base_sequence = sequence
        L = len(base_sequence)
        out.write('SCORE: ')
        for tag in score_line_fields:
            out.write(' %s'%tag)
        out.write(' description\n')

    if (SEQ_CHECK and sequence != base_sequence) or \
       len(sequence) != L:
        sys.stderr.write('bad sequence %s %d %d\n'%(file,len(sequence),L))
        continue
    if 0 and len(score.keys()) != len(score_line_fields):
        sys.stderr.write('%s missing some scoreline fields:'%file)
        for tag in score_line_fields:
            if not score.has_key(tag):
                sys.stderr.write('%8s'%tag)
            sys.stderr.write('\n')

    if not ss:
        sys.stderr.write('%s couldnt find ss/angle info\n'%file)
        for i in range(len(sequence)):
            ss[i] = 'L'
            phi[i] = 0.0
            psi[i] = 0.0
            omega[i] = 0.0

    name = 'S_'+'0'*(5-len(str(counter)))+str(counter)
    if FULLNAME:
        name = name+'_'+file
    elif NONAME:
        mod_counter = counter%9999
        rnd = int(floor(random() * 9999))
        name = 'S_%04d_%04d'%(mod_counter,rnd)
        name_file.write('%s %d %s\n'%(name,counter,file))
    else:
        name = name+'_'+string.split(file,'/')[-1]

    if JUSTNAME:
        name = basename(file).replace('.pdb','')

    name += extra_suffix

    counter = counter + 1
    Write(out,name,score,ss,phi,psi,omega,coords,residue_list)
out.close()

if NONAME:
    name_file.close()
