#!/usr/bin/env python

import sys
import logging
import logging.config
import pkg_resources
import gettext
import errno

import fuse
from fuse import Fuse

import dejumblefs.fs

gettext.install('dejumblefs')

try:
    import psyco
    psyco.full()
except ImportError:
    pass


def main():
    usage = """
dejumble: presents the content of a directory in an organized structure.

""" + Fuse.fusage

    dolog = True

    server = dejumblefs.fs.DejumbleFS(version="%%prog %s" % fuse.__version__,
                                    usage=usage, dash_s_do='setsingle')
    server.setoptions()
    server.parse(values=server, errex=1)

    if not server.fuse_args.mountpoint:
        print >> sys.stderr, (_("No mountpoint defined"))
        sys.exit(-errno.ENOENT)

    if dolog:
        filename = pkg_resources.resource_filename('dejumblefs',
                                                   'conf/logging.conf')
        logging.config.fileConfig(filename)
        # redirect stdout to a disk file
        saveout = sys.stdout
        saveerr = sys.stderr
        outfile = open('/tmp/log.txt', 'a+')
        sys.stdout = outfile
        sys.stderr = outfile
    else:
        logging.disable(logging.CRITICAL)

    dejumblefs.fs.setserver(server)
    server.main()

    if dolog:
        # restore stdout
        outfile.flush()
        outfile.close()
        sys.stdout = saveout
        sys.stderr = saveerr


if __name__ == '__main__':
    main()
