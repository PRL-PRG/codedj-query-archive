#!/usr/bin/python

## make big model file from .casp files
## first argument is the name of the big model file to be created
## remaining args are .casp files

from phil import *
from amino_acids import extra_longer_names, short_to_long

WILD_IS_DOWN = 1

DSSP_EXE = '/users/pbradley/dssp'
GET_PDB_EXE = '/users/dylan/src/pdbUtil/getPdb.pl'

if WILD_IS_DOWN:
    PDB_TMP_DIR = '/data/pbradley/rcsb/' ## location for new pdbs we download
    OBSELETE_PDB_LIST = '/data/pbradley/rcsb/obsolete.dat'
    USE_NET_PDB = 0 ## served by WILD
else:
    PDB_TMP_DIR = '/dump/pbradley/rcsb/' ## location for new pdbs we download
    OBSELETE_PDB_LIST = '/dump/pbradley/rcsb/obsolete.dat'
    USE_NET_PDB = 1

model_file_name = 'all_models.out'


def Help():
    print 'usage: %s <model-directory> <casp-file1> {<casp-file2> ... <casp-fileN>}'\
          %(argv[0])
    exit()

if len(argv)<3:
    Help()

args = argv[1:]

fasta_seq = ''
if args.count('-fasta'):
    pos = args.index('-fasta')
    fasta_file = args[pos+1]
    del args[pos]
    del args[pos]
    fasta_seq = string.join(map(lambda x:string.split(x)[0],
                                open(fasta_file,'r').readlines()[1:]),'')

    
model_dir = args[0]+'/'
casp_files = args[1:]

if not exists(model_dir):
    system('mkdir '+model_dir)

## random setup: ###########################
N  = ' N  '
CA = ' CA '
CB = ' CB '
C  = ' C  '
O  = ' O  '
bb_atoms = [N,CA,CB,C,O] ## Eposition order !!!!!!!!! 
lowercase = list('abcdefghijklmnopqrstuvwxyz')
extra_longer_names['UNK'] = 'X'
short_to_long['X'] = 'UNK'

############################################### FUNCTIONS
def Get_pdb(pdb):
    if USE_NET_PDB:
        parent_file = '/net/pdb/%s/%s.pdb'\
                      %(pdb[1:3],pdb)
    else:
        parent_file = '/users/dylan/dat/pdbs/pdb-rcsb/%s/%s.pdb.Z'\
                      %(pdb[1:3],pdb)
        
    if exists(parent_file):
        return parent_file
        

    
    CWD = getcwd()
    parent_file = '%s/%s.pdb'%(PDB_TMP_DIR,pdb)
    if not exists(parent_file):
        chdir(PDB_TMP_DIR)
        system('%s -id %s'%(GET_PDB_EXE,pdb))
        new_file = '%s.pdb.Z'%pdb
        if not exists(new_file):
            print 'WARNING: getPdb failed!',pdb
            chdir(CWD)
            return '' ## return empty filename
        system('gunzip %s'%new_file)
        if not exists(parent_file):
            print 'gunzip failed:',parent_file
            chdir(CWD)
            return ''
    chdir(CWD)
    return parent_file
        
def Get_new_pdb_id(pdb): 
    lines = map(string.split,popen('grep %s -i %s'\
                                   %(pdb,OBSELETE_PDB_LIST)).readlines())
    if not lines:
        print 'WARNING: not an obselete id',pdb
        return ''
    new_pdb = ''
    for line in lines:
        if string.lower(line[2]) == pdb:
            new_pdb = string.lower(line[3])
            break
    if not new_pdb:
        print 'WARNING: not an obselete id',pdb
    return new_pdb

