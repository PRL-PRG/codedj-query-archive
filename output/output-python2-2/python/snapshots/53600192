#!/usr/bin/env python
"""
Tests for bad pixel interpolation code

Run with:
   python Interp.py
or
   python
   >>> import Interp; Interp.run()
"""

import pdb                              # we may want to say pdb.set_trace()
import os
from math import *
import unittest
import eups
import lsst.utils.tests as tests
import lsst.pex.logging as logging
import lsst.afw.image as imageLib
import lsst.afw.display.ds9 as ds9
import lsst.detection.detectionLib as detection
import lsst.detection.defects as defects

try:
    type(verbose)
except NameError:
    verbose = 0
    logging.Trace_setVerbosity("detection.Interp", verbose)

try:
    type(display)
except NameError:
    display = False

    if display:
        import lsst.afw.display.ds9 as ds9

class interpolationTestCase(unittest.TestCase):
    """A test case for interpolation"""
    def setUp(self):
        self.mi = imageLib.MaskedImageD()
        self.FWHM = 5
        self.psf = detection.dgPSF(self.FWHM/(2*sqrt(2*log(2))))
        if eups.productDir("afwdata"):
            maskedImage = os.path.join(eups.productDir("afwdata"), "CFHT", "D4", "cal-53535-i-797722_1")
        else:
            maskedImage = "/u/rhl/LSST/imageproc-277/diffImage"
            
        self.mi.readFits(maskedImage)
        self.mi.getMask().addMaskPlane("INTERP")

        self.badPixels = defects.policyToBadRegionList(os.path.join(os.environ["DETECTION_DIR"],
                                                                    "pipeline/BadPixels.paf"))

    def tearDown(self):
        del self.mi
        del self.psf
        del self.badPixels

    def testDetection(self):
        """Test Interp detection"""

        if display:
            frame = 0
            ds9.mtv(self.mi, frame=frame)

        detection.interpolateOverDefects(self.mi, self.psf, self.badPixels)

        if display:
            ds9.mtv(self.mi, frame=frame+1)

#-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-

def suite():
    """Returns a suite containing all the test cases in this module."""
    tests.init()

    suites = []
    suites += unittest.makeSuite(interpolationTestCase)
    #suites += unittest.makeSuite(tests.MemoryTestCase)
    return unittest.TestSuite(suites)


def run(exit=False):
    """Run the tests"""
    tests.run(suite(), exit)

if __name__ == "__main__":
    run(True)
