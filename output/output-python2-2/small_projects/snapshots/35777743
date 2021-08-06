import datetime, os, os.path, sys, tarfile, time, unittest

import test_base
from pyclene import lucene


UNIQUE_TERM = 'ahfuoahgfoahoifhsdoahfoiasdhjfoihsdoahfs'


def getFullTestSuite():
    suite = unittest.TestSuite()

    suite.addTest( unittest.makeSuite(FilterTest) )
    suite.addTest( unittest.makeSuite(DateFilterTest) )

    suite.addTest( unittest.makeSuite(HitCollectorTest) )
    suite.addTest( unittest.makeSuite(HitsTest_ExplicitConstruction) )
    suite.addTest( unittest.makeSuite(HitsTest_SearcherConstruction) )
    suite.addTest( unittest.makeSuite(TopDocsTest) )
    suite.addTest( unittest.makeSuite(ScoreDocTest) )

    suite.addTest( unittest.makeSuite(IndexSearcherTest) )
    suite.addTest( unittest.makeSuite(MultiSearcherTest) )

    suite.addTest( unittest.makeSuite(QueryTest) )
    suite.addTest( unittest.makeSuite(TermQueryTest) )
    suite.addTest( unittest.makeSuite(MultiTermQueryTest) )
    suite.addTest( unittest.makeSuite(PhraseQueryTest) )

    # BooleanClause is a crucial supporting class of BooleanQuery.
    suite.addTest( unittest.makeSuite(BooleanClauseTest) )
    suite.addTest( unittest.makeSuite(BooleanQueryTest) )

    suite.addTest( unittest.makeSuite(WildcardQueryTest) )
    suite.addTest( unittest.makeSuite(PrefixQueryTest) )
    suite.addTest( unittest.makeSuite(FuzzyQueryTest) )
    suite.addTest( unittest.makeSuite(RangeQueryTest) )

    return suite


class FilterTest(test_base.CommonBaseTest):
    # Filter is exercised in IndexSearcherTest.test_search_*.
    pass


class DateFilterTest(test_base.CommonBaseTest):
    def _genTimes(self):
        curStamp = time.time()
        fromStamp = int(curStamp) - 86400 # 86400 = nSecondsPerDay
        toStamp = int(curStamp)

        fromTime = datetime.datetime.fromtimestamp(fromStamp)
        toTime = datetime.datetime.fromtimestamp(toStamp)

        return ( (fromStamp, toStamp), (fromTime, toTime) )


    def testConstructor_Range(self):
        for (start, stop) in self._genTimes():
            x = lucene.DateFilter('modified', start, stop)
        return x


    def testConstructor_Before(self):
        for (_, stop) in self._genTimes():
            x = lucene.DateFilter.Before('modified', stop)
        return x


    def testConstructor_After(self):
        for (_, stop) in self._genTimes():
            x = lucene.DateFilter.After('modified', stop)
        return x


    def _testFiltering_Boilerplate(self):
        indLoc = self.extractTestIndexToRAMDirectory()
        s = lucene.IndexSearcher(lucene.IndexReader.open(indLoc))

        analyzer = lucene.StandardAnalyzer()
        query = lucene.QueryParser.parse(UNIQUE_TERM, 'contents', analyzer)

        return s, analyzer, query


    def testFiltering_Range(self):
        s, analyzer, query = self._testFiltering_Boilerplate()

        # One hit for UNIQUE_TERM should be found in files modified between
        # 2004 and 2030.
        filt = lucene.DateFilter('modified',
            datetime.datetime(2004,1,1), datetime.datetime(2030,1,1)
          )
        hits = s.search(query, filter=filt)
        self.assertEqual(len(hits), 1)

        # Zero hits for UNIQUE_TERM should be found in files modified in
        # 1980 and 1983.
        filt = lucene.DateFilter('modified',
            datetime.datetime(1980,1,1), datetime.datetime(1983,1,1)
          )
        hits = s.search(query, filter=filt)
        self.assertEqual(len(hits), 0)


    def testFiltering_Before(self):
        s, analyzer, query = self._testFiltering_Boilerplate()

        # One hit for UNIQUE_TERM should be found in files modified before
        # 2030.
        filt = lucene.DateFilter.Before('modified', datetime.datetime(2030,1,1))
        hits = s.search(query, filter=filt)
        self.assertEqual(len(hits), 1)

        # Zero hits for UNIQUE_TERM should be found in files modified before
        # 2004.
        filt = lucene.DateFilter.Before('modified', datetime.datetime(2004,1,1))
        hits = s.search(query, filter=filt)
        self.assertEqual(len(hits), 0)


    def testFiltering_After(self):
        s, analyzer, query = self._testFiltering_Boilerplate()

        # One hit for UNIQUE_TERM should be found in files modified after
        # 2004.01.01.
        filt = lucene.DateFilter.After('modified', datetime.datetime(2004,1,1))
        hits = s.search(query, filter=filt)
        self.assertEqual(len(hits), 1)

        # Zero hits for UNIQUE_TERM should be found in files modified after
        # 2030.01.01.
        filt = lucene.DateFilter.After('modified', datetime.datetime(2030,1,1))
        hits = s.search(query, filter=filt)
        self.assertEqual(len(hits), 0)


