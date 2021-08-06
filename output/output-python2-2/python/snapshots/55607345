#!/usr/bin/env python
# -*- coding: utf-8 -*-
# vim: sw=4 ts=4 fenc=utf-8
# =============================================================================
# $Id: setup.py 125 2008-09-02 01:57:56Z s0undt3ch $
# =============================================================================
#             $URL: http://devnull.ufsoft.org/svn/TracAdsPanel/trunk/setup.py $
# $LastChangedDate: 2008-09-02 02:57:56 +0100 (Tue, 02 Sep 2008) $
#             $Rev: 125 $
#   $LastChangedBy: s0undt3ch $
# =============================================================================
# Copyright (C) 2008 UfSoft.org - Pedro Algarvio <ufs@ufsoft.org>
#
# Please view LICENSE for additional licensing information.
# =============================================================================

import re
from setuptools import setup

setup(name="TracGoogleAds",
      version='0.1.0',
      author="Pedro Algarvio",
      author_email='ufs@ufsoft.org',
      description='Trac plugin designed to display ads on your Trac environment',
      long_description=re.sub(r'(\.\.[\s]*[\w]*::[\s]*[\w+]*\n)+', r'::\n',
                              open('README.txt').read()),
      packages=['tracext', 'tracext.google', 'tracext.google.ads'],
      namespace_packages=['tracext', 'tracext.google'],
      package_data = {'tracext.google.ads': ['templates/*.html',
                                             'htdocs/*.css']},
      include_package_data = True,
      keywords = "trac plugin google ads",
      entry_points = """
      [trac.plugins]
        tracext.google.ads = tracext.google.ads
        tracext.google.ads.admin = tracext.google.ads.admin
        tracext.google.ads.web_ui = tracext.google.ads.web_ui
      """,
      classifiers = [
        'Framework :: Trac',
      ]
)
