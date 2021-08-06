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
import os
import uuid
import zope.interface
import zc.async.interfaces

# test for this file is in tests.py

msg = """
------------------------------------------------------------------------
The value above (and this file) is created and used by the zc.async
package. It is intended to uniquely identify this software instance when
it is used to start a zc.async dispatcher.  This allows multiple
dispatchers, each in its own software instance, to connect to a single
database to do work.

In order to decide where to look for this file (or to create it, if
necessary), the module looks in ``os.environ['ZC_ASYNC_UUID']`` for a
file name.

If you are using zdaemon (http://pypi.python.org/pypi/zdaemon) to
daemonize your process, you can set this in a zdaemon environment section
of your zdaemon.conf. Supervisor (http://supervisord.org/) also provides
this functionality. Other similar tools probably do as well.

If the ``ZC_ASYNC_UUID`` is not found in the environment, it will use
``os.path.join(os.getgwd(), 'uuid.txt')`` as the file name.

To get a new identifier for this software instance, delete this file,
restart Python, and import zc.async.instanceuuid.  This file will be
recreated with a new value.
"""

zope.interface.classImplements(uuid.UUID, zc.async.interfaces.IUUID)

key = 'ZC_ASYNC_UUID'

def getUUID():
    file_name = os.environ.get(key)
    if not file_name:
        file_name = os.path.join(os.getcwd(), 'uuid.txt')
    if os.path.exists(file_name):
        f = open(file_name, 'r')
        UUID = uuid.UUID(f.readline().strip())
        f.close()
    else:
        UUID = uuid.uuid1()
        f = open(file_name, 'w')
        f.writelines((str(UUID), msg))
        f.close()
    return UUID

UUID = getUUID()
