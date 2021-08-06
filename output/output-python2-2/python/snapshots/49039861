# -*- coding: UTF-8 -*-
"""Fetch seeds from a URL collection or from bzr."""

# Copyright (c) 2004, 2005, 2006 Canonical Ltd.
#
# Germinate is free software; you can redistribute it and/or modify it
# under the terms of the GNU General Public License as published by the
# Free Software Foundation; either version 2, or (at your option) any
# later version.
#
# Germinate is distributed in the hope that it will be useful, but
# WITHOUT ANY WARRANTY; without even the implied warranty of
# MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the GNU
# General Public License for more details.
#
# You should have received a copy of the GNU General Public License
# along with Germinate; see the file COPYING.  If not, write to the Free
# Software Foundation, Inc., 51 Franklin St, Fifth Floor, Boston, MA
# 02110-1301, USA.

import os
import tempfile
import atexit
import logging
import urlparse
import urllib2
import shutil

bzr_cache_dir = None

def _cleanup_bzr_cache(directory):
    shutil.rmtree(directory, ignore_errors=True)

def open_seed(seed_base, seed_file, bzr=False):
    if not seed_base.endswith('/'):
        seed_base += '/'
    if bzr:
        global bzr_cache_dir
        if bzr_cache_dir is None:
            bzr_cache_dir = tempfile.mkdtemp(prefix='germinate-')
            atexit.register(_cleanup_bzr_cache, bzr_cache_dir)
            # https://launchpad.net/products/bzr/+bug/39542
            if seed_base.startswith('http:'):
                operation = 'get'
                logging.info("Fetching branch of %s", seed_base)
            else:
                operation = 'checkout'
                logging.info("Checking out %s", seed_base)
            command = ('bzr %s %s %s' %
                       (operation, seed_base,
                        os.path.join(bzr_cache_dir, 'checkout')))
            status = os.system(command)
            if status != 0:
                raise RuntimeError("Command failed with exit status %d:\n"
                                   "  '%s'" % (status, command))
        try:
            return open(os.path.join(bzr_cache_dir, 'checkout', seed_file))
        except IOError:
            logging.warning("Could not open %s from checkout of %s",
                            seed_file, seed_base)
            return None
    else:
        url = urlparse.urljoin(seed_base, seed_file)
        logging.info("Downloading %s", url)
        try:
            req = urllib2.Request(url)
            req.add_header('Cache-Control', 'no-cache')
            req.add_header('Pragma', 'no-cache')
            return urllib2.urlopen(req)
        except urllib2.URLError:
            logging.warning("Could not open %s", url)
            return None