class HitCollectorTest(test_base.CommonBaseTest):
    # HitCollector is exercised in IndexSearcherTest.test_search_withSideEffects.
    pass


class _HitsTest_Exercise(test_base.CommonBaseTest):
    # This class servers as an "abstract base class" for classes that create
    # Hits objects in different ways.

    def _makeSupportingObjects(self, searchTerm=UNIQUE_TERM):
        reader = lucene.IndexReader.open(self.extractTestIndexToRAMDirectory())
        s = lucene.IndexSearcher(reader)
        analyzer = lucene.StandardAnalyzer()
        query = lucene.QueryParser.parse(searchTerm, 'contents', analyzer)
        return s, query


    def test_type(self):
        # Make sure that in the case of HitsTest_SearcherConstruction, the
        # returned object is actually an instance of the Python class
        # lucene.Hits, rather than the SWIG bastard class
        # lucene._cl_py.HitsPtr.
        hits = self.testConstructor()
        self.assert_(isinstance(hits, lucene.Hits))


    def _test_boundedInfoMeth(self, meth):
        hits = self.testConstructor(searchTerm='lucene')
        for i in xrange(len(hits)):
            meth(hits, i)

        self.assertRaises(IndexError, meth, hits, -sys.maxint)
        self.assertRaises(IndexError, meth, hits, -1)
        self.assertRaises(IndexError, meth, hits, len(hits))
        self.assertRaises(IndexError, meth, hits, len(hits) + 1)
        self.assertRaises(IndexError, meth, hits, sys.maxint)
    def test_doc(self):
        self._test_boundedInfoMeth(lucene.Hits.doc)
    def test___getitem__(self):
        self._test_boundedInfoMeth(lucene.Hits.__getitem__)
    def test_id(self):
        self._test_boundedInfoMeth(lucene.Hits.id)
    def test_score(self):
        self._test_boundedInfoMeth(lucene.Hits.score)


    def test___iter__(self):
        hits = self.testConstructor(searchTerm='lucene')
        for i, doc in enumerate(hits):
            # The equality operator is not defined for Documents, so we test
            # the equality of their paths instead.
            self.assertEqual( doc['path'], hits.doc(i)['path'] )


class HitsTest_ExplicitConstruction(_HitsTest_Exercise):
    # Constructs Hits objects explicitly.
    def testConstructor(self, searchTerm=UNIQUE_TERM):
        s, query = self._makeSupportingObjects()

        # With filter:
        hits = lucene.Hits(s, query, filter=UnconditionallyFalseFilter())
        self.assertEqual(len(hits), 0)

        # Without filter:
        hits = lucene.Hits(s, query)
        if searchTerm == UNIQUE_TERM:
            self.assertEqual(len(hits), 1)

        return hits


class HitsTest_SearcherConstruction(_HitsTest_Exercise):
    # Constructs Hits objects by calling Searcher.search.
    def testConstructor(self, searchTerm=UNIQUE_TERM):
        s, query = self._makeSupportingObjects()

        hits = s.search(query)
        return hits


class TopDocsTest(test_base.CommonBaseTest):
    def test(self):
        # Test the following aspects of TopDocs:
        # - the totalHits property (integer)
        # - the __len__ method (convenience method over totalHits)
        # - the scoreDocs property (sequence of ScoreDoc objects)
        # - iteration of a TopDocs objects (iterator over ScoreDoc objects)

        reader = lucene.IndexReader.open(self.extractTestIndexToRAMDirectory())
        s = lucene.IndexSearcher(reader)
        analyzer = lucene.StandardAnalyzer()
        query = lucene.QueryParser.parse(UNIQUE_TERM, 'contents', analyzer)
        hits = s.search(query, limitNDocs=100)

        self.assertEqual(hits.totalHits, 1)
        self.assertEqual(len(hits), 1)
        # The current implementation materializes hits.scoreDocs each time
        # it's requested; ensure that it returns an equal value across multiple
        # requests.
        self.assertEqual(hits.scoreDocs, hits.scoreDocs)
        # The elements of hits.scoreDocs and the elements of a hits iterator
        # should be equal.
        self.assertEqual( [x for x in hits.scoreDocs], [x for x in hits] )

        # Search for a term that appears more than once.
        query = lucene.QueryParser.parse('lucene', 'contents', analyzer)
        retLimit = 100
        hits = s.search(query, limitNDocs=retLimit)
        self.assert_(hits.totalHits > 1)
        self.assert_(len(hits) > 1)
        if len(hits) > retLimit:
            self.assert_(len(hits.scoreDocs), retLimit)


