#!/usr/bin/env python
#-*- coding: utf-8 -*-

import sys
import os.path
import unittest

sys.path.append('..')

import conf
from modules.storage import Storage

class TestStorage(unittest.TestCase):
    def setUp(self):
        abspath = os.path.abspath('../../resources/storage/dbtest.sqlite')
        conf.config['storage'] = {'path': abspath}

        if os.path.exists(conf.config['storage']['path']):
            os.unlink(conf.config['storage']['path'])
        self.storage = Storage()

    def tearDown(self):
        if os.path.exists(conf.config['storage']['path']):
            os.unlink(conf.config['storage']['path'])

    def testEmptyConnections(self):
        self.assert_(self.storage.connections() == [])

    def testConnections(self):
        conn = [('name','host', 111, 1)]
        self.storage.saveConnections(conn)
        self.assert_(self.storage.connections() == conn)

    def testConnections2(self):
        conn = [('name','host', 111, 1)]
        self.storage.saveConnections(conn)

        conn.extend([('name2','host2', 222, 1)])
        self.storage.saveConnections(conn)
        self.assert_(self.storage.connections() == conn)

    def testConnections3(self):
        conn = [('name','host', 111, 1)]
        self.storage.saveConnections(conn)

        conn = [('test','host', 111, 1)]
        self.storage.saveConnections(conn)
        self.assert_(self.storage.connections() == conn)

    def testConnections4(self):
        conn = [('name1','host1', 111, 0),
                ('name2','host2', 112, 1),
                ('name3','host3', 113, 0)]
        self.storage.saveConnections(conn)
        self.assert_(self.storage.connections() == conn)

    def testConnections5(self):
        conn = [('name1','host1', 111, 0),
                ('name2','host2', 112, 1),
                ('name3','host3', 113, 1)]
        self.storage.saveConnections(conn)

        del conn[1]
        self.storage.saveConnections(conn)
        self.assert_(self.storage.connections() == conn)

class TestStorage2(unittest.TestCase):
    def setUp(self):
        abspath = os.path.abspath('../../resources/storage/dbtest.sqlite')
        conf.config['storage'] = {'path': abspath}

        if os.path.exists(conf.config['storage']['path']):
            os.unlink(conf.config['storage']['path'])

    def tearDown(self):
        if os.path.exists(conf.config['storage']['path']):
            os.unlink(conf.config['storage']['path'])

    def testMultiConnections(self):
        conn = [('name','host', 111, 0)]
        Storage().saveConnections(conn)
        self.assert_(Storage().connections() == conn)

if __name__ == '__main__':
    unittest.main()
