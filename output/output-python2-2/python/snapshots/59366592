import unittest

import test_loader
import test_socket
import test_parser
import test_model

suite_loader = unittest.makeSuite(test_loader.TestLoader)
suite_socket = unittest.makeSuite(test_socket.TestSocket)
suite_parser = unittest.makeSuite(test_parser.TestParser)
suite_model  = unittest.makeSuite(test_model.TestCircularList)

alltests = unittest.TestSuite((suite_loader, suite_socket, suite_parser,
                               suite_model))
unittest.TextTestRunner(verbosity=2).run(alltests)