class ScoreDocTest(test_base.CommonBaseTest):
    def test(self):
        # Test the following properties of ScoreDoc:
        # - doc (int)
        # - score (float)
        reader = lucene.IndexReader.open(self.extractTestIndexToRAMDirectory())
        s = lucene.IndexSearcher(reader)
        analyzer = lucene.StandardAnalyzer()
        query = lucene.QueryParser.parse('lucene', 'contents', analyzer)
        hits = s.search(query, limitNDocs=500)
        self.assert_(len(hits) > 0)

        for sd in hits:
            self.assert_(isinstance(sd, lucene.ScoreDoc))
            self.assert_(isinstance(sd.doc, int) and sd.doc >= 0 and sd.doc < s.maxDoc())
            self.assert_(isinstance(sd.score, float) and sd.score > 0.0)


class IndexSearcherTest(test_base.CommonBaseTest):
    def testConstructor_String(self):
        indLoc = self.extractTestIndex()
        s = lucene.IndexSearcher(indLoc)
        return s


    def testConstructor_IndexReader(self):
        indLoc = self.extractTestIndexToRAMDirectory()
        s = lucene.IndexSearcher(lucene.IndexReader.open(indLoc))
        return s


    def testConstructor_InvalidDir(self):
        # Without the guards I added, a CLucene IndexSearcher crashes if
        # pointed to a nonexistent directory.
        self.assertRaises(IOError,
            lucene.IndexSearcher, 'bogus-directory-that-does-not-exist'
          )


    def test_close(self):
        s = self.testConstructor_IndexReader()
        s.close()
        self.assertRaises(IOError, s.close)


    def test_docFreq(self):
        s = self.testConstructor_IndexReader()

        t = ('contents', UNIQUE_TERM)
        self.assertEqual(s.docFreq(t), 1)

        t = ('contents', 'xYz' * 6000)
        self.assertEqual(s.docFreq(t), 0)

        s.close()
        self.assertRaises(IOError, s.docFreq, t)


    def test_doc(self):
        s = self.testConstructor_IndexReader()

        for pos in (0, 1, 2, s.maxDoc()-2, s.maxDoc()-1):
            self.assert_(isinstance(s.doc(pos), lucene.Document))
        for pos in (-1, s.maxDoc(), s.maxDoc()+1, sys.maxint):
            self.assertRaises(IndexError, s.doc, pos)

        s.close()
        self.assertRaises(IOError, s.doc, 0)


    def test_maxDoc(self):
        reader = lucene.IndexReader.open(self.extractTestIndexToRAMDirectory())
        s = lucene.IndexSearcher(reader)

        # (The index is optimized, so s.maxDoc() will equal reader.numDocs().)
        self.assertEqual(s.maxDoc(), reader.numDocs())

        s.close()
        self.assertRaises(IOError, s.maxDoc)


    def test_search_withReturn(self):
        reader = lucene.IndexReader.open(self.extractTestIndexToRAMDirectory())
        s = lucene.IndexSearcher(reader)
        analyzer = lucene.StandardAnalyzer()
        query = lucene.QueryParser.parse(UNIQUE_TERM, 'contents', analyzer)

        # Without filter/simple interface:
        hits = s.search(query)
        self.assertEqual(len(hits), 1)

        # Without filter/"expert" interface:
        topDocs = s.search(query, limitNDocs=100)
        self.assertEqual(len(topDocs), 1)

        # With filter/simple interface:
        hits = s.search(query, filter=UnconditionallyFalseFilter())
        self.assertEqual(len(hits), 0)

        # With filter/"expert" interface:
        topDocs = s.search(query, filter=UnconditionallyFalseFilter(), limitNDocs=100)
        self.assertEqual(len(topDocs), 0)


        s.close()
        self.assertRaises(IOError, s.search,
            query, filter=UnconditionallyFalseFilter(), limitNDocs=100
          )


    def test_search_withSideEffects(self):
        reader = lucene.IndexReader.open(self.extractTestIndexToRAMDirectory())
        s = lucene.IndexSearcher(reader)
        analyzer = lucene.StandardAnalyzer()

        # Without filter:
        col = SubHitCollector()
        query = lucene.QueryParser.parse(UNIQUE_TERM, 'contents', analyzer)
        s.search(query, collector=col)
        self.assertEqual(len(col.hitList), 1)

        col = SubHitCollector()
        query = lucene.QueryParser.parse('lucene', 'contents', analyzer)
        s.search(query, collector=col)
        self.assert_(len(col.hitList) > 1)

        col = SubHitCollector()
        query = lucene.QueryParser.parse('notThere' * 800, 'contents', analyzer)
        s.search(query, collector=col)
        self.assertEqual(len(col.hitList), 0)

        # With filter:
        col = SubHitCollector()
        query = lucene.QueryParser.parse(UNIQUE_TERM, 'contents', analyzer)
        s.search(query, collector=col, filter=UnconditionallyFalseFilter())
        self.assertEqual(len(col.hitList), 0)

        s.close()
        self.assertRaises(IOError, s.search,
            query, collector=col, filter=UnconditionallyFalseFilter()
          )


