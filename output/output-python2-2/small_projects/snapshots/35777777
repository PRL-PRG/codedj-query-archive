import unittest
import datetime, itertools, os, os.path, sys, tarfile

import test_base
from pyclene import lucene


def getFullTestSuite():
    suite = unittest.TestSuite()

    suite.addTest( unittest.makeSuite(TermTest) )

    suite.addTest( unittest.makeSuite(TermDocsTest) )
    suite.addTest( unittest.makeSuite(TermPositionsTest) )
    suite.addTest( unittest.makeSuite(TermEnumTest) )

    suite.addTest( unittest.makeSuite(IndexWriterTest) )
    suite.addTest( unittest.makeSuite(IndexReaderTest) )

    suite.addTest( unittest.makeSuite(CustomAnalyzerAndTokenStreamTest) )

    return suite


class TermTest(test_base.CommonBaseTest):
    # pyclene represents lucene::index::Term instances as tuples because client
    # code that traffics in Terms typically does so in high volumes, and tests
    # indicated that tuple-Terms performed about 5x as fast as regular SWIG-
    # wrapped Term objects.

    # Since it was first written as a client of SWIG-wrapped C++ Term objects
    # and later transitioned to tuples, this test class has become superfluous,
    # in that it currently tests Python tuples and nothing more.
    # I've decided to leave TermTest here in the event that pyclene's wrapping
    # of Terms becomes more complex in the future and related tests need a
    # home.

    def testConstructor(self, field='termField', text='termText'):
        t = (field, text)
        return t


    def test_field(self):
        t = self.testConstructor()
        self.assertEqual(t[0], 'termField')


    def test_text(self):
        t = self.testConstructor()
        self.assertEqual(t[1], 'termText')


    def test___eq__(self):
        t1 = self.testConstructor()
        t2 = self.testConstructor()
        self.assertEqual(t1, t2)


    def test___cmp__(self):
        t1 = self.testConstructor(field='A', text='X')
        t2 = self.testConstructor(field='B', text='Y')
        self.assert_(t1 < t2)
        self.assert_(t1 != t2)
        self.assert_(t2 > t1)

        t1 = self.testConstructor(field='A', text='X')
        t2 = self.testConstructor(field='A', text='Y')
        self.assert_(t1 < t2)
        self.assert_(t1 != t2)
        self.assert_(t2 > t1)

        t1 = self.testConstructor(field='A', text='Y')
        t2 = self.testConstructor(field='A', text='Y')
        self.assert_(not t1 < t2)
        self.assert_(not t1 != t2)
        self.assert_(not t2 > t1)


    def test___str__(self):
        t = self.testConstructor()
        str(t)


class TermDocsTest(test_base.CommonBaseTest):
    # TermDocs is exercised in IndexReaderTest.test_termDocs.
    pass


class TermPositionsTest(test_base.CommonBaseTest):
    # TermPositions is exercised in IndexReaderTest.test_termPositions.
    pass


class TermEnumTest(test_base.CommonBaseTest):
    # TermEnum is exercised in IndexReaderTest.test_terms.
    pass


