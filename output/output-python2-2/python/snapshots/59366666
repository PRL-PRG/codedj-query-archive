import unittest

import test_loader
import test_socket

suite_loader = unittest.makeSuite(test_loader.TestLoader)
suite_socket = unittest.makeSuite(test_socket.TestSocket)

alltests = unittest.TestSuite((suite_loader, suite_socket))
unittest.TextTestRunner(verbosity=2).run(alltests)
