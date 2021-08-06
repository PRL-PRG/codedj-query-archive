#!/usr/bin/python

import sys
import string
from math import floor
from os.path import basename,exists
from os import popen,system, popen2
from popen2 import popen2


###########################################

def getbestline(outfile,scoreindex):
    lines = open(outfile).readlines()

    bestline = lines[0]
    for line in lines:
        cols = string.split(bestline)
        minscore = float( cols[scoreindex-1] )
        besttag = cols[-1]
        if not besttag.count('_'):
            besttag = cols[-2]

        cols = string.split(line)
        try:
            score = float( cols[scoreindex-1] )
            if score < minscore:
                bestline = line
        except:
            continue

    return (bestline,besttag,minscore)

################# MAIN!

pdbname = sys.argv[1]
outfileprefix = sys.argv[2]
scoreindex = int( sys.argv[3])
rmsindex   = int( sys.argv[4])

doeverything = 1
if sys.argv.count('-fast'):
    print 'DOING MINIMAL STUFF'
    doeverything = 0

lowSCOREoutfile = outfileprefix+'.out'
lowRMSoutfile = outfileprefix+'.rms.out'
scorefile = outfileprefix+'.sc'

################# CLEAN

command = 'python /users/rhiju/python/clean_sc.py  '+scorefile+' FILTER.sc'
print(command)
if not exists('FILTER.sc'):
    if (doeverything):
        system(command)


###################### RMS #########################
if (doeverything):
    (bestRMSline,bestRMStag,bestRMS) = getbestline('FILTER.sc',rmsindex)
    print bestRMSline

    #bestrms = open('bestrms.txt','w')
    #bestrms.write( bestRMSline);
    #bestrms.close()
    command = 'grep '+bestRMStag+' '+scorefile+' > bestrms.txt'
    print(command)
    system(command)

#Get rid of "-fa_input" if no side chains present.

#    command = '/users/rhiju/rosetta++/rosetta.gcc -paths /users/rhiju/paths.txt -extract -fa_input -s '+lowRMSoutfile+ ' -t '+bestRMStag
    command = '/users/rhiju/rosetta++/rosetta.gcc -paths /users/rhiju/paths.txt -extract -s '+lowRMSoutfile+ ' -t '+bestRMStag
    print(command)
    if (doeverything):
        system(command)

    command = 'mv '+bestRMStag+'.pdb lowrms.pdb'
    print(command)
    system(command)

###################### SCORES #########################
if (doeverything):
    (bestSCOREline, bestSCOREtag,bestSCORE) = getbestline('FILTER.sc',scoreindex)
    print bestSCOREline

    #bestscore = open('bestscore.txt','w')
    #bestscore.write( bestSCOREline);
    #bestscore.close()
    command = 'grep '+bestSCOREtag+' '+scorefile+' > bestscore.txt'
    print(command)
    if (doeverything):
        system(command)

#    command = '/users/rhiju/rosetta++/rosetta.gcc -paths /users/rhiju/paths.txt -extract -fa_input -s '+lowSCOREoutfile+ '-t '+bestSCOREtag
        command = '/users/rhiju/rosetta++/rosetta.gcc -paths /users/rhiju/paths.txt -extract -s '+lowSCOREoutfile+ ' -t '+bestSCOREtag
    print(command)
    if (doeverything):
        system(command)

    command = 'mv '+bestSCOREtag+'.pdb lowenergy.pdb'
    print(command)
    if (doeverything):
        system(command)

###################### GET PDB #########################
command = 'cp /work/rhiju/bench_abinitio/contactmaps_homologs/'+pdbname+'.pdb .'
print(command)
if (doeverything):
    system(command)

##################### GENERATE POSTSCRIPTS  ##############
listfile = open('cartoons.list','w');
listfile.write( pdbname+'.pdb\n')
listfile.write( 'lowenergy.pdb\n')
listfile.write( 'lowrms.pdb\n')

command = 'make_aligned_pictures.py cartoons.list 4'
print(command)
#system(command)

command = 'mv 01.ps '+pdbname+'_01.ps'
print(command)
system(command)

