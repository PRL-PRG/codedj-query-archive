# -*- coding: utf-8 -*-
import unittest
import pydelicious


class TestHelperFunctions(unittest.TestCase):

    def teststr2uni(self):
        t = {'a':u'a', u'a':u'a', 'ä':u'\xe4', u'ä':u'\xe4'}
        [self.assert_(pydelicious.str2uni(i) == t[i]) for i in t]

    def teststr2utf8(self):
        t = {'a':'a', u'a':'a', 'ä':'\xc3\xa4', u'ä':'\xc3\xa4'}
        [self.assert_(pydelicious.str2utf8(i) == t[i]) for i in t]

    def testdict0(self):
        t0 = [{"a":"a", "b":"", "c":"c", "d":"", "e":" "}]
        t1 = [{"a":"a", "c":"c", "e":" "}]
        [self.assert_(pydelicious.dict0(t0[i]) == t1[i]) for i in range(len(t0))]


class TestGetrss(unittest.TestCase):

    def testsimpleruns(self):
        p = pydelicious.getrss
        self.assert_(p(tag="öko")!={})


__testcases__ = (TestGetrss, TestHelperFunctions,)

if __name__ == '__main__': unittest.main()
