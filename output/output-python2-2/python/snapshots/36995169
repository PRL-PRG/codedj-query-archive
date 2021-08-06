#!/usr/bin/python

## look at bb sampling

from phil import *
from math import floor
from amino_acids import longer_names
from sys import stdout

CHI_SQUARED = 0
MAX_DECOYS = 1000 # max in silent-file

if len(argv)<4:
## if len(argv)<5:
    print '\n\nusage: %s <out-file> <rosetta-style native pdb> <9mer fragment file> {-3}\n\n'%(argv[0])
    print '-3 means use a 3mer frag file and only show results up to length 3'
    print '  with 9mer file, only 25 frags are used; all 200 are used w/ 3mer file'
    print '\n-E will split E into a separate class, otherwise grouped with B'
##     print '\n\nusage: %s <out-file> <rosetta-style native pdb> <9mer fragment file> <plot-prefix> {-3}\n\n'%(argv[0])
##     print '-3 means use a 3mer frag file and only show results up to length 3'
##     print '-E will split E into a separate class, otherwise grouped with B'
    exit()

args = argv[1:]

if args.count('-3'):
    del args[args.index('-3')]
    SIZE = 3
    TOP_N = 200
    MAX_LEN = 3
else:
    TOP_N = 25
    MAX_LEN = 5
    SIZE = 9

if args.count('-TOP_N'):
    pos = args.index('-TOP_N')
    TOP_N = int(args[pos+1])
    print 'TOP_N=',TOP_N
    del args[pos]
    del args[pos]

if args.count('-E'): ## separate class for E, otherwise its grouped with B
    ABG2int = {'A':0,'B':1,'G':2, 'E':3, 'O':2} ## NOTE 'O' w/ 'G'
    ABG = 'ABGE'
    del args[args.index('-E')]
else:
    ABG2int = {'A':0,'B':1,'G':2, 'E':1, 'O':2} ## NOTE 'E' w/ 'B', 'O' w/ 'G'
    ABG = 'ABG'

NN = len(ABG)
lengths = range(1,MAX_LEN+1)

out_file = args[0]
native_pdb = args[1]
ninemer_fragment_file = args[2]

#prefix = '%s.%d.%s'%(args[3],SIZE,ABG)



#out_file = '/scratch/Phil/1b72/fast_decoys/xx1b72.out'
#native_pdb = '/users/baker/1b72/1b72_min.pdb'
#ninemer_fragment_file = '/users/baker/1b72/aa1b72A09_05.200_v1_3'

#out_file = '/users/baker/1b72/ab1b72_3.out'
#native_pdb = '/users/baker/1b72/1b72_min.pdb'
#ninemer_fragment_file = '/users/baker/1b72/aa1b72A09_05.200_v1_3'


#out_file = '/scratch/Phil/junk6_26/1csp_/nut.out'
#native_pdb = '/data/pbradley/alm/folding/1csp_/1csp_.rosetta_pdb'
#ninemer_fragment_file = '/data/pbradley/alm/folding/1csp_/aa1csp_09_05.200_v1_3'


#out_file = '/scratch/Phil/junk6_26/1ubq_/nut.out'
#native_pdb = '/data/pbradley/alm/folding/1ubq_/1ubq_.rosetta_pdb'
#ninemer_fragment_file = '/data/pbradley/alm/folding/1ubq_/aa1ubq_09_05.200_v1_3'


#out_file = '/data/pbradley/alm/folding/1shfA/decoys/aa1shf.out'
#ninemer_fragment_file = '/data/pbradley/alm/folding/1shfA/aa1shfA09_05.200_v1_3'
##out_file = '/data/pbradley/alm/folding/1shfA/nut.out'
#native_pdb = '/data/pbradley/alm/folding/1shfA/1shfA.rosetta_pdb'
#decoy_globber = '/data/pbradley/alm/folding/1shfA/set_refine/tmp/aaS_*0001.pdb'
##decoy_globber = '/data/pbradley/alm/folding/1shfA/set_refine/aaS_*0001.pdb'


def In_range(angle):
    while angle>180: angle = angle-360
    while angle<=-180:angle = angle+360
    return angle

