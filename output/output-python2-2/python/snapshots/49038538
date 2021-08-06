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

NOTE: It's not yet a full functional program, but almost. At todays time is be
able to:
 - Generate a chrooted directory with the base system plus the packages we pass 
   as a parameters.
 - Generate a Squashfs compressed image of the chrooted directory.
 - Generate a initrd file with the kernel, modules and so on from the chrooted
   directory.

"""  

import sys
import os
from subprocess import Popen, PIPE
from shutil import copy
import depends
import utils


_svn_revision = "$Rev$"
__revision__ = _svn_revision.split()[1]


def create_debootstrap(repo, packages=None):
    """create_debootstrap(repo, packages=None) -> tuple(list, list)

    Create a debootstrap with the base system plus the packages and their
    dependencies.

    """

    # Get the dependencies of all the passed packages
    deps = repo.get_dependencies_for_list(packages)
    if deps is None:
        deps = set()   # If deps is None we put a empty set for the merge_lists
    merged_list = repo.merge_lists()
    package_list = ','.join(merged_list)
    
    # Generate the filesystem.manifest and filesystem.manifest-desktop
    repo.create_manifests()

    # Call the debootstrap program and pray ;-)
    proc = Popen(["/usr/sbin/debootstrap", "--include=%s" % package_list, \
                 "--components=%s" % repo.components, repo.codename, \
                 "/tmp/sources", repo.mirror], \
                 # stdin=PIPE, stdout=PIPE, stderr=PIPE, close_fds=True)
                 stdin=PIPE, stdout=PIPE, close_fds=True)

    ret = proc.wait()

    if ret > 0:
        output = proc.stdout.readlines()
        # errors = proc.stderr.readlines()
        # return (output, errors)
        return (output, '')

    return None


def create_squashfs(sources='/tmp/sources', \
                    filesystem='/tmp/filesystem.squashfs'):
    """create_squashfs(sources='/tmp/sources', 
                       filesystem='/tmp/filesystem.squashfs') -> bool

    Create a squashfs file from a sources directory

    """

    binary = 'mksquashfs'
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


def generate_initrd(uname=None, chroot_dir='/tmp/sources'):
    """generte_initrd(uname=None, chroot_dir='/tmp/sources') -> bool

    Generate a initramfs file inside of the chroot, in order to use the kernel,
    modules, usplash theme and so on of the chrooted system.

    This function copy the resulted initrd.gz and his vmlinuz to outside of the
    chroot directory.

    """

    files = ['vmlinuz', 'initrd.gz']
    boot_dir = '/boot'
    initrd_path = '/tmp/initrd.gz'

    mkinitramfs_binary = 'mkinitramfs'
    mkinitramfs = utils.get_path(mkinitramfs_binary)
    chrooted_mkinitramfs = utils.join_path(chroot_dir, mkinitramfs)
    if chrooted_mkinitramfs is None:
        return False

    chroot_binary = 'chroot'
    chroot = utils.get_path(chroot_binary)
    if chroot is None:
        return False

    if uname is None:
        uname = utils.chroot_uname()

    proc = Popen([chroot, chroot_dir, mkinitramfs, '-o', initrd_path, uname], \
                  stdin=PIPE, stdout=PIPE, stderr=PIPE, close_fds=True)
    ret = proc.wait()
    if ret != 0:
        return False

    chrooted_initrd = utils.join_path(chroot_dir, initrd_path)
    if not copy(chrooted_initrd, initrd_path):
        return False

    chrooted_boot = utils.join_path(chroot_dir, boot_dir)
    dest_dir = os.path.dirname(initrd_path)
    for file_ in files:
        orig_file = utils.join_path(chrooted_boot, file_)
        if not copy(orig_file, dest_dir):
            return False

    return True


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

    pkg_name = []
    if len(sys.argv) > 1:
        pkg_name = sys.argv[1:]

    repo = depends.Depends(pkg_name, mirror, codename)
    ret = create_debootstrap(repo)
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

#        ret = generate_initrd()
#        if ret is False:
#            print >> sys.stderr, \
#                     "Error: It was imposible to generate the initrd.gz file"
#

    

if __name__ == '__main__':

    test_it()
    #generate_initrd()

# vim:ai:et:sts=4:tw=80:sw=4:
