import unittest
import pprint
pp = pprint.PrettyPrinter(indent=4)


class TestCreate(unittest.TestCase):
    def testImport(self):
        import MOPS.Config

class TestLoad(unittest.TestCase):
    def testLoad(self):
        import MOPS.Config
        cfg = MOPS.Config.LoadFile('/usr/local/MOPS_DEV/config/cluster.cf')
        self.assert_(cfg)
        self.assert_(cfg.get('hosts') is not None)


if __name__ == '__main__':
    unittest.main()

#cfg = MOPS.Config.LoadFile('/usr/local/MOPS_DEV/config/cluster.cf')
#print pp.pprint(cfg)
#print cfg
#
#cfg = MOPS.Config.LoadFile('/usr/local/MOPS_DEV/config/backend.cf')
#print pp.pprint(cfg)
#print cfg
#
#cfg = MOPS.Config.LoadFile('/usr/local/MOPS_DEV/config/master.cf')
#print pp.pprint(cfg)
#print cfg
