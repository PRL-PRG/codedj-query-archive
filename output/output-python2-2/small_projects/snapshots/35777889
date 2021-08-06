import operator, os.path, string, sys, unittest

import test_base
from pyclene import lucene


def getFullTestSuite():
    suite = unittest.TestSuite()

    suite.addTest( unittest.makeSuite(TokenTest) )
    suite.addTest( unittest.makeSuite(TokenStreamTest) )
    suite.addTest( unittest.makeSuite(StandardTokenizerTest) )
    # There is no dedicated test of StandardAnalyzer, because it really just
    # ties together a group of the objects that are tested here, and it's
    # exercised in dozens of other contexts.

    return suite


class TokenTest(test_base.CommonBaseTest):
    def testConstructor_ImplicitType(self):
        t = lucene.Token('blah', 0, 4)

        self.assertEqual(t.text, 'blah')
        self.assertEqual(t.start, 0)
        self.assertEqual(t.end, 4)
        self.assertEqual(t.sourceSlice, slice(t.start, t.end))
        self.assertEqual(t.type, 'word') # Default type tag is 'word'.

        return t

    def testConstructor_ExplicitType(self):
        t = lucene.Token('blah', 0, 4, 'custom-type-tag with a space')

        self.assertEqual(t.type, 'custom-type-tag with a space')

        return t


    def test___repr__(self):
        # Just make sure it doesn't crash.
        t = self.testConstructor_ImplicitType()
        repr(t)


    def test___str__(self):
        # Just make sure it doesn't crash.
        t = self.testConstructor_ImplicitType()
        str(t)


    def test_immutability(self):
        t = self.testConstructor_ImplicitType()

        # These properties are immutable; an exception should be raised if
        # client code tries to change them.
        for (attrName, attrValue) in (
            ('text', 'blah-plus'),
            ('start', 1),
            ('end', 5),
            ('type', 'other-type'),
          ):
            self.assertRaises(AttributeError, setattr, t, attrName, attrValue)


    def test_positionIncrement_get(self):
        t = lucene.Token('blah', 0, 4)
        self.assertEqual(t.positionIncrement, 1)


    def test_positionIncrement_setViaProperty(self):
        t = lucene.Token('blah', 0, 4)
        self.assertEqual(t.positionIncrement, 1)
        t.positionIncrement = 0
        self.assertEqual(t.positionIncrement, 0)
        t.positionIncrement = 2
        self.assertEqual(t.positionIncrement, 2)


    def test_positionIncrement_setViaConstructor(self):
        t = lucene.Token('blah', 0, 4, positionIncrement=0)
        self.assertEqual(t.positionIncrement, 0)

        t = lucene.Token('blah', 0, 4, positionIncrement=1)
        self.assertEqual(t.positionIncrement, 1)

        t = lucene.Token('blah', 0, 4, positionIncrement=2)
        self.assertEqual(t.positionIncrement, 2)


    def test_positionIncrement_nonNegative(self):
        # Via property:
        t = lucene.Token('blah', 0, 4)
        self.assertRaises(Exception, setattr, t, 'positionIncrement', -1)
        # Via constructor:
        self.assertRaises(Exception,
            lucene.Token, 'blah', 0, 4, positionIncrement=-1
          )


    def test_positionIncrement_forceTokensIntoSamePosition(self):
        class CustomAnalyzer(lucene.Analyzer):
            def tokenStream(self, fieldName, value):
                pos = 0
                for word in value.split():
                    posStop = pos + len(word)
                    yield lucene.Token(word, pos, posStop)
                    if word == 'david':
                        yield lucene.Token(word + 'artificial', pos, posStop,
                            positionIncrement=0
                          )

                    pos = posStop + 1 # Skip the space char.

        tempDir = self.getTempFilename()
        w = lucene.IndexWriter(tempDir, CustomAnalyzer(), True)
        try:
            doc = lucene.Document()
            contentsField = lucene.Field.Text('contents', 'david stephen rushby david')
            doc.add(contentsField)

            w.addDocument(doc)
        finally:
            w.close()

        # The word 'davidartificial' didn't actually exist in the input string,
        # but the CustomAnalyzer inserted it as an artificial word in the token
        # stream, recorded as occupying the same positions as 'david'.
        # Use IndexReader::termPositions to verify that the terms are recorded
        # as expected.
        r = lucene.IndexReader.open(tempDir)
        try:
            for expectedWord in ('david', 'davidartificial'):
                tp = r.termPositions(containing=('contents', expectedWord))
                tp.next()
                self.assertEqual(tp.doc(), 0) # First (and only) document.
                self.assertEqual(tp.freq(), 2) # Two occurrences of actual term.
                # First occurrence occupies first position in document.
                self.assertEqual(tp.nextPosition(), 0)
                # Second occurrence occupies fourth position in document.
                self.assertEqual(tp.nextPosition(), 3)

            # The words 'stephen' and 'rushby', on the other hand, should have
            # been recorded without embellishments.
            for word, expectedPos in (('stephen', 1), ('rushby', 2)):
                tp = r.termPositions(containing=('contents', word))
                tp.next()
                self.assertEqual(tp.doc(), 0) # First (and only) document.
                self.assertEqual(tp.freq(), 1) # One occurrence.
                # First occurrence occupies second position in document.
                self.assertEqual(tp.nextPosition(), expectedPos)
                # Second occurrence doesn't exist.
                self.assertRaises(IOError, tp.nextPosition)
        finally:
            r.close()

        # Finally, use IndexSearcher::search to verify that a PhraseQuery's
        # behavior is not disturbed by the presence of multiple terms that
        # claim to be in the same position.
        s = lucene.IndexSearcher(tempDir)
        try:
            for qs, expectedHitCount in (
                # Any combination of real ('david') and artificial
                # ('davidartificial') tokens should hit:
                ('+"david stephen rushby david"', 1),
                ('+"davidartificial stephen rushby david"', 1),
                ('+"david stephen rushby davidartificial"', 1),
                ('+"davidartificial stephen rushby davidartificial"', 1),

                # No artificial tokens were generated for 'stephen', 'rushby':
                ('+"david stephenartificial rushby david"', 0),
                ('+"david stephen rushbyartificial david"', 0),

                # Since the real token and the artificial token are recorded
                # as occupying the same position, a phrase containing one
                # followed by the other should not hit:
                ('+"david davidartificial"', 0),
                ('+"davidartificial david"', 0),
              ):
                query = lucene.QueryParser.parse(qs,
                    'contents', lucene.StandardAnalyzer()
                  )
                hits = s.search(query)
                self.assertEqual(len(hits), expectedHitCount)
        finally:
            s.close()