class MultiSearcherTest(test_base.CommonBaseTest):
    def _makeSimple(self):
        # Although their contents are the same, the two indexes on which
        # reader1 and reader2 are based are distinct:
        reader1 = lucene.IndexReader.open(self.extractTestIndexToRAMDirectory())
        reader2 = lucene.IndexReader.open(self.extractTestIndexToRAMDirectory())

        searcher1 = lucene.IndexSearcher(reader1)
        searcher2 = lucene.IndexSearcher(reader2)

        searchers = (searcher1, searcher2)
        return lucene.MultiSearcher(searchers), searchers


    def testConstructor(self):
        reader = lucene.IndexReader.open(self.extractTestIndexToRAMDirectory())
        searchers = [lucene.IndexSearcher(reader) for i in xrange(2049)]

        # Ensure that a MultiSearcher can't be instantiated unless at least one
        # constituent is provided.
        self.assertRaises(TypeError, lucene.MultiSearcher)
        self.assertRaises(ValueError, lucene.MultiSearcher, [])

        # Ensure that trying to pass None as a constituent raises an exception
        # rather than crashing.
        self.assertRaises(TypeError, lucene.MultiSearcher,
            [searchers[0], None, searchers[1]]
          )

        # Instantiate normal MultiSearchers of various sizes.
        for i in xrange(20):
            lucene.MultiSearcher(searchers[:i+1])
        lucene.MultiSearcher(searchers)


    def test_close(self):
        ms = self._makeSimple()[0]
        ms.close()
        self.assertRaises(IOError, ms.close)


    def test_docFreq(self):
        ms, searchers = self._makeSimple()

        # The docFreq reported by the MultiSearcher should equal the sum of the
        # constituents' docFreqs.
        t = ('contents', UNIQUE_TERM)
        self.assertEqual( ms.docFreq(t), sum([s.docFreq(t) for s in searchers]) )

        t = ('contents', 'xYz' * 6000)
        self.assertEqual(ms.docFreq(t), 0)

        ms.close()
        self.assertRaises(IOError, ms.docFreq, t)


    def test_doc(self):
        ms = self._makeSimple()[0]

        for pos in (0, 1, 2, ms.maxDoc()-2, ms.maxDoc()-1):
            self.assert_(isinstance(ms.doc(pos), lucene.Document))
        for pos in (-1, ms.maxDoc(), ms.maxDoc()+1, sys.maxint):
            self.assertRaises(IndexError, ms.doc, pos)

        ms.close()
        self.assertRaises(IOError, ms.doc, 0)


    def test_maxDoc(self):
        ms, searchers = self._makeSimple()
        self.assertEqual( ms.maxDoc(), sum([s.maxDoc() for s in searchers]) )

        ms.close()
        self.assertRaises(IOError, ms.maxDoc)


    def test_search_withReturn(self):
        ms, searchers = self._makeSimple()
        analyzer = lucene.StandardAnalyzer()
        query = lucene.QueryParser.parse(UNIQUE_TERM, 'contents', analyzer)

        # Without filter/simple interface:
        hits = ms.search(query)
        self.assertEqual(len(hits), 2)

        # Without filter/"expert" interface:
        topDocs = ms.search(query, limitNDocs=100)
        self.assertEqual(len(topDocs), 2)

        # With filter/simple interface:
        hits = ms.search(query, filter=UnconditionallyFalseFilter())
        self.assertEqual(len(hits), 0)

        # With filter/"expert" interface:
        topDocs = ms.search(query, filter=UnconditionallyFalseFilter(), limitNDocs=100)
        self.assertEqual(len(topDocs), 0)


        ms.close()
        self.assertRaises(IOError, ms.search,
            query, filter=UnconditionallyFalseFilter(), limitNDocs=100
          )


    def test_search_withSideEffects(self):
        ms, searchers = self._makeSimple()
        analyzer = lucene.StandardAnalyzer()

        # Without filter:
        col = SubHitCollector()
        query = lucene.QueryParser.parse(UNIQUE_TERM, 'contents', analyzer)
        ms.search(query, collector=col)
        self.assertEqual(len(col.hitList), 2)

        col = SubHitCollector()
        query = lucene.QueryParser.parse('lucene', 'contents', analyzer)
        ms.search(query, collector=col)
        self.assert_(len(col.hitList) > 2)

        col = SubHitCollector()
        query = lucene.QueryParser.parse('notThere' * 800, 'contents', analyzer)
        ms.search(query, collector=col)
        self.assertEqual(len(col.hitList), 0)

        # With filter:
        col = SubHitCollector()
        query = lucene.QueryParser.parse(UNIQUE_TERM, 'contents', analyzer)
        ms.search(query, collector=col, filter=UnconditionallyFalseFilter())
        self.assertEqual(len(col.hitList), 0)

        ms.close()
        self.assertRaises(IOError, ms.search,
            query, collector=col, filter=UnconditionallyFalseFilter()
          )


    def test_subSearcher(self):
        # For this test, the underlying searchers are assumed to be optimized
        # (so that their document identifiers are contiguous).
        ms, searchers = self._makeSimple()

        for pos in (-sys.maxint-1, -1, ms.maxDoc(), ms.maxDoc()+1, sys.maxint):
            self.assertRaises(IOError, ms.subSearcher, pos)

        # Positions that match the first subsearcher:
        self.assertEqual( ms.subSearcher(0), 0 )
        self.assertEqual( ms.subSearcher(1), 0 )
        self.assertEqual( ms.subSearcher(searchers[0].maxDoc()-2), 0 )
        self.assertEqual( ms.subSearcher(searchers[0].maxDoc()-1), 0 )
        self.assertEqual( ms.subSearcher(searchers[0].maxDoc()  ), 1 )
        self.assertEqual( ms.subSearcher(searchers[0].maxDoc()+1), 1 )
        # Positions that match the second subsearcher:
        self.assertEqual(
            ms.subSearcher(
              searchers[0].maxDoc() + searchers[1].maxDoc()-1
            ), 1
          )
        self.assertEqual( ms.subSearcher(ms.maxDoc()-1), len(searchers)-1 )

        ms.close()
        self.assertRaises(IOError, ms.subSearcher, 0)


