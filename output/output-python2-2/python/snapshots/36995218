#!/usr/bin/python

##
## pick homologs from a PSI-blast file
##

from phil import *
from parse_blast_whip import Read_psiblast
from amino_acids import amino_acids

fasta_file = argv[1]
psiblast_file = argv[2]

NOPICK = argv.count('-NOPICK') ## dont actually make the fasta files

## if NOPICK == 1: just create query-anchored align file, filter by max_gap,
## and seq filter

#############
## CONFIG

## first we sequence filter to get FILTER_N seqs
## then we sort by length, and reduce the threshold
## until we get N_HOMS_THRESHOLD
## then we take the first N_HOMS of these (enriching for short seqs)

N_HOMS = 30
N_HOMS_THRESHOLD = 60


NR = '/scratch/Phil/genomes/nr'
SEQUENCE_CLUSTER = '/users/pbradley/C/sequence_cluster.out'
FILTER_N = 300

## use these parameters for alpha_homs
MAX_INSERT = 5
MAX_DELETE = 10
MAX_N_DEL = 12
MAX_C_DEL = 12
MAX_N_GAP = 5
MAX_C_GAP = 5

## I used these parameters for resample v1
## MAX_INSERT = 5
## MAX_DELETE = 10
## MAX_N_DEL = 12
## MAX_C_DEL = 12
## MAX_N_GAP = 5
## MAX_C_GAP = 5
##
## filenames
full_seq_file = psiblast_file + '.full_seq'
gi_file = psiblast_file + '.gi_list'
target_alfas_file = psiblast_file + '.target_alfas'
filtered_alfas_file = psiblast_file + '.filtered_alfas'
fasta_file_base = psiblast_file +'_'

######################################################################
#############
def Parse_id( id ):
    tag = string.join(string.split(id,'_')[:-1],'_')
    start = int(string.split(string.split(id,'_')[-1],'-')[0])
    stop = int(string.split(string.split(id,'_')[-1],'-')[1])
    return tag,start,stop

def New_id( tag, start, stop ):
    return '%s_%d-%d'%(tag,start,stop)

## read fasta
fasta_seq = string.join(map(lambda x:string.split(x)[0],
                            open(fasta_file,'r').readlines()[1:]),'')
L = len(fasta_seq)


## read sequences from file
align2query, full_seq = Read_psiblast( fasta_seq, psiblast_file )
id_list = align2query.keys()


#######################################
## get list of gi's from psi-blast file
## get full-length sequences for the gi's here


if not exists(full_seq_file):
    out = open(gi_file,'w')
    for id in id_list:
        assert id[:3] == 'gi|' ## if this fails, use "-I T" when psiblasting
        out.write('%d\n'%(int(string.split(id,'|')[1])))
    out.close()

    command = '/users/pbradley/python/get_full_nr_sequences_whip.py %s %s > %s'\
              %(NR, gi_file, full_seq_file )
    print command
    system( command )

## read nr sequences
data = open( full_seq_file,'r')
line = data.readline()
nr_seq = {}
while line:
    assert line[0] == '>'
    gi = int(line[1:-1])
    nr_seq[gi] = data.readline()[:-1]
    line = data.readline()
data.close()


## filter sequences with big insertions or deletions, long unaligned termini,
## or bad aa's

max_insert_count = 0
max_delete_count = 0
n_del_count = 0
c_del_count = 0
n_gap_count = 0
c_gap_count = 0

## also, make query-anchored alfas file
out = open( target_alfas_file,'w')


#h_align = {}
q_align = {}

