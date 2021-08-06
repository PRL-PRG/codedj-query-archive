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
            self.creds = usr, pwd
            self.api = DeliciousAPI(usr, pwd, codec)

    def setUp(self):
        "Default setUp before each ApiSystemTest testcase"
        return self.setUpApi()


class TestApiCalls(ApiSystemTest):

    """Test wether simply calling stuff goes smoothly
    """

    def test_tags_get(self):
        self.api.tags_get()

    def test_tags_rename(self):
        self.api.tags_rename("tag", "taag")
        self.api.tags_rename("taag", "tag")

    def test_posts_update(self):
        self.api.posts_update()

    def test_posts_dates(self):
        self.api.posts_dates()

    def test_post_get(self):
        self.api.posts_get(tag="akjs")

    def test_posts_recent(self):
        self.api.posts_recent()

    def test_posts_all(self):
        self.api.posts_all()

    def test_posts_add(self):
        self.api.posts_add("http://url.de/", "desc")
        self.api.posts_delete("http://url.de/")

def get_credentials():
    if len(sys.argv)>1 and sys.argv[1][0:4]=="--p=":
        user, passwd = sys.argv[1][4:].split(":")
        if passwd == "":
            passwd = user
        sys.argv.pop(1)
    else:
        print "Enter del.icio.us test account login (hit return to skip API tests)"
        user = raw_input("Username: ")
        if user:
            passwd = getpass.getpass("Password for %s: " % user)
        else:
            passwd = ""

    return user, passwd

# Get credentials, just once, save on module.
usr, pwd = get_credentials()

if __name__ == '__main__': unittest.main()
