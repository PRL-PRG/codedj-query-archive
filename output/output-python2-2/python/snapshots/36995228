#!/usr/bin/python

from sys import argv
from os import popen,system
import string

outfile = argv[1]
tags = argv[2:]

for tag in tags:
    lines = popen('grep %s %s' % (tag,outfile) ).readlines()


    N =104
    chainA =[]
    chainB =[]
    for line in lines:
        cols=string.split( line )
        try:
            res = int( cols[0] )
        except:
            continue
        if res <= N:
            chainA.append( [ float( cols[5] ), float( cols[6] ), float( cols[7] ) ] )
        else:
            chainB.append( [ float( cols[5] ), float( cols[6] ), float( cols[7] ) ] )



    def outputCA( tag, chain, filename):
        lines = open( tag+'.pdb').readlines()
        fid=open(filename,'w')
        for line in lines:
            if line[0:4] == 'ATOM' and line[12:15]==' CA':
                res =  int( line[23:26 ] )
                if res <= N:
                    line = '%s%8.3f%8.3f%8.3f%s' % (line[0:30],
                                                    chain[res-1][0],
                                                    chain[res-1][1],
                                                    chain[res-1][2],
                                                    line[56:])
                    fid.write(line)
        fid.close()



    outputCA( tag, chainA, 'chainA.pdb')
    outputCA( tag, chainB, 'chainB.pdb')


    command = 'termini_truncate_pdb.py %s 1 %d > monomer.pdb' % (tag+'.pdb', N)
    print(command)
    system(command)


    def rotate_pdb(monomerfile, chainfile, chainid):
        command = '/users/rhiju/python/superimpose.py %s %s > blah.pdb' % (chainfile,monomerfile)
        print(command)
        system(command)

        # Following from superimpose.py (Phil's script)
        file = 'maxsub_sup.pdb'
        matrix = map(lambda x:map(float,string.split(x)[1:]),
                     popen('grep -A3 "Transformation Matrix" %s'%file).readlines()[1:4])
        P_translation = map(float,
                            string.split(popen('grep -A1 "Translation vector (Pred" %s'\
                                               %file).readlines()[1])[1:])

        E_translation = map(float,
                            string.split(popen('grep -A1 "Translation vector (Exp" %s'\
                                               %file).readlines()[1])[1:])

        lines = []

        def E_transform(v,matrix,tP,tE):
            ans = [0.0]*3
            for i in range(3):
                for j in range(3):
                    ans[i] = ans[i] + matrix[i][j]*(v[j]-tE[j])
                ans[i] = ans[i] + tP[i]

            return ans

        atom_count = 0
        data = open(monomerfile,'r')
        line = data.readline()
        while line:
            if line[:4] in ['ATOM','HETA']:
                atom_count = atom_count + 1
                pos = E_transform(map(float,[line[30:38],line[38:46],line[46:54]]),
                                  matrix,
                                  P_translation,
                                  E_translation)

                lines.append( '%s  %s%1s%s%8.3f%8.3f%8.3f%s\n'\
                           %(line[:4],line[6:21],chainid,line[22:30],pos[0],pos[1],pos[2],line[54:-1]))
            elif line[:6] == 'ENDMDL':break
            line = data.readline()
        data.close()

        return lines


    linesA = rotate_pdb( 'monomer.pdb','chainA.pdb','A')
    linesB = rotate_pdb( 'monomer.pdb','chainB.pdb','B')

    fid = open(tag+'.pdb','w')
    for line in linesA:
        fid.write(line)
    for line in linesB:
        fid.write(line)

command = 'rm blah.pdb chainA.pdb chainB.pdb rasmol.tcl monomer.pdb maxsub*pdb'