skip_count = 0
counter = 0
for id in id_list:
    counter = counter + 1
    if not counter%100:
        print counter,skip_count,len(id_list)
    ## parse id
    assert id[:3] == 'gi|'
    gi = int(string.split(id,'|')[1])

    tag,start,stop = Parse_id(id)
    assert stop-start+1 == len(full_seq[id])

    ## setup mapping to nr_seq
    h_al = {}
    h_al_old = align2query[id]
    h_seq_old = full_seq[id]
    h_seq = nr_seq[gi]
    h_L = len(h_seq)
    q_L = len(fasta_seq)
    assert string.count( h_seq, h_seq_old ) >= 1
    assert h_seq[start] == h_seq_old[0] and \
           h_seq[start+1] == h_seq_old[1]

    for h_pos in range(h_L):
        h_al[ h_pos ] = -1
    for h_pos in range(len(h_seq_old)):
        new_h_pos = h_pos + start
        h_al[ new_h_pos ] = h_al_old[ h_pos ]

    del align2query[id]

    ## check for bad aa in hit_seq
    bad_aa = 0
    for aa in h_seq:
        if aa not in amino_acids:
            #print 'bad aa:',aa
            bad_aa = 1
            break
    if bad_aa:
        skip_count+=1
        continue

    q_al = {} ## a mapping from the query back to the hit
    for q_pos in range(q_L):
        q_al[ q_pos ] = -1

    max_delete = 0
    max_insert = 0
    for h_pos in range(h_L):
        q_pos = h_al[ h_pos ] ## position in the query sequence
        assert q_pos < q_L
        if q_pos != -1:
            q_al[ q_pos ] = h_pos ## map from query back to hit
            if h_pos+1 < h_L and h_al[ h_pos+1 ] != -1:
                deletion = h_al[ h_pos+1 ] - h_al[ h_pos ] - 1
                max_delete = max(max_delete, deletion)

    for q_pos in range(q_L-1):
        if q_al[ q_pos ] != -1 and q_al[ q_pos+1 ] != -1:
            insert = q_al[ q_pos+1 ] - q_al[ q_pos ] - 1
            max_insert = max(max_insert,insert)

    ## calculate terminal deletions
    n_del = 0
    while q_al[ n_del ] == -1:
        n_del = n_del + 1
    first_aligned = n_del
    assert q_al[ first_aligned ] == start

    c_del = 0
    while q_al[ q_L - c_del - 1 ] == -1:
        c_del = c_del + 1
    last_aligned = q_L - c_del - 1
    assert q_al[ last_aligned ] == stop


    n_extra = start
    c_extra = h_L - 1 - stop
    n_gap = max(0, n_del - n_extra )
    c_gap = max(0, c_del - c_extra )

    #print id,max_insert,max_delete,n_del,c_del,n_gap,c_gap

    ######################################
    ## align extra residues at the termini,
    ## and update start and stop
    n_shift = start - first_aligned
    for q_pos in range( first_aligned ):
        assert q_al[q_pos] == -1
        h_pos = q_pos + n_shift
        if h_pos >= 0:
            start = start - 1
            q_al[q_pos] = h_pos

    c_shift = stop - last_aligned
    for q_pos in range( last_aligned+1, q_L):
        assert q_al[q_pos] == -1
        h_pos = q_pos + c_shift
        if h_pos < h_L:
            stop = stop + 1
            assert stop == h_pos
            q_al[q_pos] = h_pos


    ## should we skip this guy?
    skip_me = 0
    if max_insert > MAX_INSERT:
        skip_me = 1
        max_insert_count = max_insert_count + 1
    if max_delete > MAX_DELETE:
        skip_me = 1
        max_delete_count = max_delete_count + 1
    if n_del > MAX_N_DEL:
        skip_me = 1
        n_del_count = n_del_count + 1
    if c_del > MAX_C_DEL:
        skip_me = 1
        c_del_count = c_del_count + 1
    if n_gap > MAX_N_GAP:
        skip_me = 1
        n_gap_count = n_gap_count + 1
    if c_gap > MAX_C_GAP:
        skip_me = 1
        c_gap_count = c_gap_count + 1


    if skip_me:
        skip_count+=1
        #skip_count = skip_count + 1
        continue

    tag = string.join(string.split(id,'_')[:-1],'_') ## gi + db ref
    new_id = New_id( tag, start, stop )

    nr_seq[ new_id] = nr_seq[gi]

    #h_align[ new_id ] = h_al
    q_align[ new_id ] = q_al

    out.write('>%s\n'%new_id)
    for q_pos in range(q_L):
        h_pos = q_al[ q_pos ]
        if h_pos == -1:
            out.write('-')
        else:
            out.write(h_seq[ h_pos ] )
    out.write('\n')
out.close()


