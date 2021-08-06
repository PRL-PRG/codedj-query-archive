#!/usr/bin/python

"""
Usage: bcfg2cp file dst
copies file to dst/file/file for bcfg2 Cfg structure
"""

import distutils.dir_util
import distutils.file_util
import os
import sys

srcs = sys.argv[1:-1]
dst = sys.argv[-1]

for src in srcs:
    basename = os.path.basename(src)
    distutils.dir_util.mkpath(os.path.join(dst, basename))
    distutils.file_util.copy_file(src, os.path.join(dst, basename, basename))


