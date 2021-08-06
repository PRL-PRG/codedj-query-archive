from twisted.trial import unittest
from TwistedPythy import clients

class DummyClientTestCase(unittest.TestCase):

    def setUp(self):
        self.client = clients.DummyClient()

    def test_getDummyClientStr(self):
        self.assertEquals('Dummy_10', self.client.getClient('10'))

    def test_getDummyStrippedClient(self):
        # line with length >20 must bet stripped
        self.assertEquals('Dummy_1234567890abcd', self.client.getClient('1234567890abcdefgh'))

    def test_getDummyClientUnicode(self):
        self.assertEquals('Dummy_10', self.client.getClient(u'10'))

class UnicodeDummyClientTestCase(unittest.TestCase):

    def setUp(self):
        self.client = clients.UnicodeDummyClient()

    def test_getDummyClientUnicode(self):
        self.assertEquals(u'Dummy_10', self.client.getClient(u'10'))

    def test_getDummyStrippedClient(self):
        # line with length >20 must bet stripped
        self.assertEquals(u'Dummy_1234567890abcd', self.client.getClient(u'1234567890abcdefgh'))

    def test_getDummyClientStr(self):
        # str instead of unicode raises AssertError
        self.assertRaises(AssertionError, self.client.getClient, '10')