stderr.write('filtered out %6d of %6d sequences (%3d percent) by max-gap'\
             %( skip_count, len(id_list), (100*skip_count)/len(id_list)))

for n in [max_insert_count,max_delete_count,n_del_count,c_del_count,
          n_gap_count,c_gap_count]:
    stderr.write('%4d'%((100*n)/len(id_list)))
stderr.write('\n')


command = '%s %s -n %d > %s 2> %s.err'\
          %(SEQUENCE_CLUSTER, target_alfas_file, FILTER_N,
            filtered_alfas_file, filtered_alfas_file )
print command
system(command)


########################
## now pick the homologs

## figure out what the threshold was for filtering
line = popen('grep "numClusters" %s.err'%filtered_alfas_file).readlines()[0]
print line[:-1]

N = int(string.split(line)[2])
start_threshold = float(string.split(line)[4])/100

## sort the id's by internal length
id_list = map(lambda x:x[1:-1],
              popen('grep ">" %s'%filtered_alfas_file).readlines())
assert len(id_list) == N

## add QUERY
L = len(fasta_seq)
QUERY = 'QUERY_0-%d'%L
nr_seq[QUERY] = fasta_seq
length_list = [[0,QUERY]] ## pretend query has length 0 --> force inclusion
q_align[QUERY] = {}
for i in range(L):
    q_align[QUERY][i] = i

for id in id_list:
    #h_al = h_align[id]
    q_al = q_align[id]
    tag,start,stop = Parse_id( id )
    L = len(q_al.keys())
    assert L == len(fasta_seq)
    n_gap = 0
    while q_al[ n_gap ] == -1: n_gap+=1
    c_gap = 0
    while q_al[ L-c_gap-1 ] == -1: c_gap+=1

    length = stop - start + 1 + n_gap + c_gap

    length_list.append( [length, id ] )

length_list.sort()

if 1:
    N = len(length_list)
    print 'lengths: ',\
          length_list[1][0],\
          length_list[ ( 1*N)/100 ][0],\
          length_list[ (10*N)/100 ][0],\
          length_list[ (50*N)/100 ][0],\
          length_list[-1][0]


length_list = map(lambda x:x[1], length_list)


def Sim(id1,id2,q_align=q_align,L=L):
    al1 = q_align[id1]
    al2 = q_align[id2]
    seq1 = nr_seq[id1]
    seq2 = nr_seq[id2]
    sim = 0
    for pos in range(L):
        pos1 = al1[pos]
        if pos1 == -1:
            s1 = '-'
        else:
            s1 = seq1[pos1]
        pos2 = al2[pos]
        if pos2 == -1:
            s2 = '-'
        else:
            s2 = seq2[pos2]
        if s1 == s2:
            sim+=1
    return float(sim)/L

threshold = start_threshold
homs = length_list[:]

prev_homs = homs[:]

while threshold > .05 and ( not homs or len(homs) > N_HOMS_THRESHOLD ):
    prev_homs = homs[:]
    homs = []
    for id in length_list:
        ok = 1
        for id2 in homs:
            if Sim(id,id2) > threshold:
                ok = 0
                break
        if ok:
            homs.append( id )
    print 'filter_threshold= %f nhoms= %d'%(threshold, len(homs))
    #threshold -= .1 * threshold
    threshold -= .025 * threshold


homs = prev_homs[:N_HOMS]

if 0:
    stderr.write(fasta_seq+'\n')
    for id in homs:
        if id != QUERY:
            q_al = q_align[ id ]
            gi = int(string.split(id,'|')[1])
            seq = nr_seq[gi]
            for i in range(L):
                if q_al[i] == -1:
                    stderr.write('-')
                else:
                    stderr.write( seq[ q_al[i] ] )
            stderr.write(' %f\n'%(Sim( id, QUERY ) ) )

if NOPICK:exit()

counter = 0
for id in homs:
    counter = counter + 1
    hom_id = 'h%03d_'%counter
    hom_fasta = fasta_file_base + hom_id + '.fasta'
    tag,start,stop = Parse_id(id)

    seq = nr_seq[id][start:stop+1]
    out = open( hom_fasta,'w')
    out.write('>%s %s %f\n%s\n'%(hom_id,id,100*Sim(QUERY,id),seq))
    out.close()
