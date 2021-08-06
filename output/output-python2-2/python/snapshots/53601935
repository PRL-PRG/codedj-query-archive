#! /usr/bin/env python
#
import re, sys, os, os.path, shutil, subprocess
import optparse, traceback
from lsst.mwi.logging import Log
from lsst.mwi.policy import Policy

usage = """usage: %prog [-vqs] [-V int] [-p dc2pipe_policy_file] [-r runId] [node ...]

Kill the pipelines running on a give set of head nodes.
"""

cl = optparse.OptionParser(usage)
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
cl.add_option("-p", "--policy", action="store", dest="policy", 
              default=None, metavar="policy_file",
              help="the dc2pipe policy file used to launch the pipelines")
cl.add_option("-r", "--runid", action="store", dest="runid", 
              default="", metavar="runid",
              help="restrict the kill to pipelines running with this runid")

# command line results
cl.opts = {}
cl.args = []

pkgdirvar = "DC2PIPE_DIR"
defDomain = ".ncsa.uiuc.edu"
remkill = "killpipe.sh"

def createLog():
    log = Log(Log.getDefaultLog(), "dc2pipe")
    return log

def setVerbosity(verbosity):
    logger.setThreshold(-10 * verbosity)  

logger = createLog()

def main():
    try:
        (cl.opts, cl.args) = cl.parse_args();
        setVerbosity(cl.opts.verbosity)

        nodes = []
        if cl.opts.policy is not None:
            policy = Policy.createPolicy(cl.opts.policy)
            nodes = getHeadNodes(policy)
        nodes.extend(cl.args)

        logger.log(Log.DEBUG, "Killing pipelines on " + ", ".join(nodes))

        remcmd = "%s %s" % \
            (os.path.join(os.environ[pkgdirvar], "bin", remkill),cl.opts.runid)
        remcmd = remcmd.strip()

        for node in nodes:
            cmd = ("ssh", node, remcmd)
            logger.log(Log.INFO, "executing: %s %s '%s'" % cmd)
            if subprocess.call(cmd) != 0:
                logger.log(Log.WARN, "Failed to kill processes on " + node)

    except:
        tb = traceback.format_exception(sys.exc_info()[0],
                                        sys.exc_info()[1],
                                        sys.exc_info()[2])
        logger.log(Log.FATAL, tb[-1].strip())
        logger.log(Log.DEBUG, "".join(tb[0:-1]).strip())
        sys.exit(1)

    sys.exit(0)

def getHeadNodes(pol):
    pipepol = pol.get("pipelines")
    pipelines = pipepol.policyNames(True)
    procs = re.compile(r":.*$")

    nodes = []
    for pipeline in pipelines:
        ppol = pipepol.get(pipeline)
        if ppol.get("launch", 1) != 0:
            pnode = ppol.getArray("nodes")
            if pnode is not None:
                pnode = pnode[0]
                pnode = procs.sub("", pnode).strip()
                nodes.append(pnode)
            
    return nodes
    

if __name__ == "__main__":
    main()
