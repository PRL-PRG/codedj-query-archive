import unittest

import test_loader
import test_socket
import test_parser
import test_model
import test_storage

loader = unittest.makeSuite(test_loader.TestLoader)
socket = unittest.makeSuite(test_socket.TestSocket)
parser = unittest.makeSuite(test_parser.TestParser)
model  = unittest.makeSuite(test_model.TestCircularList)
storage = unittest.makeSuite(test_storage.TestStorage)
storage2 = unittest.makeSuite(test_storage.TestStorage2)

alltests = unittest.TestSuite((loader, socket, parser,
                               model, storage, storage2))
unittest.TextTestRunner(verbosity=2).run(alltests)
