# Since it is thought that it would not be easy to extend the C++ members of
# lucene::queryParser in Python, that package is wrapped only minimally, and
# this test suite is limited in scope.

import sets, string, sys, unittest

import test_base
from pyclene import lucene


_UNPRINTABLE_ASCII = sets.Set([chr(i) for i in range(128)]) - sets.Set(string.printable)


def getFullTestSuite():
    suite = unittest.TestSuite()

    suite.addTest( unittest.makeSuite(QueryParserTest) )

    return suite


class QueryParserTest(test_base.CommonBaseTest):
    def testConstructor(self):
        a = lucene.StandardAnalyzer()

        # The field name and analyzer are not allowed to be NULL:
        self.assertRaises(TypeError, lucene.QueryParser, 'contents', None)
        self.assertRaises(TypeError, lucene.QueryParser, None, a)
        # But the field name may be the empty string:
        lucene.QueryParser('', a)

        lucene.QueryParser('contents', a)
        lucene.QueryParser('biglongfieldname'*1024, a)

        # Ensure that a reference to the Python Analyzer proxy is retained at
        # the Python level long enough so that the QueryParser never holds
        # an invalid Analyzer reference.
        qp = lucene.QueryParser('contents', a)
        del a
        qp.parse('+jack +london')


    def _allowButDoNotRequireExceptionFor(self,
        queryInputs=[''],
        fieldInputs=['contents'],
        allowedExceptionTypes=[Exception]
      ):
        a = lucene.StandardAnalyzer()
        for qi in queryInputs:
            for fi in fieldInputs:
                try:
                    lucene.QueryParser.parse(qi, fi, a)
                except:
                    exType, ex = sys.exc_info()[:2]
                    if exType not in allowedExceptionTypes:
                        raise ex


    # Empty string:
    def testParsingEmptyString(self):
        # Make sure an attempt to parse the empty string doesn't segfault.
        self._allowButDoNotRequireExceptionFor(queryInputs=[''])

    def testParsingWithFieldName_EmptyString(self):
        # Make sure an attempt to parse with an empty field name doesn't segfault.
        self._allowButDoNotRequireExceptionFor(queryInputs=['blah'], fieldInputs=[''])


    # Printable chars and null char:
    def testParsingASCIIPrintableCharsAndNull(self):
        self._allowButDoNotRequireExceptionFor(queryInputs='\0' + string.printable)

    def testParsingWithFieldName_ASCIIPrintableCharsAndNull(self):
        self._allowButDoNotRequireExceptionFor(fieldInputs='\0' + string.printable)


    # Unprintable chars:
    def testParsingASCIIUnprintableChars(self):
        self._allowButDoNotRequireExceptionFor(queryInputs=_UNPRINTABLE_ASCII)

    def testParsingWithFieldName_ASCIIUnprintableChars(self):
        self._allowButDoNotRequireExceptionFor(fieldInputs=_UNPRINTABLE_ASCII)


    # Unbalanced:
    def testParsingParenthesizedClauseWithoutMatchingParenthesis(self):
        # Under these circumstances, CLucene leaked a BooleanQuery instance
        # until DSR fixed the leak on 2004.11.01.
        self._allowButDoNotRequireExceptionFor(queryInputs='()')


    def testParsingStopWordThatDisruptsBooleanClauseBalance(self):
        # This test case was added on 2004.11.01 in response to SF bug report
        # 1051984.
        a = lucene.StandardAnalyzer()
        def q(queryString, parsedString):
            self.assertEqual(
                lucene.QueryParser.parse(queryString, 'contents', a).toString('contents'),
                parsedString
              )

        # 'a' is a stopword, so it should be eliminated by the StandardFilter.
        # 'b' should remain.

        q('a AND b',         '+b'  ) # Arguably, the query parser should collapse
                                     # this one into a TermQuery rather than a
                                     # BooleanQuery.
        q('b AND a',         'b'   )
        q('  AND a',         ''    )
        q('a AND  ',         ''    )
        q('  AND b',         'b'   )
        q('b AND  ',         'b'   )

        q('a OR b',          'b'   )
        q('b OR a',          'b'   )
        q('  OR a',          ''    )
        q('a OR  ',          ''    )
        q('b OR  ',          'b'   )
        q('  OR b',          'b'   )


if __name__ == '__main__':
    import test
    test.main(suite=getFullTestSuite(), createTestIndex=False)