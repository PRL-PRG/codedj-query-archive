"""
Tests for Footprints, DetectionSets, and Measure

Run with:
   python Measure_1.py
or
   python
   >>> import unittest; T=load("Measure_1"); unittest.TextTestRunner(verbosity=1).run(T.suite())
"""

import pdb                              # we may want to say pdb.set_trace()
import unittest
import lsst.mwi.tests as tests
import lsst.mwi.utils as mwiu
import lsst.fw.Core.fwLib as fw
import lsst.fw.Core.fwCatalog as fwCat
import lsst.detection.detectionLib as detection

try:
    type(verbose)
except NameError:
    verbose = 0
    mwiu.Trace_setVerbosity("detection.Measure", verbose)

try:
    type(display)
except NameError:
    display = False

def toString(*args):
    """toString written in python"""
    if len(args) == 1:
        args = args[0]

    y, x0, x1 = args
    return "%d: %d..%d" % (y, x0, x1)

#-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-

class MeasureTestCase(unittest.TestCase):
    """A test case for Measure"""
    class Object(object):
        def __init__(self, val, spans):
            self.val = val
            self.spans = spans

        def insert(self, im):
            """Insert self into an image"""
            for sp in self.spans:
                y, x0, x1 = sp
                for x in range(x0, x1+1):
                    im.set(x, y, self.val)

        def __eq__(self, other):
            for osp, sp in zip(other.getSpans(), self.spans):
                if osp.toString() != toString(sp):
                    return False
                
            return True
    
    def setUp(self):
        self.ms = fw.MaskedImageD(12, 8)
        im = self.ms.getImage()
        #
        # Objects that we should detect
        #
        self.objects = []
        self.objects += [self.Object(10, [(1, 4, 4), (2, 3, 5), (3, 4, 4)])]
        self.objects += [self.Object(20, [(5, 7, 8), (5, 10, 10), (6, 8, 9)])]
        self.objects += [self.Object(20, [(6, 3, 3)])]

        im.set(0)                       # clear image
        for obj in self.objects:
            obj.insert(im)

        if display:
            import lsst.fw.Display.ds9 as ds9
            ds9.mtv(im, frame=0)
        
    def tearDown(self):
        del self.ms

    def testFootprintsMeasure(self):
        """Check that we can measure the objects in a detectionSet"""

        xcentroid = [4.0, 8.4, 3.0]
        ycentroid = [2.0, 5.4, 6.0]
        flux = [50.0, 100.0, 20.0]
        
        ds = detection.DetectionSetD(self.ms, detection.Threshold(10), "FP")
        objects = ds.getFootprints()
        measure = detection.MeasureD(self.ms)
        diaptr = fwCat.DiaSourcePtr()

        for i in range(len(objects)):
            diaptr.setId(i)
            measure.measureSource(diaptr, objects[i], 0.0)
            assert(abs(diaptr.getColc()-xcentroid[i]) < 1.0e-5)
            assert(abs(diaptr.getRowc()-ycentroid[i]) < 1.0e-5)
            assert(abs(diaptr.getFlux()-flux[i]) < 1.0e-5)


#-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-

def suite():
    """Returns a suite containing all the test cases in this module."""
    tests.init()

    suites = []
    suites += unittest.makeSuite(MeasureTestCase)
    suites += unittest.makeSuite(tests.MemoryTestCase)
    return unittest.TestSuite(suites)

if __name__ == "__main__":
    tests.run(suite())
