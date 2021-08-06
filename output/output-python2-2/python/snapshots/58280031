#!/usr/bin/python

"""
wrapper around "rsync --server" to maintain history of uploaded files
"""

# add following line to public key in .ssh/authorized_keys to use
# command="~/rsync_server_wrapper.py",no-port-forwarding,no-X11-forwarding,no-pty 

import datetime
import os
import re
import shutil
import subprocess
import tempfile
import time
import unittest

# check its there
subprocess.check_call


DATE_FORMAT = "%Y%m%d"


def in_date_format(s):
    try:
        time.strptime(s, DATE_FORMAT)
        return True
    except ValueError:
        return False


def get_latest_backup_dir(dirs):
    backup_dirs = [d for d in dirs if in_date_format(d)]
    backup_dirs.sort()
    return backup_dirs[-1]


def main(root, call, now):
    root = os.path.abspath(root)
    work_dir = os.path.join(root, "work")
    latest_dir = os.path.join(root, get_latest_backup_dir(os.listdir(root)))
    # link-dest needs to be absolute path
    call(["rsync", "--server", "-vlogDtprRz",
          "--link-dest", latest_dir, ".", work_dir])
    if now.hour < 12:
        now -= datetime.timedelta(days=1)
    new_dir = os.path.join(root, now.strftime(DATE_FORMAT))
    if os.path.exists(new_dir):
        shutil.rmtree(new_dir)
    os.rename(work_dir, new_dir)


class Tests(unittest.TestCase):

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
        self.assertEqual(get_latest_backup_dir(["20060322", "20060321"]),
                         "20060322")
        self.assertRaises(IndexError, get_latest_backup_dir, [])

    def test_main(self):
        self.assertRaises(Exception, get_latest_backup_dir, self.tempdir)
        work_dir = os.path.join(self.tempdir, "work")
        def call(cmd):
            for i, s in enumerate(cmd):
                if s == "--link-dest":
                    link_dest_arg_index = i + 1
                    break
            self.assertEqual(cmd[:2], ["rsync", "--server"])
            self.assertEqual(cmd[link_dest_arg_index], latest_dir)
            self.assertEqual(cmd[-1], work_dir)
            os.mkdir(work_dir)
        now = datetime.datetime(2006, 3, 20, 18, 0)
        
        # currently no support if dont have at least one backup
        self.assertRaises(Exception, main, self.tempdir, call, now)
        
        
        latest_dir = os.path.join(self.tempdir,
                         datetime.date(2006, 3, 20).strftime(DATE_FORMAT))
        os.mkdir(latest_dir)
        main(self.tempdir, call, now)
        now_dir = os.path.join(self.tempdir, now.strftime(DATE_FORMAT))
        self.assert_(os.path.isdir(now_dir))
        
        latest_dir = os.path.join(self.tempdir, 
                         datetime.date(2006, 3, 20).strftime(DATE_FORMAT))
        # go again to confirm will handle existing dir
        main(self.tempdir, call, now)
        self.assert_(os.path.isdir(now_dir))
        
        # backup before 12 counts as yesterdays
        now = datetime.datetime(2006, 3, 20, 8, 0)
        main(self.tempdir, call, now)
        now_dir = os.path.join(self.tempdir,
                      datetime.date(2006, 3, 19).strftime(DATE_FORMAT))        
        self.assert_(os.path.isdir(now_dir))


if __name__ == "__main__":
    main("/home/tiber/rsync2",
         subprocess.check_call,
         datetime.datetime.utcnow())