command = 'mv 02.ps '+pdbname+'_03.ps'
print(command)
system(command)

command = 'mv 03.ps '+pdbname+'_04.ps'
print(command)
system(command)

####### gnuplot to get best score/rms #########
scorefile = 'FILTER.sc'

if (doeverything):
    gpout, gpin = popen2('gnuplot')
    gpin.write('set nokey\n')
    gpin.write('set xrange [0:20]\n')
    gpin.write('set yrange [%f:%f]\n' % (bestSCORE-5, bestSCORE+80))
    gpin.write('set xlabel "RMSD"\n');
    gpin.write('set ylabel "Energy"\n');
    gpin.write('plot "'+scorefile+'" u %d:%d \n' % (rmsindex,scoreindex));
    gpin.write('replot "bestscore.txt" u %d:%d points 2\n' % (rmsindex,scoreindex));
    gpin.write('replot "bestscore.txt" u %d:%d points 2\n' % (rmsindex,scoreindex));
    gpin.write('replot "bestscore.txt" u %d:%d points 2\n' % (rmsindex,scoreindex));
    gpin.write('set terminal postscript color \n');
    gpin.write('set output "'+pdbname+'_plot.ps"\n')
    gpin.write('replot\n');
    gpin.write('set terminal x11\n');
    gpin.write('set output\n');


    gpin.write('set nokey\n')
    gpin.write('set xrange [0:20]\n')
    gpin.write('set yrange [%f:%f]\n' % (bestSCORE-5, bestSCORE+80))
    gpin.write('set xlabel "RMSD"\n');
    gpin.write('set ylabel "Energy"\n');
    gpin.write('plot "'+scorefile+'" u %d:%d\n' % (rmsindex,scoreindex));
    gpin.write('replot "bestrms.txt" u %d:%d points 2\n' % (rmsindex,scoreindex));
    gpin.write('replot "bestrms.txt" u %d:%d points 2\n' % (rmsindex,scoreindex));
    gpin.write('replot "bestrms.txt" u %d:%d points 2\n' % (rmsindex,scoreindex));
    gpin.write('set terminal postscript color \n');
    gpin.write('set output "'+pdbname+'_plot_rms.ps"\n')
    gpin.write('replot\n');
    gpin.write('set terminal x11\n');
    gpin.write('set output\n');
    gpin.write('exit\n');

    gpin.close()

################################# RASMOL ########
command = 'superimpose.py '+pdbname+'.pdb lowenergy.pdb > '+pdbname+'_lowenergy_sup.pdb'
print(command)
if (doeverything):
    system(command)

if (doeverything):
    #rin = open('rasmol.script','w')
    rout,rin = popen2('/net/local/bin/rasmol')
    filename =  pdbname+'_lowenergy_sup.pdb'
    rin.write('load %s\nstructure\ncartoons\nset background white\ncolor chain\n'%filename)
    rin.write('wireframe off\n')
#    rin.write('select buried and not hydrogen\nwireframe 50\n')
    rin.flush()
    raw_input()
    rin.write('pause\n');
    rin.write('write ps '+pdbname+'_lowenergy_sup.ps\n');
    rin.write('quit\n');
    rin.flush()
    rin.close()

#command = '/net/local/bin/rasmol -script rasmol.script'
#print(command)
#system(command)

##### CONVERT TO PNG ************

command = 'convert '+pdbname+'_01.ps '+pdbname+'_01.png'
print(command)
system(command)

command = 'convert '+pdbname+'_03.ps '+pdbname+'_03.png'
print(command)
system(command)

command = 'convert '+pdbname+'_04.ps '+pdbname+'_04.png'
print(command)
system(command)


command = 'convert -rotate 90 '+pdbname+'_plot.ps '+pdbname+'_plot.png'
print(command)
system(command)

command = 'convert -rotate 90 '+pdbname+'_plot_rms.ps '+pdbname+'_plot_rms.png'
print(command)
system(command)

command = 'convert '+pdbname+'_lowenergy_sup.ps '+pdbname+'_lowenergy_sup.png'
print(command)
system(command)


lines = open(scorefile).readlines()