class IndexWriterTest(test_base.CommonBaseTest):
    def testConstructor_String(self):
        tempIndexDir = self.getTempFilename()
        a = lucene.StandardAnalyzer()
        self.assert_(not os.path.exists(tempIndexDir))
        w = lucene.IndexWriter(tempIndexDir, a, True)
        self.assert_(os.path.isdir(tempIndexDir))
        return w


    def testConstructor_Directory(self, inRAM=False):
        if inRAM:
            d = lucene.RAMDirectory()
        else:
            tempIndexDir = self.getTempFilename()
            self.assert_(not os.path.exists(tempIndexDir))
            d = lucene.FSDirectory(tempIndexDir, True)
            self.assert_(os.path.isdir(tempIndexDir))
        a = lucene.StandardAnalyzer()
        w = lucene.IndexWriter(d, a, True)
        return w, d


    def testConstructor_overwriteExistingIndex(self):
        indLoc = self.extractTestIndex()
        self.assert_(os.path.isdir(indLoc))

        r = lucene.IndexReader.open(indLoc)
        numDocsOrig = r.numDocs()
        r.close()

        # Should replace any index already present in the directory rather than
        # raising an exception if the directory already exists:
        w = lucene.IndexWriter(indLoc, lucene.StandardAnalyzer(), True)
        w.close()

        r = lucene.IndexReader.open(indLoc)
        numDocsNow = r.numDocs()
        r.close()

        # Previously there were numDocsOrig docs in the index; now there are 0.
        self.assertEqual(numDocsNow, 0)
        self.assert_(numDocsOrig > numDocsNow)


    def test_close(self):
        w = self.testConstructor_String()
        w.close()
        self.assertRaises(IOError, w.close)
        self.assertRaises(IOError, w.docCount)
        self.assertRaises(IOError, w.addDocument, None)
        self.assertRaises(IOError, w.optimize)
        self.assertRaises(IOError, w.addIndexes, None)


    def _test_addDocument_AND_docCount_AND_optimize(self, useRamDir):
        w, d = self.testConstructor_Directory(inRAM=useRamDir)
        w.maxFieldLength = sys.maxint
        self.assertEqual(w.docCount(), 0)

        allFilenames = []
        for root, subdirs, filenames in os.walk(os.path.abspath(os.pardir)):
            filenames = [os.path.join(root, fn) for fn in filenames
                if test_base.testIndexFilenameFilter(fn)
              ]
            allFilenames.extend(filenames)

        for filename in allFilenames:
            doc = FileDocument(filename)
            w.addDocument(doc)

        w.optimize()

        self.assertEqual(w.docCount(), len(allFilenames))

        w.close()
        return d

    def test_addDocument_AND_docCount_AND_optimize__RAMDir(self):
        return self._test_addDocument_AND_docCount_AND_optimize(True)
    def test_addDocument_AND_docCount_AND_optimize__FSDir(self):
        return self._test_addDocument_AND_docCount_AND_optimize(False)


    def test_addIndexes(self):
        # There's some redundancy in this test case because CLucene's merging
        # was quite buggy (invalid mem ops) when I first began to wrap the
        # addIndexes method, and I wanted the associated test to stress that
        # area of the C++ code.

        # First, merge 1 RAMDirectory-based index into an FSDirectory-based
        # index.
        fsDir = self.test_addDocument_AND_docCount_AND_optimize__FSDir()
        ramDir = self.test_addDocument_AND_docCount_AND_optimize__RAMDir()

        w = lucene.IndexWriter(fsDir, lucene.StandardAnalyzer(), False)

        # Make sure trying to add None raises an exception rather than crashing.
        self.assertRaises(TypeError, w.addIndexes, [None])

        oldCount = w.docCount()
        w.addIndexes([ramDir])
        newCount = w.docCount()
        w.close()

        w = lucene.IndexWriter(ramDir, lucene.StandardAnalyzer(), False)
        ramCount = w.docCount()
        w.close()

        self.assertEqual( newCount, oldCount + ramCount )
        fsCount = newCount

        # Next, merge 1 FSDirectory-based index into an RAMDirectory-based
        # index.
        w = lucene.IndexWriter(ramDir, lucene.StandardAnalyzer(), False)
        oldCount = w.docCount()
        w.addIndexes([fsDir])
        newCount = w.docCount()
        w.close()
        self.assertEqual( newCount, oldCount + fsCount )
        ram1Count = newCount

        # Next, merge an FS AND a RAM into a RAM.
        ramDir2 = lucene.RAMDirectory()
        w = lucene.IndexWriter(ramDir2, lucene.StandardAnalyzer(), True)
        w.addIndexes([fsDir, ramDir])
        newCount = w.docCount()
        w.close()
        self.assertEqual( newCount, fsCount + ram1Count )
        ram2Count = newCount

        # Next, merge a RAM AND an FS AND a RAM into an FS.
        w = lucene.IndexWriter(self.getTempFilename(), lucene.StandardAnalyzer(), True)
        w.addIndexes([ramDir, fsDir, ramDir2])
        newCount = w.docCount()
        w.close()
        self.assertEqual( newCount, fsCount + ram1Count + ram2Count )


    def _test_maxFieldLength_base(self, # Added 2005.02.01
        maxFieldLength=None, expectBaitToken=False
      ):
        inputFN = os.path.join(os.pardir, '_clucene_wrap.cpp')

        tempIndexDir = self.getTempFilename()
        w = lucene.IndexWriter(tempIndexDir, lucene.StandardAnalyzer(), True)
        if maxFieldLength is not None:
            w.maxFieldLength = maxFieldLength

        doc = FileDocument(inputFN)
        w.addDocument(doc)

        w.close()

        r = lucene.IndexReader.open(tempIndexDir)
        # The test index in this case contains only one document
        # (_clucene_wrap.cpp), and we know that the bait token appears in it
        # exactly twice, right near the end.
        td = r.termDocs(
            ('contents',
             'hi__i_am_a_bait_token_deliberately_inserted_at_end_of_huge_file'
             '__so_leave_me_here'
            )
          )

        baitTokenAppearedInNDocs = 0
        baitTokenDocNo = -1
        baitTokenFreq = -1
        for docNo, freq in td:
            baitTokenDocNo = docNo
            baitTokenFreq = freq
            baitTokenAppearedInNDocs += 1

        if expectBaitToken:
            self.assertEqual(baitTokenAppearedInNDocs, 1)
            self.assertEqual(baitTokenDocNo, 0) # The first document in the index.
            self.assertEqual(baitTokenFreq, 2) # Appears twice.
        else:
            self.assertEqual(baitTokenAppearedInNDocs, 0)
            self.assertEqual(baitTokenDocNo, -1)
            self.assertEqual(baitTokenFreq, -1)

        r.close()

    def test_maxFieldLength_requireComplaintForHugeFileIfNoExplicitAction(self):
        self.assertRaises(Exception, self._test_maxFieldLength_base)

    def test_maxFieldLength_requireNoComplaintAndNoTruncationForHugeFileIfHighLimit(self):
        self._test_maxFieldLength_base(maxFieldLength=sys.maxint, expectBaitToken=True)

    def test_maxFieldLength_requireNoComplaintAndTruncationForHugeFileIfLowLimit(self):
        self._test_maxFieldLength_base(maxFieldLength=100, expectBaitToken=False)

    def test_maxFieldLength_restartabilityAfterTruncationException(self):
        # This test case first indexes a small file, then tries unsuccessfully
        # to index a huge one, catches the resulting exception, adjusts the
        # IndexWriter so that it will be able to index the huge file, indexes
        # the huge file, and finally, indexes a second small file to ensure
        # that the exception didn't cause the index to become corrupt.
        hugeFN = os.path.join(os.pardir, '_clucene_wrap.cpp')
        baitFN1 = os.path.join('data', 'bait_token_1.txt')
        baitFN2 = os.path.join('data', 'bait_token_2.txt')

        tempIndexDir = self.getTempFilename()
        w = lucene.IndexWriter(tempIndexDir, lucene.StandardAnalyzer(), True)

        w.addDocument(FileDocument(baitFN1))

        self.assertRaises(Exception, w.addDocument, FileDocument(hugeFN))
        w.maxFieldLength = sys.maxint
        w.addDocument(FileDocument(hugeFN))

        w.addDocument(FileDocument(baitFN2))

        w.close()

        # Finally, verify that all "bait tokens" are found where they should be.
        r = lucene.IndexReader.open(tempIndexDir)

        for (baitToken, reqDocNo, reqFreq) in (
            ('t_830ee61' 'f7e73014e5e61df046676ec1b', 0, 1),
            ('hi__i_am_a_bait_token_deliberately_inserted_at_end_of_huge_file'
                '__so_leave_me_here',                 1, 2
            ),
            ('t_baf89766' '83c8390923e959e89d1917f3', 2, 1),
          ):
            td = r.termDocs(('contents', baitToken))
            self.assertEqual( [(reqDocNo, reqFreq)],
                [(docNo, freq) for (docNo, freq) in td]
              )


