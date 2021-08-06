#! /bin/env python
#
import sys
import os
import os.path
import cStringIO
import re
from copy import copy
# import cgitb; cgitb.enable()

defaultManifestHeader = \
"""EUPS distribution manifest for %s (%s). Version 1.0
#
"""

defaultColumnNames = \
"pkg flavor version tablefile installation_directory installID".split()

class Manifest:
    """an in-memory representation of a package manifest."""

    def __init__(self, name, version, flavor="generic"):
        """create a manifest for a given package

        @param name     the name of the package this manifest is for
        @param version  the version of the package
        @param flavor   the name of the platform type supported by this
                          installation of the package
        """
        self.recs = {}
        self.keys = []
        self.name = name
        self.vers = version
        self.flav = flavor
        self.hdr = defaultManifestHeader
        self.colnames = copy(defaultColumnNames)
        self.colnames[0] = "# " + self.colnames[0]
        self.commcount = 0

    def getNameVerFlav(self):
        """return the package name, version, and flavor as a 3-tuple"""
        return (self.name, self.vers, self.flav)

    def addComment(self, comment):
        """append a comment to the manifest"""
        self.commcount += 1
        key = '#'+str(self.commcount)
        self.keys.append(key)
        self.recs[key] = [ '' ] * len(self.colnames)
        self.recs[key][-1] = comment

    def addRecord(self, pkgname, flavor, version,
                  tablefile, installdir, installid):
        """append a record to the manifest list.  This method does not
        prevent duplicate records.

        @param pkgname    the name of the package
        @param flavor     the name of the platform type supported by this
                            installation of the package
        @param version    the version of the package
        @param tablefile  the name of the EUPS table file for this package
        @param installdir the directory (relative to $LSST_HOME) where
                             this package should be installed by default.
        @param installid  a complete handle for the deployment bundle.
        """
        key = ":".join([pkgname, flavor, version])
        if not self.recs.has_key(key):
            self.keys.append(key)
            self.recs[key] = [pkgname, flavor, version,
                              tablefile, installdir, installid]

    def addLSSTRecord(self, pkgname, version, pkgpath=None, flavor="generic",
                      id="pacman"):
        """append a standard build record for an LSST package.

        @param pkgname    the name of the package
        @param version    the version of the package
        @param pkgpath    if non-None, a path to be prepended to the standard
                             pkgname/version install directory (default:
                             None)
        @param flavor     the name of the platform type supported by this
                            installation of the package (default: "generic")
        @param id         the installid or abbreviateion (default: "pacman")
        """
        fpkgpath = "%s/%s" % (pkgname, version)
        if (pkgpath is not None and len(pkgpath) > 0):
            fpkgpath = "%s/%s" % (pkgpath, fpkgpath)
            
        self.addRecord(pkgname, flavor, version, pkgname+".table", fpkgpath,
                       self.defaultID(pkgname, flavor, version, id))
                       
    def addExtRecord(self, pkgname, version, pkgpath="external", 
                     flavor="generic", id="pacman"):
        """append a standard build record for an LSST package

        @param pkgname    the name of the package
        @param version    the version of the package
        @param pkgpath    if non-None, a path to be prepended to the standard
                             pkgname/version install directory (default:
                             "external")
        @param flavor     the name of the platform type supported by this
                            installation of the package (default: "generic")
        @param id         the installid or abbreviateion (default: "pacman")
        """
        self.addLSSTRecord(pkgname, version, pkgpath, flavor, id)

    def addSelfRecord(self, pkgpath=None, flavor="generic", id="pacman"):
        """append a standard build record for the package that this
        manifest is for

        @param pkgpath    if non-None, a path to be prepended to the standard
                             pkgname/version install directory (default:
                             None)
        @param flavor     the name of the platform type supported by this
                            installation of the package (default: "generic")
        @param id         the installid or abbreviateion (default: "pacman")
        """
        self.addLSSTRecord(self.name, self.vers, pkgpath, flavor, id)

    def defaultID(self, pkgname, flavor, version, id):
        """create an installid from an abbreviation that is consistent
        with the package name and version.  If the input id is not
        recognized as an abbreviation, it is returned untransformed.

        Recognized ids include "pacman", representing a standard LSST
        pacman script having the name of the form, "package-version.pacman".  

        @param pkgname    the name of the package
        @param flavor     the name of the platform type supported by this
                            installation of the package
        @param version    the version of the package
        @param id    either an id abbreviation or a full installid
        """
        if id == "pacman":
            id = "%s-%s" % (pkgname, version)
            if (flavor != "generic"):
                id = "%s/%s" % (flavor, id)
            id = "pacman:LSST:" + id
        return id

    def hasRecord(self, pkgname, flavor, version):
        """return true if this manifest has a record matching the
        package name, flavor, and version

        @param pkgname    the name of the package
        @param flavor     the name of the platform type supported by this
                            installation of the package
        @param version    the version of the package
        """
        return self.recs.has_key(":".join([pkgname, flavor, version]))

    def recordToString(self, pkgname, flavor, version):
        """return the requested record in manifest format.
        @param pkgname    the name of the package
        @param flavor     the name of the platform type supported by this
                            installation of the package
        @param version    the version of the package
        """
        if (not self.hasRecord(pkgname, flavor, version)):
            raise RuntimeError("record not found in manifest")
        return " ".join(self.recs(":".join([pkgname, flavor, version])))

    def __repr__(self):
        """return all lines of the manifest in proper manifest format"""
        out = cStringIO.StringIO()
        self.printRecord(out)
        return out.getvalue()

    def str(self):
        """return all lines of the manifest in proper manifest format"""
        return str(self)

    def printRecord(self, strm):
        """print the lines of the manifest to a give output stream.

        @param strm  the output stream to write the records to
        """
        collen = self._collen()
        fmt = "%%-%ds %%-%ds %%-%ds %%-%ds %%-%ds %%s\n" % tuple(collen[:-1])
        
        strm.write(self.hdr % (self.name, self.vers))
        strm.write((fmt % tuple(self.colnames)))
        strm.write("#" + " ".join(map(lambda x: '-' * x, collen))[1:79])
        strm.write("\n")

        for key in self.keys:
            if key.startswith('#'):
                strm.write("# %s\n" % self.recs[key][-1])
            else:
                strm.write(fmt % tuple(self.recs[key]))
            
    def _collen(self):
        x = self.recs.values()
        x.append(self.colnames)
        return map(lambda y: max(map(lambda x: len(x[y]), x)),
                   xrange(0,len(self.colnames)))
    
