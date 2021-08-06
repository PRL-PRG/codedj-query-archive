## generate coord files for scop_family

import string
from glob import glob
from amino_acids import longer_names,extra_longer_names
from math import sqrt,floor
from blast import NBAlign
import sys
from os.path import exists


class Atom_line_object:
    def __init__(self,line):
        assert line[:4] in ['ATOM','HETA']
        self.chain = line[21]
        self.atom_number = int(line[6:11])
        self.atom_name = line[12:16]
        self.position = [float(line[30:38]),float(line[38:46]),float(line[46:54])]
        self.alt_loc = line[16]
        self.residue_name = line[17:20]
        self.residue_number = int(line[22:26])
        self.insertion_code = line[26]
        self.residue_id = line[22:27]

short_to_long = {'X':'UNK'}
for a in longer_names.keys():short_to_long[longer_names[a]] = a

def Pad43(x):
    # pad a float to fit the PDB file format: 4.3
    x = floor(1000*x)/1000
    x = str(x)
    assert string.count(x,'.')
    x = string.split(x,'.')
    x = ' '*(4-len(x[0]))+x[0]+'.'+x[1]+'0'*(3-len(x[1]))
    return x
def Pad32(x):
    # pad a float to fit the PDB file format: 3.2
    x = floor(100*x)/100
    x = str(x)
    assert string.count(x,'.')
    x = string.split(x,'.')
    x = ' '*(3-len(x[0]))+x[0]+'.'+x[1]+'0'*(3-len(x[1]))
    return x



def Atom_line(atom_number,name,rsd,chain,rsd_number,position,b_factor=50.0):
    assert 0<=b_factor<=100.0
    line = 'ATOM'
    an = str(atom_number)
    line = line+' '*(7-len(an))+an+' '
    if len(name)==4:
        line = line+name+' '
    else:
        line = line+' '+name+' '*(4-len(name))
    line = line+short_to_long[rsd]+' '+chain
    rn = str(rsd_number)
    line = line + ' '*(4-len(rn))+rn+'    '
    line = line+Pad43(position[0])
    line = line+Pad43(position[1])
    line = line+Pad43(position[2])+'  1.00%6.2f'%b_factor+' '*11+name[0]+'\n'
    return line

def Length(v):return sqrt(v[0]**2+v[1]**2+v[2]**2)
def Dist(a,b): return Length([a[0]-b[0],a[1]-b[1],a[2]-b[2]])

MAX_D = 4.5

def Get_chains(file):
    data = open(file,'r')
    line = data.readline()

    prev_chain = ''
    chains = []

    while line:
        if line[:4] in ['ATOM','HETA'] and \
           line[12:16] == ' CA ' and \
           line[16] in [' ','1','A']:
            chain = line[21]
            if chain != prev_chain:
                if chain not in chains:chains.append(chain)
        line = data.readline()
    data.close()
    return chains

def Check_rosetta(file,chain): ## check for rosetta problems:
    ## 1.chain breaks
    ## 2.missing bb atoms
    ## 3.non-standard aa names
    if not exists(file):
        sys.stderr.write('Check_rosetta: missing file: %s\n'%file)
        return 0

    bb_atoms = {' N  ':0,
                ' CA ':1,
                ' C  ':2,
                ' O  ':3,
                ' OT1':3}

    if chain == '_':chain = ' '

    prev_number = -10000
    prev_insert = ''
    in_chain = 0
    counter = -1
    prev_ca_pos = []

    chain_break = 0
    bad_residue = 0
    bb = {}

    data = open(file,'r')
    line = data.readline()
    while line:
        if (line[:3] == 'TER' or line[:6] == 'ENDMDL') and in_chain:
            break

        if line[:6] != 'ATOM  ':
            if line[:6] == 'HETATM ':
                print 'SKIP:',line[:-1]
            line = data.readline()
            continue

        l = Atom_line_object(line)
        if l.chain == chain and \
           l.atom_name in bb_atoms.keys() and \
           l.alt_loc not in ['B','C','D']: ## alt_loc check in rosetta

            in_chain = 1

            if prev_number != l.residue_number or \
               prev_insert != l.insertion_code: ## in a new residue

                counter = counter + 1
                prev_number = l.residue_number
                prev_insert = l.insertion_code
                bb[counter] = {}

            bb [counter] [bb_atoms[l.atom_name] ] = 1


            ## check for chain break
            if l.atom_name == ' CA ':
                ca_pos = l.position

                if prev_ca_pos and Dist(ca_pos,prev_ca_pos) > MAX_D:

                    chain_break = 1
                    sys.stderr.write('Check_rosetta: chain_break %s %s %s %d %f\n'\
                                 %(file,l.residue_number,l.insertion_code,
                                   counter,Dist(ca_pos,
                                                prev_ca_pos)))
                    break
                prev_ca_pos = ca_pos

            ## check for non-standard residue name
            if not longer_names.has_key (l.residue_name):
                bad_residue = 1
                sys.stderr.write('Check_rosetta: bad_residue %s %s %s %d %s\n'\
                             %(file,l.residue_number,l.insertion_code,
                               counter,l.residue_name))
                break


        line = data.readline()

    data.close()

    if counter == -1:
        sys.stderr.write('Check_rosetta: failed to find coordinates %s %s\n'\
                         %(file,chain))
        return 0

    if chain_break or bad_residue:
        return 0

    missing_atoms = 0
    for i in range( counter+1):
        if len(bb[i].keys()) != 4:
            sys.stderr.write('Check_rosetta: %s rsd %d is missing bbatoms %s\n'\
                         %(file,i,`bb[i].keys()`))
            missing_atoms = 1

    if missing_atoms:
        return 0
    return 1

