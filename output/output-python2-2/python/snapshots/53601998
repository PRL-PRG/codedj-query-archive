#! /usr/bin/env python
#
from __future__ import with_statement
import re, sys, os, os.path, shutil, subprocess
import optparse, traceback, time
from lsst.mwi.logging import Log
from lsst.mwi.policy import Policy

usage = """usage: %%prog [-vqsfD] [-V int] [-r dir] [-e script] dc2pipe_policy_file runId
                   [ exposureList ... ]

Launch all or parts of the DC2 pipeline according to a given DC2 policy file.  
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
              help="print no messages")
cl.add_option("-f", "--forcerunid", action="store_true", dest="forceRunId",
              help="allow the reuse of an existing Run ID")
cl.add_option("-D", "--nodbcreate", action="store_false", dest="createDB",
              help="do not create the database tables for this run")
cl.add_option("-m", "--mpdconfset", action="store_true", dest="forceMpdConf",
              help="force a check for a .mpd.conf file on every desired node")
cl.add_option("-e", "--envscript", action="store", dest="envscript", 
              default=None, metavar="script",
              help="an environment-setting script to source on pipeline platform")

# command line results
cl.opts = {}
cl.args = []

pkgdirvar = "DC2PIPE_DIR"
defDomain = ".ncsa.uiuc.edu"
secretsfile = os.path.join("etc","mpd.conf")
eventgenerator = "eventgenerator.py lsst4"
DbHost = "lsst10.ncsa.uiuc.edu "
DbUser = "test"
DbPassword = "globular.test"
DbCmdFiles = ["lsstSchema4mysql.sql", "lsstPipelineSetup4mysql.sql"]

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

        if len(cl.args) < 1:
            print usage
            raise RuntimeError("Missing arguments: dc2pipe_policy_file runId")
        if len(cl.args) < 2:
            print usage
            raise RuntimeError("Missing argument: runid")
    
        launchDC2(cl.args[0], cl.args[1], cl.args[2:])

    except:
        tb = traceback.format_exception(sys.exc_info()[0],
                                        sys.exc_info()[1],
                                        sys.exc_info()[2])
        logger.log(Log.FATAL, tb[-1].strip())
        logger.log(Log.DEBUG, "".join(tb[0:-1]).strip())
#        sys.exit(1)

def launchDC2(policyFile, runid, exposureFiles):
    if not os.environ.has_key(pkgdirvar):
        raise RuntimeError("DC2PIPE_DIR env. var not set (setup dc2pipe)")

    # load the launching policy file
    if not os.path.exists(policyFile):
        raise RuntimeError(policyFile + ": policy file not found")

    pol = Policy.createPolicy(policyFile)
    if cl.opts.policyRepository is not None:
        pol.set("policyRepository", cl.opts.policyRepository);

    # find the policy repository
    defRepository = os.path.join(os.environ[pkgdirvar], "pipeline")
    repository = pol.get("policyRepository", defRepository)

    if not os.path.exists(repository):
        raise RuntimeError(repository + ": directory not found");
    if not os.path.isdir(repository):
        raise RuntimeError(repository + ": not a directory");

    # set the exposure lists
    if len(exposureFiles) == 0:
        exposureFiles = pol.getArray("exposureList", [])
    if len(exposureFiles) == 0:
        raise RuntimeError("No exposure list files specified in policy")

    # determine the parent of the working directories
    home = pol.get("workingHome", "/share/DC2root")

    # deploy each pipeline:  create nodelist.scr, copy over policy files, etc.
    pipepol = pol.get("pipelines")
    pipelines = pipepol.policyNames(True)
    workingdir = {}
    masternode = {}
    envscript = {}
    willrun = []
    for pipeline in pipelines:
        ppol = pipepol.get(pipeline)
        if ppol.get("launch", 1) != 0:
            (masternode[pipeline], workingdir[pipeline], envscript[pipeline]) \
                = prepPipeline(pipeline, ppol, runid, home, repository)
            willrun.append(pipeline)
            
    # now launch each pipeline
    for pipeline in willrun:
        ppol = pipepol.get(pipeline)
        launchPipeline(pipeline, runid, 
                       masternode[pipeline], workingdir[pipeline],
                       envscript[pipeline])

    # send input data events
    for efile in exposureFiles:
        if not os.path.isabs(efile):
            if os.path.exists(efile) and not efile.startswith("."):
                efile = os.path.join(".", efile)
            elif os.path.exists(os.path.join(os.environ[pkgdirvar],
                                             "exposureLists",efile)):
                efile = os.path.join(os.environ[pkgdirvar],
                                    "exposureLists",efile)
        if not os.path.exists(efile):
            logger.log(Log.WARN, "Exposure list file not found: " + efile)

        logger.log(Log.DEBUG, "Pausing for 15s, waiting for setup...")
        time.sleep(15)

        logger.log(Log.INFO, "Sending exposure data from " + efile)
        logger.log(Log.DEBUG,
                   "executing: %s < %s"  % (eventgenerator, efile))

        with file(efile) as expfile:
            if subprocess.call(eventgenerator.split(), stdin=expfile) != 0:
                raise RuntimeError("Failed to execute eventgenerator")

def prepPipeline(pname, pol, runid, home, repos):
    
    # ensure the existence of the working directory
    pdir = pol.get("shortName", pname)

    # ensure the existance of the input/output directories
    wdir = os.path.join(home, runid, pdir)
    if not cl.opts.forceRunId and os.path.exists(os.path.dirname(wdir)):
        raise RuntimeError("Run ID already used (use -f to override)")
    dir = os.path.join(wdir, "input")
    if not os.path.exists(dir): os.makedirs(dir)
    dir = os.path.join(wdir, "output")
    if not os.path.exists(dir): os.makedirs(dir)
    if pname == "assoc":
        os.makedirs(os.path.join(wdir, "update"))

    wdir = os.path.join(runid, pdir, "work")
    wdir = pol.get("workingDir", wdir)

    if not os.path.isabs(wdir):
        wdir = os.path.join(home, wdir)

    if not os.path.exists(wdir): os.makedirs(wdir)
    logger.log(Log.INFO, "Working directory for %s: %s" % (pname, wdir))

    # create the nodelist file
    nodes = map(expandNodeHost, pol.getArray("nodes"))

    nodelist = open(os.path.join(wdir, "nodelist.scr"), 'w')
    for node in nodes:
        print >> nodelist, node
    nodelist.close()

    # copy the environment-setting script
    script = "setup.sh"
    if os.environ.has_key("SHELL") and os.environ["SHELL"].find("csh") >= 0:
        script = "setup.csh"
    script = os.path.join(os.environ[pkgdirvar], "etc", script)
    if cl.opts.envscript is not None:
        script = cl.opts.envscript
    shutil.copy(script, wdir)

    # copy the policies to the working directory
    polfile = os.path.join(repos, pname+".paf")
    polbasefile = os.path.basename(polfile)
    if os.path.exists(os.path.join(wdir, pname+".paf")):
        logger.log(Log.WARN, 
                   "Working directory already contains %s; won't overwrite" % \
                       polbasefile)
    else:
        shutil.copy(polfile, wdir)

    if os.path.exists(os.path.join(wdir, pname)):
        logger.log(Log.WARN, 
          "Working directory already contains %s directory; won't overwrite" % \
                       pname)
    else:
        shutil.copytree(os.path.join(repos,pname), os.path.join(wdir,pname))

    return (getNode(nodes[0]), wdir, script)

def createDatabase(runid):
    dbcmdbase = "mysql -h lsst10.ncsa.uiuc.edu -u%s -p%s " % (DbUser, DbPassword)
    sqldir = os.path.join(os.environ[pkgdirvar], "etc")

    logger.log(Log.INFO, "Creating database tables for run " + runid)
    cmd = '%s -e create database "%s"' % (dbcmdbase, runid)

    if logger.willSend(Log.DEBUG):
        logger.log(Log.DEBUG, "executing: " + cmd)

    if (subprocess.call(cmd.split()) != 0):
        raise RuntimeError("Failed to create database for run " + runid)

    logger.log(Log.DEBUG, "Created pipeline database")

    for sqlCmdFile in DbCmdFiles:
        cmd = dbcmdbase + runid
        if logger.willSend(Log.DEBUG):
            logger.log(Log.DEBUG, "sending %s to: %s" % (sqlCmdFile,cmd))

        with file(os.path.join(sqldir, sqlCmdFile)) as sqlFile:
            if subprocess.call(cmd, stdin=sqlFile) != 0:
                raise RuntimeError("Failed to create execute " + sqlCmdFile)

        if logger.willSend(Log.DEBUG):
            logger.log(Log.DEBUG, "sql script completed: " % sqlCmdFile)

def launchPipeline(pname, runid, node, wdir, script):

    launchcmd = os.path.join(os.environ[pkgdirvar], "bin", "launchPipeline.sh")
    cmd = ["ssh", node, 
           "cd %s; source %s; %s %s %s -V %s" \
        % (wdir, script, launchcmd, pname+".paf", runid, cl.opts.verbosity) ]

    logger.log(Log.INFO, "launching %s on %s" % (pname, node) )
    logger.log(Log.DEBUG, "executing: " + " ".join(cmd))

    if subprocess.call(cmd) != 0:
        raise RuntimeError("Failed to launch " + pname)

def getNode(nodeentry):
    colon = nodeentry.find(':')
    if colon < 1:  
        return nodeentry
    else: 
        return nodeentry[0:colon]

def deployMPISecrets(nodelist):
    defSecrets = os.path.join(os.environ[pkgdirvar], secretsfile)
    secrets = "." + os.path.basename(defSecrets)

    for node in nodelist:
        node = getNode(node)

        cmd = "scp %s %s:%s" % (defSecrets, node, secrets)

        if logger.willSend(Log.DEBUG):
            logger.log(Log.DEBUG, "exec: " + cmd)
        ok = subprocess.call(cmd.split())
        if ok != 0:
            raise RuntimeError("Failed to copy secrets file to %s" % node);

    return True


def expandNodeHost(nodeentry):
    """Add a default network domain to a node list entry if necessary """

    if nodeentry.find(".") < 0:
        node = nodeentry
        colon = nodeentry.find(":")
        if colon == 0:
            raise RuntimeError("bad nodelist format: " + nodeentry)
        elif colon > 0:
            node = nodeentry[0:colon]
            if len(node) < 3:
                logger.log(Log.WARN, "Suspiciously short node name: " + node)

            node += defDomain
            nodeentry = "%s:%s" % (node, nodeentry[colon+1:])
        else:
            nodeentry = "%s%s:1" % (node, defDomain)

    return nodeentry
    

if __name__ == "__main__":
    main()
    
