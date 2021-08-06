# -*- coding: cp1252 -*-
'''Ich brauche mehr tests, so wie in feedparser aufgezogen?'''

##
# License: pydelicious is released under the bsd license. 
# See 'license.txt' for more informations.
#

import unittest
import pydelicious
import time
reload(pydelicious)

class TestWaiter(unittest.TestCase):

    def testwait1(self):
        pydelicious.Waiter.wait()
        t = time.time()
        pydelicious.Waiter.wait()
        self.assert_(time.time()-t>=0.9)
        pydelicious.Waiter.wait()
        self.assert_(time.time()-t>=1.9)


class TestHelperFunctions(unittest.TestCase):

    def setUp(self):
        pass
    
    def tearDown(self):
        pass
    
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

class TestApiCalls(unittest.TestCase):

    def setUp(self):
        self.a = pydelicious.DeliciousAPI(user, passwd)

    def test_tags_get(self):
        self.a.tags_get()

    def test_tags_rename(self):
        self.a.tags_rename("tag", "taag")        
        self.a.tags_rename("taag", "tag")        

    def test_posts_update(self):
        self.a.posts_update()

    def test_posts_dates(self):
        self.a.posts_dates()

    def test_post_get(self):
        self.a.posts_get(tag="akjs")

    def test_posts_recent(self):
        self.a.posts_recent()

    def test_posts_all(self):
        self.a.posts_all()

    def test_posts_add(self):
        self.a.posts_add("http://url.de/", "desc")
        self.a.posts_delete("http://url.de/")


class TestAdd(unittest.TestCase):

    def testadd1(self):
        if not (user  and passwd): return ""
        p = pydelicious.add
        self.assert_(p(user, passwd, "http://www.testurl.de/", "description", tags=u"täg tag tuck", replace="yes")==1)
        self.assert_(p(user, passwd, "http://www.testurl.de/", "description", tags=u"täg tag tuck")==0)

class TestDelete(unittest.TestCase):

    def testdelete1(self):
        if not (user  and passwd): return ""
        p = pydelicious.delete
        self.assert_(p(user, passwd, "http://www.testurl.de/")==1)
        self.assert_(p(user, passwd, "http://www.testurl.de/")==1)


class TestGetrss(unittest.TestCase):

    def testsimlperuns(self):
        p = pydelicious.getrss
        self.assert_(p()!={})
        self.assert_(p(popular=1)!={})
        self.assert_(p(tag="python")!={})
        self.assert_(p(tag="öko")!={})
        self.assert_(p(user="pydelicious")!={})
        self.assert_(p(url="http://deliciouspython.python-hosting.com/")!={})

class TestBug(unittest.TestCase):
    
    def testBug2(self):
        '''via deepak.jois@gmail.com
        missing "" in {"user":user}'''
        self.assertEqual(
            type(pydelicious.getrss(tag="read",user="deepakjois")),
            type(pydelicious.posts()))

if __name__ == '__main__':
    import sys
    if len(sys.argv)>1 and sys.argv[1][0:4]=="--p=":
        user, passwd = sys.argv[1][4:].split(":")
        if passwd =="": passwd = user
        sys.argv.pop(1)
    else:
        user = raw_input("Username (hit return to skip API test):")
        if user:  passwd = raw_input("Passwd (hit return to skip API test):")
        else: passwd = ""
    unittest.main()
