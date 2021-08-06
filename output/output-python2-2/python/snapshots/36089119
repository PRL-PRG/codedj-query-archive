import sys
import time
import getpass
import unittest

from pydelicious import DeliciousAPI


class ApiSystemTest(unittest.TestCase):
	"""Base class for API system tests.

	Derive API system tests from this baseclass.
	"""

	def setUpApi(self, codec='latin-1'):
		"Setup API with custom encoding."
		if usr == '' or pwd == '':
			self.api = None;
		else:
			self.api = DeliciousAPI(usr, pwd, codec)

	def setUp(self):
		"Default setUp before each ApiSystemTest testcase"
		return self.setUpApi()

	def testInstance(self):
		"Test for valid DeliciousAPI instance"
		self.assert_(isinstance(self.api, DeliciousAPI))

	# precautions so we don't query to quickly:

	def _get_post(self, url):
		p = self.api.posts_get(url=url)
		time.sleep(5)
		tries = 3
		while tries:
			p = self.api.posts_get(url=url)
			if p['posts'] == []:
				tries  = tries - 1
			else:
				return p
		return p

	def _get_nonpost(self, url):
		p = self.api.posts_get(url=url)
		time.sleep(5)
		tries = 3
		while tries:
			p = self.api.posts_get(url=url)
			if p['posts'] == []:
				return p
			else:
				tries  = tries - 1
		return p

def get_credentials():
	if len(sys.argv)>1 and sys.argv[1][0:4]=="--p=":
		user, passwd = sys.argv[1][4:].split(":")
		if passwd == "":
			passwd = user
		sys.argv.pop(1)
	else:
		print
		print "Enter del.icio.us test account login (hit return to skip API tests)"
		user = raw_input("Username: ")
		if user:
			passwd = getpass.getpass("Password for %s: " % user)
		else:
			passwd = ""

	if not user:
		print "Continuing without test account!"
	else:
		print "Continuing with test account '%s'" % user
	print

	return user, passwd

# Get credentials just once...
usr, pwd = get_credentials()

