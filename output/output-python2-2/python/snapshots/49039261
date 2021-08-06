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
import os
from subprocess import Popen, PIPE
import depends
import utils


__revision__ = "$Rev"

def create_debootstrap(repo, packages=None):
    """create_debootstrap(repo, packages=None) -> tuple(list, list)

    Create a debootstrap with the base system plus the packages and their
    dependencies.

    """

    # Get the dependencies of all the passed packages
    deps = repo.get_dependencies_for_list(packages)
    if deps is None:
        deps = set()   # If deps is None we put a empty set for the merge_lists
    merged_list = repo.merge_lists(deps)
    package_list = ','.join(merged_list)

    # Call the debootstrap program and pray ;-)
    proc = Popen(["/usr/sbin/debootstrap", "--include=%s" % package_list, \
                 "--components=%s" % repo.components, repo.codename, \
                 "/tmp/sources", repo.mirror], \
                 stdin=PIPE, stdout=PIPE, stderr=PIPE, close_fds=True)

    ret = proc.wait()

    if ret > 0:
        output = proc.stdout.readlines()
        errors = proc.stderr.readlines()
        return (output, errors)

    return None


def create_squashfs():
    """create_squashfs() -> bool

    Create a squashfs file from a sources directory

    """

    binary = 'mksquashfs'
    sources = '/tmp/sources'
    filesystem = '/tmp/filesystem.squashfs'

    mksquashfs = utils.get_path(binary)
    if mksquashfs is None:
        return False

    # Remove olds squashfs files if any
    if os.path.isfile(filesystem):
        os.remove(filesystem)
    proc = Popen([mksquashfs, sources, filesystem], \
                  stdin=PIPE, stdout=PIPE, stderr=PIPE, close_fds=True)
    ret = proc.wait()
    if ret == 0:
        return True
    else:
        return False


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
    repo = depends.Depends(mirror, codename)

    pkg_name = []
    if len(sys.argv) > 1:
        pkg_name = sys.argv[1:]

    ret = create_debootstrap(repo, pkg_name)
    if ret is not None:
        output, errors = ret
        print "Outputs: \n"
        for line in output:
            print line
        print "\nErrors: \n"
        for line in errors:
            print line
    else:
        ret = create_squashfs()
        if ret is False:
            print >> sys.stderr, \
                     "Error: It was imposible to create a squashfs file"
    

if __name__ == '__main__':

    test_it()

# vim:ai:et:sts=4:tw=80:sw=4:
