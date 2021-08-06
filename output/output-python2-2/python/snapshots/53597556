#!/usr/bin/env python
"""
Given a directory containing head and .rdcat files as produced by Abi's photometry
pipeline, create the exposureList files and the DIASource database tables for use
by the Association pipeline

Run as AbiToLSST.py dirName
"""
import string, sys, os
import glob
import re
import time
import ephem
import math
#
# Date string is expected to be like: 2008-04-25T21:37:20
def calcMJD(dateStr):
    stripFracSecRE = re.compile(r'([^\.]+)')
    match = stripFracSecRE.match(dateStr)
    timeTuple = time.strptime(match.group(1), '%Y-%m-%dT%H:%M:%S')
    dateStrForEphem = time.strftime('%Y/%m/%d %H:%M:%S', timeTuple)
    dayNum = ephem.Date(dateStrForEphem)
    return dayNum + 15019.5
    
    
def lookupFilter(filtName):
    filters = {'u':0, 'g':1, 'r':2, 'i':3, 'z':4, 'y':5}
    return filters[filtName]

def getValues(fileName, varList):
    lines = open(fileName).readlines()
    valList = []
    fitsStringRE = re.compile(r'\'(\S+)\'')
    for var in varList:
        varRE = re.compile(r'^' + var + r'\s*=\s*([\S]+)')
        for line in lines:
            match=varRE.match(line)
            if match:
                value = match.group(1)
                match2 = fitsStringRE.match(value)
                if match2:
                    value = match2.group(1)
                valList.append(value)
            
    return valList

def execQuery(mySqlQuery, mySqlHost, mySqlUser, mySqlPasswd, mySqlDb ):

    mySqlCmd = 'mysql -h %s -u %s -p%s -e \'%s\' %s' % (mySqlHost, mySqlUser, mySqlPasswd, mySqlQuery, mySqlDb)

    print os.popen(mySqlCmd).read()
    

def createMopsPredTable(obsNum):
    MopsPredTableName = 'MopsPreds_visit%s' % obsNum
    createTableCmd = \
                   'DROP TABLE IF EXISTS %s; \
                   CREATE TABLE %s ( \
                   orbit_id BIGINT NOT NULL, \
                   ra_deg DOUBLE NOT NULL, \
                   dec_deg DOUBLE NOT NULL, \
                   mjd DOUBLE NOT NULL, \
                   smia DOUBLE NOT NULL, \
                   smaa DOUBLE NOT NULL, \
                   pa DOUBLE NOT NULL, \
                   mag DOUBLE NOT NULL, \
                   magErr FLOAT(0) NOT NULL \
                   ) TYPE=MyISAM;' % (MopsPredTableName, MopsPredTableName)

#    if debug: print createTableCmd
    if not debug: execQuery(createTableCmd, mySqlHost, mySqlUser, mySqlPasswd, mySqlDb)

    
def createDIASourceTable(obsNum, filterNum, catFile, mySqlHost, mySqlUser, mySqlPasswd, mySqlDb):
    maxChunk = 1000
    DIATableName = 'DiaSources_visit%s' % obsNum
    createTableCmd = \
                   'DROP TABLE IF EXISTS %s; \
                   CREATE TABLE %s ( \
                   diaSourceId BIGINT NOT NULL, \
                   ccdExposureId BIGINT NOT NULL, \
                   filterId TINYINT NOT NULL, \
                   objectId BIGINT NULL, \
                   movingObjectId BIGINT NULL, \
                   scId INTEGER NOT NULL, \
                   colc DOUBLE NOT NULL, \
                   colcErr FLOAT(0) NOT NULL, \
                   rowc DOUBLE NOT NULL, \
                   rowcErr FLOAT(0) NOT NULL, \
                   dcol DOUBLE NOT NULL, \
                   drow DOUBLE NOT NULL, \
                   ra DOUBLE NOT NULL, \
                   decl DOUBLE NOT NULL, \
                   raErr4detection DOUBLE NOT NULL, \
                   decErr4detection DOUBLE NOT NULL, \
                   raErr4wcs DOUBLE NULL, \
                   decErr4wcs DOUBLE NULL, \
                   cx DOUBLE NOT NULL, \
                   cy DOUBLE NOT NULL, \
                   cz DOUBLE NOT NULL, \
                   taiMidPoint DOUBLE NOT NULL, \
                   taiRange DOUBLE NOT NULL, \
                   fwhmA FLOAT(0) NOT NULL, \
                   fwhmB FLOAT(0) NOT NULL, \
                   fwhmTheta FLOAT(0) NOT NULL, \
                   flux DOUBLE NOT NULL, \
                   fluxErr DOUBLE NOT NULL, \
                   psfMag DOUBLE NOT NULL, \
                   psfMagErr DOUBLE NOT NULL, \
                   apMag DOUBLE NOT NULL, \
                   apMagErr DOUBLE NOT NULL, \
                   modelMag DOUBLE NOT NULL, \
                   modelMagErr DOUBLE NOT NULL, \
                   apDia FLOAT(0) NULL, \
                   Ixx FLOAT(0) NULL, \
                   IxxErr FLOAT(0) NULL, \
                   Iyy FLOAT(0) NULL, \
                   IyyErr FLOAT(0) NULL, \
                   Ixy FLOAT(0) NULL, \
                   IxyErr FLOAT(0) NULL, \
                   snr FLOAT(0) NOT NULL, \
                   chi2 FLOAT(0) NOT NULL, \
                   flag4association SMALLINT NULL, \
                   flag4detection SMALLINT NULL, \
                   flag4wcs SMALLINT NULL, \
                   _dataSource TINYINT NOT NULL, \
                   PRIMARY KEY (diaSourceId), \
                   KEY (ccdExposureId), \
                   KEY (filterId), \
                   KEY (movingObjectId), \
                   KEY (objectId), \
                   KEY (scId) \
                   ) TYPE=MyISAM;' % (DIATableName, DIATableName)

