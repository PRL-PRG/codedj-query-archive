#!/usr/bin/env python
# -*- coding: utf-8 -*-
# vim: sw=4 ts=4 fenc=utf-8
# =============================================================================
# $Id: setup.py 6 2008-01-14 13:06:00Z s0undt3ch $
# =============================================================================
#             $URL: http://devnull.ufsoft.org/svn/TracAdsPanel/trunk/setup.py $
# $LastChangedDate: 2008-01-14 13:06:00 +0000 (Mon, 14 Jan 2008) $
#             $Rev: 6 $
#   $LastChangedBy: s0undt3ch $
# =============================================================================
# Copyright (C) 2008 UfSoft.org - Pedro Algarvio <ufs@ufsoft.org>
#
# Please view LICENSE for additional licensing information.
# =============================================================================

import re
from setuptools import setup, find_packages

setup(name="TracAdsPanel",
      version='0.1.0',
      author="Pedro Algarvio",
      author_email='ufs@ufsoft.org',
      description='Trac plugin designed to display ads on your Trac environment',
      long_description=re.sub(r'(\.\.[\s]*[\w]*::[\s]*[\w+]*\n)+', r'::\n', open('README.txt').read()),
      packages=find_packages(),
      entry_points = {
        'trac.plugins': [ 'adspannel = adspanel' ]
      }
)
