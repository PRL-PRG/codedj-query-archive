#!/usr/bin/python

from sys import argv,exit


from glob import glob
from os.path import basename,exists
from os import popen,system
import string

outdirs = argv[1:]

native_supplied = 0
if (outdirs[0][-4:] == '.pdb' ): # Native specified
    native_pdb = outdirs[ 0 ]
    del( outdirs[ 0 ] )
    native_supplied = 1


# Do not use outfile!!

for outdir in outdirs:

    assert( outdir.count('_OUT' ) )

    ######################################
    # look for rms column in outfile
    #outfile_lines = popen( 'grep SCORE '+outfile ).readlines()
    #headerline = outfile_lines[0]
    #try:
    #cols = string.split( headerline )
    #    found_rms_col = cols.index( 'rms' )
    #except:
    #    found_rms_col = -1
    #
    #if (found_rms_col > -1 ):
    #    for line in outfile_lines[1:] :
    #        cols = string.split( line )
    #        tag = cols[-1]
    #        try:
    #            rms = float( cols[ found_rms_col ] )
    #            rms_vals[ tag ] = rms
    #        except:
    #            continue

    native_exists = native_supplied
    if not native_exists:
        # Still look for native in rhiju's directory.
        pos = outdir.index( 'chunk')
        rna_name = outdir[pos:(pos+13)]
        native_pdb = '/work/rhiju/projects/rna_new_benchmark/bench_final/%s_RNA.pdb' % rna_name
        if exists( native_pdb ):
            native_exists = 1


    #############################################
    # Go through scorefile from tinker minimization

    globfiles = glob( outdir+'/*OUT/*sc' )
    globfiles.sort()


    all_scores = {}
    score_terms = []

    obligate_score_terms =  ['Total_Potential_Energy', 'Intermolecular_Energy']
    for score_term in obligate_score_terms:
        score_terms.append( score_term )
        all_scores[ score_term ] = {}

    files_to_calc_rms = []
    for file in globfiles:

        for score_term in obligate_score_terms: all_scores[score_term][file] = 0.0 #Always have one of these terms.
        lines = open( file ).readlines()
        for line in lines:
            cols = string.split( line.replace(':','')  )
            try:
                score = float( cols[-2] )
                score_term = string.join(cols[:-2],'_')
                if score_term not in score_terms:
                    score_terms.append( score_term )
                    all_scores[ score_term ] = {}
                all_scores[ score_term ][ file ] = score
            except:
                continue


        if native_exists:
            rmsfile = file.replace('.sc','.rms.txt')
            if  exists( rmsfile.replace('.rms.txt','.pdb') ) and not exists( rmsfile ):
                files_to_calc_rms.append( rmsfile.replace('.rms.txt','.pdb' ) )

            rmsfile = rmsfile.replace('min_','').replace('minimize_','')
            if  exists( rmsfile.replace('.rms.txt','.pdb') ):# and not exists( rmsfile ):
                files_to_calc_rms.append( rmsfile.replace('.rms.txt','.pdb' ) )


    #print score_terms

    ##########################################################################
    # superimpose any files that haven't been supermposed
    if native_exists and len( files_to_calc_rms) > 0 :

        # Create a text file with the pdbs. This used to go in on command line
        # but unix has a limit.
        fid = open( 'list.txt', 'w' )
        for file in files_to_calc_rms:
            fid.write( file+'\n')
        fid.close()

        command = '~rhiju/python/tinker_superimpose.py '+native_pdb+' list.txt'
        print( command )
        system( command )


    ##########################################################################
    # read out rms's from saved files.
    rms_vals = {}
    rms_vals_init = {}

    for file in globfiles:
        rmsfile = file.replace( '.sc','.rms.txt' )
        if not exists( rmsfile): continue
        rmslines = popen( 'tail -n 1 '+rmsfile ).readlines()
        if len( rmslines ) < 1: continue
        rmsline = rmslines[-1]
        if len( string.split(rmsline) ) < 1: continue
        rms_vals[ file ] = float( string.split( rmsline )[-1] )

        rmsfile = rmsfile.replace('min_','').replace('minimize_','')
        if not exists( rmsfile): continue
        rmslines = popen( 'tail -n 1 '+rmsfile ).readlines()
        if len( rmslines ) < 1: continue
        rmsline = rmslines[-1]
        if len( string.split(rmsline) ) < 1: continue
        rms_vals_init[ file ] = float( string.split( rmsline )[-1] )


    ######################################################
    # Output unified scorefile.
    new_scorefile = outdir.replace( '_OUT', '_minimize.sc' )
    print ' Creating '+new_scorefile
    fid = open( new_scorefile ,'w')

    fid.write( 'SCORE: ' )
    for score_term in score_terms:
        fid.write( ' %10s' % score_term[:10] )

    if native_exists:
        fid.write( ' %8s' % 'rms_init' )
        fid.write( ' %8s' % 'rms' )

    fid.write( ' description\n' )

    for file in globfiles:

        if not file in rms_vals.keys(): continue
        if not file in rms_vals_init.keys(): continue

        score_term_present = 1
        for score_term in score_terms:
            if not file in all_scores[score_term].keys():
                score_term_present = 0
                break
        if not score_term_present: continue

        fid.write( 'SCORE: ' )

        for score_term in score_terms:
            fid.write( ' %10.4f' % all_scores[ score_term ][file ] )

        if native_exists:
            #tag = basename( file ).replace('min_','').replace('.sc','').replace('minimize_','')
            fid.write( ' %8.4f' % rms_vals_init[ file ] )
            fid.write( ' %8.4f' % rms_vals[ file ] )

        fid.write( ' '+basename(file).replace('.sc','') + '\n' )

    fid.close()
