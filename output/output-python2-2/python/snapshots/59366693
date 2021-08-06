#!/usr/bin/env python
#-*- coding: utf-8 -*-
#

import sys
import unittest

sys.path.append('../modules')

import exception
from loader import Loader

class TestLoader(unittest.TestCase):
    def setUp(self):
        config = {'main': {'module_path': '../modules'}}
        self.loader = Loader(config)

    def testLoadModules(self):
        self.loader._loadModules()
        self.assert_('thread_abstract' in self.loader.modules)

    def testLoadClasses(self):
        self.loader._loadModules()
        classes = self.loader._loadClasses(['Thread'])
        self.assert_('Thread' in classes)

    def testLoadClassesFailure(self):
        self.loader._loadModules()
        self.assertRaises(exception.ClassNotFound,
                          self.loader._loadClasses,
                          ['Fake'])


if __name__ == '__main__':
    unittest.main()