def Get_server_sequence(file):
    suffix = string.split(string.split(file,'/')[-1],'.')[-1]

    seq = ''
    
    if suffix in ['bas_c','bas_b','orfeus','orfbc']:
        lines = popen('grep "^# SEQUENCE" %s'%file).readlines()
        if not lines:
            log('missing SEQUENCE info: %s\n'%file)
            return ''

        seq = string.split(lines[0])[1]
        if seq[:9] != 'SEQUENCE:':
            log('bad line: %s'%(lines[0]))
            return ''
        seq = seq[9:]

    elif suffix == 'prof':
        lines = popen('grep "^# default" -A1 %s'%file).readlines()
        if len(lines) != 2:
            log('bad prof file %s\n'%file)
            return ''
        seq = string.split( lines[1] )[0]

    elif suffix == 'psipred':
        lines = map(string.split,popen('grep "^  AA" %s'%file).readlines())
        if not lines or lines[0][1] != 'Target':
            log('bad psipred file: %s\n'%file)
            return ''
        seq = ''
        for line in lines[1:]:
            if len(line) == 2 and line[0] == 'AA:':
                seq = seq + line[1]

    return seq
############################################################



## now start looping #############################
if 1:
    

    ## get fasta sequence: #######################
    if not fasta_seq:
        server_files = map(lambda x:string.join(string.split(x,'.')[:-1],'.'),
                           casp_files)

        ## add prof,psipred
        file = casp_files[0]
        if string.count(file,'/'):
            dir = string.join(string.split(casp_files[0],'/')[:-1],'/')+'/'
        else:
            dir = ''
        server_files.append(dir+'prof')
        server_files.append(dir+'psipred')

        for file in server_files:
            if exists(file):
                fasta_seq = Get_server_sequence(file)
                if fasta_seq:
                    print 'taking %d-length sequence from %s'\
                          %(len(fasta_seq),file)
                    break

                

        if not fasta_seq:
            log('WARNING: couldnt parse sequence from server files!! %s\n'\
                %(`server_files`))
            exit()
            

