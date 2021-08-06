#!/usr/bin/env python
# -*- coding: utf-8 -*-
# vim: sw=4 ts=4 fenc=utf-8 et
# ==============================================================================
# Copyright © 2008 UfSoft.org - Pedro Algarvio <ufs@ufsoft.org>
#
# Please view LICENSE for additional licensing information.
# ==============================================================================

import re
from setuptools import setup
import tracext.google.ads as tg

setup(
    name = tg.__package__,
    version = tg.__version__,
    author = tg.__author__,
    author_email = tg.__email__,
    description = tg.__summary__,
    license = tg.__license__,
    url = tg.__url__,
    download_url = 'http://python.org/pypi/%s' % tg.__package__,
    long_description = re.sub(r'(\.\.[\s]*[\w]*::[\s]*[\w+]*\n)+', r'::\n',
                              open('README.txt').read()),
    packages = ['tracext', 'tracext.google', 'tracext.google.ads'],
    namespace_packages = ['tracext', 'tracext.google'],
    package_data = {'tracext.google.ads': ['templates/*.html', 'htdocs/*.css']},
    include_package_data = True,
    install_requires = ['Trac>=0.11'],
    keywords = "trac plugin google ads",
    entry_points = """
    [trac.plugins]
      tracext.google.ads = tracext.google.ads
      tracext.google.ads.admin = tracext.google.ads.admin
      tracext.google.ads.web_ui = tracext.google.ads.web_ui
    """,
    classifiers = ['Framework :: Trac']
)
