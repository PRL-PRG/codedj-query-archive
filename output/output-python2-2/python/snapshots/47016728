#!/usr/bin/env python

import commands
import sys
import errno
import os.path
import gettext

from .. import util

gettext.install('dejumblefs')


def main():
    path = sys.argv[1]
    command_path = os.path.join(path, util.ORIGINAL_DIR, 'command')

    if not os.path.isdir(os.path.join(path, util.ORIGINAL_DIR)):
        print >> sys.stderr, (_('Not a dejumble filesystem'))
        sys.exit(-errno.ENOENT)

    # FIXME: send umount signal before umounting
    #status, output = commands.getstatusoutput('echo "umount" > "%s"' % command_path)

    #if status != 0:
    #    print >> sys.stderr, (output)
    #    sys.exit(status)

    status, output = commands.getstatusoutput('umount "%s"' % path)
    
    if status != 0:
        print >> sys.stderr, (output)
        sys.exit(status)


if __name__ == '__main__':
    main()