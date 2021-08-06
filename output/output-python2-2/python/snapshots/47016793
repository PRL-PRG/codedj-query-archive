import os
import os.path

from ..filter import FileListFilter


class CompleteDirectoryFileListFilter(FileListFilter):

    def filelist(self):
        return list(self.generatefilelistrecursive(self.root))

    def generatefilelistrecursive(self, dirname):
        for path in os.listdir(dirname):
            path = os.path.join(dirname, path)
            if os.path.isdir(path) and not os.path.islink(path):
                for realpath in self.generatefilelistrecursive(path):
                    yield realpath
            else:
                yield path