def Chain_breaks(file,chain):
    data = open(file,'r')
    line = data.readline()

    prev = []
    in_chain = 0
    ca_seq = ''
    chain_breaks = []

    while line:

        if line[:4] in ['ATOM','HETA'] and \
           line[21] == chain and \
           line[12:16] == ' CA ' and \
           line[16] in [' ','1','A']:

            in_chain = 1
            p = [float(line[30:38]),float(line[38:46]),float(line[46:54])]


            if prev and Dist(p,prev) > MAX_D:
                ##just crossed a gap
                chain_breaks.append(Dist(p,prev))

            if line[17:20] in extra_longer_names.keys():
                ca_seq = ca_seq + extra_longer_names[line[17:20]]
            else:
                ca_seq = ca_seq + 'X'

            prev = p
        elif (line[:3] == 'TER' or line[:6] == 'ENDMDL') and in_chain: break

        line = data.readline()
    data.close()

    if not in_chain:
        print 'couldnt find chain'

    return len(chain_breaks),ca_seq


def Get_full_coords(file,sequence,chain,VERBOSE=0,SKIP_TER=0):
    ## read all-atom coords from pdb file, parse into segments, align these to sequence

    data = open(file,'r')
    line = data.readline()
    #coords = {}
    prev = []
    word = ''
    #positions = []
    resnum_list = []
    last_found = -1
    in_chain = 0
    full_coords = {} ## indexed by resnum id's
    alignment = {} ## mapping from sequence index to resnum id's
    if chain == '_': chain = ' '

    while line:
        if line[:4] in ['ATOM','HETA'] and \
           line[21] == chain and \
           line[16] in [' ','1','A']:
            atom = line[12:16]
            resnum = line[22:27]
            if resnum not in full_coords.keys():
                full_coords[resnum] = {}
            if atom not in full_coords[resnum].keys():
                p = [float(line[30:38]),float(line[38:46]),float(line[46:54])]
                full_coords[resnum][atom] = p
        if line[:4] in ['ATOM','HETA'] and \
           line[21] == chain and \
           line[12:16] == ' CA ' and \
           line[16] in [' ','1','A']:

            in_chain = 1
            p = [float(line[30:38]),float(line[38:46]),float(line[46:54])]
            resnum = line[22:27]

            if prev and Dist(p,prev) > MAX_D:
                ##just crossed a gap
                if VERBOSE:
                    print file,'chain break',Dist(p,prev),len(word),last_found

                if string.count(sequence,word) == 1:
                    pos = string.find(sequence,word)
                    if pos > last_found:
                        for i in range(len(word)):
                            if pos+i in alignment.keys():
                                sys.stderr.write('error1: already in alignment.keys() '+file+' '+str(i)+'\n')
                                break
                            alignment[pos+i] = resnum_list[i]
                            #coords[pos+i] = positions[i]
                        last_found = pos+len(word)-1

                elif string.count(sequence,word) == 0:
                    #print sequence
                    #print word
                    if VERBOSE:
                        print 'bad word:',word
                    al = NBAlign(word,sequence)
                    if al and min(al.values()) > last_found:
                        for i in al.keys():
                            if al[i] in alignment.keys():
                                sys.stderr.write('error2: already in alignment.keys() '+file+`i`+'\n')
                                break
                            alignment[al[i]] = resnum_list[i]
                            #coords[al[i]] = positions[i]
                        last_found = max(al.values())
                else:
                    sys.stderr.write('multiple occurrences: '+file+'\n')
                    pos = string.find(sequence,word)
                    if pos > last_found:
                        for i in range(len(word)):
                            if pos+i in alignment.keys():
                                sys.stderr.write('error1: already in alignment.keys() '+file+' '+str(i)+'\n')
                                break
                            alignment[pos+i] = resnum_list[i]
                            #coords[pos+i] = positions[i]
                        last_found = pos+len(word)-1

                word = ''
                resnum_list = []
                #positions = []

            if line[17:20] in extra_longer_names.keys():
                rsd = extra_longer_names[line[17:20]]
            else:
                rsd = 'X'
            word = word+rsd
            resnum_list.append(resnum)
            #positions.append(p)
            prev = p
        elif (line[:3] == 'TER' or line[:6] == 'ENDMDL') and in_chain and\
             not SKIP_TER: break

        line = data.readline()

    #print len(word),word
    if word:
        if string.count(sequence,word) == 1:
            pos = string.find(sequence,word)
            if pos > last_found:
                for i in range(len(word)):
                    if pos+i in alignment.keys():
                        sys.stderr.write('error1: already in alignment.keys()'+' '+file+' '+`i`+'\n')
                        break
                    alignment[pos+i] = resnum_list[i]
                    #coords[pos+i] = positions[i]
                last_found = pos+len(word)-1

        elif string.count(sequence,word) == 0:
            #print sequence
            #print word
            if VERBOSE:
                print 'bad_word:',word
            al = NBAlign(word,sequence)
            if al and min(al.values()) > last_found:
                for i in al.keys():
                    if al[i] in alignment.keys():
                        sys.stderr.write('error2: already in alignment.keys() '+file+' '+`al[i]`+'\n')
                        break
                    alignment[al[i]] = resnum_list[i]
                    #coords[al[i]] = positions[i]
                last_found = max(al.values())
        else:
            sys.stderr.write('multiple occurrences '+file+'\n')
            pos = string.find(sequence,word)
            if pos > last_found:
                for i in range(len(word)):
                    if pos+i in alignment.keys():
                        sys.stderr.write('error1: already in alignment.keys() '+file+' '+str(i)+'\n')
                        break
                    alignment[pos+i] = resnum_list[i]
                    #coords[pos+i] = positions[i]
                last_found = pos+len(word)-1

    data.close()

    coords = {}
    for i in range(len(sequence)):
        if i in alignment.keys():
            resnum = alignment[i]
            assert resnum in full_coords.keys()
            coords[i] = full_coords[resnum]
        #else:
        #    coords[i] = {} ### Why empty dictionary??

    return coords


