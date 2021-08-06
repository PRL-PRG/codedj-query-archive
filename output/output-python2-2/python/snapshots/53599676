#!/usr/bin/env python

"""Ephemeris.py:

Contains an Ephemeris class to be used for Day/Night MOPS.  Not to be
confused with ephem.ephemeris, which is a wrapper around a FORTRAN
ephemeris calculation function.

"""



class Ephemeris(object):
    def __init__(self, movingObjectId, movingObjectVersion, 
                 mjd, ra, dec, mag, smaa, smia, pa):
        """
        Ephemeris: A point in the sky of a moving object at some given time

        movingObjectId: integer, ID of obj associated with this Ephemeris
        movingObjectVersion: version of the obj associated with this Ephemeris
        mjd: Mean Julian Date, floating point
        ra: Right Ascension (deg)
        dec: Declination (deg)
        mag: Magnitude (optical)
        smaa: error ellipse semi major axis (deg)
        smia: error ellipse semi minor axis (deg)
        pa:  error ellipse position angle
        """
        self.movingObjectId = movingObjectId
        self.movingObjectVersion = movingObjectVersion
        self.mjd = mjd
        self.ra = ra
        self.dec = dec
        self.mag = mag
        self.smaa = smaa
        self.smia = smia
        self.pa = pa
        return

    def pr(self):
        print self.orbitId, self.MJD, self.Dec, self.Mag, self.SMAA, self.SMIA, self.PA

