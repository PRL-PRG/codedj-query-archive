# -*- coding: utf-8 -*-

# «validation» - miscellaneous validation of user-entered data
# 
# Copyright (C) 2005 Junta de Andalucía
# Copyright (C) 2005, 2006 Canonical Ltd.
# 
# Authors:
# 
# - Antonio Olmo Titos <aolmo#emergya._info>
# - Javier Carranza <javier.carranza#interactors._coop>
# - Juan Jesús Ojeda Croissier <juanje#interactors._coop>
# - Colin Watson <cjwatson@ubuntu.com>
# 
# This file is part of Ubiquity.
# 
# Ubiquity is free software; you can redistribute it and/or modify it under
# the terms of the GNU General Public License as published by the Free
# Software Foundation; either version 2 of the License, or at your option)
# any later version.
# 
# Ubiquity is distributed in the hope that it will be useful, but WITHOUT
# ANY WARRANTY; without even the implied warranty of MERCHANTABILITY or
# FITNESS FOR A PARTICULAR PURPOSE. See the GNU General Public License for
# more details.
# 
# You should have received a copy of the GNU General Public License along
# with Ubiquity; if not, write to the Free Software Foundation, Inc., 51
# Franklin St, Fifth Floor, Boston, MA  02110-1301  USA

""" U{pylint<http://logilab.org/projects/pylint>} mark: 6.67 """

# Validation library.
# Created by Antonio Olmo <aolmo#emergya._info> on 26 jul 2005.

from string            import whitespace, uppercase
import glob
from ubiquity.settings import *

HOSTNAME_LENGTH = 1
HOSTNAME_WHITESPACE = 2
HOSTNAME_BADCHAR = 3

def check_hostname(name):

    """ Check the correctness of a proposed host name.

        @return empty list (valid) or list of:
            - C{HOSTNAME_LENGTH} wrong length.
            - C{HOSTNAME_WHITESPACE} contains white spaces.
            - C{HOSTNAME_BADCHAR} contains invalid characters."""

    import re
    result = set()

    if len (set(name).intersection(set(whitespace))) > 0:
        result.add(HOSTNAME_WHITESPACE)
    if len (name) < 3 or len (name) > 18:
        result.add(HOSTNAME_LENGTH)

    regex = re.compile(r'^[a-zA-Z0-9-]+$')
    if not regex.search(name):
        result.add(HOSTNAME_BADCHAR)

    return sorted(result)

MOUNTPOINT_NOROOT = 1
MOUNTPOINT_DUPPATH = 2
MOUNTPOINT_BADSIZE = 3
MOUNTPOINT_BADCHAR = 4
MOUNTPOINT_XFSROOT = 5
MOUNTPOINT_XFSBOOT = 6
MOUNTPOINT_UNFORMATTED = 7
MOUNTPOINT_NEEDPOSIX = 8
MOUNTPOINT_NONEWWORLD = 9

def check_mountpoint(mountpoints, size):

    """ Check the correctness of a proposed set of mountpoints.

        @return empty list (valid) or list of:
            - C{MOUNTPOINT_NOROOT} Doesn't exist root path.
            - C{MOUNTPOINT_DUPPATH} Path duplicated.
            - C{MOUNTPOINT_BADSIZE} Size incorrect.
            - C{MOUNTPOINT_BADCHAR} Contains invalid characters.
            - C{MOUNTPOINT_XFSROOT} XFS used on / (with no /boot).
            - C{MOUNTPOINT_XFSBOOT} XFS used on /boot.
            - C{MOUNTPOINT_UNFORMATTED} System filesystem not reformatted.
            - C{MOUNTPOINT_NEEDPOSIX} Non-POSIX filesystem required here.
            - C{MOUNTPOINT_NONEWWORLD} NewWorld boot partition missing."""

    import re
    result = set()
    root = False
    root_size = 0.0
    xfs_root = False
    xfs_boot = False

    for mountpoint, format, fstype, flags in mountpoints.itervalues():
        if mountpoint == 'swap':
            root_minimum_KB = MINIMAL_PARTITION_SCHEME['root'] * 1024
            break
    else:
        root_minimum_KB = (MINIMAL_PARTITION_SCHEME['root'] +
                           MINIMAL_PARTITION_SCHEME['swap']) * 1024

    if glob.glob('/lib/partman/finish.d/*newworld'):
        result.add(MOUNTPOINT_NONEWWORLD)

    seen_mountpoints = set()
    for device, (path, format, fstype, flags) in mountpoints.items():
        # TODO cjwatson 2006-09-26: Duplication from
        # partman-newworld/finish.d/newworld.
        if path == 'newworld':
            result.remove(MOUNTPOINT_NONEWWORLD)
            continue
        elif fstype is None:
            # Some other special-purpose partition we don't know about for
            # whatever reason.
            continue

        if path == '/':
            root = True
            root_size += float(size[device.split('/')[2]])
        elif '/'.join(path.split('/')[:2]) in ('/boot', '/usr', '/var'):
            root_size += float(size[device.split('/')[2]])

        if path != 'swap' and path in seen_mountpoints:
            result.add(MOUNTPOINT_DUPPATH)
        else:
            seen_mountpoints.add(path)
        regex = re.compile(r'^[a-zA-Z0-9/\-\_\+]+$')
        if not regex.search(path):
            result.add(MOUNTPOINT_BADCHAR)

        if fstype == 'xfs':
            if path == '/':
                xfs_root = True
            elif path == '/boot':
                xfs_boot = True

        if not format:
            pathtop = '/'.join(path.split('/')[:2])
            if (pathtop in ('/', '/boot', '/usr', '/var') and
                path not in ('/usr/local', '/var/local')):
                result.add(MOUNTPOINT_UNFORMATTED)

        # TODO cjwatson 2006-09-13: Duplication from
        # partman-basicfilesystems/finish.d/mountpoint_fat; as if the rest
        # of all this doesn't duplicate partman too ...
        if fstype in ('vfat', 'ntfs'):
            if path in ('/', '/boot', '/home', '/opt', '/srv', '/tmp', '/usr',
                        '/usr/local', '/var'):
                result.add(MOUNTPOINT_NEEDPOSIX)

    if not root:
        result.add(MOUNTPOINT_NOROOT)
    elif root_size < root_minimum_KB:
        result.add(MOUNTPOINT_BADSIZE)

    if '/boot' in seen_mountpoints:
        if xfs_boot:
            result.add(MOUNTPOINT_XFSBOOT)
    else:
        if xfs_root:
            result.add(MOUNTPOINT_XFSROOT)

    return sorted(result)
