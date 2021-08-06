import unittest

from ...organizers.iso9660 import ISO9660Organizer


class ISO9660OrganizerTestCase(unittest.TestCase):

    def setUp(self):
        self.organizer = ISO9660Organizer(None)

    def testincreasefilename(self):
        self.assertEquals(self.organizer.increasefilename("A.TXT"),
                          "A~1.TXT")
        self.assertEquals(self.organizer.increasefilename("A~1.TXT"),
                          "A~2.TXT")
        self.assertEquals(self.organizer.increasefilename("123456~9.TXT"),
                          "12345~10.TXT")

    def test_path(self):
        self.assertEquals(self.organizer.convertpath("a.txt"), "A.TXT")
        self.assertEquals(self.organizer.convertpath("1+2 345.txt"),
                          "1_2345.TXT")
        self.assertEquals(self.organizer.convertpath("1234567890.text"),
                          "123456~1.TEX")
