#!/usr/bin/env python

import commands
import sys
import errno
import os.path
import gettext
import platform

from .. import util

gettext.install('dejumblefs')


def main():
    commandname = os.path.basename(sys.argv[0])
    if len(sys.argv) == 1:
        print "usage: %s [mountpoint]" % commandname
        sys.exit(1)

    path = sys.argv[1]
    command_path = os.path.join(path, util.ORIGINAL_DIR, 'commands', 'umount')

    if not os.path.isdir(os.path.join(path, util.ORIGINAL_DIR)):
        print >> sys.stderr, _('%s: %s: not a dejumble filesystem') % \
              (commandname, path)
        sys.exit(-errno.ENOENT)

    status, output = commands.getstatusoutput('echo 1 > "%s"' % command_path)

    if status != 0:
        print >> sys.stderr, _('%s: %s') % (commandname, output)
        sys.exit(status)

    if platform.system() == 'Darwin':
        status, output = commands.getstatusoutput('umount "%s"' % path)
    else:
        status, output = commands.getstatusoutput('fusermount -u "%s"' % path)

    if status != 0:
        print >> sys.stderr, _('%s: %s') % (commandname, output)
        sys.exit(status)


if __name__ == '__main__':
    main()
