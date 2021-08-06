import os

from ..base import BaseFileListFilterTestCase
from ...filters.shell import ShellFileListFilter


class ShellFileListFilterTestCase(BaseFileListFilterTestCase):

    def testfilelist(self):
        os.chdir('/tmp')
        filelist = list(ShellFileListFilter('echo /dev/null && echo %s/null'
                                            % os.getcwd(), '/').filelist())
        self.assertEqual(len(filelist), 2)
        self.assertTrue(filelist[0], '/dev/null')
        self.assertEqual(filelist[1], './null')