class QueryTest(test_base.CommonBaseTest):
    # lucene.Query is an abstract class.  Its subclasses (TermQuery,
    # MultiTermQuery, PhraseQuery, BooleanQuery, WildcardQuery, PrefixQuery,
    # FuzzyQuery, RangeQuery) are exercised in their own test classes below.
    def testConstructor(self):
        # Although the typical "I'm-an-abstract-class" exception is
        # NotImplementedError, SWIG raises RuntimeError.
        self.assertRaises(RuntimeError, lucene.Query)


class _BaseTestForConcreteQueries(test_base.CommonBaseTest):
    def _requireParsedHits(self, requiredHitCount, rawQueryText, searcher):
        analyzer = lucene.StandardAnalyzer()
        parsedQ = lucene.QueryParser.parse(rawQueryText, 'contents', analyzer)
        hits = searcher.search(parsedQ)
        parsedHitCount = len(hits)
        self.assertEqual(requiredHitCount, parsedHitCount)


    def test_repr(self):
        # Just make sure it doesn't crash.
        q = self.testConstructor()
        repr(q)


    def test_name(self):
        # The name property is present mainly for the benefit of the
        # introspection-challenged C++.
        q = self.testConstructor()
        self.assertEqual(q.name, q.__class__.__name__)
        # Verify immutability:
        self.assertRaises(AttributeError, setattr, q, 'name', 'other')


    def test_prepare(self):
        q = self.testConstructor()
        reader = lucene.IndexReader.open(self.extractTestIndexToRAMDirectory())
        # YYY: Don't know at present how to validate prepare.
        q.prepare(reader)


    # def test_scorer(self):
        # # YYY: In initial pass, not wrapping the Scorer class or methods used to
        # # access instances of it.
        # q = self.testConstructor()
        # reader = lucene.IndexReader.open(self.extractTestIndexToRAMDirectory())
        # # YYY: Don't know at present how to validate scorer.
        # print q.scorer(reader)


    def test_boost(self):
        q = self.testConstructor()
        self.assert_( isinstance(q.boost, float) )

        q.boost = 2.5
        self.assertEqual( q.boost, 2.5 )

        # YYY: Ought to test the effects of the boost change.


    def test_sumOfSquaredWeights(self):
        q = self.testConstructor()
        reader = lucene.IndexReader.open(self.extractTestIndexToRAMDirectory())
        s = lucene.IndexSearcher(reader)

        # Some CLucene Query subclasses will crash if sumOfSquaredWeights is
        # called before the prepare method has been called; at the Python
        # level, we raise an exception instead.
        self.assertRaises(IOError, q.sumOfSquaredWeights, s)
        q.prepare(reader)

        # YYY: Don't know at present how to validate sumOfSquaredWeights.
        self.assert_(isinstance(q.sumOfSquaredWeights(s), float))


    def test_normalize(self):
        q = self.testConstructor()

        # Some CLucene Query subclasses will crash if normalize is called
        # before the prepare method has been called; at the Python level, we
        # raise an exception instead.
        self.assertRaises(IOError, q.normalize, 1.5)
        reader = lucene.IndexReader.open(self.extractTestIndexToRAMDirectory())
        q.prepare(reader)

        # YYY: Don't know at present how to validate normalize.
        q.normalize(1.5)


