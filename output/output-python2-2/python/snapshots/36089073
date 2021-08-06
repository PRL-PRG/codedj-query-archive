import unittest
import urllib
import pydelicious
import time


def api_request_dummy(path, params='', user='', passwd=''):

    """Instead of mimicking the server responses this will return a tuple
    including the url.
    """

    if params:
        url = "%s/%s?%s" % (pydelicious.DLCS_API, path, urllib.urlencode(params))
    else:
        url = "%s/%s" % (pydelicious.DLCS_API, path)
    return url, user, passwd

def parser_dummy(data, split_tags=False):
    return {'not-parsed':data}


class TestWaiter(unittest.TestCase):

    def testwait1(self):
        wt = pydelicious.DLCS_WAIT_TIME

        # First call, no wait needed
        t = time.time()
        pydelicious.Waiter()
        waited = round(time.time() - t, 1)
        self.assert_(waited < wt,
                "unneeded wait of %s" % (waited,))

        # Some values between full wait intervals
        for w in .4, .7, 1.5:
            time.sleep(w)
            t = time.time()
            pydelicious.Waiter()
            waited = round(time.time() - t, 1)
            self.assert_(waited <= pydelicious.DLCS_WAIT_TIME,
                    "unneeded wait of %s (not %s)" % (w,
                        pydelicious.DLCS_WAIT_TIME-w))

        # Some more regular intervals
        t = time.time()
        for i in range(0, 2):
            pydelicious.Waiter()
            waited = time.time() - t
            self.assert_(waited >= i*wt,
                    "needed wait of %s, not %s" % (i*wt, waited,))


class TestGetrss(unittest.TestCase):

    "run getrss against server"

    def testsimpleruns(self):
        p = pydelicious.getrss
        self.assert_(p()!={})
        self.assert_(p(popular=1)!={})
        self.assert_(p(tag="python")!={})
        self.assert_(p(tag="oko")!={})
        self.assert_(p(user="pydelicious")!={})
        self.assert_(p(url="http://deliciouspython.python-hosting.com/")!={})


class TestBug(unittest.TestCase):

    def testBug2(self):
        '''testbug2: via deepak.jois@gmail.com
        missing "" in {"user":user}'''
        self.assertEqual(
            type(pydelicious.getrss(tag="read",user="deepakjois")),
            type([]))


class DeliciousApiUnitTest(unittest.TestCase):

    """Simply tests wether DeliciousAPI.request(`path`, `args`) results in the same URL as
    DeliciousAPI.`path`(`args`)
    """

    def setUp(self):
        self.api_utf8 = pydelicious.DeliciousAPI('testUser', 'testPwd', 
            'utf-8', api_request=api_request_dummy, xml_parser=parser_dummy)

        self.api_latin1 = pydelicious.DeliciousAPI('testUser', 'testPwd', 
            'latin-1', api_request=api_request_dummy, xml_parser=parser_dummy)

    def test_param_encoding(self):
        a = self.api_utf8
        params = {
            'foo': '\xe2\x98\x85',
            'bar': '\xc3\xa4'
        }
        params = a._encode_params(params)
        self.assert_('foo=%E2%98%85' in urllib.urlencode(params))
        self.assert_('bar=%C3%A4' in urllib.urlencode(params))

        a = self.api_latin1
        params = {
            'bar': '\xe4',
            'baz': '\xa4'
        }
        params = a._encode_params(params)
        self.assert_('bar=%C3%A4' in urllib.urlencode(params))
        self.assert_('baz=%C2%A4' in urllib.urlencode(params))

    def test_fetch_vs_methods(self):
        a = self.api_utf8

        self.assertEqual(a.request('tags/get'), a.tags_get())
        self.assertEqual(a.request('tags/rename', old='tag1', new='tag2'), a.tags_rename('tag1', 'tag2'))

        self.assertEqual(a.request('posts/update'), a.posts_update())
        self.assertEqual(a.request('posts/dates'), a.posts_dates())
        self.assertEqual(a.request('posts/get', meta='yes'), a.posts_get())
        self.assertEqual(a.request('posts/get', meta=True), a.posts_get())
        self.assertEqual(a.request('posts/recent'), a.posts_recent())
        self.assertEqual(a.request('posts/all', meta=True), a.posts_all())
        self.assertEqual(a.request('posts/add', url='url1', description='descr1', replace='no', shared='no'), a.posts_add('url1', 'descr1', replace='no', shared='no'))
        self.assertEqual(a.request('posts/delete', url='url1'), a.posts_delete('url1'))

        self.assertEqual(a.request('tags/bundles/all'), a.bundles_all())
        self.assertEqual(a.request('tags/bundles/set', bundle='bundle1', tags='tag1 tag2'), a.bundles_set('bundle1', 'tag1 tag2'))
        self.assertEqual(a.request('tags/bundles/delete', bundle='bundle1'), a.bundles_delete('bundle1'))

    def test_fetch_raw_vs_methods(self):
        a = self.api_utf8

        self.assertEqual(a.request_raw('tags/get'), a.tags_get(_raw=True))
        self.assertEqual(a.request_raw('tags/rename', old='tag1', new='tag2'), a.tags_rename('tag1', 'tag2', _raw=True))

        self.assertEqual(a.request_raw('posts/update'), a.posts_update(_raw=True))
        self.assertEqual(a.request_raw('posts/dates'), a.posts_dates(_raw=True))
        self.assertEqual(a.request_raw('posts/get', meta=True), a.posts_get(_raw=True))
        self.assertEqual(a.request_raw('posts/get', meta='yes'), a.posts_get(_raw=True))
        self.assertEqual(a.request_raw('posts/recent'), a.posts_recent(_raw=True))
        self.assertEqual(a.request_raw('posts/all', meta=True), a.posts_all(_raw=True))
        self.assertEqual(a.request_raw('posts/add', url='url1', description='descr1', replace='no', shared='no'), a.posts_add('url1', 'descr1', replace='no', shared='no', _raw=True))
        self.assertEqual(a.request_raw('posts/delete', url='url1'), a.posts_delete('url1', _raw=True))

        self.assertEqual(a.request_raw('tags/bundles/all'), a.bundles_all(_raw=True))
        self.assertEqual(a.request_raw('tags/bundles/set', bundle='bundle1', tags='tag1 tag2'), a.bundles_set('bundle1', 'tag1 tag2', _raw=True))
        self.assertEqual(a.request_raw('tags/bundles/delete', bundle='bundle1'), a.bundles_delete('bundle1', _raw=True))


__testcases__ = (TestGetrss, TestBug, DeliciousApiUnitTest, TestWaiter, )

if __name__ == '__main__': unittest.main()
