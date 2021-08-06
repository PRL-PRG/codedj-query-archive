#!/usr/bin/env python
# -*- coding: utf-8 -*-

# Authors: 
#     Juan Jesús Ojeda Croissier (juanje) <jojeda@emergya.es>   
#
# Last modified: 
#     $Date$ 
#     $Author$
#

""" Utils is a bunch os utils functions

List of functons:
 - get_path() -> string
 - chroot_uname() -> string

"""


import sys
import os.path
import os

__revision__ = '0.01'


def get_path(binary):
    """get_path(binary) -> bool

    Check if the binary is installed in the path of the user.
    If there it is, returns the path, if not, returns None

    """

    # Get the PATH of the current user
    full_path = os.environ['PATH']
    path_list = full_path.split(':')

    for path in path_list:
        bin = os.path.join(path, binary)
        if os.path.isfile(bin):
            return bin

    print >> sys.stderr, \
             "Error: It was imposible to find the binary %s in your PATH=%s\n" \
             % (binary, full_path), \
             "Please, check if you have installed this program"
    return None
    

def chroot_uname(chroot_dir='/tmp/sources'):
    """chroot_uname(chroot_dir='/tmp/sources') -> string

    Get the version and subversion of the latest kernel installed in the chroot
    directory.

    """

    kernel_dir = '/boot/'
    kernel_prefix = 'vmlinuz-'

    chroot_kernel_dir = os.path.join(chroot_dir, kernel_dir)
    dir_ls = os.listdir(chroot_kernel_dir)

    kernels_list = []
    for file_name in dir_ls:
        if file_name.startswith(kernel_prefix):
            kernels_list.append(file_name)

    last_kernel = kernels_list.pop()
    kernel_split = last_kernel.split('-', 1)  # ['vmlinuz', '2.6.??-??-386']
    uname = kernel_split[1]                   # '2.6.??-??-386'

    return uname


# vim:ai:et:sts=4:tw=80:sw=4:
