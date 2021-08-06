from ..filter import FileListFilter


class NullFileListFilter(FileListFilter):

    def __init__(self, query='', root=None): #IGNORE:W0613
        FileListFilter.__init__(self, query, '/dev')

    def filelist(self):
        yield '/dev/null'
