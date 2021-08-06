#!/usr/bin/python
# Copyright 2007 Tristan Hill

# This program is free software: you can redistribute it and/or modify
# it under the terms of the GNU General Public License as published by
# the Free Software Foundation, either version 3 of the License, or
# (at your option) any later version.
#
# This program is distributed in the hope that it will be useful,
# but WITHOUT ANY WARRANTY; without even the implied warranty of
# MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
# GNU General Public License for more details.

# You should have received a copy of the GNU General Public License
# along with this program.  If not, see <http://www.gnu.org/licenses/>.

"""
wrapper around "rsync --server" to maintain history of uploaded files
"""

# add following line to public key in .ssh/authorized_keys to use
# command="~/rsync_server_wrapper.py dir",no-port-forwarding,no-X11-forwarding,no-pty 
#
# client command example:
# rsync -avzRe "ssh -i backup.key" --stats --link-dest overridden \
#     /home/stan/Documents remotehost:overridden


import datetime
import os
import re
import shutil
import subprocess
import sys
import tempfile
import time
import unittest


DATE_FORMAT = "%Y%m%d"


def in_date_format(s):
    try:
        time.strptime(s, DATE_FORMAT)
        return True
    except ValueError:
        return False


def get_latest_backup_dir(root):
    dirs = os.listdir(root)
    backup_dirs = [d for d in dirs if in_date_format(d)]
    backup_dirs.sort()
    if len(backup_dirs) > 0:
       return os.path.join(root, backup_dirs[-1])
    else:
       return None


def main(root, call, now):
    root = os.path.abspath(os.path.expanduser(root))
    work_dir = os.path.join(root, "work")
    # link-dest needs to be absolute path
    latest_dir = get_latest_backup_dir(root)
    if latest_dir is not None:
       link_dest_args = ["--link-dest", latest_dir]
    else:
       link_dest_args = []
    call(["rsync", "--server", "-vlogDtprRz"] + link_dest_args +
         [".", work_dir])
    if now.hour < 12:
        now -= datetime.timedelta(days=1)
    new_dir = os.path.join(root, now.strftime(DATE_FORMAT))
    if os.path.exists(new_dir):
        shutil.rmtree(new_dir)
    os.rename(work_dir, new_dir)


class Tests(unittest.TestCase):

    class MockCall(object):

        def __init__(self, tempdir):
            self.tempdir = tempdir
            self.cmd = None

        def call(self, cmd):
            self.cmd = cmd
            os.mkdir(os.path.join(self.tempdir, "work"))

    def setUp(self):
        unittest.TestCase.setUp(self)
        self.tempdir = tempfile.mkdtemp()

    def tearDown(self):
        shutil.rmtree(self.tempdir)
        unittest.TestCase.tearDown(self)

    def test_in_date_format(self):
        self.assert_(in_date_format("20060320"))
        self.assertFalse(in_date_format("work"))

    def test_get_latest_backup_dir(self):
        self.assertEqual(get_latest_backup_dir(self.tempdir), None)
        def mkdir(dirname):
            os.mkdir(os.path.join(self.tempdir, dirname))
        mkdir("20060322")
        mkdir("20060321")
        self.assertEqual(get_latest_backup_dir(self.tempdir),
                         os.path.join(self.tempdir, "20060322"))

    def test_main_empty(self):
        now = datetime.datetime(2006, 3, 20, 18, 0)

        mock = Tests.MockCall(self.tempdir)
        main(self.tempdir, mock.call, now)
        self.assertEqual(mock.cmd[:-1],
                         ["rsync", "--server", "-vlogDtprRz", "."])

    def test_main(self):
        now = datetime.datetime(2006, 3, 20, 18, 0)
        
        latest_dir = os.path.join(self.tempdir,
                         datetime.date(2006, 3, 20).strftime(DATE_FORMAT))
        os.mkdir(latest_dir)

        mock = Tests.MockCall(self.tempdir)        
        main(self.tempdir, mock.call, now)
        self.assertEqual(mock.cmd[:-1],
                         ["rsync", "--server", "-vlogDtprRz", "--link-dest",
                          latest_dir, "."])
        
        now_dir = os.path.join(self.tempdir, now.strftime(DATE_FORMAT))
        self.assert_(os.path.isdir(now_dir))
        
        latest_dir = os.path.join(self.tempdir, 
                         datetime.date(2006, 3, 20).strftime(DATE_FORMAT))
        # go again to confirm will handle existing dir
        main(self.tempdir, mock.call, now)
        self.assert_(os.path.isdir(now_dir))

        # backup before 12 counts as yesterdays
        now = datetime.datetime(2006, 3, 20, 8, 0)
        main(self.tempdir, mock.call, now)
        now_dir = os.path.join(self.tempdir,
                      datetime.date(2006, 3, 19).strftime(DATE_FORMAT))        
        self.assert_(os.path.isdir(now_dir))


def check_call(*popenargs, **kwargs):
    # copied from 2.5's subprocess.py
    retcode = subprocess.call(*popenargs, **kwargs)
    cmd = kwargs.get("args")
    if cmd is None:
        cmd = popenargs[0]
    if retcode:
        raise RuntimeError(retcode, cmd)
    return retcode


if __name__ == "__main__":
    main(sys.argv[1],
         check_call,
         datetime.datetime.utcnow())