class TokenStreamTest(test_base.CommonBaseTest):
    def _makeReader(self, reader):
        if reader is None:
            reader = lucene.FileReader(_nameOfTestFile())
        return reader


    def testConstructor(self, reader=None):
        reader = self._makeReader(reader)
        a = lucene.StandardAnalyzer()
        ts = a.tokenStream('contents', reader)
        return ts


    def test___repr__(self):
        # Just make sure it doesn't crash.
        # Defined locally to facilitate future examination of class-specific contents.
        ts = self.testConstructor()
        repr(ts)


    def test___str__(self):
        # Just make sure it doesn't crash.
        # Defined locally to facilitate future examination of class-specific contents.
        ts = self.testConstructor()
        str(ts)


    def _test_iteration(self, reader, cancelEffectsOfStandardFilter=True):
        # Implicitly exercises the next() method.
        sourceText = _textOfTestFile()
        assert '\0' not in sourceText

        ts = self.testConstructor(reader=reader)

        if not cancelEffectsOfStandardFilter:
            tokensConsideredEqual = operator.eq # straightforward equality
        else:
            def tokensConsideredEqual(ref, target):
                # StandardFilter applies the following transformations:
                #   A) Removes 's (apostrophe-ess) from the end of words.
                #   B) Removes periods from acronyms.
                # This predicate tests a reference string $ref (which might've
                # been passed through StandardFilter) for equality against
                # another string $target, which has not been passed through
                # StandardFilter.
                target = target.lower()
                if ref == target:
                    return True
                elif target.endswith("'s") and ref == target[:-2]: # transformation A
                    return True
                elif ref == target.replace('.', ''): # transformation B
                    return True
                else:
                    return False

        diffs = []
        for t in ts:
            sourceSnippet = sourceText[t.sourceSlice]
            assert '\0' not in sourceSnippet
            if not tokensConsideredEqual(t.text, sourceSnippet):
                msg = (
                    't.text ("%s") != sourceText[%d:%d] ("%s")'
                    % (t.text, t.start, t.end, sourceSnippet)
                  )
                diffs.append(msg)

        if len(diffs) > 0:
            msg = 'TOKEN DIFFERENCES:\n' + '\n-----\n'.join(diffs)
            self.fail(msg)


    def test_iteration_withFileReader(self):
        return self._test_iteration(lucene.FileReader(_nameOfTestFile()))

    def test_iteration_withStringReader(self):
        return self._test_iteration(lucene.StringReader(_textOfTestFile()))


    def test_close(self):
        ts = self.testConstructor()
        ts.close()
        self.assertRaises(IOError, ts.close) # Already closed.