class TermQueryTest(_BaseTestForConcreteQueries):
    def testConstructor(self):
        t = ('contents', 'blah')
        q = lucene.TermQuery(t)
        return q


    def test_toString(self):
        q = self.testConstructor()
        self.assertEqual( q.toString('contents'), 'blah' )


    def test_queryInAction(self):
        reader = lucene.IndexReader.open(self.extractTestIndexToRAMDirectory())
        s = lucene.IndexSearcher(reader)

        # A blank search should not yield any hits:
        q = lucene.TermQuery(('contents', ''))
        hits = s.search(q)
        self.assertEqual(len(hits), 0)

        q = lucene.TermQuery(('contents', ' \t\n '))
        hits = s.search(q)
        self.assertEqual(len(hits), 0)

        q = lucene.TermQuery(('contents', 'lucene'))
        hits = s.search(q)
        self.assert_(len(hits) >= 1)

        q = lucene.TermQuery(('contents', 'thisandthat' * 10000))
        hits = s.search(q)
        self.assertEqual(len(hits), 0)


class MultiTermQueryTest(test_base.CommonBaseTest):
    # According to the Lucene documentation, "MultiTermQuery is not designed
    # to be used by itself." (but rather, via its subclasses, including
    # WildcardQuery and FuzzyQuery).  See the test classes for those subclasses.
    pass


class PhraseQueryTest(_BaseTestForConcreteQueries):
    def testConstructor(self):
        q = lucene.PhraseQuery()
        q.add(('contents', 'lucene'))
        return q


    def test_toString(self):
        q = self.testConstructor()
        self.assertEqual( q.toString('contents'), '"lucene"' )


    def test_queryInAction(self):
        # This method tests the following PhraseQuery-specific members:
        # - add method
        # - terms property
        # - slop property
        DECOY_PHRASE = 'decoy phrase here remagen'.split()

        reader = lucene.IndexReader.open(self.extractTestIndexToRAMDirectory())
        s = lucene.IndexSearcher(reader)

        q = lucene.PhraseQuery()
        for word in DECOY_PHRASE:
            q.add(('contents', word))
        hits = s.search(q)
        self.assert_(len(hits) >= 1)

        self.assertEqual([t[1] for t in q.terms], DECOY_PHRASE)
        # PhraseQuery.terms is a read-only property (enforcing that behavior
        # required bullying SWIG).
        self.assertRaises(AttributeError, setattr, q, 'terms', None)

        # Test the 'slop' property (see the docs for PhraseQuery.setSlop in
        # the Java impl of Lucene).
        DECOY_PHRASE_REV = DECOY_PHRASE[:]
        DECOY_PHRASE_REV.reverse()
        q = lucene.PhraseQuery()
        # A slop of 6 will match the original 4-word decoy phrase with the
        # order of the words reversed.
        q.slop = 6
        for word in DECOY_PHRASE_REV:
            q.add(('contents', word))
        hits = s.search(q)
        self.assert_(len(hits) >= 1)


