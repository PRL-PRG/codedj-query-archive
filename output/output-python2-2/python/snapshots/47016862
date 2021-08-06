import tempfile

from ..base import BaseFileListFilterTestCase
from ...filters.completedirectory import CompleteDirectoryFileListFilter


class CompleteDirectoryFileListFilterTestCase(BaseFileListFilterTestCase):
    def testfilelist(self):
        original_file1 = tempfile.mkstemp('', '', self.original_dir)
        original_file2 = tempfile.mkstemp('', '', self.original_dir)
        original_subdir = tempfile.mkdtemp('', '', self.original_dir)
        original_file3 = tempfile.mkstemp('', '', original_subdir)

        filter = CompleteDirectoryFileListFilter('', self.original_dir)
        filelist = list(filter.filelist())

        self.assertEqual(len(filelist), 3)
        self.assertTrue(original_file1[1] in filelist)
        self.assertTrue(original_file2[1] in filelist)
        self.assertTrue(original_file3[1] in filelist)