class IndexReaderTest(test_base.CommonBaseTest):
    def setUp(self):
        test_base.CommonBaseTest.setUp(self)
        if test_base.CommonBaseTest.getTestArchivePath() is None:
            raise IOError('No sample index available (should have been created'
                ' by test.createTestIndexArchive).'
              )


    def testConstructor_String(self):
        indLoc = self.extractTestIndex()
        r = lucene.IndexReader.open(indLoc)
        return r

    def testConstructor_Directory(self):
        indLoc = self.extractTestIndex()
        d = lucene.FSDirectory(indLoc, False)
        r = lucene.IndexReader.open(d)
        return r

    def testConstructor_NonexistentDirectory(self):
        # Point an IndexReader to a nonexistent directory and make sure it
        # behaves reasonably (i.e., raises an IOError).
        # We need not test the Directory-based variant of IndexReader.open in
        # this case because FSDirectory's own constructor validates the
        # existence of its input.
        self.assertRaises(IOError,
            lucene.IndexReader.open, os.path.join('gott', 'in', 'himmel')
          )

    def testConstructor_EmptyDirectory(self):
        # Point an IndexReader to an empty directory and make sure it behaves
        # reasonably (i.e., raises an IOError).
        tempDir = self.getTempFilename()
        os.makedirs(tempDir)
        string = (lucene.UNICODE_BUILD and unicode) or str
        for dirArgType in (string, lucene.FSDirectory):
            self.assertRaises(IOError,
                lucene.IndexReader.open, dirArgType(tempDir)
              )


    def test_close(self):
        r = self.testConstructor_String()
        r.close()
        self.assertRaises(IOError, r.close)


    def test_dirStillOpenAfterIndexReaderCloses(self):
        ramDir = self.extractTestIndexToRAMDirectory()
        r = lucene.IndexReader.open(ramDir)
        r.close()
        del r
        self.assert_( len(ramDir.list()) > 0 )


    def test_lastModified(self):
        # To allow for non-filesystem-based Directory implementations,
        # CLucene's {IndexReader.lastModified(Directory&)} rendition actually
        # checks the modtime of the 'segments' file within the Directory, not
        # the stat-based modtime of the directory itself.
        #
        # The {IndexReader.lastModified(char_t*)} rendition stats the directory
        # itself.
        #
        # The two modtimes therefore cannot be expected to be equal, but we can
        # verify them individually against the equivalent Python stdlib
        # facility.

        indLoc = self.extractTestIndex()

        # Directory-based variant:
        lastMod_Dir_CL = lucene.IndexReader.lastModified(lucene.FSDirectory(indLoc))
        lastMod_Dir_Py = datetime.datetime.fromtimestamp(
            os.path.getmtime(os.path.join(indLoc, 'segments'))
          )
        self.assertEqual(lastMod_Dir_CL, lastMod_Dir_Py)

        # String-based variant:
        lastMod_String_CL = lucene.IndexReader.lastModified(indLoc)
        lastMod_String_Py = datetime.datetime.fromtimestamp(
            os.path.getmtime(indLoc)
          )
        self.assertEqual(lastMod_String_CL, lastMod_String_Py)


    def test_indexExists(self):
        indLoc = self.extractTestIndex()

        emptyDir = self.getTempFilename()
        os.makedirs(emptyDir)

        # String-based variant:
        self.assert_( lucene.IndexReader.indexExists(indLoc) )
        self.assert_( not lucene.IndexReader.indexExists(emptyDir) )
        self.assert_( not lucene.IndexReader.indexExists('nonexistent-dir') )

        # Directory-based variant:
        self.assert_( lucene.IndexReader.indexExists(lucene.FSDirectory(indLoc)) )
        self.assert_( not lucene.IndexReader.indexExists(lucene.FSDirectory(emptyDir)) )
        # A nonexistent-dir case is not necessary for the Directory-based
        # variant because the FSDirectory constructor disallows the creation
        # of an FSDirectory object pointing to a nonexistent directory.
        # We do, however, test against an empty RAMDirectory.
        self.assert_( not lucene.IndexReader.indexExists(lucene.RAMDirectory()) )


    def test_isLocked_AND_unlock(self):
        indLoc = self.extractTestIndex()

        # Exist, contain index, not locked:
        self.assert_(not lucene.IndexReader.isLocked(indLoc))
        fsDir = lucene.FSDirectory(indLoc)
        self.assert_(not lucene.IndexReader.isLocked(fsDir))
        lucene.IndexReader.unlock(fsDir) # Shouldn't complain.
        # Does not exist:
        self.assertRaises(IOError, lucene.IndexReader.isLocked, 'nonexistent-dir')
        # Exist, but do not contain index:
        emptyDir = self.getTempFilename()
        os.makedirs(emptyDir)
        fsDir = lucene.FSDirectory(emptyDir)
        self.assertRaises(IOError, lucene.IndexReader.isLocked, fsDir)
        self.assertRaises(IOError, lucene.IndexReader.unlock, fsDir)
        self.assertRaises(IOError, lucene.IndexReader.isLocked, lucene.RAMDirectory())
        self.assertRaises(IOError, lucene.IndexReader.unlock, lucene.RAMDirectory())

        # Now lock the directory and make sure isLocked detects that state.
        w = lucene.IndexWriter(indLoc, lucene.StandardAnalyzer(), False)
        self.assert_(lucene.IndexReader.isLocked(indLoc))
        w.close()
        self.assert_(not lucene.IndexReader.isLocked(indLoc))


    def test_numDocs_AND_maxDoc(self):
        indLoc = self.extractTestIndex()
        r = lucene.IndexReader.open(indLoc)

        totalDocsInFileSystem = len(test_base.listFilesDestinedForTestIndex())
        self.assertEqual(r.numDocs(), totalDocsInFileSystem)

        # maxDoc is one greater than the greatest document identifier in the
        # index.  Since this index is perfectly optimized (no gaps in doc ids),
        # maxDoc will be equal to numDocs (maxDoc is 0-based; numDocs is
        # (obviously) 1-based).
        self.assertEqual(r.maxDoc(), r.numDocs())

        r.close()
        self.assertRaises(IOError, r.numDocs)
        self.assertRaises(IOError, r.maxDoc)


    def test_document(self):
        r = self.testConstructor_String()
        for pos in (0, 1, 2, r.maxDoc()-2, r.maxDoc()-1):
            self.assert_(isinstance(r.document(pos), lucene.Document))
        for pos in (-1, r.maxDoc(), r.maxDoc()+1, sys.maxint):
            self.assertRaises(IOError, r.document, pos)

        r.close()
        self.assertRaises(IOError, r.document, 0)


    def test_isDeleted(self):
        r = self.testConstructor_String()
        for pos in (0, 1, 2, r.maxDoc()-2, r.maxDoc()-1):
            self.assert_(not r.isDeleted(pos))
        for pos in (-1, r.maxDoc(), r.maxDoc()+1, sys.maxint):
            self.assertRaises(IOError, r.isDeleted, pos)

        r.close()
        self.assertRaises(IOError, r.isDeleted, 0)

    # getNorms method not wrapped.

    def test_terms(self):
        r = self.testConstructor_String()

        # All:
        allTerms = r.terms()
        self.assert_(isinstance(allTerms, lucene.TermEnum))
        for iAll, ((field, text), freq) in enumerate(allTerms):
            pass
        self.assert_(iAll > 0)
        self.assert_(not allTerms.next())
        self.assertEqual(allTerms.term(), None)

        # Specific:
        termsAfterX = r.terms(after=('contents', 'lucene'))
        for iAfter, ((field, text), freq) in enumerate(termsAfterX):
            pass
        self.assert_(iAfter > 0)
        self.assert_(iAfter < iAll)

        r.close()
        self.assertRaises(IOError, r.terms)


    def test_docFreq(self):
        # thistargettermissetoffsoitwontbeconcatenatedwithothersasitisinthecodebelow
        r = self.testConstructor_String()
        self.assert_(r.docFreq(('contents', 'lucene')) > 0)
        self.assertEqual(1, r.docFreq(('contents',
            'thistargettermissetoffsoitwontbeconcatenatedwithothersasitisinthecodebelow'
          )))
        self.assertEqual(0, r.docFreq(('path',
            'thistargettermissetoffsoitwontbeconcatenatedwithothersasitisinthecodebelow'
          )))

        r.close()
        self.assertRaises(IOError, r.docFreq, ('contents', 'lucene'))


    def test_termDocs(self):
        r = self.testConstructor_String()

        # Unpositioned at the outset:
        td = r.termDocs()
        self.assert_(not td.next())
        td.seek(('contents', 'lucene'))
        nFromSeek = 0
        for docNo, freq in td:
            self.assert_(freq > 0)
            nFromSeek += 1
        self.assert_(nFromSeek > 0)
        self.assert_(not td.next())

        # Positioned at the outset:
        td = r.termDocs(containing=('contents', 'lucene'))
        nFromContaining = 0
        docNoOfFirst, termFreqOfFirst = 0, 0
        for docNo, freq in td:
            self.assert_(freq > 0)
            if nFromContaining == 0:
                docNoOfFirst, termFreqOfFirst = docNo, freq
            nFromContaining += 1
        self.assertEqual(nFromContaining, nFromSeek)
        self.assert_(not td.next())

        # Use the non-Pythonic-iterator accessor methods:
        td = r.termDocs(containing=('contents', 'lucene'))
        self.assert_(td.next())
        self.assertEqual(td.doc(), docNoOfFirst)
        self.assertEqual(td.freq(), termFreqOfFirst)
        td.skipTo(docNoOfFirst+1)
        self.assert_(td.next())
        self.assert_(td.doc() > docNoOfFirst)

        td.close()
        self.assertRaises(IOError, td.close)
        self.assertRaises(IOError, td.doc)
        self.assertRaises(IOError, td.freq)
        self.assertRaises(IOError, td.next)
        self.assertRaises(IOError, td.seek, ('contents', 'lucene'))
        self.assertRaises(IOError, td.skipTo, 0)

        r.close()
        self.assertRaises(IOError, r.termDocs)


    def test_termPositions(self):
        # TermPositions is a subclass of TermDocs, so most of its methods
        # are already tested by test_termDocs.

        r = self.testConstructor_String()
        tp = r.termPositions()
        self.assert_(not tp.next())
        tp.close()
        self.assertRaises(IOError, tp.next)

        # Search for a decoy term (it's broken up below to prevent the engine
        # from finding it in this document also).
        prefix = 'klinkamagigaliciousdecoy' # Broken up, eh?
        suffix = 'pretzelexedrinecoyotedoorsofmuthaphukkinperception'
        tp = r.termPositions(containing=('contents', prefix + suffix))
        self.assert_(tp.next())
        self.assert_(r.document(tp.doc())['path'].endswith('term_decoy_single.txt'))
        self.assertEqual(tp.nextPosition(), 2) # It's the third term in the doc.

        # It only occurs once in that doc, so calling termPositions a second
        # time is an error, and calling tp.next a second time should return
        # False.
        self.assertEqual(tp.freq(), 1)
        self.assertRaises(IOError, tp.nextPosition)
        self.assert_(not tp.next())


    def test_delete(self):
        r = self.testConstructor_String()
        r.delete(0)
        self.assertRaises(IOError, r.delete, 0)
        self.assertRaises(IOError, r.delete, -1)
        self.assertRaises(IOError, r.delete, sys.maxint)
        r.close()
        self.assertRaises(IOError, r.delete, 1) # Valid number, but r is closed.


