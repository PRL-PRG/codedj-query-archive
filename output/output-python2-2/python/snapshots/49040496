#!/usr/bin/env python
# -*- coding: utf-8 -*-

# Authors: 
#     Juan Jesús Ojeda Croissier (juanje) <jojeda@emergya.es>   
#
# Last modified: 
#     $Date$ 
#     $Author$
#

""" GLIG is a Guadalinex Live Image Generator.

This program generate a chroot system from a debian repository in order
to create a squashfs compressed image of that system for being able to run
as a live GNU/Linux system.

NOTE: It's not just a draft. It is NOT functional at all. It just does some
things we need to control before doing the main stuff.

"""  

import sys
import apt
import apt_pkg
import os.path
import urllib2
import bz2


__revision__ = "0.01"

def get_packages(mirror, codename, component='main', arch='i386'):
    """get_packages(mirror, codename[, component="main"[, arch="i386"]])
    
    get_packages get the Packages.bz2 from the mirror. Either local or remote.
    After decompress the file it writes a file /tmp/status with the content.
    
    TODO: To support more kind of files: Packages, Packages.gz
    
    """
    
    # Create a URI like 
    # http://mirror.server.org/ubuntu/dists/edgy/main/binary-i386/Packages.bz2
    uri = os.path.join(mirror, 'dists', codename, component, \
                       'binary-' + arch, 'Packages.bz2')

    # Extract the protocol from the URI
    protocol, path = urllib2.splittype(uri)

    # Get the Packages.bz2
    if protocol in ['http', 'ftp']:
        packages = urllib2.urlopen(uri)
    elif protocol == 'file':
        packages = open(path, 'r')
    else:
        # FIXME: I have to improve/create the logs system
        print "Error: must be http:// , ftp:// or file://" >> sys.stderr
        return -1        # A code of error 

    # Decompress the Packages.bz2
    compress_file = packages.read()
    plane = bz2.decompress(compress_file)

    # Create the status file
    status = open('/tmp/status', 'w')
    status.write(plane)

    # Close the open file descriptors
    status.close()
    packages.close()


def get_base_dependencies(mirror, codename):
    """get_base_dpendencies(mirror, codename) -> list

    Get the list of packages which the debootstrap try to install
    
    """

    from subprocess import Popen, PIPE

    # Simulate a standar debootstrap for getting the list of packages
    # are going to be installed as a base system
    proc = Popen(["/usr/sbin/debootstrap", "--print-debs", codename, \
                 "/tmp/test", mirror], stdin=PIPE, stdout=PIPE, \
                 stderr=PIPE, close_fds=True)
    output = proc.stdout.readline().strip()
    packages = output.split()

    return packages


def get_all_dependencies(package_name):
    """get_all_dependencies(package_name) -> set

    Get a set of packages from which the package
    package_name depends.

    """

    # Set the Packages file as apt's status file
    status_file = "/tmp/status"
    if not os.path.isfile(status_file):
        print "Error: %s doesn't exist" % status_file >> sys.stderr
        return set()
    apt_pkg.Config.Set("Dir::State::status", status_file)
    cache = apt.Cache()
    pkg = cache[package_name]
 
    def get_dependencies(cache, pkg, deps, key):
        """get_dependencies(cache, pkg, key) -> set

        Get the dependencies or predependencies for a specific package.

        """

        candidate_ver = cache._depcache.GetCandidateVer(pkg._pkg)
        if candidate_ver == None:
            return deps
        depends_list = candidate_ver.DependsList
        if depends_list.has_key(key):
            for depends_ver_list in depends_list[key]:
                for dep in depends_ver_list:
                    if cache.has_key(dep.TargetPkg.Name):
                        if pkg.name != dep.TargetPkg.Name and \
                           not dep.TargetPkg.Name in deps:
                            deps.add(dep.TargetPkg.Name)
                            get_dependencies(cache, \
                            cache[dep.TargetPkg.Name], deps, key)
        return deps
     
    deps = set()
   
    deps = get_dependencies(cache, pkg, deps, "Depends")
    deps = get_dependencies(cache, pkg, deps, "PreDepends")

    return deps


def create_debootstrap(mirror, codename, packages=None, components='main'):
    """create_debootstrap(mirror, codename, packages=None, component='main')

    Create a debootstrap with the base system plus the packages and their
    dependencies.

    """

    from subprocess import Popen, PIPE

    # Get the debootstrap base package list
    base = get_base_dependencies(mirror, codename)
    depends = set(base)

    # Get the dependencies of all the passed packages
    if packages is not None:
        for pkg in packages:
            sub_depends = get_all_dependencies(pkg)
            depends.update(sub_depends)
    package_list = ','.join(depends)

    # Call the debootstrap program and pray ;-)
    proc = Popen(["/usr/bin/sudo", "/usr/sbin/debootstrap", "--include=%s" % package_list, \
                 "--components=%s" % components, "--print-debs", codename, \
                 "/tmp/test", mirror], \
                 stdin=PIPE, stdout=PIPE, stderr=PIPE, close_fds=True)
    output = proc.stdout.readline().strip()
    errors = proc.stderr.readline().strip()

    return (output, errors)


def test_it():
    """test_it() -> None

    This is a dumpy function just for testing purposes.
    It takes a package from the ARGV and search all the packages
    it the debootstrap has to install. The base system, the new packages
    passed and their dependencies.

    """

    # mirror = 'http://mirror.emergya.info/ubuntu'
    mirror = 'file:///var/mirror/ubuntu'
    codename = 'edgy'
    get_packages(mirror, codename)

    # FIXME: Do this in a better way
    pkg_name = []
    pkg_name.append(sys.argv[1])

    output, errors = create_debootstrap(mirror, codename, pkg_name)
    print "Outputs: \n\n" + output
    print "Errors: \n\n" + errors
    
    # depends = get_base_dependencies(mirror, codename)
    # depends =  get_all_dependencies(pkg_name, depends)
    # print ", ".join(depends)


if __name__ == '__main__':

    test_it()
