import os
import unittest

from .. import util


class UtilTestCase(unittest.TestCase):

    def testpathparts(self):
        self.assertEquals(len(util.pathparts('')), 0)
        self.assertEquals(len(util.pathparts('/')), 1)
        parts = util.pathparts('/a/b/c')
        self.assertEquals(len(parts), 3)
        self.assertEquals(parts[0], 'a')
        self.assertEquals(parts[1], 'b')
        self.assertEquals(parts[2], 'c')

    def testflags2mode(self):
        self.assertEqual(util.flags2mode(os.O_RDONLY), 'r')
        self.assertEqual(util.flags2mode(os.O_WRONLY), 'w')
        self.assertEqual(util.flags2mode(os.O_RDWR), 'w+')
        self.assertEqual(util.flags2mode(os.O_RDONLY), 'r')
        self.assertEqual(util.flags2mode(os.O_WRONLY | os.O_APPEND), 'a')
        self.assertEqual(util.flags2mode(os.O_RDWR | os.O_APPEND), 'a+')

    def testaddtrailingslash(self):
        self.assertEquals(util.addtrailingslash('a/b/c'), '/a/b/c')
        self.assertEquals(util.addtrailingslash('/a/b/c'), '/a/b/c')
        self.assertEquals(util.addtrailingslash('/'), '/')
        self.assertEquals(util.addtrailingslash(''), '/')

    def testignoretag(self):
        self.assertTrue(util.ignoretag('/abc'))
        self.assertFalse(util.ignoretag('/..'))
        self.assertFalse(util.ignoretag('/.'))
        self.assertFalse(util.ignoretag('/.dejumblefs'))

    def testgetbasefilelist(self):
        self.assertTrue('..' in util.getbasefilelist())
        self.assertTrue('.' in util.getbasefilelist())
        self.assertEquals(len(util.getbasefilelist()), 2)

    def testunique(self):
        list1 = [5, 2, 3, 4]
        list2 = [5, 2, 3, 4, 5, 3]
        self.assertEquals(str(sorted(util.unique(list1))),
                          str(sorted(list1)))
        self.assertEquals(str(sorted(util.unique(list2))),
                          str(sorted(list1)))
        self.assertNotEquals(str(sorted(util.unique(list2))),
                             str(sorted(list2)))
