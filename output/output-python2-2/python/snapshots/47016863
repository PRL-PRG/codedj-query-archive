from ..filters.completedirectory import CompleteDirectoryFileListFilter


class OriginalDirectoryFileListFilter(CompleteDirectoryFileListFilter):
    def __init__(self, query=None, root=None):
        CompleteDirectoryFileListFilter.__init__(self, query, '.')

    def filelist(self):
        return list(self._generatefilelist('.'))