defaultCurrentFile = "current.list"

defaultManfileName = "manifest.list"

class Loader:
    """a class that can load a Manifest from directive files"""

    def __init__(self, basedir=".", strict=True):
        """create a loader

        @param basedir   the base directory under which manifest directive
                            files can be found
        """
        self.strict = strict
        self.openfiles = []
        self.visited = []
        self.basedir = basedir
        self.pkgPath = {}
        self.currentfile = defaultCurrentFile
        self.manfile = defaultManfileName

    def loadFromFile(self, manifest, file, pkgpath=None, 
                     pkgname=None, version=None, flavor=None):
        """load records into a manifest according to directives in the
        given file.

        @param manifest   the Manifest object to add to
        @param file       a manifest directive file
        @param pkgpath    the path to this package; if None (default),
                            no extra path will be assumed
        @param pkgname    the name of the package associate with this
                            manifest file; if None (default), the name
                            associated with the manifest will be assumed
        @param version    the version of the package associate with this
                            manifest file; if None (default), the version
                            associated with the manifest will be assumed
        @param flavor     the flavor associate with this package; if None,
                            (default), the flavor associated with the
                            manifest will be assumed.
        """
        if file in self.openfiles:
            raise ValueError, "Circular inclusion detected!"
        if file in self.visited:
            # silently skip this file since we've already processed it
            return
        
        if pkgname is None:
            if version is None:  version = manifest.vers
            if flavor is None: flavor = manifest.flav
            pkgname = manifest.name
        if version is None:
            ValueError, "can't set a default version for " + pkgname
        if flavor is None:
            flavor = "generic"
            
        try:
            mf = open(file, "r")
            self.openfiles.append(file)
        except IOError, (enum, emsg):
            if (self.strict or enum != 2):
                raise IOError, (enum, "%s: %s" % (file, emsg))
            else:
                if pkgpath is None:  pkgpath = ''
                if len(pkgpath) > 0: pkgpath = "(%s)" % pkgpath
                manifest.addComment("No manifest found for %s %s %s: %s" %
                                    (pkgname, version, pkgpath, file))
                self.visited.append(file)
                return

        try:
            for line in mf:
                ppath = None
                flavor = "generic"
                line = line.strip()
                if line.startswith('#'):
                    continue

                if line.startswith('>'):
                    if line[1:].startswith("merge:"):
                        # merge the manifest for the given package into
                        # our manifest
                        args = re.findall("\S+", line)

                        if len(args) < 2:
                            self.badFileSyntax("bad syntax: merge needs at " +
                                               "least one argument: " +
                                               line[1:], manifest)
                            continue
                        pkg = args[1]
                        
                        if len(args) > 2:
                            ver = args[2]
                        else:
                            lu = self.lookupCurrent(pkg)
                            if len(lu) == 0:
                                self.badFileSyntax("unknown current version " +
                                                   "for " + pkg + ": " +
                                                   line[1:], manifest)
                                continue
                            ver = lu[0]
                            if len(lu) > 1:  ppath = lu[1]

                        if len(args) > 3:
                            ppath = args[3]

                        if len(args) > 4:
                            flavor = args[4]

                        (file, pp) = self.getFileFor(pkg, ver, flavor)
                        if ppath is None:  ppath = pp
                            
                        self.loadFromFile(manifest, file, ppath,
                                          pkg, ver, flavor)

                    elif line[1:].startswith("self"):
                        # create a record for the package this file describes
                        manifest.addLSSTRecord(pkgname, version,pkgpath,flavor)

                    elif line[1:].startswith("add:"):
                        # add a standard record for a package (ignoring its
                        # dependencies
                        args = re.findall("\S+", line)
                        if len(args) < 2:
                            self.badFileSyntax("bad syntax: add needs at " +
                                               "least one argument: " +
                                               line[1:], manifest)
                            continue
                        pkg = args[1]
                        
                        if len(args) > 2:
                            ver = args[2]
                        else:
                            lu = self.lookupCurrent(pkg)
                            if len(lu) == 0:
                                self.badFileSyntax("lack of current version " +
                                                   "for " + pkg + ": " +
                                                   line[1:], manifest)
                                continue
                            ver = lu[0]
                            if len(lu) > 1:  ppath = lu[1]

                        if len(args) > 3:
                            ppath = args[3]

                        if len(args) > 4:
                            flavor = args[4]

                        manifest.addLSSTRecord(pkg, ver, ppath, flavor)

                    else:
                        msg = "unrecognized directive"
                        if not re.match("\S+:", line[1:]):
                            msg += " (missing :?)"
                        msg += ": %s" % line[1:]
                        self.badFileSyntax(msg, manifest)
                        continue

                elif re.search(r'\S', line):
                    match = re.match(r"(\S+)\s+(\S+)\s+(\S+)\s+" +
                                     r"(\S+)\s+(\S+)\s+(\S+)", line)
                    if match is not None:
                        (pkg, flav, ver, tbl, instdir, instid) = match.groups()
                        manifest.addRecord(pkg, flav, ver, tbl,instdir,instid)
                    else:
                        self.badFileSyntax("unrecognized syntax: " + line)
                        continue
                
        finally:
            mf.close()
            self.openfiles.pop(-1)
            self.visited.append(file)
            
        

    def load(self, manifest):
        """load records into a manifest.

        Loading will start by opening the directive file corresponding to
        the name, version, and flavor associated with the manifest.  

        @param manifest   the Manifest object to add to
        @param file       a manifest directive file
        """
        (pkgname, version, flavor) = manifest.getNameVerFlav()
        (mfilename, pkgpath) = self.getFileFor(pkgname, version, flavor)
        self.loadFromFile(manifest, mfilename, pkgpath)


    def lookupCurrent(self, pkgname):
        """look up the current version of and relative path to a given
        package. 

        This specifically finds the path to the package name directory
        (containing the various version directories) relative to the
        base directory.
        @param pkgname   the name of the package to look up
        @return an array [] where the first element is the version,
                    the second is the relative path (which may be an empty
                    string), and the third (which may be empty) is the
                    package directory (overriding the default pkg/ver
                    pattern)
        """
        if self.pkgPath.has_key(pkgname):
            return self.pkgPath[pkgname]

        cf = open(os.path.join(self.basedir, self.currentfile))
        try: 
            parts = []
            for line in cf:
                line = line.strip()
                if line.startswith('#'):
                    continue

                parts = re.findall(r"\S+", line)
                if len(parts) > 0 and parts[0] == pkgname:
                    break
        finally:
            cf.close()

        out = []
        if len(parts) < 3 or parts[0] != pkgname:
            return out

        out.append(parts[2])

        out.append('')
        if len(parts) > 3:
            out[1] = parts[3]

        out.append('')
        if len(parts) > 4:
            out[2] = parts[4]

        self.pkgPath[pkgname] = out
        return out

    def getFileFor(self, pkgname, version=None, flavor="generic"):
        """determine the path to the manifest directive file for a
        specified package along with the extra path to the package

        @return  a 2-element tuple containing the file path (or None
                   if that path cannot be determined because version
                   is None and pkgname is not in the current file)
                   and the extra package path (or None if there is
                   none).
        """
        lu = self.lookupCurrent(pkgname)
        if version is None:
            if len(lu) == 0:
                return None
            version = lu[0]
        
        file = self.basedir
        pkgpath = None
        if len(lu) > 1 and len(lu[1]) > 0:
            pkgpath = lu[1]            
            file = os.path.join(file, pkgpath)

        if len(lu[2]) == 0:
            # form default package directory 
            lu[2] = os.path.join(pkgname, version)
        
        file = os.path.join(file, lu[2])
        if flavor is not None and flavor != '' and flavor != "generic":
            file = os.path.join(file, flavor)

        file = os.path.join(file, self.manfile)
        return (file, pkgpath)

    def badFileSyntax(self, msg, manifest):
        if self.strict:
            raise RuntimeError, msg
        else:
            manifest.addComment(msg)

