# -*- coding: utf-8 -*-
import unittest
import urllib

from pydelicious import DeliciousAPI, dict0, DLCS_API


def http_request_dummy(path, params='', user='', passwd=''):
	if params:
		# params come as a dict => dict0 => urlencode
		url = "%s/%s?%s" % (DLCS_API, path, urllib.urlencode(params))
	else:
		url = "%s/%s" % (DLCS_API, path)
	return url, user, passwd

def parser_dummy(data, split_tags=False):
	return {'parsed':data}

class DeliciousApiUnitTest(unittest.TestCase):
	def setUp(self):
		self.api_utf8 = DeliciousAPI('testUser', 'testPwd', 'utf-8', api_request=http_request_dummy, xml_parser=parser_dummy)
		self.api_utf8._test_mode = True
		self.api_latin1 = DeliciousAPI('testUser', 'testPwd', 'latin-1', api_request=http_request_dummy, xml_parser=parser_dummy)
		self.api_latin1._test_mode = True

	def test_encoding(self):
		a = self.api_utf8
		a2 = self.api_latin1

		print "del.icio.us appears to always convert to unicode? > do blackbox API test"
		print #a.request('tags/rename', old=u'★', new=u'☆')
		print a.request('tags/rename', old=u'à', new=u'á')
		print a2.request('tags/rename', old=u'à', new=u'á')

		raise NotImplementedError


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


if __name__ == '__main__':
	unittest.main()

