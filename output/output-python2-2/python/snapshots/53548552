#! /usr/bin/env python
# -*- coding: utf-8 -*-

try:
    from sugar.activity import bundlebuilder
    bundlebuilder.start()
except ImportError:
    import os
    os.system("find ./ | sed 's,^./,Clock/,g' > MANIFEST")
    os.chdir("..")
    os.system("zip -r Clock.xo Clock")
    os.system("mv Clock.xo ./Clock")
    os.chdir("Clock")
