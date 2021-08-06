from ..filter import FileListFilter


class NullFileListFilter(FileListFilter):
    def __init__(self, query='', root='/dev'):
        FileListFilter.__init__(self, query, '/dev')
        
    def filelist(self):
        yield '/dev/null'

