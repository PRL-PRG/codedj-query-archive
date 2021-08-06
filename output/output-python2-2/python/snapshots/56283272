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
import persistent.interfaces
import zope.interface
import zope.component

import zc.async.interfaces

@zope.component.adapter(persistent.interfaces.IPersistent)
@zope.interface.implementer(zc.async.interfaces.IQueue)
def defaultQueueAdapter(obj):
    return obj._p_jar.root()[zc.async.interfaces.KEY]['']