class StandardTokenizerTest(TokenStreamTest):
    # Note that this class is a subclass of TokenStreamTest.

    def testConstructor(self, reader=None):
        reader = self._makeReader(reader)
        tk = lucene.StandardTokenizer(reader)
        return tk


    def test_iteration_withFileReader(self):
        return self._test_iteration(lucene.FileReader(_nameOfTestFile()),
            cancelEffectsOfStandardFilter=False
          )

    def test_iteration_withStringReader(self):
        return self._test_iteration(lucene.StringReader(_textOfTestFile()),
            cancelEffectsOfStandardFilter=False
          )


    def test_recognitionOfInheritanceRelationship(self):
        # Make sure StandardTokenizer is recognized as a descendant of
        # TokenStream, even though SWIG doesn't generate such code by default
        # (see note in lucene.py/definition of StandardTokenizer).
        self.failUnless( issubclass(lucene.StandardTokenizer, lucene.TokenStream) )


    def tok(self, text, requiredTokens):
        ts = lucene.StandardTokenizer(text)
        generatedTokens = [(t.start, t.end, t.type, t.text) for t in ts]
        self.assertEqual(generatedTokens, requiredTokens)


    def test_Empty(self):
        self.tok('', [])
        self.tok(None, [])
        self.tok(' \t\n  ', [])
        self.tok('...', [])

    def test_debatableCases(self):
        self.tok('abc--def', [(0,3,'<ALPHANUM>','abc'), (5,8,'<ALPHANUM>','def')])
        # Even though the following is a valid host name, it's far more typical
        # in English soure text for this pattern to be a non-host.
        self.tok('abc.--def', [(0,3,'<ALPHANUM>','abc'), (6,9,'<ALPHANUM>','def')])
        self.tok('abc--.def', [(0,3,'<ALPHANUM>','abc'), (6,9,'<ALPHANUM>','def')])
        self.tok('abc.-def',  [(0,3,'<ALPHANUM>','abc'), (5,8,'<ALPHANUM>','def')])
        self.tok('abc-.def',  [(0,3,'<ALPHANUM>','abc'), (5,8,'<ALPHANUM>','def')])
        self.tok('Visit windowsupdate.microsoft.com--update today!',
            [
             ( 0,  5, '<ALPHANUM>', 'Visit'                         ),
             ( 6, 33, '<HOST>',     'windowsupdate.microsoft.com'   ),
             (35, 41, '<ALPHANUM>', 'update'                        ),
             (42, 47, '<ALPHANUM>', 'today'                         ),
            ]
          )
        self.tok('In the U.S.A.--yes, even there!',
            [
             ( 0,  2, '<ALPHANUM>', 'In'                            ),
             ( 3,  6, '<ALPHANUM>', 'the'                           ),
             ( 7, 13, '<ACRONYM>',  'U.S.A.'                        ),
             (15, 18, '<ALPHANUM>', 'yes'                           ),
             (20, 24, '<ALPHANUM>', 'even'                          ),
             (25, 30, '<ALPHANUM>', 'there'                         ),
            ]
          )

    def test_plainAlphanum(self):
        self.tok('excellent', [(0,9,'<ALPHANUM>','excellent')])
        self.tok('test_plainAlphanum', [(0,18,'<ALPHANUM>','test_plainAlphanum')])
        self.tok('l33tm0f0z', [(0,9,'<ALPHANUM>','l33tm0f0z')])
        self.tok('l33tm0f0z_4u', [(0,12,'<ALPHANUM>','l33tm0f0z_4u')])

    def test_apostrophe(self):
        self.tok("can't", [(0,5,'<APOSTROPHE>',"can't")])
        self.tok("I'll", [(0,4,'<APOSTROPHE>',"I'll")])
        self.tok("Said, 'Hello'.", [(0,4,'<ALPHANUM>', "Said"), (7,12,'<ALPHANUM>', "Hello")])
        self.tok("blah'46.7", [(0,4,'<ALPHANUM>', "blah"), (5,9,'<NUM>', "46.7")])
        self.tok("blah1.blah2't", [(0, 11, '<HOST>', "blah1.blah2"), (12, 13, '<ALPHANUM>', "t")])
        self.tok("blah1'blah2.blah3", [
            (0, 10, '<APOSTROPHE>', "blah1'blah"),
            (10, 11, '<NUM>', '2'),
            (12, 17, '<ALPHANUM>', 'blah3')
          ])

    def test_acronym(self):
        self.tok('U.S.A.', [(0,6,'<ACRONYM>','U.S.A.')])
        self.tok('U.S.A..', [(0,6,'<ACRONYM>','U.S.A.')])

        self.tok('U.', [(0,1,'<ALPHANUM>','U')])
        self.tok('U.4.A.', [(0,5,'<HOST>','U.4.A')])
        self.tok('U.S.A', [(0,5,'<HOST>','U.S.A')])
        self.tok('UU.SS.AA.', [(0,8,'<HOST>','UU.SS.AA')])



    def test_company(self):
        self.tok('Greedy&Greedier', [(0,15,'<COMPANY>','Greedy&Greedier')])
        self.tok('Excite@Home', [(0,11,'<COMPANY>','Excite@Home')])
        # E-mail because contains non-letter:
        self.tok('Exc1te@Home', [(0,11,'<EMAIL>','Exc1te@Home')])
        # E-mail because contains dotted component:
        self.tok('Excite@Home.com', [(0,15,'<EMAIL>','Excite@Home.com')])

    def test_email(self):
        self.tok('david.rushby@internal_mail', [(0,26,'<EMAIL>','david.rushby@internal_mail')])
        self.tok('david.rushby-lewis.1933@dork.org', [(0,32,'<EMAIL>','david.rushby-lewis.1933@dork.org')])
        # Num+Host because the first component is purely numeric:
        self.tok('123@dork.org', [(0,3,'<NUM>','123'), (4,12,'<HOST>','dork.org')])

    def test_host(self):
        self.tok('host.localdomain',  [(0,16,'<HOST>','host.localdomain')])
        self.tok('host.localdomain.', [(0,16,'<HOST>','host.localdomain')])
        self.tok('192.168.1.2', [(0,11,'<HOST>','192.168.1.2')])

        reallyLongHost = '.'.join(string.ascii_lowercase*2000)
        self.tok(reallyLongHost, [(0,len(reallyLongHost),'<HOST>',reallyLongHost)])

        self.tok('self.bray()', [(0,9,'<HOST>','self.bray')])
        self.tok('SWIG (http://www.swig.org).', [
            (0, 4, '<ALPHANUM>', 'SWIG'),
            (6, 10, '<ALPHANUM>', 'http'),
            (13, 25, '<HOST>', 'www.swig.org')
          ])

    def test_number(self):
        self.tok('.',   [])
        self.tok('. ',  [])
        self.tok('-',   [])
        self.tok('- ',  [])
        self.tok('+',   [])
        self.tok('+ ',  [])
        self.tok('+.',  [])
        self.tok('+. ', [])
        self.tok('-.',  [])
        self.tok('-. ', [])
        self.tok('.-',  [])
        self.tok('.- ', [])

        self.tok('.[1]', [(2,3,'<NUM>','1')])

        self.tok('0', [(0,1,'<NUM>','0')])
        self.tok('0.', [(0,1,'<NUM>','0')])
        self.tok('-1', [(0,2,'<NUM>','-1')])
        self.tok('+1', [(1,2,'<NUM>','1')])
        self.tok('+12.', [(1,3,'<NUM>','12')])
        self.tok('-12.', [(0,3,'<NUM>','-12')])
        self.tok('+.1', [(1,3,'<NUM>','.1')])
        self.tok('+.1.', [(1,3,'<NUM>','.1')])
        self.tok('-.1', [(0,3,'<NUM>','-.1')])
        self.tok('-.1.', [(0,3,'<NUM>','-.1')])
        self.tok('-1.5abc', [(0,4,'<NUM>','-1.5'), (4,7,'<ALPHANUM>','abc')])
        self.tok('-123.4567-abc', [(0,9,'<NUM>','-123.4567'), (10,13,'<ALPHANUM>','abc')])


### SUPPORT ###
def _nameOfTestFile():
    # Can't naively use __file__ because it changes from 'test_analysis.py'
    # to 'test_analysis.py[c|o]' upon bytecode compilation, and the bytecode
    # file isn't suitable for [ASCII-oriented] tokenization.
    return test_base.getFilenameOfThisPythonTestModule(__file__)


def _textOfTestFile():
    return file(_nameOfTestFile(), 'rb').read()


if __name__ == '__main__':
    import test
    test.main(suite=getFullTestSuite(), createTestIndex=False)
