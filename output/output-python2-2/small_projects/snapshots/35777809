# The tests of TransactionalRAMDirectory are segregated from the tests of other
# lucene::store members because TransactionalRAMDirectory is exposed solely as
# a target for tests, not for use by ordinary Python client programmers.

import unittest

import test_base
from pyclene import lucene


# Extract a reference to the non-public TransactionalRAMDirectory class:
TransactionalRAMDirectory = lucene._pc.TransactionalRAMDirectory
# It was necessary to use a prefix to circumvent the SWIG directive issued at
# the RAMDirectory level to ignore certain members.  Here, we retrofit
# TransactionalRAMDirectory to offer the members under their proper names.
for memberName in [n for n in dir(TransactionalRAMDirectory) if n.startswith('dummy_')]:
    member = getattr(TransactionalRAMDirectory, memberName)
    setattr(TransactionalRAMDirectory, memberName[len('dummy_'):], member)
    delattr(TransactionalRAMDirectory, memberName)


def getFullTestSuite():
    suite = unittest.TestSuite()

    suite.addTest( unittest.makeSuite(TransactionalRAMDirectoryTest) )

    return suite


class TransactionalRAMDirectoryTest(test_base.CommonBaseTest):
    # Supporting methods:
    def setUp(self):
        test_base.CommonBaseTest.setUp(self)
        self.d = TransactionalRAMDirectory()

    def _rl(self, requiredList):
        # Require that d.list() equal the specified list.
        dList = self.d.list()
        dList.sort()

        requiredListSorted = requiredList[:]
        requiredListSorted.sort()

        self.assertEqual( dList, requiredListSorted )

    def _rc(self, name, contents):
        self.assertEqual( self._rf(name), contents )

    def _cf(self, name='a', contents='sample contents'):
        self.d.createFile(name, contents)
        self.assertEqual(self._rf(name), contents)

    def _rf(self, name='a'):
        return self.d.readFile(name)

    def _df(self, name='a'):
        self.d.deleteFile(name)

    def _shortcuts(self):
        return self.d, self._rl, self._rc, self._cf, self._rf, self._df


    # Test methods:
    def testCommit_withNoChanges(self):
        d, rl, rc, cf, rf, df = self._shortcuts()

        rl([])
        d.transStart()
        rl([])
        d.transCommit()
        rl([])


    def testAbort_withNoChanges(self):
        d, rl, rc, cf, rf, df = self._shortcuts()

        rl([])
        d.transStart()
        rl([])
        d.transAbort()
        rl([])


    def testCommit_withSimpleAddition(self):
        d, rl, rc, cf, rf, df = self._shortcuts()

        d.transStart()
        cf('a')
        rl(['a'])
        d.transCommit()
        rl(['a'])


    def testCommit_withSimpleDeletion(self):
        d, rl, rc, cf, rf, df = self._shortcuts()

        cf('a')
        rl(['a'])
        d.transStart()
        df('a')
        d.transCommit()
        rl([])


    def testAbort_withSimpleAddition(self):
        d, rl, rc, cf, rf, df = self._shortcuts()

        d.transStart()
        cf('a')
        rl(['a'])
        d.transAbort()
        rl([])


    def testAbort_withSimpleDeletion(self):
        d, rl, rc, cf, rf, df = self._shortcuts()

        cf('a')
        rl(['a'])
        d.transStart()
        df('a')
        rl([])
        d.transAbort()
        rl(['a'])


    def testCommit_withOverwrite(self):
        d, rl, rc, cf, rf, df = self._shortcuts()

        cf('a', 'contents1')
        rl(['a'])
        rc('a', 'contents1')
        d.transStart()
        cf('a', 'contents2')
        d.transCommit()
        rc('a', 'contents2')


    def testAbort_withOverwrite(self):
        d, rl, rc, cf, rf, df = self._shortcuts()

        cf('a', 'contents1')
        rl(['a'])
        rc('a', 'contents1')
        d.transStart()
        cf('a', 'contents2')
        d.transAbort()
        rc('a', 'contents1')


    def testAbort_withDeletionThenOverwrite(self):
        d, rl, rc, cf, rf, df = self._shortcuts()

        cf('a', 'contents1')
        rl(['a'])
        rc('a', 'contents1')
        d.transStart()
        df('a')
        cf('a', 'contents2')
        d.transAbort()
        rc('a', 'contents1')


    def testAbort_withOverwriteThenDeletion(self):
        d, rl, rc, cf, rf, df = self._shortcuts()

        cf('a', 'contents1')
        rl(['a'])
        rc('a', 'contents1')
        d.transStart()
        cf('a', 'contents2')
        df('a')
        d.transAbort()
        rc('a', 'contents1')


    def testAbort_withMultipleAdditionsAndDeletions(self):
        _, rl, rc, cf, rf, df = self._shortcuts()

        def createRange(start, stop):
            for i in range(start, stop):
                si = str(i)
                cf(si, 'contents-' + si)

        def reqRange(start, stop):
            rl([str(i) for i in range(start, stop)])
            for i in range(start, stop):
                si = str(i)
                rc(si, 'contents-' + si)


        # Create numerous files, then abort all.
        d = self.d = TransactionalRAMDirectory()
        d.transStart()
        createRange(1, 31)
        reqRange(1, 31)
        d.transAbort()
        rl([])

        # Create numerous files, commit, then create more, delete some of the
        # originals, abort, and verify that the original set is intact.
        d = self.d = TransactionalRAMDirectory()
        d.transStart()
        createRange(1, 31)
        reqRange(1, 31)
        d.transCommit()
        reqRange(1, 31)

        d.transStart()
        createRange(31, 2001)
        reqRange(1, 2001)
        df('10'); df('1'); df('30');
        d.transAbort()
        reqRange(1, 31)


    def testRedundantTransStart(self):
        d, rl, rc, cf, rf, df = self._shortcuts()

        d.transStart()
        self.assertRaises(Exception, d.transStart)


if __name__ == '__main__':
    import test
    test.main(suite=getFullTestSuite(), createTestIndex=False)