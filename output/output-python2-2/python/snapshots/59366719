#!/usr/bin/env python
#-*- coding: utf-8 -*-
#

import sys
import unittest

sys.path.append('..')

import core

class TestLoader(unittest.TestCase):
    def setUp(self):
        config = {'module_path': '../modules'}
        self.loader = core.Loader(config)

    def testLoadModules(self):
        self.loader._loadModules()
        self.assert_('abstract_factory' in self.loader.modules)

    def testLoadClasses(self):
        self.loader._loadModules()
        classes = self.loader.loadClasses(['AbstractFactory'])
        self.assert_('AbstractFactory' in classes)

    def testLoadClassesFailure(self):
        self.loader._loadModules()
        self.assertRaises(core.ClassNotFoundException,
                          self.loader.loadClasses,
                          ['FakeFactory'])


if __name__ == '__main__':
    unittest.main()