def pp_class(pp): ## E G A B and O
    pp = ( In_range( pp[0]), In_range(pp[1]), In_range(pp[2]))
    assert -180<=pp[0]<=180 and -180<=pp[1]<=180 and -180<=pp[2]<=180

    if abs(pp[2]) <90:
        return 'O'
    elif pp[0]>=0:
        if -100< pp[1] <= 100:return 'G'
        else: return 'E'
    elif -125 < pp[1] <= 50: return 'A'
    else: return 'B'


def Convert(s):
    ans=0
    for i in range(len(s)):
        ans = ans + NN**i * ABG2int[s[i]]
    return ans

def Convert_back(c,l):
    ans = ''
    for i in range(l):
        j = NN**( l-1-i )
        ans = ABG [ c/j ] + ans
        c = c - j * (c/j)
    return ans

def Torsion_counts_fragments(filename,L): ## 9mer fragment file
    assert SIZE >= max(lengths) ## needs to be bigger than all lengths

    print 'LENGTHS: ', lengths

    ## read the fragment file
    data = open(filename,'r')
    line = data.readline()
    fragments = {}
    for pos in range(L-SIZE+1):
        fragments[pos] = []

    current_fragment = ''
    prev_pos = 0
    pos = 0
    while line:
        if len(line) < 10:
            frag += 1
            line = data.readline()
            continue

        if line[1:4] == 'pos':
            pos = int(string.split(line)[1]) - 1
            frag = 0
            if not pos%10:stderr.write('read frag file: %s %d of %d\n'\
                                       %(filename,pos+1,L))
            if not fragments.has_key(pos):
                stderr.write('length mismatch!! ignoring some fragments!!\n')
                break
            line = data.readline()
            continue

        if frag>TOP_N:
            line = data.readline()
            continue

        current_fragment = current_fragment +\
                           pp_class( map(float,string.split(line)[5:8]))
        if len(current_fragment) == SIZE:
            fragments[pos].append( current_fragment)
            current_fragment = ''
        line = data.readline()
    data.close()


    counts = {} ## initialize counts:
    for l in lengths:
        counts[l] = {}
        for pos in range(L-l+1):
            counts[l][pos] = {}
            for t in range(NN**l):
                counts[l][pos][t] = 0


    for l in lengths:
        for pos in range(L-l+1):
            total = 0.0
            for fpos in range(max(0,pos-SIZE+l),min(L-SIZE+1,pos+1)):
                for frag in fragments[fpos]:
                    total = total + 1
                    c = Convert ( frag[pos-fpos: pos-fpos + l])
                    counts[l][pos][c] = counts[l][pos][c] + 1
            #stderr.write('summing: %d %d total=%d\n'%(l,pos,int(total)))
            for c in counts[l][pos].keys():
                counts[l][pos][c] = counts[l][pos][c] / total

    return counts






def Torsion_counts_silent(filename):
    data = open(filename,'r')
    line = data.readline()

    sequence = string.split(line)[1]
    L = len(sequence)
    stderr.write('read silent-file: %s L = %d\n'%(filename,L))
    line = data.readline()
    line = data.readline()

    counts = {}
    for l in lengths:
        counts[l] = {}
        for pos in range(L-l+1):
            counts[l][pos] = {}
            for t in range(NN**l):
                counts[l][pos][t] = 0


    total = 0
    while line:
        assert line[:5] == 'SCORE'
        torsions = ''
        for i in range(L):
            line = data.readline()
            while line[:5]=='JUMPS' or line[:9]=='FOLD_TREE': line = data.readline()
            l = string.split(line)
            #            if len(l) not in [9,10] or int(l[0]) -1 != len(torsions):break
            torsions = torsions + pp_class( map(float,l[2:5]) )
        if len(torsions) == L:
            total = total + 1
            if not total%50:stderr.write(`total`+'\n')

            for l in lengths:
                for pos in range(L+1-l):
                    c = Convert ( torsions[pos:pos+l] )
                    counts[l][pos][c] = counts[l][pos][c] + 1

            if total>= MAX_DECOYS:
                break

        line = data.readline()
        while line and line[:5] != 'SCORE': line = data.readline()
    data.close()

    for l in lengths:
        for pos in range(L-l+1):
            for t in range(NN**l):
                counts[l][pos][t] = float( counts[l][pos][t] ) / total


    return counts,L,sequence,total

