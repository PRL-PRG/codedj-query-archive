#!/usr/bin/env python
"""
Given a MySQL LSST schema database that has been processed by the
Association PL, plot the lightcurve of a given object and filter.
psfMag is the raw instrumental mags, modelMag is the corrected

Run as plotLsstLightCurve.py <db> <objectid> <filter>
"""
import string, sys, os
import glob
import re
import math
import MySQLdb

def getLC(mySqlDb, objectId, filter):
    #
    # set up mysql access params
    mySqlHost = 'lsst10.ncsa.uiuc.edu'
    mySqlUser = 'test'
    mySqlPasswd = 'globular.test'
    
    #
    filterMap = { "u":0, "g":1, "r":2, "i":3, "z":4 }
    
    filterId = filterMap[filter]
    #
    # Set up connection to db
    #
    db = MySQLdb.connect(host=mySqlHost, user=mySqlUser, passwd=mySqlPasswd, db=mySqlDb)
    
    c=db.cursor()
    #
    # Get the needed objects
    #
    query = "SELECT ex.mjdObs, ds.psfMag, ds.modelMag from DIASource as ds, Raw_FPA_Exposure as ex  where ds.objectId=%s and ds.filterId=%d and ex.rawFPAExposureId=ds.ccdExposureId" % (objectId, filterId)
    
    c.execute(query)
    
    srcList = c.fetchall()
    mjd = []
    instMag = []
    corrMag = []
    for s in srcList:
        mjd.append(s[0])
        instMag.append(s[1])
        corrMag.append(s[2])
        
    return (mjd, instMag, corrMag)



