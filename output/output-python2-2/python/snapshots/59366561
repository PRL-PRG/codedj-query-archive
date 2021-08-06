#!/usr/bin/env python
#-*- coding: utf-8 -*-
#

import sys
import unittest

sys.path.append('..')

import modules.exception as exception
from modules.loader import Loader

class TestLoader(unittest.TestCase):
    def setUp(self):
        config = {'modules': {'path': '../modules'}}
        self.loader = Loader(config)

    def testFindModules(self):
        self.loader._findModules()
        self.assert_('thread' in self.loader.modules)

    def testLoadClasses(self):
        self.loader._findModules()
        classes = self.loader._loadClasses(['Thread'])
        self.assert_('Thread' in classes)

    def testLoadClassesFailure(self):
        self.loader._findModules()
        self.assertRaises(exception.ClassNotFound,
                          self.loader._loadClasses,
                          ['Fake'])


if __name__ == '__main__':
    unittest.main()
