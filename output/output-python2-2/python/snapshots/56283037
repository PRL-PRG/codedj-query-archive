##############################################################################
#
# Copyright (c) 2006-2008 Zope Corporation and Contributors.
# All Rights Reserved.
#
# This software is subject to the provisions of the Zope Public License,
# Version 2.1 (ZPL).  A copy of the ZPL should accompany this distribution.
# THIS SOFTWARE IS PROVIDED "AS IS" AND ANY AND ALL EXPRESS OR IMPLIED
# WARRANTIES ARE DISCLAIMED, INCLUDING, BUT NOT LIMITED TO, THE IMPLIED
# WARRANTIES OF TITLE, MERCHANTABILITY, AGAINST INFRINGEMENT, AND FITNESS
# FOR A PARTICULAR PURPOSE.
#
##############################################################################
import zc.async.job


class Partial(zc.async.job.Job):
    """ BBB This class exists for backward compatibility with databases that
        BBB used prerelease versions of zc.async.  It has to stay forever."""

    def __init__(self, *args, **kwargs):
        raise Exception(
            "zc.async.partial.Partial exists purely"
            " for backward compatibility reasons.")
