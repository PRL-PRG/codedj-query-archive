#!/usr/bin/python

# Copyright (C) 2006, Red Hat, Inc.
#
# This program is free software; you can redistribute it and/or modify
# it under the terms of the GNU General Public License as published by
# the Free Software Foundation; either version 2 of the License, or
# (at your option) any later version.
#
# This program is distributed in the hope that it will be useful,
# but WITHOUT ANY WARRANTY; without even the implied warranty of
# MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
# GNU General Public License for more details.
#
# You should have received a copy of the GNU General Public License
# along with this program; if not, write to the Free Software
# Foundation, Inc., 51 Franklin St, Fifth Floor, Boston, MA  02110-1301  USA

from sugar.activity import bundlebuilder
import shutil
import os

#We are removing the symlink to common and replacing it with the real common folder
os.system('unlink common')
shutil.copytree('../common', 'common')

try:
    bundlebuilder.start('TamTamMini')
except Exception, err:
    print err
    os.system('rm -rf common')
    os.system('ln -s ../common common')
#Restore the symlink
os.system('rm -rf common')
os.system('ln -s ../common common')
