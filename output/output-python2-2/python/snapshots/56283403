import os
import unittest
from zope.testing import doctest, module
import zope.component.testing
import zc.ngi.async # to quiet the thread complaints from the testing
# infrastructure, because there is no API way to stop the z3monitor server or
# the zc.ngi.async thread. :-(

import zc.async.tests

def setUp(test):
    zc.async.tests.modSetUp(test)
    # make the uuid stable for these tests
    f = open(os.environ["ZC_ASYNC_UUID"], 'w')
    # make this stable for test purposes
    f.writelines(('d10f43dc-ffdf-11dc-abd4-0017f2c49bdd',))
    f.close()
    zc.async.instanceuuid.UUID = zc.async.instanceuuid.getUUID()

def test_suite():
    return unittest.TestSuite((
        doctest.DocFileSuite(
            'monitor.txt',
            setUp=setUp, tearDown=zc.async.tests.modTearDown,
            optionflags=doctest.INTERPRET_FOOTNOTES),
        doctest.DocFileSuite(
            'README_3.txt',
            setUp=zope.component.testing.setUp,
            tearDown=zope.component.testing.tearDown,
            optionflags=doctest.INTERPRET_FOOTNOTES),
        ))


if __name__ == '__main__':
    unittest.main(defaultTest='test_suite')
