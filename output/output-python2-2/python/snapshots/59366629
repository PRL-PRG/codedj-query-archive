import unittest

import test_loader
import test_socket
import test_parser

suite_loader = unittest.makeSuite(test_loader.TestLoader)
suite_socket = unittest.makeSuite(test_socket.TestSocket)
suite_parser = unittest.makeSuite(test_parser.TestParser)

alltests = unittest.TestSuite((suite_loader, suite_socket, suite_parser))
unittest.TextTestRunner(verbosity=2).run(alltests)
