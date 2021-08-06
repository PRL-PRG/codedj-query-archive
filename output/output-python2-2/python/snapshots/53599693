#!/usr/bin/env python

"""Ephemeris.py:

Contains an Ephemeris class to be used for Day/Night MOPS.  Not to be
confused with ephem.ephemeris, which is a wrapper around a FORTRAN
ephemeris calculation function.

"""



class Ephemeris(object):
    def __init__(self, orbitId, MJD, RA, Dec, Mag, SMAA, SMIA, PA):
        """
        Ephemeris: A point in the sky of a moving object at some given time

        orbitId: integer, ID of orbit associated with this Ephemeris
        MJD: Mean Julian Date, floating point
        RA: Right Ascension (deg)
        Dec: Declination (deg)
        Mag: Magnitude (optical)
        SMAA: error ellipse semi major axis (deg)
        SMIA: error ellipse semi minor axis (deg)
        PA:  error ellipse position angle
        """
        self.orbitId = orbitId
        self.MJD = MJD
        self.RA = RA
        self.Dec = Dec
        self.Mag = Mag
        self.SMAA = SMAA
        self.SMIA = SMIA
        self.PA = PA
        return

    def pr(self):
        print self.orbitId, self.MJD, self.Dec, self.Mag, self.SMAA, self.SMIA, self.PA

