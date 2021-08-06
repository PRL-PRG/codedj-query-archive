#!/usr/bin/env python
"""
Tests for cosmic ray detection

Run with:
   python CR.py
or
   python
   >>> import CR; CR.run()
"""

import pdb                              # we may want to say pdb.set_trace()
import os
from math import *
import unittest
import eups
import lsst.utils.tests as tests
import lsst.pex.logging as logging
import lsst.pex.policy as policy
import lsst.afw.image.imageLib as imageLib
import lsst.afw.display.ds9 as ds9
import lsst.detection.detectionLib as detection
import lsst.detection.defects as defects

try:
    type(verbose)
except NameError:
    verbose = 0
    logging.Trace_setVerbosity("detection.CR", verbose)

try:
    type(display)
except NameError:
    display = False

    if display:
        import lsst.afw.display.ds9 as ds9

#-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-

class CosmicRayTestCase(unittest.TestCase):
    """A test case for Cosmic Ray detection"""
    def setUp(self):
        self.mi = imageLib.MaskedImageD()
        self.FWHM = 5                   # pixels
        self.psf = detection.dgPSF(self.FWHM/(2*sqrt(2*log(2))))
        if eups.productDir("afwdata"):
            maskedImage = os.path.join(eups.productDir("afwdata"), "CFHT", "D4", "cal-53535-i-797722_1")
        else:
            maskedImage = "/u/rhl/LSST/imageproc-277/diffImage"
            
        self.mi.readFits(maskedImage)
        self.mi.getMask().addMaskPlane("DETECTED")

        # I'd use eups.productDir("detection", "setup"), except that there's a bug in eups.py with listing
        # -r setups; fixed in v0_7_47
        self.policy = policy.Policy.createPolicy(os.path.join(os.environ["DETECTION_DIR"], "pipeline/CosmicRays.paf"))

    def tearDown(self):
        del self.mi
        del self.psf
        del self.policy

    def testDetection(self):
        """Test CR detection"""

        if display:
            frame = 0
            ds9.mtv(self.mi, frame=frame) # raw frame
            ds9.pan(260, 944)
        #
        # Mask known bad pixels
        #
        badPixels = defects.policyToBadRegionList(os.path.join(os.environ["DETECTION_DIR"], "pipeline/BadPixels.paf"))
        detection.interpolateOverDefects(self.mi, self.psf, badPixels)

        background = imageLib.mean_channel_value(self.mi.getImage())
        crs = detection.findCosmicRays(self.mi, self.psf, background, self.policy)

        if display:
            ds9.mtv(self.mi.getImage(), frame=frame+1)
            ds9.pan(260, 944)

            ds9.mtv(self.mi, frame=frame+2)
            ds9.pan(260, 944)

        self.assertEqual(len(crs), 1094)
        print "Detected %d CRs" % len(crs)

#-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-

def suite():
    """Returns a suite containing all the test cases in this module."""
    tests.init()

    suites = []
    suites += unittest.makeSuite(CosmicRayTestCase)
    suites += unittest.makeSuite(tests.MemoryTestCase)
    return unittest.TestSuite(suites)


def run(exit=False):
    """Run the tests"""
    tests.run(suite(), exit)

if __name__ == "__main__":
    run(True)
