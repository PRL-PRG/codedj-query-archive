from ..base import BaseFileListFilterTestCase
from ...filters.null import NullFileListFilter


class NullFileListFilterTestCase(BaseFileListFilterTestCase):
    def testfilelist(self):
        filelist = list(NullFileListFilter().filelist())
        self.assertEqual(len(filelist), 1)
        self.assertEqual(filelist[0], '/dev/null')
