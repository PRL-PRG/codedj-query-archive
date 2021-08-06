# This module can be invoked from the command line via 'python test_prep.py'
# to ensure that the supporting framework (including sample index creation) is
# working properly.
#
# When searching for memory leaks or refcount leaks, testing this module will
# reveal whether the leaks are present in setup code.  If not, individual
# modules can be tested in isolation, with the knowledge that the setup code
# common to all modules is clean.

import unittest
import os.path

import test_base


def getFullTestSuite():
    suite = unittest.TestSuite()
    suite.addTest( unittest.makeSuite(DummyTest) )
    return suite


class DummyTest(test_base.CommonBaseTest):
    def testNothing(self):
        self.assert_(os.path.isfile(test_base.CommonBaseTest.getTestArchivePath()))


if __name__ == '__main__':
    import test
    test.main(suite=getFullTestSuite())
