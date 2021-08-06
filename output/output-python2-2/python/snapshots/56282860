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
import zc.async.monitortests

def tearDown(test):
    import zc.async.dispatcher
    zc.async.dispatcher.pop()
    zope.component.testing.tearDown(test)

def test_suite():
    return unittest.TestSuite((
        doctest.DocFileSuite(
            'z3.txt',
            setUp=zc.async.monitortests.setUp,
            tearDown=zc.async.tests.modTearDown,
            optionflags=doctest.INTERPRET_FOOTNOTES),
        doctest.DocFileSuite(
            'README_3a.txt',
            'README_3b.txt',
            setUp=zope.component.testing.setUp,
            tearDown=tearDown,
            optionflags=doctest.INTERPRET_FOOTNOTES),
        ))


if __name__ == '__main__':
    unittest.main(defaultTest='test_suite')
