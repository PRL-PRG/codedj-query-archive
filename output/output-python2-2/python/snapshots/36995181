#!/usr/bin/python

from sys import argv, exit
from os import system, popen
from os.path import basename
import string

pdbfile_nterm = argv[1]
pdbfile_cterm = argv[2]
res1 = int( argv[3] )
res2 = int( argv[4] )

pdbfile_nterm_list = [pdbfile_nterm]
pdbfile_cterm_list = [pdbfile_cterm]

if pdbfile_nterm[-5:] == '.list':
    pdbfile_nterm_list = map( lambda x:x[:-1], open(pdbfile_nterm,'r').readlines())

if pdbfile_cterm[-5:] == '.list':
    pdbfile_cterm_list = map( lambda x:x[:-1], open(pdbfile_cterm,'r').readlines())

for pdbfile_nterm in pdbfile_nterm_list:
    for pdbfile_cterm in pdbfile_cterm_list:

        command = '/users/rhiju/python/termini_truncate_pdb.py %s %d %d > temp_nterm.pdb' % (pdbfile_nterm, res1, res2)
        print(command)
        system(command)

        command = '/users/rhiju/python/termini_truncate_pdb.py %s %d %d > temp_cterm.pdb' % (pdbfile_cterm, res1, res2)
        print(command)
        system(command)

        command = '/users/rhiju/python/superimpose.py temp_nterm.pdb temp_cterm.pdb > blah2.pdb'
        print(command)
        system(command)

        # Following from superimpose.py (Phil's script)
        pdb1 = pdbfile_nterm
        pdb = pdbfile_cterm

        file = 'maxsub_sup.pdb'
        matrix = map(lambda x:map(float,string.split(x)[1:]),
                     popen('grep -A3 "Transformation Matrix" %s'%file).readlines()[1:4])


        P_translation = map(float,
                            string.split(popen('grep -A1 "Translation vector (Pred" %s'\
                                               %file).readlines()[1])[1:])

        E_translation = map(float,
                 string.split(popen('grep -A1 "Translation vector (Exp" %s'\
                                    %file).readlines()[1])[1:])


        def E_transform(v,matrix,tP,tE):
            ans = [0.0]*3
            for i in range(3):
                for j in range(3):
                    ans[i] = ans[i] + matrix[i][j]*(v[j]-tE[j])
                ans[i] = ans[i] + tP[i]

            return ans

        atom_count = 0
        fid = open('rotated_nterm.pdb','w')
        data = open(pdb1,'r')
        line = data.readline()
        while line:
            if line[:4] in ['ATOM','HETA']:
                atom_count = atom_count + 1
                fid.write('%s  %5d%s\n'%(line[:4],atom_count,line[11:-1]))
            elif line[:6] == 'ENDMDL':break
            line = data.readline()
        data.close()
        fid.close()


        atom_count = 0
        fid = open('rotated_cterm.pdb','w')
        data = open(pdb,'r')
        line = data.readline()
        while line:
            if line[:4] in ['ATOM','HETA']:
                    atom_count = atom_count + 1
                    pos = E_transform(map(float,[line[30:38],line[38:46],line[46:54]]),
                                      matrix,
                                      P_translation,
                                      E_translation)

                    fid.write( '%s  %s%8.3f%8.3f%8.3f%s\n'\
                              %(line[:4],line[6:30],pos[0],pos[1],pos[2],line[54:-1]))


            elif line[:6] == 'ENDMDL':break
            line = data.readline()
        data.close()
        fid.close()


        ### OK, now get N terminus from one, C terminus from the other.
        if len(argv)>5:
            cutpoint = int( argv[5] )
        else:
            cutpoint = (res1 + res2) / 2

        command = '/users/rhiju/python/termini_truncate_pdb.py rotated_nterm.pdb 1 %d > recombinant.pdb' % cutpoint
        print(command)
        system(command)

        command = '/users/rhiju/python/termini_truncate_pdb.py rotated_cterm.pdb %d 9999 >> recombinant.pdb' % (cutpoint+1)
        print(command)
        system(command)


        #### Idealize
        command = '/users/rhiju/rosetta++/rosetta.gcc -idealize -s recombinant.pdb -paths /users/rhiju/paths.txt '
        print(command)
        system(command)

        ### Clean up time!

        new_pdb_file_name = basename(string.split(pdbfile_nterm,'.pdb')[0]) + '_' + basename(pdbfile_cterm)
        command = 'mv recombinant_0001.pdb %s ' % new_pdb_file_name
        print(command)
        system(command)

#        command = 'rm blah.pdb blah2.pdb maxsub*pdb recom*pdb rot*pdb temp*pdb'
        print(command)
        system(command)



