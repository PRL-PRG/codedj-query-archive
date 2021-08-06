# -*- coding: cp1252 -*-
import unittest
import time
import blackbox_api


class ApiLatin1Test(blackbox_api.ApiSystemTest):
    def setUp(self):
        super(ApiLatin1Test, self).setUpApi('latin-1')
        self.post = (
            "urn:pydelicious/systemtest#ApiLatin1Test",
            "ApiUnicodeTest description",
            "ApiUnicodeTest",
            u"tests tåg tãg tâg"
        )

    def test1AddPost(self):
        a = self.api
        url, descr, extd, tags = self.post

        # Add post
        v = a.posts_add(url, descr, tags=tags, extended=extd, shared="no")
        self.assert_(v['result'][0],
                "Unexpected response to posts_delete: %s" % v)

    def test2JustPosted(self):
        a = self.api
        url, descr, extd, tags = self.post

        p = self._get_post(url)
        self.assert_(len(p['posts']) == 1,
                "URL does not appear in collection after posts_add")

        # Check post
        post = p['posts'][0]
        self.assert_(post['href'] == url)
        self.assert_(post['shared'] == 'no')
        self.assert_(post['tag'] == tags)
        self.assert_(post['description'] == descr)
        self.assert_(post['extended'] == extd)

    def test3DeletePost(self):
        a = self.api
        url, descr, extd, tags = self.post

        # Delete post
        v = a.posts_delete(url)
        self.assert_(v['result'][0],
                "Unexpected response to posts_delete: %s" % v)

        # Check post
        p = a.posts_get(url)
        self.assert_(p['posts'] == [],
                "Posted URL did not dissappear after posts_delete")


if __name__ == '__main__': unittest.main()
