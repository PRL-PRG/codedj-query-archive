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
from numarray import *
from rpy import r
import rpy
#
# Unless you do this, summary.lm doesn't work right from python...
#
r.lm.local_mode(rpy.NO_CONVERSION)
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

# TRUNCATE DIA_Poly here
query = "TRUNCATE DIA_Poly"
c.execute(query)

for e in expList:
    exp = e[0]
    query = "SELECT ds1.diaSourceId, ds1.colc, ds1.rowc, ds1.modelMag-ds2.ModelMag FROM DIASource as ds1, DIASource as ds2 where ds1.ccdExposureId=%s and ds2.ccdExposureId=%s and ds1.objectId=ds2.objectId" % (exp, refExp)
    c.execute(query)
    surfPoints = c.fetchall()
    py_id = []
    py_x = []
    py_y = []
    py_xy = []
    py_delta = []
    for point in surfPoints:
        py_id.append(point[0])
        py_x.append(float(point[1]))
        py_y.append(float(point[2]))
        py_xy.append(float(point[1])*float(point[2]))
        py_delta.append(float(point[3]))
        # py_sigma.append() rss of two sigmas?

    # Now need to evaluate poly's at py_x and py_y, insert into DIA_Poly

    poly_x_vals = array(r.poly(py_x, 2)) # poly_x_vals is a matrix with two columns, for x^1, x^2
    poly_y_vals = array(r.poly(py_y, 2)) # poly_y_vals is a matrix with two columns, for y^1, y^2
    poly_xy_vals = array(r.poly(py_xy, 1)) # poly_xy_vals is a matrix with one column
    query = "INSERT INTO DIA_Poly (diaSourceId, valx1, valx2, valy1, valy2, valxy) VALUES "
    poly_values = ""
    # make list of x,y pairs from poly_x_vals, poly_y_vals
    i=0
    for id in py_id:
        x1 = poly_x_vals[i,0]
        x2 = poly_x_vals[i,1]
        y1 = poly_y_vals[i,0]
        y2 = poly_y_vals[i,1]
        xy = poly_xy_vals[i]
        if poly_values: poly_values = poly_values + ","
        poly_values += "(%s, %f, %f, %f, %f, %f)" % (id, x1, x2, y1, y2, xy)
        i = i+1

    query = query + poly_values
#    print query
    c.execute(query)

    
    model = r.lm(r("delta ~ poly(x, 2) + poly(y, 2) + poly(x*y, 1)"), data=r.data_frame(x=py_x, y=py_y, delta=py_delta))
    model_summary = r.summary(model)
    model_coeff = model_summary['coefficients']
    c0 = model_coeff[0][0]
    c0_sigma = model_coeff[0][1]
    cx1 = model_coeff[1][0]
    cx1_sigma = model_coeff[1][1]
    cx2 = model_coeff[2][0]
    cx2_sigma = model_coeff[2][1]
    cy1 = model_coeff[3][0]
    cy1_sigma = model_coeff[3][1]
    cy2 = model_coeff[4][0]
    cy2_sigma = model_coeff[4][1]
    cxy = model_coeff[5][0]
    cxy_sigma = model_coeff[5][1]
    if values: values = values + ","
    values = values + "(%s, %s, %s, %s, %s, %s, %s, %s, %s, %s, %s, %s, %s)" % (exp, c0, c0_sigma, cx1, cx1_sigma, cx2, cx2_sigma, cy1, cy1_sigma, cy2, cy2_sigma, cxy, cxy_sigma)

query = "TRUNCATE Gray_Surf; INSERT INTO Gray_Surf (ccdExposureId, c0, c0_sigma, cx1, cx1_sigma, cx2, cx2_sigma, cy1, cy1_sigma, cy2, cy2_sigma, cxy, cxy_sigma ) VALUES " + values
#print query
c.execute(query)





