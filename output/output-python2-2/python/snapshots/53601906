#! /usr/bin/env python
#
import re, sys, os, os.path, shutil, traceback, stat
import optparse
from lsst.pex.logging import Log
from lsst.pex.policy import Policy

usage = """usage: %%prog [-vqs] [-V int]

Ensures that the HOME directory contains a .mpd.conf file.  If one does not 
exists, a default is copied into it from the $DC2PIPE_DIR/etc directory. 
"""

cl = optparse.OptionParser(usage)
cl.add_option("-r", "--policyRepository", type="string", action="store",
              dest="policyRepository", default=None, metavar="dir",
              help="directory containing policy files (def: $DC2PIPE_DIR/pipeline")
cl.add_option("-V", "--verbosity", type="int", action="store",
              dest="verbosity", default=0, metavar="int",
              help="verbosity level (0=normal, 1=debug, -1=quiet, -3=silent)")
cl.add_option("-v", "--verbose", action="store_const", const=1,
              dest="verbosity",
              help="print extra messages")
cl.add_option("-q", "--quiet", action="store_const", const=-1,
              dest="verbosity",
              help="print only warning & error messages")
cl.add_option("-s", "--silent", action="store_const", const=-3,
              dest="verbosity",
              help="print only warning & error messages")

# command line results
cl.opts = {}
cl.args = []

pkgdirvar = "DC2PIPE_DIR"
mpdconf = ".mpd.conf"
pkgmpdconf = os.path.join("etc", "mpd.conf")

def createLog():
    log = Log(Log.getDefaultLog(), "dc2pipe.ensureMpdConf")
    return log

def setVerbosity(verbosity):
    logger.setThreshold(-10 * verbosity)  

logger = createLog()

def main():
    try:
        (cl.opts, cl.args) = cl.parse_args();
        setVerbosity(cl.opts.verbosity)
        ensureMpdConf()

    except:
        tb = traceback.format_exception(sys.exc_info()[0],
                                        sys.exc_info()[1],
                                        sys.exc_info()[2])
        logger.log(Log.FATAL, tb[-1].strip())
        logger.log(Log.DEBUG, "".join(tb[0:-1]).strip())
        sys.exit(1)

def ensureMpdConf():
    if not os.environ.has_key(pkgdirvar):
        raise RuntimeError("DC2PIPE_DIR env. var not set (setup dc2pipe)")

    pkgdir = os.environ[pkgdirvar]
    path = os.path.join(os.environ["HOME"], mpdconf)
    if not os.path.exists(path):
        logger.log(Log.DEBUG, "copying new .mpd.conf")
        shutil.copyfile(os.path.join(pkgdir, pkgmpdconf), path)
        os.chmod(path, stat.S_IREAD|stat.S_IWRITE)
    else:
        logger.log(Log.DEBUG, "$HOME/.mpd.conf already exists")


if __name__ == "__main__":
    main()

