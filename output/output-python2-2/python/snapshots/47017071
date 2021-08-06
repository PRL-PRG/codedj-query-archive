#!/usr/bin/env python

import unittest

import dejumble.organizers.iso9660
from dejumble.organizers.iso9660 import *

class ISO9660OrganizerTestCase(unittest.TestCase):
    def setUp(self):
        self.o = ISO9660Organizer(None)
    
    def testincreasefilename(self):
        self.assertEquals(self.o.increasefilename("A.TXT"), "A~1.TXT")
        self.assertEquals(self.o.increasefilename("A~1.TXT"), "A~2.TXT")
        self.assertEquals(self.o.increasefilename("123456~9.TXT"), "12345~10.TXT")

    def test_path(self):
        o = ISO9660Organizer(None)
        self.assertEquals(self.o._path("a.txt"), "A.TXT")
        self.assertEquals(self.o._path("1+2 345.txt"), "1_2345.TXT")
        self.assertEquals(self.o._path("1234567890.text"), "123456~1.TEX")