def Torsion_counts_pdbs(globber):
    files = glob(globber)
    stderr.write('%s: %d\n'%(globber,len(files)))


    total = 0
    L = 0
    for file in files:
        lines = map(string.split,popen('grep "complete" -A10000 '+file).readlines())
        if not lines:
            stderr.write('empty: %s\n'%file)
            continue
        lines = lines[1:]
        if not L:
            L = len(lines)


            counts = {}
            for l in lengths:
                counts[l] = {}
                for pos in range(L-l+1):
                    counts[l][pos] = {}
                    for t in range(NN**l):
                        counts[l][pos][t] = 0


            sequence = string.join(map(lambda x:longer_names[string.split(x)[1]],
                                       popen('grep "res aa    Eatr" -A%d %s'\
                                             %(L,file)).readlines()[1:]),'')

        elif len(lines) != L:
            stderr.write('bad length: %s %d %d\n'%(file,len(lines),L))
            continue

        torsions = map(lambda x:pp_class(map(float,x[2:5])),lines)

        assert len(torsions) == L
        if 1:
            total = total + 1
            if not total%50:stderr.write(`total`+'\n')

            for l in lengths:
                for pos in range(L+1-l):
                    c = Convert ( torsions[pos:pos+l] )
                    counts[l][pos][c] = counts[l][pos][c] + 1





    for l in lengths:
        for pos in range(L-l+1):
            for t in range(NN**l):
                counts[l][pos][t] = float( counts[l][pos][t] ) / total


    return counts,L,sequence,total


## read AI decoy counts
AI_counts,L,sequence,total_decoys = Torsion_counts_silent( out_file )

if ninemer_fragment_file:
    frag_counts = Torsion_counts_fragments( ninemer_fragment_file ,L)
else:
    frag_counts = {}



#AI_counts,L,sequence = Torsion_counts_pdbs( decoy_globber )

## read the native torsions
lines = map(string.split,
            popen('grep "complete" -A10000 %s'%native_pdb).readlines()[1:])
assert len(lines) == L
native_ss = ''
native_secstruct = ''
for line in lines:
    native_ss = native_ss + pp_class ( map(float,line[2:5]))
    native_secstruct = native_secstruct + line[1]


decoy_scores = {}
fragment_scores = {}


## out = open('%s.info'%prefix,'w')
out = stdout
lengths = [3] ## disable this output right now
for l in lengths:
    for pos in range(L-l+1):
        ns = native_ss[pos:pos+l]


        ## show the decoy info:
        out.write('%2d %d %-6s %-6s %-6s %4d | '\
              %(pos+1,l,
                sequence[pos:pos+l],native_secstruct[pos:pos+l],ns,
                int(floor(1000* AI_counts[l][pos][ Convert(ns)]))))


        ll = map(lambda x: [AI_counts[l][pos][x],x],AI_counts[l][pos].keys())
        ll.sort()
        ll.reverse()


        decoy_scores[(l,pos)] = [ AI_counts[l][pos][ Convert(ns)],
                              ll[0][0]]


        for n in range(min(len(ll),5)):
            if ll[n][0]<0.001:break
            out.write(' %4d %-5s'%(int(floor(1000*ll[n][0])),Convert_back( ll[n][1],l)))
        out.write('\n')


        if frag_counts:## show the fragment info
            out.write('%2d %d %-6s %-6s %-6s %4d + '\
                  %(pos+1,l,
                    sequence[pos:pos+l],native_secstruct[pos:pos+l],ns,
                    int(floor(1000* frag_counts[l][pos][ Convert(ns)]))))


            ll = map(lambda x: [frag_counts[l][pos][x],x],frag_counts[l][pos].keys())
            ll.sort()
            ll.reverse()
            fragment_scores[(l,pos)] = [ frag_counts[l][pos][ Convert(ns)],
                                     ll[0][0]]

            for n in range(min(len(ll),5)):
                if ll[n][0]<0.001:break
                out.write(' %4d %-5s'%(int(floor(1000*ll[n][0])),Convert_back( ll[n][1],l)))
            out.write('\n')
out.write('TOTAL DECOYS   : %d\n' % total_decoys)
out.write('TOTAL FRAGMENTS: %d\n' % TOP_N)

