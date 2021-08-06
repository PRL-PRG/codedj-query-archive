import unittest
import urllib
import pydelicious
import time


def api_request_dummy(path, params='', user='', passwd=''):

    """Instead of mimicking the server responses this will return a tuple
    including the url.
    """

    if params:
        # params come as a dict => dict0 => urlencode
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

class TestPost(unittest.TestCase):

    def testpost1(self):
        t0 = [dict(href = "h"), dict(dt = "dt"), dict(time = "time"), dict(tag = "tag"), dict(tags = "tags")]
        t1 = [{'count': '', 'extended': '', 'hash': '', 'description': '', 'href': 'h', 'tags': '', 'user': '', 'dt': ''},
              {'count': '', 'extended': '', 'hash': '', 'description': '', 'href': '', 'tags': '', 'user': '', 'dt': 'dt'},
              {'count': '', 'extended': '', 'hash': '', 'description': '', 'href': '', 'tags': '', 'user': '', 'dt': 'time'},
              {'count': '', 'extended': '', 'hash': '', 'description': '', 'href': '', 'tags': 'tag', 'user': '', 'dt': ''},
              {'count': '', 'extended': '', 'hash': '', 'description': '', 'href': '', 'tags': 'tags', 'user': '', 'dt': ''}]
        for i in range(len(t0)):
            self.assert_(pydelicious.post(**t0[i]) == t1[i])

    def testpost2(self):
        t0 = [dict(href = "h"), dict(dt = "dt"), dict(time = "time"), dict(tag = "tag"), dict(tags = "tags")]
        t2 = [dict(href = "h"), dict(dt = "dt"), dict(dt = "time"), dict(tags = "tag"), dict(tags = "tags")]
        for i in range(len(t0)):
            self.assert_(pydelicious.dict0(pydelicious.post(**t0[i])) == t2[i])

    def testpost3(self):
        t0 = [pydelicious.post(href="href").href, pydelicious.post(href="href")["href"],
              pydelicious.post(user="user").user, pydelicious.post(user="user")["user"],
              pydelicious.post().user, pydelicious.post()["user"]]
        t1 = ["href", "href", "user", "user", "", ""]
        for i in range(len(t0)):
              self.assert_(t0[i] == t1[i])


class TestPosts(unittest.TestCase):

    def testposts1(self):
        p  = pydelicious.post
        pp = pydelicious.posts
        t0 = [pp(p(href="href"), p(href="href2"))]
        t1 = [['href', 'href2']]
        for i in range(len(t0)):
            self.assert_(t0[i].href == t1[i])

class TestGetrss(unittest.TestCase):

    def testsimpleruns(self):
        p = pydelicious.getrss
        self.assert_(p()!={})
        self.assert_(p(popular=1)!={})
        self.assert_(p(tag="python")!={})
        self.assert_(p(tag="öko")!={})
        self.assert_(p(user="pydelicious")!={})
        self.assert_(p(url="http://deliciouspython.python-hosting.com/")!={})

class TestBug(unittest.TestCase):

    def testBug2(self):
        '''testbug2: via deepak.jois@gmail.com
        missing "" in {"user":user}'''
        self.assertEqual(
            type(pydelicious.getrss(tag="read",user="deepakjois")),
            type(pydelicious.posts()))


class DeliciousApiUnitTest(unittest.TestCase):

    """Simply tests wether DeliciousAPI.request(`path`, `args`) results in the same URL as
    DeliciousAPI.`path`(`args`)
    """

    def setUp(self):
        self.api_utf8 = pydelicious.DeliciousAPI('testUser', 'testPwd', 'utf-8', api_request=api_request_dummy, xml_parser=parser_dummy)
        self.api_latin1 = pydelicious.DeliciousAPI('testUser', 'testPwd', 'latin-1', api_request=api_request_dummy, xml_parser=parser_dummy)

# TODO
#    def test_encoding(self):
#        a = self.api_utf8
#        a2 = self.api_latin1
#
#        #print "del.icio.us appears to always convert to unicode? > do blackbox API test"
#        #print a.request('tags/rename', old='foo', new='bar')
#        #print a.tags_rename(old='foo', new='bar')
#        #print a2.request('tags/rename', old=u'', new=u'')
#
#        raise NotImplementedError

    def test_fetch_vs_methods(self):
        a = self.api_utf8

        self.assert_(a.request('tags/get') == a.tags_get())
        self.assert_(a.request('tags/rename', old='tag1', new='tag2') == a.tags_rename('tag1', 'tag2'))

        self.assert_(a.request('posts/update') == a.posts_update())
        self.assert_(a.request('posts/dates') == a.posts_dates())
        self.assert_(a.request('posts/get') == a.posts_get())
        self.assert_(a.request('posts/recent') == a.posts_recent())
        self.assert_(a.request('posts/all') == a.posts_all())
        self.assert_(a.request('posts/add', url='url1', description='descr1', replace='no', shared='no') == a.posts_add('url1', 'descr1', replace='no', shared='no'))
        self.assert_(a.request('posts/delete', url='url1') == a.posts_delete('url1'))

        self.assert_(a.request('tags/bundles/all') == a.bundles_all())
        self.assert_(a.request('tags/bundles/set', bundle='bundle1', tags='tag1 tag2') == a.bundles_set('bundle1', 'tag1 tag2'))
        self.assert_(a.request('tags/bundles/delete', bundle='bundle1') == a.bundles_delete('bundle1'))

    def test_fetch_raw_vs_methods(self):
        a = self.api_utf8

        self.assert_(a.request_raw('tags/get') == a.tags_get(_raw=True))
        self.assert_(a.request_raw('tags/rename', old='tag1', new='tag2') == a.tags_rename('tag1', 'tag2', _raw=True))

        self.assert_(a.request_raw('posts/update') == a.posts_update(_raw=True))
        self.assert_(a.request_raw('posts/dates') == a.posts_dates(_raw=True))
        self.assert_(a.request_raw('posts/get') == a.posts_get(_raw=True))
        self.assert_(a.request_raw('posts/recent') == a.posts_recent(_raw=True))
        self.assert_(a.request_raw('posts/all') == a.posts_all(_raw=True))
        self.assert_(a.request_raw('posts/add', url='url1', description='descr1', replace='no', shared='no') == a.posts_add('url1', 'descr1', replace='no', shared='no', _raw=True))
        self.assert_(a.request_raw('posts/delete', url='url1') == a.posts_delete('url1', _raw=True))

        self.assert_(a.request_raw('tags/bundles/all') == a.bundles_all(_raw=True))
        self.assert_(a.request_raw('tags/bundles/set', bundle='bundle1', tags='tag1 tag2') == a.bundles_set('bundle1', 'tag1 tag2', _raw=True))
        self.assert_(a.request_raw('tags/bundles/delete', bundle='bundle1') == a.bundles_delete('bundle1', _raw=True))


__testcases__ = (TestWaiter, TestPosts, TestPost, TestGetrss, TestBug,
              DeliciousApiUnitTest)

if __name__ == '__main__': unittest.main()