class BooleanClauseTest(test_base.CommonBaseTest):
    def testConstructor(self):
        # Positive
        c = lucene.BooleanClause(
            lucene.TermQuery(('contents', 'lucene')),
            True, False
          )
        self.assert_(c.required)
        self.assert_(not c.prohibited)

        # Negative
        c = lucene.BooleanClause(
            lucene.TermQuery(('contents', 'lucene')),
            False, True
          )
        self.assert_(not c.required)
        self.assert_(c.prohibited)


    def test_SpecificMemoryManagementSequence(self):
        # These calls are arranged in a specific sequence to exercise the
        # convenient Python refcount facade over the tricky memory management
        # schemes of CLucene's Term and Query classes.  Anyone modifying this
        # test suite should refrain from altering this sequence.
        indLoc = self.extractTestIndexToRAMDirectory()
        s = lucene.IndexSearcher(lucene.IndexReader.open(indLoc))

        q1 = lucene.TermQuery(('contents', 'lucene'))
        c = lucene.BooleanClause(
            q1,
            True, False
          )
        bq = lucene.BooleanQuery()
        bq.add(c)

        c = lucene.BooleanClause(
            lucene.TermQuery(('contents', 'python')),
            True, False
          )
        bq.add(c)
        del c

        self.assertNotEqual(q1.toString('contents'), '')
        del q1

        for cl in bq.clauses:
            self.assertNotEqual(cl.query.toString('contents'), '')

        hits = s.search(bq)
        self.assert_(len(hits) >= 1)


class BooleanQueryTest(_BaseTestForConcreteQueries):
    def testConstructor(self):
        q = lucene.BooleanQuery()
        return q


    def test_toString(self):
        q = self.testConstructor()
        self.assertEqual(q.toString('contents'), '')

        q.add(lucene.BooleanClause(
            lucene.TermQuery(('contents', 'lucene')),
            True, False
          ))
        self.assertEqual(q.toString('contents'), '+lucene')

        q.add(lucene.BooleanClause(
            lucene.TermQuery(('contents', 'CLucene')),
            True, False
          ))
        # Case is preserved because we're not using a normalizing Analyzer.
        self.assertEqual(q.toString('contents'), '+lucene +CLucene')


    def test_queryInAction(self):
        # This method tests the following BooleanQuery-specific members:
        # - add method
        # - clauses property

        indLoc = self.extractTestIndexToRAMDirectory()
        s = lucene.IndexSearcher(lucene.IndexReader.open(indLoc))

        # Positive
        q = self.testConstructor()
        self.assertEqual(len(q.clauses), 0)
        q.add(lucene.BooleanClause(
            lucene.TermQuery(('contents', 'lucene')),
            True, False
          ))
        self.assertEqual(len(q.clauses), 1)

        hits = s.search(q)
        manualHitCount = len(hits)
        self.assert_(manualHitCount >= 1)
        self._requireParsedHits(manualHitCount, '+lucene', s)

        q.add(lucene.BooleanClause(
            lucene.TermQuery(('contents', 'python')),
            True, False
          ))
        self.assertEqual(len(q.clauses), 2)
        hits = s.search(q)
        manualHitCount = len(hits)
        self.assert_(manualHitCount >= 1)
        self._requireParsedHits(manualHitCount, '+lucene +python', s)

        # python lucene andfriends
        q.add(lucene.BooleanClause(
            lucene.TermQuery(('contents', 'andfriends')),
            True, False
          ))
        self.assertEqual(len(q.clauses), 3)
        hits = s.search(q)
        manualHitCount = len(hits)
        self.assert_(manualHitCount >= 1)
        self._requireParsedHits(manualHitCount, '+lucene +python +andfriends', s)

        # Positive/Negative
        # The docs for the Java impl of Lucene say:
        # """
        # Note: The NOT operator cannot be used with just one term. For
        # example, the following search will return no results:
        #   NOT "jakarta apache"
        # """
        # Apparently, purely negative searches never yield any hits.
        q = self.testConstructor()
        q.add(lucene.BooleanClause(
            lucene.TermQuery(('contents', 'lucene')),
            False, True
          ))
        q.add(lucene.BooleanClause(
            lucene.TermQuery(('contents', 'quahog')),
            False, True
          ))
        q.add(lucene.BooleanClause(
            lucene.TermQuery(('contents', 'python')),
            True, False
          ))
        hits = s.search(q)
        manualHitCount = len(hits)
        self.assert_(manualHitCount >= 1)
        self._requireParsedHits(manualHitCount, '-lucene -quahog +python', s)

        q = self.testConstructor()
        pq = lucene.PhraseQuery()
        for word in 'newyears resolution smoke more herb'.split():
            pq.add(('contents', word))
        q.add(lucene.BooleanClause(pq, True, False))
        hits = s.search(q)
        manualHitCount = len(hits)
        self.assertEqual(manualHitCount, 1)
        self._requireParsedHits(manualHitCount,
            '+"newyears resolution smoke more herb"', s
          )

        pq = lucene.PhraseQuery()
        for word in 'jim morrison mp3s facilitate goal fulfilment'.split():
            pq.add(('contents', word))
        q.add(lucene.BooleanClause(pq, False, True))
        hits = s.search(q)
        manualHitCount = len(hits)
        self.assertEqual(manualHitCount, 0)
        self._requireParsedHits(manualHitCount,
            '+"newyears resolution smoke more herb"'
            '-"jim morrison mp3s facilitate goal fulfilment"',
            s
          )