if 0:
    seq = 'MVKLTAELIEQAAQYTNAVRDRELDLRGYKIPVIENLGATLDQFDAIDFSDNEIRKLDGFPLLRRLKTLLVNNNRICRIGEGLDQALPDLTELILTNNSLVELGDLDPLASLKSLTYLCILRNPVTNKKHYRLYVIYKVPQVRVLDFQKVKLKERQEAEKMFKGKRGAQLAKDIAR'
    seq = 'MLTEGISIQSYDGHTFGALVGSPAKAPAPVIVIAQEIFGVNAFMRETVSWLVDQGYAAVCPDLYARQAPGTALDPQDERQREQAYKLWQAFDMEAGVGDLEAAIRYARHQPYSNGKVGLVGYXLGGALAFLVAAKGYVDRAVGYYGVGLEKQLNKVPEVKHPALFHMGGQDHFVPAPSRQLITEGFGANPLLQVHWYEEAGHSFARTSSSGYVASAAALANERTLDFLAPLQSKKP'
    coords = Get_coords('/net/pdb/d/1din.pdb',seq,' ')
    print len(seq),len(coords.keys())


if 0:
    files = glob('/net/pdb/q/*.pdb')
    for file in files:
        print file,Check_rosetta(file,'A')


if 0:
    ids = map(lambda x:x[:-1],open('junk','r').readlines())
    for id in ids:
        pdb_file = '/net/pdb/%s/%s.pdb'%(id[1],id[:4])
        print id,Check_rosetta(pdb_file,id[4])
