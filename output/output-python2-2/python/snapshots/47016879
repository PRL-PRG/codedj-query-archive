import commands
import errno

from ..filter import FileListFilter


class ShellFileListFilter(FileListFilter):
    def filelist(self):
        status, output = commands.getstatusoutput(self.query)

        if status == -1:
            return -errno.ENOENT

        filenames = output.splitlines()
        
        # TODO: convert files inside the mount directory to relative paths.

        return filenames