lengths = []
if CHI_SQUARED:
    for l in lengths:
        for pos in range(1,L-l): ## skip first and last fragments
            ns = native_ss[pos:pos+l]

            ## per-position chi-squared
            chi_squared_pos = 0.0
            for pp in AI_counts[l][pos].keys():
                actual = total_decoys * AI_counts[l][pos][pp]
                expected = total_decoys * frag_counts[l][pos][pp]

                if expected<1:
                    log('skipping: %d %d %s %f\n'%(pos,l,`pp`,expected))
                    continue
                chi_squared_pos = chi_squared_pos + (actual - expected)**2 /expected

            ## chi-squared contribution from most common decoy feature:
            ll = map(lambda x: [AI_counts[l][pos][x],x],AI_counts[l][pos].keys())
            ll.sort()
            ll.reverse()

            pp = ll[0][1]
            actual = total_decoys * ll[0][0]
            expected = total_decoys * frag_counts[l][pos][pp]


            if expected>=0.5:
                chi_squared = (actual-expected)**2/expected
            else:
                chi_squared = 9999999


            ## modify the output:
            ## column
            ll = map(lambda x: [frag_counts[l][pos][x],x],frag_counts[l][pos].keys())
            ll.sort()
            ll.reverse()

            pp = ll[0][1]
            if AI_counts[l][pos].has_key(pp):
                actual = 100.0 * AI_counts[l][pos][pp]
            else:
                actual = 0.0
            expected = 100.0 * frag_counts[l][pos][pp]

            print 'CHI %3d %d %-6s %-6s %-6s %6.2f %6.2f | %-6s %6.2f %6.2f %12.1f'\
                  %(pos+1,l,
                    sequence[pos:pos+l],native_secstruct[pos:pos+l],ns,
                    100.0 * AI_counts[l][pos][ Convert(ns)],
                    100.0 * frag_counts[l][pos][ Convert(ns)],
                    Convert_back(pp,l),
                    actual,expected,chi_squared_pos)

##             out.write('CHI %2d %d %-6s %-6s %-6s %4d | %-6s %9.1f %9.1f %12.1f %12.1f\n'\
##                   %(pos+1,l,
##                     sequence[pos:pos+l],native_secstruct[pos:pos+l],ns,
##                     int(floor(1000* AI_counts[l][pos][ Convert(ns)])),
##                     Convert_back(pp,l),
##                     actual,expected,chi_squared,chi_squared_pos))

exit()

## make some plots
gpout,gpin = popen2('gnuplot')
gpin.write('set terminal postscript color\n')
gpin.flush()

for l in lengths:
    plot_file = '%s.%d.plot'%(prefix,l)
    label_file_decoys = '%s.%d.decoy_labels'%(prefix,l)
    label_file_fragments = '%s.%d.fragment_labels'%(prefix,l)
    ps_file_decoys = '%s.%d.decoy_labels.ps'%(prefix,l)
    ps_file_fragments = '%s.%d.fragment_labels.ps'%(prefix,l)

    out = open(plot_file,'w')
    out2 = open(label_file_decoys,'w')
    out3 = open(label_file_fragments,'w')

    for pos in range(1, L-l): ## skip first and last fragments
        x = fragment_scores[(l,pos)] [0] ## native score
        y = fragment_scores[(l,pos)] [1] ## top score
        x_delta = decoy_scores[(l,pos)] [0] - x
        y_delta = decoy_scores[(l,pos)] [1] - y

        out.write('%f %f %f %f position: %d\n'\
                  %(x,y,x_delta,y_delta,pos+1))

        out2.write('set label "%d" at %f,%f\n'\
                   %(pos+1,x+x_delta,y+y_delta))
        out3.write('set label "%d" at %f,%f\n'\
                   %(pos+1,x,y))

    out.close()
    out2.close()
    out3.close()



    ####### arrows labeled at decoy end:
    gpin.write('set output "%s"\n'%ps_file_decoys)
    gpin.write('set title "%s frag length: %d decoy labels"\n'\
               %(string.split(prefix,'/')[-1],l))

    gpin.write('set nokey\n')
    gpin.write('set nolabel\n')
    gpin.write('load "%s"\n'%label_file_decoys)
    gpin.write('plot "%s" w vector\n'%plot_file)
    gpin.flush()


    ####### arrows labeled at fragments end:
    gpin.write('set output "%s"\n'%ps_file_fragments)
    gpin.write('set title "%s frag length: %d fragment labels"\n'\
               %(string.split(prefix,'/')[-1],l))

    gpin.write('set nokey\n')
    gpin.write('set nolabel\n')
    gpin.write('load "%s"\n'%label_file_fragments)
    gpin.write('plot "%s" w vector\n'%plot_file)
    gpin.flush()