class WildcardQueryTest(_BaseTestForConcreteQueries):
    def testConstructor(self):
        q = lucene.WildcardQuery(('contents', 'pytho*'))
        return q


    def test_toString(self):
        q = self.testConstructor()
        self.assertEqual( q.toString('contents'), 'pytho*' )


    def test_queryInAction(self):
        indLoc = self.extractTestIndexToRAMDirectory()
        s = lucene.IndexSearcher(lucene.IndexReader.open(indLoc))

        for pattern in ('pytho*', 'py*n', 'p*th?n', 'itjusta?nt*here'):
            q = lucene.WildcardQuery(('contents', pattern))
            hits = s.search(q)
            manualHitCount = len(hits)
            self._requireParsedHits(manualHitCount, pattern, s)


class PrefixQueryTest(_BaseTestForConcreteQueries):
    def testConstructor(self):
        q = lucene.PrefixQuery(('contents', 'py'))
        return q


    def test_toString(self):
        q = self.testConstructor()
        self.assertEqual( q.toString('contents'), 'py*' )


    def test_queryInAction(self):
        indLoc = self.extractTestIndexToRAMDirectory()
        s = lucene.IndexSearcher(lucene.IndexReader.open(indLoc))

        for prefix in ('py', 'lu', 'clu', 'pycle'):
            q = lucene.PrefixQuery(('contents', prefix))
            hits = s.search(q)
            manualHitCount = len(hits)
            self._requireParsedHits(manualHitCount, prefix + '*', s)


class FuzzyQueryTest(_BaseTestForConcreteQueries):
    def testConstructor(self):
        q = lucene.FuzzyQuery(('contents', 'pithon'))
        return q


    def test_toString(self):
        q = self.testConstructor()
        self.assertEqual( q.toString('contents'), 'pithon~' )


    def test_queryInAction(self):
        indLoc = self.extractTestIndexToRAMDirectory()
        s = lucene.IndexSearcher(lucene.IndexReader.open(indLoc))

        for fuzz in ('pithon', 'jython', 'pyclane', 'blahblahblah'):
            q = lucene.FuzzyQuery(('contents', fuzz))
            hits = s.search(q)
            manualHitCount = len(hits)
            self._requireParsedHits(manualHitCount, fuzz + '~', s)


class RangeQueryTest(_BaseTestForConcreteQueries):
    def testConstructor(self):
        q = lucene.RangeQuery(('contents', '123'), ('contents', '126'), True)
        return q


    def test_toString(self):
        q = self.testConstructor()
        self.assertEqual( q.toString('contents'), '[123-126]' )


    def test_queryInAction(self):
        indLoc = self.extractTestIndexToRAMDirectory()
        s = lucene.IndexSearcher(lucene.IndexReader.open(indLoc))

        for lower, upper in ( ('123','126'), ('jython','python') ):
            q = lucene.RangeQuery(
                ('contents', lower), ('contents', upper),
                True
              )
            hits = s.search(q)
            manualHitCount = len(hits)
            self._requireParsedHits(manualHitCount,
                'contents:[%s TO %s]' % (lower, upper), s
              )


### SUPPORT ###
class UnconditionallyFalseFilter(lucene.Filter):
    # UnconditionallyFalseFilter is a trivial subclass of Filter used to verify
    # that subclassing the C++ class Filter in Python works as expected.

    def bits(self, indReader):
        # The bits method returns a sequence of booleans indicating which
        # document have been left in the set (True) and which filtered out
        # (False).
        return [False] * indReader.maxDoc()


class SubHitCollector(lucene.HitCollector):
    # SubHitCollector is a trivial subclass of HitCollector used to verify that
    # subclassing the C++ class HitCollector in Python works as expected.

    def __init__(self):
        lucene.HitCollector.__init__(self)
        self.hitList = []

    def collect(self, docNo, score):
        self.hitList.append(docNo)


if __name__ == '__main__':
    import test
    test.main(suite=getFullTestSuite())