class CustomAnalyzerAndTokenStreamTest(test_base.CommonBaseTest):
    ### SUPPORT ###
    def _createIndexWithCustomAnalyzer(self, analyzer):
        tempIndexDir = self.getTempFilename()
        w = lucene.IndexWriter(tempIndexDir, analyzer, True)
        try:
            doc = lucene.Document()
            contentsField = lucene.Field.Text('contents', 'submitted tokens here')
            doc.add(contentsField)
            w.addDocument(doc)
        finally:
            w.close()

        r = lucene.IndexReader.open(tempIndexDir)
        return r


    def _tokenStreamReturnType_verifyOutput(self, r):
        # This verifies that, no matter what specific type the Python program
        # presented the TokenStream to the C++ layer as, the results were the
        # same.

        # AAA appeared twice; BBB once:
        self.assertEqual(
            [(0, 2)],
            [t for t in r.termPositions(('contents', 'AAA'))]
          )
        self.assertEqual(
            [(0, 1)],
            [t for t in r.termPositions(('contents', 'BBB'))]
          )

        # Each term appeared in only one document:
        terms = [t for t in r.terms()]
        terms.sort()
        assert terms == [ (('contents', 'AAA'), 1), (('contents', 'BBB'), 1) ]


    def _tokenStreamReturnType_baseTokens(self):
        return [
            lucene.Token('AAA', 0, 2), lucene.Token('BBB', 3, 5),
            lucene.Token('AAA', 6, 8)
          ]


    def _testBase_tokenStreamReturnType(self, customAnalyzerClass):
        a = customAnalyzerClass()
        r = self._createIndexWithCustomAnalyzer(a)
        try:
            self._tokenStreamReturnType_verifyOutput(r)
        finally:
            r.close()


    ### ACTUAL TEST CASES ###
    def test_tokenStreamReturnType_list(tSelf):
        # Verify that pyclene can handle a custom Analyzer that returns a
        # *list* from its tokenStream method.
        class CustomAnalyzer(lucene.Analyzer):
            def tokenStream(self, fieldName, reader):
                return list(tSelf._tokenStreamReturnType_baseTokens())

        tSelf._testBase_tokenStreamReturnType(CustomAnalyzer)


    def test_tokenStreamReturnType_tuple(tSelf):
        # Verify that pyclene can handle a custom Analyzer that returns a
        # *tuple* from its tokenStream method.
        class CustomAnalyzer(lucene.Analyzer):
            def tokenStream(self, fieldName, reader):
                return tuple(tSelf._tokenStreamReturnType_baseTokens())

        tSelf._testBase_tokenStreamReturnType(CustomAnalyzer)


    def test_tokenStreamReturnType_iter(tSelf):
        # Verify that pyclene can handle a custom Analyzer that returns an
        # *iter* from its tokenStream method (a Python iterator is iterable by
        # definition; it is its own iterator).
        class CustomAnalyzer(lucene.Analyzer):
            def tokenStream(self, fieldName, reader):
                return iter(tSelf._tokenStreamReturnType_baseTokens())

        tSelf._testBase_tokenStreamReturnType(CustomAnalyzer)


    def test_tokenStreamReturnType_customIterable(tSelf):
        # Verify that pyclene can handle a custom Analyzer that returns a
        # *user-defined class that adheres to the iterator protocol* from its
        # tokenStream method.
        class CustomAnalyzer(lucene.Analyzer):
            def tokenStream(self, fieldName, reader):
                tokens = tSelf._tokenStreamReturnType_baseTokens()

                class CustomIterable(object):
                    def __init__(self, tokens):
                        self.i = -1
                        self.tokens = tokens
                    def __iter__(self):
                        return self
                    def next(self):
                        self.i += 1
                        if self.i > len(self.tokens) - 1:
                            raise StopIteration()
                        return self.tokens[self.i]

                return CustomIterable(tokens)

        tSelf._testBase_tokenStreamReturnType(CustomAnalyzer)


    def test_tokenStreamReturnType_containingValuesOfWrongType(self):
        # Make sure the Python wrapper complains before passing the non-Tokens
        # in this TokenStream through to the C++ layer, where they'd cause a
        # crash.
        class CustomAnalyzer(lucene.Analyzer):
            def tokenStream(self, fieldName, reader):
                class Anything(object):
                    pass
                return [lucene.Token('AAA', 0, 2), Anything()]
        self.assertRaises(TypeError,
            self._testBase_tokenStreamReturnType, CustomAnalyzer
          )


    def test_tokenStreamReturnType_containingValuesOfNoneType(self):
        # As a special case that's notorious for slipping past SWIG to cause
        # segaults, try returning None.
        class CustomAnalyzer(lucene.Analyzer):
            def tokenStream(self, fieldName, reader):
                return [lucene.Token('AAA', 0, 2), None]
        self.assertRaises(TypeError,
            self._testBase_tokenStreamReturnType, CustomAnalyzer
          )


    def test_tokenStreamReturnType_NotIterable(self):
        # Make sure the Python wrapper reacts gracefully to an Analyzer whose
        # tokenStream an object that's not iterable.
        class CustomAnalyzer(lucene.Analyzer):
            def tokenStream(self, fieldName, reader):
                return 33
        self.assertRaises(TypeError,
            self._testBase_tokenStreamReturnType, CustomAnalyzer
          )


    def test_tokenStreamReturnType_None(self):
        # As a special case that's notorious for slipping past SWIG to cause
        # segaults, try returning None from Analyzer.tokenStream.
        class CustomAnalyzer(lucene.Analyzer):
            def tokenStream(self, fieldName, reader):
                return None
        self.assertRaises(TypeError,
            self._testBase_tokenStreamReturnType, CustomAnalyzer
          )


    def test_tokenStreamAsGenerator(tSelf):
        # Verify that pyclene can handle a custom Analyzer whose tokenStream
        # member is a *generator*.
        class CustomAnalyzer(lucene.Analyzer):
            def tokenStream(self, fieldName, reader):
                for token in tSelf._tokenStreamReturnType_baseTokens():
                    yield token

        tSelf._testBase_tokenStreamReturnType(CustomAnalyzer)


    def test_analyzerMethod_tokenStream_RaisesException(self):
        # This test case exercises the code path in which the
        # Analyzer.tokenStream method raises an exception, to ensure that
        # CLucene's internals don't leak or corrupt memory.
        class MarxistLeninistIdeologicalException(Exception):
            pass

        class RevolutionaryAnalyzer(lucene.Analyzer):
            def tokenStream(self, fieldName, reader):
                raise MarxistLeninistIdeologicalException(
                    'The International Alliance of Subclasses protests the'
                    ' exploitation of TokenStreams by foul agents of the'
                    ' Inverted Index bourgeois.'
                  )

        self.assertRaises(MarxistLeninistIdeologicalException,
            self._createIndexWithCustomAnalyzer, RevolutionaryAnalyzer()
          )


    def test_tokenStreamMethod_next_RaisesException(tSelf):
        # This test case exercises the code path in which the
        # Analyzer.tokenStream method returns an iterable that *later* raises
        # an exception, to ensure that CLucene's internals don't leak or
        # corrupt memory.
        class CustomAnalyzer(lucene.Analyzer):
            def tokenStream(self, fieldName, reader):
                tokens = tSelf._tokenStreamReturnType_baseTokens()
                for i in range(len(tokens)):
                    if i > 1:
                        raise ValueError('Deliberately failed partway through.')
                    yield tokens[i]

        tSelf.assertRaises(ValueError,
            tSelf._createIndexWithCustomAnalyzer, CustomAnalyzer()
          )


    def test_tokenStreamMethod_next_ReturnsSameTokenObjectMultipleTimes(self):
        # This test case causes the Python client program to return the same
        # token object multiple times from TokenStream.next.  The C++ code in
        # DocumentWriter.cpp deletes the returned object, so the wrapper code
        # must ensure that the Python programmer cannot accidentally corrupt
        # memory by reusing a single Token object across multiple calls.
        class CustomAnalyzer(lucene.Analyzer):
            def tokenStream(self, fieldName, reader):
                token = lucene.Token('AAA', 0, 2)
                for t in itertools.repeat(token, 5):
                    yield t

        self.assertRaises(RuntimeError,
            self._createIndexWithCustomAnalyzer, CustomAnalyzer()
          )


### SUPPORT ###
def FileDocument(filename):
    # XXX: Adding files that contain null bytes (without interpreting them as
    # unicode or something) causes CLucene to corrupt memory.
    doc = lucene.Document()
    # Path:
    doc.add(lucene.Field.Text('path', filename))
    # Modtime:
    doc.add(lucene.Field.Keyword('modified',
        lucene.DateField.timeToString(os.path.getmtime(filename))
      ))

    # Contents:
    # Reader-based impl:
    # YYY: Does the current version of pyclene still support the Reader-based impl?

    # String-based impl:
    data = file(filename, 'rb').read()
    if '\0' in data:
        raise ValueError('Null byte in data.')
    contentsField = lucene.Field.Text('contents', data)
    doc.add(contentsField)

    return doc


if __name__ == '__main__':
    import test
    test.main(suite=getFullTestSuite())