##         command = 'grep "^# SEQUENCE:" '+string.join(server_files)
##         lines = map(string.split,popen(command).readlines())
##         if not lines:
##             log('WARNING: couldnt parse sequence data from server files: %s\n'%(`server_files`))
##             exit()
##         fasta_seq = ''
##         for line in lines:
##             if len(line) != 2 or line[1][:9] != 'SEQUENCE:':
##                 print 'funny SEQUENCE line:',line
##                 continue
##             seq = line[1][9:]
##             if fasta_seq:
##                 if seq != fasta_seq:
##                     print 'sequence mismatch in server_files:%s\n%s\n%s\n'\
##                           %(line[0],fasta_seq,seq)
##                     exit()
##             else:
##                 fasta_seq = seq
##                 print 'taking sequence from: %s len= %d'\
##                       %(line[0],len(fasta_seq))
##         if not fasta_seq:
##             print 'couldnt find sequence in server files:',server_files
##             exit()

    fasta_file = model_dir+'t000_.fasta' ## make a fasta file for running rosetta
    out = open(fasta_file,'w')
    out.write('>t000_\n%s\n'%fasta_seq)
    out.close()
        
    big_model_file = model_dir+model_file_name
    big_out = open(big_model_file,'w')

    for casp_file in casp_files:
        if not exists(casp_file):
            print 'missing casp_file:', casp_file
            continue
        
        server = string.join(string.split(string.split(casp_file,'/')[-1],'.')[:-1],'.')

        model_count = 1
        for model_num in range(10): ## this loop is historical, see combine_models.py

            model = '%s.%s'%(server,model_num)

            data = open(casp_file,'r')

            in_model = 0
            seq = {}
            align = {}
            line = data.readline()
            while line:
                if line[:5] == 'MODEL':
                    if model_num+1 == int(string.split(line)[1]): ## CASP model numbering
                        in_model = 1
                    else:
                        in_model = 0
                elif line[:3] == 'TER':
                    in_model = 0
                elif not in_model:
                    pass
                elif line[:6] == 'PARENT':
                    id = string.split(line)[1]
                    pdb = string.lower(id[:4])
                    if len(id)==6:
                        chain = id[5]
                    elif len(id) == 4:
                        chain = ' '
                    else:
                        print 'WARNING: funny pdb id, assuming chain=_:',id
                        chain = ' '

                elif line[:6] == 'REMARK':
                    pass
                else:
                    l = string.split(line)
                    if len(l) <4:
                        print 'WARNING: bad casp line',line[:-1]
                        line = data.readline()
                        continue
                    try:
                        pos = int(l[1])
                    except:
                        log('funny casp line: %s %s'%(casp_file,line))
                        line = data.readline()
                        continue
                    
                    rsd = l[0]
                    if fasta_seq[pos-1] != rsd:
                        log('WARNING: seq mismatch in casp file: %s %d %s %s\n'\
                            %(casp_file,pos,rsd,fasta_seq[pos-1]))
                    else:
                        seq[pos] = rsd
                        align[pos] = (l[2],l[3]) ## rsd,pos
                line = data.readline()
            data.close()

            if not seq:
                print 'cant find model %d in casp_file: %s'\
                      %(model_num,casp_file)
                continue

            if chain == '_':chain = ' '

            
            parent_file = Get_pdb(pdb)
            original_pdb = pdb
            
            if not parent_file:
                pdb = Get_new_pdb_id(pdb)
                if not pdb:
                    log('SKIPPING TEMPLATE: cant find pdb-file or new id: %s\n'\
                        %original_pdb)
                    continue
                parent_file = Get_pdb(pdb)
                if not parent_file:
                    log('SKIPPING TEMPLATE: neither id works: %s %s\n'\
                        %(original_pdb,pdb))
                    continue

            tag = string.split(parent_file,'.')[-1]
            if tag in ['gz','Z']:
                command = 'gzip -dc %s | '%parent_file
            else:
                command = 'cat %s | '%parent_file


                
            command = command + "awk '(( substr($0,22,1)==\"%s\" && (/^ATOM / || /^HETATM/)) || /^TER/ || /^ENDMDL/)'"%chain
            

            atom_lines = popen(command).readlines()

            coords = {}
            pdb_seq = {}
            in_chain = 0
            for line in atom_lines:
                if line[:3] in ['TER','END']:
                    if in_chain:
                        break
                else: ## ATOM or HETATM
                    in_chain = 1
                    resnum = string.split(line[22:26])[0]
                    if line[26]!=' ':
                        #print 'insertion!!',line[:-1]
                        resnum = resnum+line[26] 
                    if not coords.has_key( resnum):
                        coords[resnum] = {}
                        rsd = line[17:20]
                        if not extra_longer_names.has_key(rsd):
                            pdb_seq[resnum] = 'X'
                        else:
                            pdb_seq[resnum] = extra_longer_names[ rsd ]
                    atom = line[12:16]

                    if atom in bb_atoms:
                        coords[resnum][atom] = map(float,[line[30:38],line[38:46],line[46:54]])


            new_pdb_file = '%s/%s.pdb'%(model_dir,model)
            new_coords = {}
            out = open(new_pdb_file,'w')
            ks = align.keys()
            ks.sort()
            atom_count = 1
            for pos in ks:
                s = align[pos][0]
                resnum = align[pos][1]
                if not coords.has_key(resnum):
                    print 'no coords!',resnum
                    continue
                if len(coords[resnum].keys()) < 4:
                    print 'bb missing:',coords[resnum].keys()
                if pdb_seq[resnum] != s:
                    print 'WARNING: pdb_seq mismatch: %d %s %s'\
                          %(pos,s,pdb_seq[resnum])
                rsd = short_to_long[seq[pos]]

                new_coords[pos] = {}
                for atom in bb_atoms:
                    if seq[pos] == 'G' and atom == CB:continue
                    if not coords[resnum].has_key(atom):
                        if  atom != CB:
                            print 'bb missing:',atom
                        continue
                    c = coords[resnum][atom]
                    new_coords[pos][atom] = c ## assign to the new_coords
                    out.write('ATOM  %5d %4s %s %s%4d    %8.3f%8.3f%8.3f  1.00  1.00\n'\
                              %(atom_count,atom,rsd,' ',pos,
                                c[0],c[1],c[2])) ## NOTE: empty chain
                    atom_count = atom_count + 1

            out.close()

            ## run dssp
            dssp_file = new_pdb_file[:-4]+'.dssp'
            if not exists(dssp_file):
                command = '%s %s > %s 2> %s_error'\
                          %(DSSP_EXE,
                            new_pdb_file,
                            dssp_file,
                            dssp_file)
                system(command)

            lines = popen('grep "RESIDUE AA" -A10000 %s | grep "^.[ 0-9][ 0-9][ 0-9][ 0-9]"'\
                          %dssp_file).readlines()
            ss = {}
            for line in lines:
                rsd = line[13]
                if rsd == '!': continue ## chain break character
                pos = int(line[6:10])
                #if rsd != seq[pos] and seq[pos] != 'C': ## Cys SS are lowercase letters in dssp
                if rsd in lowercase:
                    rsd = 'C'
                
                if rsd != seq[pos]: ## Cys SS are lowercase letters in dssp
                    print 'WARNING: dssp sequence mismatch!',pos,seq[pos],rsd

                s = line[16]
                if s not in [' ','E','B','H','G','I','S','T']:
                    log('undefined ss character? '+s+'\n')
                    ss[pos] = 'L'
                elif s in ['E','B']: ## send B to E
                    ss[pos] = 'E'
                elif s in ['H','G']:
                    ss[pos] = 'H'
                else:
                    ss[pos] = 'L' ## send I helix to L
                
            ## add to giant coordinate file
            big_out.write('MODEL %d %s\n'%(model_count,model))
            model_count = model_count + 1
            for i in range(len(fasta_seq)):
                pos = i+1 ## ROSETTA numbering
                rsd = fasta_seq[i]
                c = [] ## list of coords
                m = [] ## list of occupancy
                for atom in bb_atoms:
                    if not new_coords.has_key(pos) or \
                       not new_coords[pos].has_key(atom) or \
                       (rsd == 'G' and atom == CB):
                        c.append([0.0,0.0,0.0])
                        m.append(0)
                    else:
                        c.append( new_coords[pos][atom] )
                        m.append(1)
                if not ss.has_key(pos):ss[pos] = 'L'
                big_out.write('%4d %s %s'%(pos,rsd,ss[pos]))
                for j in range(5):
                    big_out.write('%2d'%(m[j]))
                for j in range(5):
                    for k in range(3):
                        big_out.write('%9.3f'%(c[j][k]))
                big_out.write('\n')
            
    big_out.close()
            #break