#    if debug: print createTableCmd
    if not debug: execQuery(createTableCmd, mySqlHost, mySqlUser, mySqlPasswd, mySqlDb)

    # process the catalog file for values to put into DIASource table

    commentRE = re.compile(r'^#.*|^\s*$')
    catLines = open(catFile).readlines()

    print "%d sources for exposure %d" % (len(catLines), obsNum)
    if len(catLines)==0:
        print "\tNo sources for exposure %d!" % obsNum
        return

    nSources = 0;
    while len(catLines) > 0:
        chunkLines = catLines[0:maxChunk]
        del catLines[0:maxChunk]
        insertCmd = ''
        for line in chunkLines:
            if commentRE.match(line)==None:
                (seq, flag, ra, dec, mag, magErr, stuff) = line.split(None, 6)
                if int(flag)==1:
                    id = obsNum*100000 + int(seq)
                    if insertCmd == '':
                        insertCmd = 'INSERT INTO \'%s\' (diaSourceId, ccdExposureId, filterId, ra, decl, psfMag, psfMagErr) VALUES ' % (DIATableName)
                    insertCmd += '(%d,%d,%d,%.5f,%.5f,%.3f,%.3f),' % (id, obsNum, filterNum, float(ra), float(dec), float(mag), float(magErr))
                    nSources += 1
                    
        insertCmd2 = insertCmd.rstrip(',') + ';'

#        if debug: print insertCmd2
        if not debug: execQuery(insertCmd2, mySqlHost, mySqlUser, mySqlPasswd, mySqlDb)

    print "%d used sources for exposure %d" % (nSources, obsNum)

# ---------------

debug = 0

# set up mysql access
mySqlHost = 'lsst10.ncsa.uiuc.edu'
mySqlUser = 'test'
mySqlPasswd = 'globular.test'

# get input args
mySqlDb = sys.argv[1]
photDir = sys.argv[2]
exposureListFileName = sys.argv[3]
exposureListFile = open(exposureListFileName, "w")
insertCmdFile = open("insertCmd", "w")


# regexp to get file root from .head file
headSuffixRE = re.compile(r'.head')
obsNumRE = re.compile(r'.*n(\d+).head')

# get list of exposures to process, create the exposureListFile from data in the .head files
# and create the database tables for each one

zero = 0.0
insertCmd = 'INSERT INTO Raw_FPA_Exposure (rawFPAExposureId, ra, decl, filterId, equinox, dateObs, mjdObs, expTime, airmass) VALUES '
insertVals = ''
headList = glob.glob(photDir + '/*.head')
for headFile in headList:
    catFile = headSuffixRE.sub('.rdcap', headFile)
    obsNumMatch = obsNumRE.match(headFile)
    obsNum = int(obsNumMatch.group(1))
    # get values we need from headFile
    (raDeg, decDeg, filtName, date, am, exp, epoch) = getValues(headFile,
          ['CRVAL1', 'CRVAL2', 'FILTER2', 'DATE-OBS', 'AIRMASS', 'EXPTIME', 'EPOCH'])
    # calculate MJD from date
    mjd = calcMJD(date)
    filtNum = lookupFilter(filtName)
    print >>exposureListFile, obsNum, float(raDeg), float(decDeg), filtName, mjd, date, float(am), float(exp), float(epoch)
    if not insertVals == '':
        insertVals += ','
    insertVals += '(%d, %.7f, %.7f, %d, %.7f, \'%s\', %.5f, %.2f, %.2f)' % (obsNum, float(raDeg), float(decDeg), filtNum, float(epoch), date, mjd, float(exp), float(am))
    # must pass obsNum and filtNum to createDIASourceTable
    createDIASourceTable(obsNum, filtNum, catFile, mySqlHost, mySqlUser, mySqlPasswd, mySqlDb)
    createMopsPredTable(obsNum)

insertCmd += insertVals
if debug: print >>insertCmdFile, insertCmd
if not debug: execQuery(insertCmd, mySqlHost, mySqlUser, mySqlPasswd, mySqlDb)
