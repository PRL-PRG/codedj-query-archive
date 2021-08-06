from ..base import BaseFileListFilterTestCase
from ...filters.shell import ShellFileListFilter


class ShellFileListFilterTestCase(BaseFileListFilterTestCase):
    def testfilelist(self):
        filelist = list(ShellFileListFilter('echo /dev/null', '/').filelist())
        self.assertEqual(len(filelist), 1)
        self.assertEqual(filelist[0], '/dev/null')
