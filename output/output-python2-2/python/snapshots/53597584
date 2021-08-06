#!/usr/bin/env python
"""
Given a reference exposure, for all other exposures in the specified db fit a
surface as a function of x and y which approximately registers the corrected
magnitudes to those of the reference exposure. Store the polynomial coefficients
that describe the surface in the db table GraySurf

Run as fitGray <db_name> <ref_exp>
"""
import string, sys, os
import glob
import re
import math
import MySQLdb
from rpy import r
#
# set up mysql access
mySqlHost = 'lsst10.ncsa.uiuc.edu'
mySqlUser = 'test'
mySqlPasswd = 'globular.test'

# get input args
mySqlDb = sys.argv[1]
refExp = sys.argv[2]

# open the DB
db = MySQLdb.connect(host=mySqlHost, user=mySqlUser, passwd=mySqlPasswd, db=mySqlDb)
c=db.cursor()

# get the list of exposures

query = "SELECT rawFPAExposureId from Raw_FPA_Exposure ORDER BY rawFPAExposureId"

c.execute(query)

expList = c.fetchall()

values = ""

for e in expList:
    exp = e[0]
    query = "SELECT ds1.colc, ds1.rowc, ds1.modelMag-ds2.ModelMag FROM DIASource as ds1, DIASource as ds2 where ds1.ccdExposureId=%s and ds2.ccdExposureId=%s and ds1.objectId=ds2.objectId" % (exp, refExp)
    c.execute(query)
    surfPoints = c.fetchall()
    py_x = []
    py_y = []
    py_delta = []
    for point in surfPoints:
        py_x.append(float(point[0]))
        py_y.append(float(point[1]))
        py_delta.append(float(point[2]))

    
    model = r.lm(r("delta ~ poly(x, 2, raw=TRUE) + poly(y, 2, raw=TRUE) + poly(x*y, 1, raw=TRUE)"), data=r.data_frame(x=py_x, y=py_y, delta=py_delta))
    c0 = model['coefficients']['(Intercept)']
    cx1 = model['coefficients']['poly(x, 2, raw = TRUE)1']
    cx2 = model['coefficients']['poly(x, 2, raw = TRUE)2']
    cy1 = model['coefficients']['poly(y, 2, raw = TRUE)1']
    cy2 = model['coefficients']['poly(y, 2, raw = TRUE)2']
    cxy = model['coefficients']['poly(x * y, 1, raw = TRUE)']
    if values: values = values + ","
    values = values + "(%s, %s, %s, %s, %s, %s, %s)" %(exp, c0, cx1, cx2, cy1, cy2, cxy)

query = "TRUNCATE Gray_Surf; INSERT INTO Gray_Surf (ccdExposureId, c0, cx1, cx2, cy1, cy2, cxy) VALUES " + values
print query
c.execute(query)





