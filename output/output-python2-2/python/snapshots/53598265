#!/usr/bin/env python
"""
Unit Tets for the ephem module

Given an orbit with its covariance matrix and a list of dates (in MJD),
ephem.ephemerides() computes the position of that orbit at each of the input
dates. Computed positions are given as a 6-element numpy array of the form
    [RA, Dec, mag, MJD, Err1, Err2, PA]
Where all angles are given in radians. Err1, Err2 and PA define the error
ellipses on the sky plane.

Input and ground truth data from these tests was obtained XXX
"""
import math
import time

import numpy
import ephem

import unittest



# Constants
# Ppercent results have to agree within (1 = 100%)
THRESH = 0.00001


class TestEphemerides(object):
    orbitType = None
    orbitElements = None
    covar = None
    
    def testEphemerides(self):
        t0 = time.time()
        if(self.norm == None):
            res = ephem.ephemerides(elementsType=self.orbitType,
                                    orbitElements=self.orbitElements,
                                    covariance=self.covar,
                                    orbitEpoch=self.orbitEpoch,
                                    absMag=self.absMag,
                                    obsCode=self.obscode,
                                    times=self.times)
        else:
            res = ephem.ephemerides(elementsType=self.orbitType,
                                    orbitElements=self.orbitElements,
                                    covariance=self.covar,
                                    normal=self.norm,
                                    orbitEpoch=self.orbitEpoch,
                                    absMag=self.absMag,
                                    obsCode=self.obscode,
                                    times=self.times)
        
        # print('Speed: %.02f ephemerides/sec' \
        #       %(float(len(self.times)) / float(time.time() - t0)))
        
        # Check the results (RA, Dec, mag, t, smaa, smia, PA).
        # print(res)
        # print(self.truth)
        for i in range(len(self.times)):
            # Check RA, Dec and t only since:
            #  1. Magnitue prediction is very hard and it always have very
            #     large errors.
            #  2. Positional error ellipses are not fully usable for the time
            #     being.
            for j in (0, 1, 3):
                difference = abs(self.truth[i][j] - res[i][j]) / \
                             abs(self.truth[i][j])
                try:
                    self.assert_(difference < THRESH)
                except:
                    raise(AssertionError('%f differs from %f more than %f%%' \
                                         %(res[i][j], self.truth[i][j],
                                           THRESH * 100.)))
            # <-- end for
        # <-- end for
        return




class Test1999AN10(TestEphemerides, unittest.TestCase):
    """
    Unit test for ephem.ephemerides() and asteroid 1999 AN10

    Since there will be some rounding errors in the floats we get, we simply ask
    that numbers agree up to a fraction of a percent (see THRESH above).

    This uses cometary elements.
    """
    def setUp(self):
        self.orbitType = 'COM'
        self.obscode = 568              # MPC observatory code.
        self.orbitElements = numpy.array([.6387361553178337,
                                          .5621362112034225,
                                          39.93129014386989,
                                          314.5520574846994,
                                          268.2537813995116,
                                          51274.77983127534])
        self.orbitEpoch = 51420.
        self.absMag = 17.807
        self.covar = numpy.array([3.434026484028151E-14,
                                  -2.252900193599293E-14,
                                  -2.31486106541803E-14,
                                  8.804533822918358E-15,
                                  -5.750454707798054E-14,
                                  -2.500427830392214E-13,
                                  1.479245725339502E-14,
                                  1.672832546975585E-14,
                                  -6.412082055503635E-15,
                                  3.825067156115482E-14,
                                  2.027219619498807E-13,
                                  5.837364709325289E-13,
                                  -1.98397641969486E-13,
                                  1.950227533551067E-13,
                                  6.273124142282174E-12,
                                  1.085206138823795E-13,
                                  -9.72249850255963E-14,
                                  -2.257160097586615E-12,
                                  2.036315284520349E-13,
                                  4.529840561194134E-12,
                                  3.341341415398777E-10])
        self.norm = None
        self.times = numpy.array([54314., 53949., 53584.])
        self.truth = numpy.array([[78.392125000000007,
                                   55.358083333333333,
                                   22.72,
                                   53584.,
                                   0.00010544599444444444,
                                   5.6436113888888893e-05,
                                   74.62],
                                  [8.2724999999999991,
                                   41.620694444444446,
                                   19.90,
                                   53949.,
                                   0.00016466414722222223,
                                   0.00013237720555555555,
                                   -73.83],
                                  [101.74604166666666,
                                   47.661972222222218,
                                   22.31,
                                   54314.,
                                   7.7105591666666657e-05,
                                   5.963831111111111e-05,
                                   71.67]])
        return


class TestApophis(TestEphemerides, unittest.TestCase):
    """
    Unit test for ephem.ephemerides() and asteroid Apophis

    Since there will be some rounding errors in the floats we get, we simply ask
    that numbers agree up to a fraction of a percent (see THRESH above).

    This uses cometary elements.
    """
    def setUp(self):
        self.orbitType = 'COM'
        self.obscode = 568              # MPC observatory code.
        self.orbitElements = numpy.array([.7461531666439291,
                                          .191054853916459,
                                          3.330920734685656,
                                          204.4721548004914,
                                          126.3839706419199,
                                          53600.23498111405])
        self.orbitEpoch = 53442.
        self.absMag = 19.7
        self.covar = numpy.array([7.518251074220359E-15,
                                  -6.504414664700051E-15,
                                  8.215916414938688E-16,
                                  -1.578026884493206E-14,
                                  1.193529524001131E-14,
                                  -4.263653663812154E-14,
                                  5.730327870907439E-15,
                                  -7.498262965084406E-16,
                                  1.566175622641857E-14,
                                  -1.181292654647408E-14,
                                  1.134268411193072E-13,
                                  1.247219145661404E-15,
                                  3.889263373904193E-14,
                                  -3.826636504981605E-14,
                                  1.921993283602571E-14,
                                  3.50195906053127E-12,
                                  -3.466330674715631E-12,
                                  8.828071281990863E-13,
                                  3.444075540643674E-12,
                                  1.441608009844297E-13,
                                  1.020488486250131E-10])
        self.norm = None
        self.times = numpy.array([54314., 53949., 53584.])
        self.truth = numpy.array([[151.28620833333335,
                                   21.500527777777776,
                                   0., 53584.,
                                   8.8201022222222209e-05,
                                   2.6395397222222224e-05, -24.10],
                                  [82.416208333333344,
                                   23.854416666666669,
                                   21.32,
                                   53949.,
                                   3.040366111111111e-05,
                                   1.5596772222222221e-05,
                                   -7.20],
                                  [100.85083333333333,
                                   21.818638888888888,
                                   21.80,
                                   54314.,
                                   5.5275827777777783e-05,
                                   6.468525e-06,
                                   -5.91]])
        return
    



if(__name__ == '__main__'):
    unittest.main()
