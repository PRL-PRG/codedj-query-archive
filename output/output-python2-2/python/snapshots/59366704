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
        self.assert_('thread_abstract' in self.loader.modules)

    def testLoadClasses(self):
        self.loader._loadModules()
        classes = self.loader._loadClasses(['Thread'])
        self.assert_('Thread' in classes)

    def testLoadClassesFailure(self):
        self.loader._loadModules()
        self.assertRaises(core.ClassNotFoundException,
                          self.loader._loadClasses,
                          ['FakeFactory'])


if __name__ == '__main__':
    unittest.main()
