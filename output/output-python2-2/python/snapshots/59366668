#!/usr/bin/env python
#-*- coding: utf-8 -*-

import sys
import unittest
import os
import time

sys.path.append('../modules')

from socket_module import Socket

class TestSocket(unittest.TestCase):
    def setUp(self):
        i, o = os.popen2("python server_echo.py")
        time.sleep(1)

    def testData(self):
        s = Socket()
        s.connect("localhost", 6666)
        s.write("prova")
        time.sleep(1)
        self.assert_(s.read() == "prova\n")

if __name__ == '__main__':
    unittest.main()
