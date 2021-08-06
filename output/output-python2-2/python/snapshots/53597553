#!/usr/bin/env python
"""
Given a MySQL LSST schema database that has been processed by the
Association PL, and subsequently by fitGray, etc,
calculate the shape of the gray extinction surface for the reference
exposure, from the weighted mean of the other exposure shapes.

Then, subtracts the reference surface from all surface entries in Gray_Surf

Reference for std. error of weighted mean:  J. Scarborough, American Math. Monthly,
42, 286(1935), found on JStor

Run as correctMeanRefSurface.py <db> <refExp>
"""
import string, sys, os
import glob
import re
import math
from numpy import *
import MySQLdb

#
# set up mysql access params
mySqlHost = 'lsst10.ncsa.uiuc.edu'
mySqlUser = 'test'
mySqlPasswd = 'globular.test'

#
mySqlDb = sys.argv[1]
refExposure = sys.argv[2]

#
# Set up connection to db
#
db = MySQLdb.connect(host=mySqlHost, user=mySqlUser, passwd=mySqlPasswd, db=mySqlDb)

c=db.cursor()
#
# Get the needed objects
#
query = "SELECT c0, c0_sigma, cx1, cx1_sigma, cx2, cx2_sigma, cy1, cy1_sigma, cy2, cy2_sigma, cxy, cxy_sigma from Gray_Surf WHERE NOT ccdExposureId = %s " % refExposure

c.execute(query)

coeffList = c.fetchall()
c0 = []
cx1 = []
cx2 = []
cy1 = []
cy2 = []
cxy = []
c0_sigma = []
cx1_sigma = []
cx2_sigma = []
cy1_sigma = []
cy2_sigma = []
cxy_sigma = []

for s in coeffList:
    c0.append(s[0])
    cx1.append(s[2])
    cx2.append(s[4])
    cy1.append(s[6])
    cy2.append(s[8])
    cxy.append(s[10])
    c0_sigma.append(s[1])
    cx1_sigma.append(s[3])
    cx2_sigma.append(s[5])
    cy1_sigma.append(s[7])
    cy2_sigma.append(s[9])
    cxy_sigma.append(s[11])

c0_vari = 1.0/array(c0_sigma)**2
cx1_vari = 1.0/array(cx1_sigma)**2
cx2_vari = 1.0/array(cx2_sigma)**2
cy1_vari = 1.0/array(cy1_sigma)**2
cy2_vari = 1.0/array(cy2_sigma)**2
cxy_vari = 1.0/array(cxy_sigma)**2

c0_wt = sum(array(c0)*c0_vari)/sum(c0_vari)
c0_wt_sigma = 1.0 / sqrt(sum(c0_vari))
print "c0:", c0_wt, " +/- ", c0_wt_sigma

cx1_wt = sum(array(cx1)*cx1_vari)/sum(cx1_vari)
cx1_wt_sigma = 1.0 / sqrt(sum(cx1_vari))
print "cx1:", cx1_wt,  " +/- ", cx1_wt_sigma

cx2_wt = sum(array(cx2)*cx2_vari)/sum(cx2_vari)
cx2_wt_sigma = 1.0 / sqrt(sum(cx2_vari))
print "cx2:", cx2_wt,  " +/- ", cx2_wt_sigma

cy1_wt = sum(array(cy1)*cy1_vari)/sum(cy1_vari)
cy1_wt_sigma = 1.0 / sqrt(sum(cy1_vari))
print "cy1:", cy1_wt,  " +/- ", cy1_wt_sigma

cy2_wt = sum(array(cy2)*cy2_vari)/sum(cy2_vari)
cy2_wt_sigma = 1.0 / sqrt(sum(cy2_vari))
print "cy2:", cy2_wt,  " +/- ", cy2_wt_sigma

cxy_wt = sum(array(cxy)*cxy_vari)/sum(cxy_vari)
cxy_wt_sigma = 1.0 / sqrt(sum(cxy_vari))
print "cxy:", cxy_wt,  " +/- ", cxy_wt_sigma

#
# Now correct all the entries in Gray_Surf
# 
query = "UPDATE Gray_Surf SET cx1=cx1-(%f), cx2=cx2-(%f), cy1=cy1-(%f), cy2=cy2-(%f), cxy=cxy-(%f)" % (cx1_wt, cx2_wt, cy1_wt, cy2_wt, cxy_wt)
print query

c.execute(query)





