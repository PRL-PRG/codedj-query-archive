from twisted.trial import unittest
from twisted.internet import protocol, defer

from TwistedPythy import clients, proto

from twisted.test.test_protocols import StringIOWithoutClosing as SIOWOC

class TestPythyProto(proto.PythyProto):
    def __init__(self):
        self.debug = False

class TestAsyncPythyProto(proto.AsyncPythyProto):
    def __init__(self):
        self.deferred = defer.Deferred()
        self.debug = False

    def sendAnswer(self, *args, **kwargs):
        proto.AsyncPythyProto.sendAnswer(self, *args, **kwargs)
        self.deferred.callback('answer sent')

class TestAsyncUnicodePythyProto(proto.AsyncUnicodePythyProto):
    """ AsyncUnicodePythyProto for unittests"""
    def __init__(self):
        self.deferred = defer.Deferred()
        self.debug = False

    def sendAnswer(self, *args, **kwargs):
        proto.AsyncUnicodePythyProto.sendAnswer(self, *args, **kwargs)
        self.deferred.callback('answer sent')



class PythyProtoTestCase(unittest.TestCase):

    def setUp(self):
        self.p = TestPythyProto()
        self.c = clients.DummyClient()
        self.f = proto.PythyFactory(self.c, 'koi8-r')
        self.p.factory = self.f

    def test_sendLineUnicode(self):
        s = SIOWOC()
        self.p.makeConnection(protocol.FileWrapper(s))
        self.p.sendLine(u'test')
        self.assertEquals('test\r\n', s.getvalue())

    def test_sendLineStr(self):
        s = SIOWOC()
        self.p.makeConnection(protocol.FileWrapper(s))
        self.p.sendLine('test')
        self.assertEquals('test\r\n', s.getvalue())

    def test_sendAnswerUnicode(self):
        s = SIOWOC()
        self.p.makeConnection(protocol.FileWrapper(s))
        self.p.sendAnswer(u'test')
        self.assertEquals('dummypacketmaker012345678test\r\n', s.getvalue())

    def test_sendAnswerStr(self):
        s = SIOWOC()
        self.p.makeConnection(protocol.FileWrapper(s))
        self.p.sendAnswer('test')
        self.assertEquals('dummypacketmaker012345678test\r\n', s.getvalue())

    def test_lineReceived(self):
        req = '0123456789abcdefghijk'
        ans = 'dummypacketmaker012345678Dummy_abcde\r\n'
        s = SIOWOC()
        self.p.makeConnection(protocol.FileWrapper(s))
        self.p.lineReceived(req)
        self.assertEquals(ans, s.getvalue())


class AsyncPythyProtoTestCase(unittest.TestCase):

    def setUp(self):
        self.p = TestAsyncPythyProto()
        self.c = clients.DummyClient()
        self.f = proto.AsyncPythyFactory(self.c, 'koi8-r')
        self.p.factory = self.f

    def cbCheckReceived(self, res, data):
        orig_data, pseudo_handler = data
        self.assertEquals(orig_data, pseudo_handler.getvalue())

    def test_sendLineUnicode(self):
        s = SIOWOC()
        self.p.makeConnection(protocol.FileWrapper(s))
        self.p.sendLine(u'test')
        self.assertEquals('test\r\n', s.getvalue())

    def test_sendLineStr(self):
        s = SIOWOC()
        self.p.makeConnection(protocol.FileWrapper(s))
        self.p.sendLine('test')
        self.assertEquals('test\r\n', s.getvalue())

    def test_sendAnswerUnicode(self):
        s = SIOWOC()
        self.p.makeConnection(protocol.FileWrapper(s))
        self.p.sendAnswer(u'test')
        self.assertEquals('dummypacketmaker012345678test\r\n', s.getvalue())

    def test_sendAnswerStr(self):
        s = SIOWOC()
        self.p.makeConnection(protocol.FileWrapper(s))
        self.p.sendAnswer('test')
        self.assertEquals('dummypacketmaker012345678test\r\n', s.getvalue())

    def test_lineReceived(self):
        req = '0123456789abcdefghijk'
        ans = 'dummypacketmaker012345678Dummy_abcde\r\n'
        s = SIOWOC()
        self.p.makeConnection(protocol.FileWrapper(s))
        self.p.lineReceived(req)
        self.p.deferred.addCallback(self.cbCheckReceived, (ans, s))
        return self.p.deferred


class AsyncUnicodePythyProtoTestCase(unittest.TestCase):

    def setUp(self):
        self.p = TestAsyncUnicodePythyProto()
        self.c = clients.UnicodeDummyClient()
        self.f = proto.AsyncUnicodePythyFactory(self.c, 'koi8-r')
        self.p.factory = self.f

    def cbCheckReceived(self, res, data):
        orig_data, pseudo_handler = data
        self.assertEquals(orig_data, pseudo_handler.getvalue())

    def test_sendLineUnicode(self):
        s = SIOWOC()
        self.p.makeConnection(protocol.FileWrapper(s))
        self.p.sendLine(u'test')
        self.assertEquals('test\r\n', s.getvalue())

    def test_sendLineStr(self):
        s = SIOWOC()
        self.p.makeConnection(protocol.FileWrapper(s))
        self.assertRaises(AssertionError, self.p.sendLine, 'test')

    def test_sendAnswerUnicode(self):
        s = SIOWOC()
        self.p.makeConnection(protocol.FileWrapper(s))
        self.p.sendAnswer(u'test')
        self.assertEquals('\xd4\xc5\xd3\xd4test\r\n', s.getvalue())

    def test_sendAnswerStr(self):
        s = SIOWOC()
        self.p.makeConnection(protocol.FileWrapper(s))
        self.assertRaises(AssertionError, self.p.sendAnswer, 'test')

    def test_lineReceived(self):
        req = '0123456789abcdefghijk'
        ans = '\xd4\xc5\xd3\xd4Dummy_abcde\r\n'
        s = SIOWOC()
        self.p.makeConnection(protocol.FileWrapper(s))
        self.p.lineReceived(req)
        self.p.deferred.addCallback(self.cbCheckReceived, (ans, s))
        return self.p.deferred
