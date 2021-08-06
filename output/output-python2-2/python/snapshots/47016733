import commands
import errno
import os
import re

from ..filter import FileListFilter


class ShellFileListFilter(FileListFilter):

    def filelist(self):
        status, output = commands.getstatusoutput(self.query)

        if status != 0:
            return -errno.ENOENT

        filenames = [re.sub('^%s' % os.getcwd(), '.', o)
                     for o in output.splitlines()]

        return filenames
