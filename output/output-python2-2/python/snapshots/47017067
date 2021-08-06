#!/usr/bin/env python

import os
import unittest

import dejumble.util
from dejumble.util import *

class UtilTestCase(unittest.TestCase):
    def testpathparts(self):
        self.assertEquals(len(pathparts('')), 0)
        self.assertEquals(len(pathparts('/')), 1)
        parts = pathparts('/a/b/c')
        self.assertEquals(len(parts), 3)
        self.assertEquals(parts[0], 'a')
        self.assertEquals(parts[1], 'b')
        self.assertEquals(parts[2], 'c')
    
    def testflags2mode(self):
        self.assertEqual(flags2mode(os.O_RDONLY), 'r')
        self.assertEqual(flags2mode(os.O_WRONLY), 'w')
        self.assertEqual(flags2mode(os.O_RDWR), 'w+')
        self.assertEqual(flags2mode(os.O_RDONLY), 'r')
        self.assertEqual(flags2mode(os.O_WRONLY | os.O_APPEND), 'a')
        self.assertEqual(flags2mode(os.O_RDWR | os.O_APPEND), 'a+')
    
    def testaddtrailingslash(self):
        self.assertEquals(addtrailingslash('a/b/c'), '/a/b/c')
        self.assertEquals(addtrailingslash('/a/b/c'), '/a/b/c')
        self.assertEquals(addtrailingslash('/'), '/')
        self.assertEquals(addtrailingslash(''), '/')
    
    def testignoretag(self):
        self.assertTrue(ignoretag('abc'))
        self.assertFalse(ignoretag('..'))
        self.assertFalse(ignoretag('.'))
        self.assertFalse(ignoretag('.dejumble'))
    
    def testgetbasefilelist(self):
        self.assertTrue('..' in getbasefilelist())
        self.assertTrue('.' in getbasefilelist())
        self.assertEquals(len(getbasefilelist()), 2)

    def testunique(self):
        list1 = [ 5, 2, 3, 4 ]
        list2 = [ 5, 2, 3, 4, 5, 3 ]
        self.assertEquals(str(sorted(unique(list1))), str(sorted(list1)))
        self.assertEquals(str(sorted(unique(list2))), str(sorted(list1)))
        self.assertNotEquals(str(sorted(unique(list2))), str(sorted(list2)))
 
