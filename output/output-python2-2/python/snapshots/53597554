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
from numpy.numarray import array
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
    query = "SELECT ex.mjdObs, ds.modelMag, ds.apMag, ds.rowc, ds.colc, ex.airmass, gs.c0, gs.cx1, gs.cx2, gs.cy1, gs.cy2, gs.cxy from DIASource as ds, Raw_FPA_Exposure as ex, Gray_Surf as gs  where ds.objectId=%s and ds.filterId=%d and ex.rawFPAExposureId=ds.ccdExposureId and gs.ccdExposureId=ex.rawFPAExposureId" % (objectId, filterId)
    
    c.execute(query)
    
    srcList = c.fetchall()
    mjd = []
    corr1Mag = []
    corr2Mag = []
    rowc = []
    colc = []
    airmass = []
    c0 = []
    cx1 = []
    cx2 = []
    cy1 = []
    cy2 = []
    cxy = []

    for s in srcList:
        mjd.append(s[0])
        corr1Mag.append(s[1])
        corr2Mag.append(s[2])
        rowc.append(s[3])
        colc.append(s[4])
        airmass.append(s[5])
        c0.append(s[6])
        cx1.append(s[7])
        cx2.append(s[8])
        cy1.append(s[9])
        cy2.append(s[10])
        cxy.append(s[11])
        
        
        
    return (array(mjd), array(corr1Mag), array(corr2Mag), array(colc), array(rowc), array(airmass), array(c0), array(cx1), array(cx2), array(cy1), array(cy2), array(cxy))



