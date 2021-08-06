#!/usr/bin/env python
"""
Given a MySQL LSST schema database that has been processed by the
Association PL, prepare the input file that will be used by FindGray

Run as preparefindGray <db_name>
"""
import string, sys, os
import glob
import re
import math
import MySQLdb
#
# minObs is the minimum number of obs for an object and filter to use it for gray determination

mingObs = 999
minrObs = 10
miniObs = 999
minzObs = 999

# set up default extinction table, order is u, g, r, i, z, y
extinction = [0.4, 0.15, 0.06, 0.06, 0.06, 0.06]

# set up mysql access
mySqlHost = 'lsst10.ncsa.uiuc.edu'
mySqlUser = 'test'
mySqlPasswd = 'globular.test'

# get input args
mySqlDb = sys.argv[1]

db = MySQLdb.connect(host=mySqlHost, user=mySqlUser, passwd=mySqlPasswd, db=mySqlDb)

c=db.cursor()

# First, get all the Objects

c.execute("""SELECT objectId, gMag, rMag, iMag, zMag, gNumObs, rNumObs, iNumObs, zNumObs FROM Object""")

objList = c.fetchall()

# print column header line, to be read by R prog findGray.r
print "Name ObjFiltId Time Flux FluxErr"

oFCount = 0
for o in objList:
    objectId = o[0]
    gNumObs = o[5]
    rNumObs = o[6]
    iNumObs = o[7]
    zNumObs = o[8]
    # Selection on mags or xNumObs goes here ... later
    goodFilters = [0, 0, 0, 0, 0, 0]
    objIdByFilter = [-1, -1, -1, -1, -1, -1]
    goodFilters[1] = (gNumObs > mingObs)
    goodFilters[2] = (rNumObs > minrObs)
    goodFilters[3] = (iNumObs > miniObs)
    goodFilters[4] = (zNumObs > minzObs)
    
    c.execute("""SELECT ds.diaSourceId, ex.mjdObs, ex.airmass, ds.filterId, ds.psfMag, ds.psfMagErr from DIASource as ds, \
              Raw_FPA_Exposure as ex WHERE ds.objectId = %s AND ex.rawFPAExposureId = ds.ccdExposureId""", (objectId,))
    srcList = c.fetchall()
    for s in srcList:
        sourceId = s[0]
        mjd = s[1]
        airMass = s[2]
        filterId = s[3]
        mag = s[4]
        magErr = s[5]
        if goodFilters[filterId]:
            if objIdByFilter[filterId] == -1:
                objIdByFilter[filterId] = oFCount;
                oFCount += 1;
            correctedMag = mag - extinction[filterId]*airMass
            correctedFlux = 10**(-0.4*correctedMag)
            flux = 10**(-0.4*mag)
            fluxErr = flux*(1.0 - 10**(-0.4*magErr))
            print sourceId, objIdByFilter[filterId], mjd, correctedFlux, fluxErr

        

