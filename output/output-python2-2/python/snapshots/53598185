import unittest
import pprint
pp = pprint.PrettyPrinter(indent=4)


class TestCreate(unittest.TestCase):
    def testImport(self):
        import MOPS.Instance

class TestInstance(unittest.TestCase):
    def setUp(self):
        import MOPS.Instance
        self.inst = MOPS.Instance.Instance(dbname='psmops_test')

    def testCreate(self):
        self.assert_(self.inst)

    def testEnvironment(self):
        self.assert_(self.inst.environment.get('HOMEDIR'))
        self.assert_(self.inst.environment.get('VARDIR'))
        self.assert_(self.inst.environment.get('CONFIGDIR'))
        self.assert_(self.inst.environment.get('LOGDIR'))
        self.assert_(self.inst.environment.get('DTCTLDIR'))
        self.assert_(self.inst.environment.get('LODCTLDIR'))
        self.assert_(self.inst.environment.get('PRECOVDIR'))
        self.assert_(self.inst.environment.get('OBJECTSDIR'))

    def testConfig(self):
        self.assert_(self.inst.getConfig())

class TestDB(unittest.TestCase):
    def setUp(self):
        import MOPS.Instance
        self.inst = MOPS.Instance.Instance(dbname='psmops_test')

    def testGetDBH(self):
        self.assert_(self.inst.get_dbh())

    def testNewDBH(self):
        old_dbh = self.inst.get_dbh()
        self.inst.forget_dbh()                  # clear dbh
        self.assert_(self.inst.dbh is None)     # check for it

        new_dbh = self.inst.new_dbh()           # get new one
        self.assert_(new_dbh)
        self.assert_(new_dbh != self.inst.get_dbh()) # check not same as instance's

if __name__ == '__main__':
    unittest.main()
