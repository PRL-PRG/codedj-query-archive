##############################################################################
#
# Copyright (c) 2006-2008 Zope Corporation and Contributors.
# All Rights Reserved.
#
# This software is subject to the provisions of the Zope Public License,
# Version 2.1 (ZPL).  A copy of the ZPL should accompany this distribution.
# THIS SOFTWARE IS PROVIDED "AS IS" AND ANY AND ALL EXPRESS OR IMPLIED
# WARRANTIES ARE DISCLAIMED, INCLUDING, BUT NOT LIMITED TO, THE IMPLIED
# WARRANTIES OF TITLE, MERCHANTABILITY, AGAINST INFRINGEMENT, AND FITNESS
# FOR A PARTICULAR PURPOSE.
#
##############################################################################
import os
import unittest
from zope.testing import doctest, module
import zope.component.testing
import zc.ngi.async # to quiet the thread complaints from the testing
# infrastructure, because there is no API way as of this writing to stop the
# z3monitor server or the zc.ngi.async thread. :-(

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
            'monitordb.txt',
            setUp=setUp, tearDown=zc.async.tests.modTearDown,
            optionflags=doctest.INTERPRET_FOOTNOTES),
        ))


if __name__ == '__main__':
    unittest.main(defaultTest='test_suite')