def EUPSManifestService():
    path = os.environ["PATH_INFO"]
    if (path is None or len(path) == 0):
        raise ValueError, "no manifest file provided"

#    sys.stderr.write("path: %s" % path)

    ldr = Loader("/lsst/softstack/pkgs")
    ldr.strict = False

    if path.endswith(".manifest"):
        path = path[:-len(".manifest")]
    if path.startswith("/"):
        path = path[1:]

    dir = None
    if path.find("/") > 0:
#        (dir, path) = path.rsplit("/", 1)
        dir = os.path.dirname(path)
        path = os.path.basename(path)

    if path.find("-") >= 0:
        (pkg, version) = path.split("-", 1)
    else:
        pkg = path
        lu = ldr.lookupCurrent(pkg)
        if lu is None or len(lu) == 0:
            raise ValueError, pkg + ": current version for package not found: "
        version = lu[0]

    if len(pkg) == 0 or len(version) == 0:
        raise ValueError, "bad manifest file name: " + os.environ["PATH_INFO"]

    flavor = dir
    if flavor is None or len(flavor) == 0:  flavor = "generic"
    out = Manifest(pkg, version, flavor)
    ldr.load(out)

    sys.stdout.write("Content-type: text/plain\n\n")
    out.printRecord(sys.stdout)
    sys.stdout.close()

def test():
    test = Manifest("fw", "0.3")
    ldr = Loader(".")
    ldr.strict = False
    ldr.load(test)
    test.printRecord(sys.stdout)
    
if __name__ == "__main__":
    EUPSManifestService()
    
