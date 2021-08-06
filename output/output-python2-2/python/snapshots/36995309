#!/usr/bin/python

from sys import stdout,argv


actualpdbname = argv[1]
newsequence = argv[2]


removechain = 0
if argv.count('-nochain'):
    removechain = 1

#New sequence  -- assume its a fasta file.
lines = open(newsequence,'r').readlines();
new_sequence = lines[1][:-1]


# OK, read PDB.
lines = open(actualpdbname,'r').readlines()

outid = stdout
#outid = open( outfile, 'w')
#print 'Writing ... '+pdbname


oldresnum = '   '
count = 0;

for line in lines:
    if len(line)>5 and line[:6]=='ENDMDL':break #Its an NMR model.

    line_edit = line
    if line[0:3] == 'TER':
        continue
    elif (line[0:6] == 'HETATM') & (line[17:20]=='MSE'): #Selenomethionine
        line_edit = 'ATOM  '+line[6:17]+'MET'+line[20:]
        if (line_edit[12:14] == 'SE'):
            line_edit = line_edit[0:12]+' S'+line_edit[14:]
        if len(line_edit)>75:
            if (line_edit[76:78] == 'SE'):
                line_edit = line_edit[0:76]+' S'+line_edit[78:]

    if line_edit[0:4] == 'ATOM' or line_edit[0:6] == 'HETATM':
        resnum = line_edit[23:26]
        if not resnum == oldresnum:
            count = count + 1
            longname = line_edit[17:20]
        oldresnum = resnum

        if count > len(new_sequence): break

        newnum = '%3d' % count
        line_edit = 'ATOM  ' + line_edit[6:]
        line_edit = line_edit[0:23] + newnum + line_edit[26:]
        if removechain:
            line_edit = line_edit[0:21]+' '+line_edit[22:]

        line_edit = line_edit[0:17]+ '  '+new_sequence[count-1]+line_edit[20:]
        outid.write(line_edit)


outid.close()