###############################################
## def Get_pdb(pdb):
##     parent_file = '/net/pdb/%s/%s.pdb'\
##                   %(pdb[1:3],pdb)
##     if exists(parent_file):
##         return parent_file
    
##     CWD = getcwd()
##     parent_file = '%s/%s.pdb'%(PDB_TMP_DIR,pdb)
##     if not exists(parent_file):
##         chdir(PDB_TMP_DIR)
##         system('%s -id %s'%(GET_PDB_EXE,pdb))
##         new_file = '%s.pdb.Z'%pdb
##         if not exists(new_file):
##             print 'WARNING: getPdb failed!',pdb
##             chdir(CWD)
##             return '' ## return empty filename
##         system('gunzip %s'%new_file)
##         if not exists(parent_file):
##             print 'gunzip failed:',parent_file
##             chdir(CWD)
##             return ''
##     chdir(CWD)
##     return parent_file
        
## def Get_new_pdb_id(pdb): 
##     lines = map(string.split,popen('grep %s -i %s'\
##                                    %(pdb,OBSELETE_PDB_LIST)).readlines())
##     if not lines:
##         print 'WARNING: not an obselete id',pdb
##         return ''
##     new_pdb = ''
##     for line in lines:
##         if string.lower(line[2]) == pdb:
##             new_pdb = string.lower(line[3])
##             print 'obselete id:',pdb,'-->',new_pdb
##             break
##     if not new_pdb:
##         print 'WARNING: not an obselete id',pdb
##     return new_pdb
