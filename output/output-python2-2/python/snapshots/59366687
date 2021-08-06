import unittest

import test_loader

suite_loader = unittest.makeSuite(test_loader.TestLoader)
alltests = unittest.TestSuite((suite_loader))
unittest.TextTestRunner(verbosity=2).run(alltests)
