#!/usr/bin/python

from os import system, popen
from sys import argv, exit
import string
import time
if len(argv)<3:
    print '\n\nusage: %s  protein chain <list of clean outfiles> date  '%argv[0]
    print '\nlist should have two columns.  first column is generic'
    print 'description of outfiles for each homologue: '
    print 'if there is more than 1 outfile for each homologue '
    print 'use common substring and "*" to specify those to use.'
    print 'second column is identifier for homolog (eg hom001) '
    print '\n  '
    exit()
args = argv[1:]
native_pdb=args[0]
chain=args[1]
date=args[3]
list=args[2]
code=native_pdb[1:3]
test=0
if args.count('-test'):
    pos = args.index('-test')
    test=1
    del args[pos]

out_file_list=map(string.split,open(list,'r').readlines())
print out_file_list
command = 'mkdir clusters; cp /work/baker/paths.txt_will clusters/paths.txt;'
print(command)
system(command)

for line in out_file_list:
            out_file=line[0]
            name = line[1]
            command = ' cat %s > big.out'%out_file
            print(command)
            system(command)

            command = '~pbradley/python/make_sub_silent_file.py big.out %s_top10.out -1 10 '%name
            print(command)
            system(command)

            command='cd clusters; ~pbradley/C/cluster_info_silent.out ../%s_top10.out -  %s%s -sc 0 -ms 0,0,0 0,0 -sc 0  >& temp.log '%(name,name,date)
            print command
            system(command)

##            command='cd clusters; ~pbradley/tools/make_new_plot.py %s%s.contacts'%(protein,date)
##             command='cd clusters; ~pbradley/tools/make_new_plot.py %s%s.contacts -f aat283_03_05.200_v1-3 '%(protein,date)
##             print command
##             system(command)
            
	    for iter in range(10):

		rms_thresh=1.5 + iter*0.5

		command='cd clusters; ~pbradley/C/cluster_info_silent.out ../%s_top10.out - %s%s%s -sc 0 1,1,10,10 %s,%s -trim 2  -colony 1 -fa >& %s%s%s_colony.log '%(name,name,rms_thresh,date,rms_thresh,rms_thresh + 0.1,name,rms_thresh,date)
		print command
		system(command)

		command="cd clusters; grep CLUSTER_INFO %s%s%s.info | awk '{print $3,$6,$7,$8}' > %s_%s_%s.cluster_stats"%(name,rms_thresh,date,name,rms_thresh,date)
		print command
		system(command)

            command='cd ../../'
            print(command)
            system(command)

for iter in range(10):
    rms_thresh=1.5 + iter*0.5
    lines = map(string.split,popen('grep "^cluster00"  clusters/*%s_%s.cluster_stats'%(rms_thresh,date)).readlines())

    data_out=[]
    for line in lines:
        new_line=(line[0][-2:],line[0],line[1],'  ',line[2],'\n')
        data_out.append(new_line)
    data_out.sort()
    outfile=open('summary%s_%s.table'%(rms_thresh,date),'w')
    for line in data_out:
        out_line=string.join(line)
        outfile.write(out_line)
