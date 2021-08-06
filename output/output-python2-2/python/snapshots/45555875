#/usr/bin/env python
# -*- coding: koi8-r -*-
# clients.py: clients library
# Pythy <the.pythy@gmail.com>

import time

from zope.interface import Interface, Attribute, implements

from twisted.python import log

from pysqlite2 import dbapi2 as pysqlite

class IClient(Interface):
    """An object that returns info about client"""

    encoding = Attribute("Encoding of client's backend")

    def getClient(agreem_number):
        """Returns an info about client"""


class DummyClient(object):
    """Dummy client for testing purposes"""
    implements(IClient)

    def __init__(self, encoding=None):
        self.encoding = encoding
        self.pause_time = 0

    def getClient(self, agreem_num):
        res = 'Dummy_%s' % agreem_num
        # в описании было одно требование: чтобы возвращаемы результат
        # не был длиннее 20 символов
        if len(res) > 20:
            res = res[:20]
        # для имитации задержки выборки данных
        time.sleep(self.pause_time)

        return res


class UnicodeDummyClient(DummyClient):
    """Dummy client for testing purposes with I/O data in unicode"""
    def getClient(self, agreem_num):
        assert isinstance(agreem_num, unicode)
        res = u'Dummy_%s' % agreem_num
        if len(res) > 20:
            res = res[:20]
        # для имитации задержки выборки данных
        time.sleep(self.pause_time)

        return res        
