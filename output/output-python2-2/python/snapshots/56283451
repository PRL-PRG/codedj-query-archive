
import os
import uuid
import zope.interface
import zc.async.interfaces

# test for this file is in tests.py

msg = """
------------------------------------------------------------------------
The value above (and this file) is created and used by the zc.async
package. It is intended to uniquely identify this software instance when
it is used to start a zc.async worker process.  This allows multiple
workers to connect to a single database to do work.  The software
expects an instance home to only generate a single process.

To get a new identifier for this software instance, delete this file and
restart Zope (or more precisely, delete this file, restart Python, and
import zc.async.instanceuuid).  This file will be recreated with a new value.
"""

zope.interface.classImplements(uuid.UUID, zc.async.interfaces.IUUID)

def getUUID():
    file_name = os.path.join(
        os.environ.get("INSTANCE_HOME"), 'etc', 'uuid.txt')
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
