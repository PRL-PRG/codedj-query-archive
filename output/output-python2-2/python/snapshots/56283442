import os
import shutil
import unittest

from zope.testing import doctest, module
import zope.component
import zope.component.testing
import zope.component.eventtesting
import zc.async.partial
import zc.async.subscribers

def modSetUp(test):
    zope.component.testing.setUp(test)
    module.setUp(test, 'zc.async.doctest_test')

def modTearDown(test):
    module.tearDown(test)
    zope.component.testing.tearDown(test)

def uuidSetUp(test):
    test.globs['old_instance_home'] = os.environ.get("INSTANCE_HOME")
    os.environ['INSTANCE_HOME'] = os.path.join(os.path.dirname(
        zc.async.interfaces.__file__), '_test_tmp')
    os.mkdir(os.environ['INSTANCE_HOME'])
    os.mkdir(os.path.join(os.environ['INSTANCE_HOME'], 'etc'))

def uuidTearDown(test):
    shutil.rmtree(os.environ['INSTANCE_HOME'])
    if test.globs['old_instance_home'] is None:
        del os.environ['INSTANCE_HOME']
    else:
        os.environ['INSTANCE_HOME'] = test.globs['old_instance_home']
    del test.globs['old_instance_home']

def readmeSetUp(test):
    modSetUp(test)
    uuidSetUp(test)
    zope.component.eventtesting.setUp(test)
    test.globs['installerAndNotifier'] = (
        zc.async.subscribers.basicInstallerAndNotifier)
    from zc.async import instanceuuid
    instanceuuid.UUID = instanceuuid.getUUID()
    zope.component.provideUtility(instanceuuid.UUID, name='instance')

def altReadmeSetUp(test):
    modSetUp(test)
    uuidSetUp(test)
    zope.component.eventtesting.setUp(test)
    test.globs['installerAndNotifier'] = (
        zc.async.subscribers.installerAndNotifier)
    from zc.async import instanceuuid
    instanceuuid.UUID = instanceuuid.getUUID()
    zope.component.provideUtility(instanceuuid.UUID, name='instance')

def readmeTearDown(test):
    r = test.globs.get('faux')
    if r:
        for when, eventname, callable in r.triggers:
            if eventname == 'shutdown': # test didn't run to completion
                # let's clean up
                callable()
    uuidTearDown(test)
    modTearDown(test)

def test_instanceuuid():
    """This module provides access to a UUID that is intended to uniquely
    identify this software instance.  Read the `msg` value below for more
    information.
    
    The uuid is generated and then stashed in a file.  It only works if
    the INSTANCE_HOME environment variable is set to a folder that has an
    `etc` folder in it--a standard Zope set up.  For this test, we mock it
    up in uuidSetUp and uuidTearDown below.
    
        >>> import zc.async.instanceuuid
        >>> import uuid
        >>> isinstance(zc.async.instanceuuid.UUID, uuid.UUID)
        True
        >>> (zc.async.instanceuuid.getUUID() ==
        ...  zc.async.instanceuuid.UUID ==
        ...  zc.async.instanceuuid.getUUID())
        True

    uuid.UUIDs now provide zc.async.interfaces.IUUID
    
        >>> import zc.async.interfaces
        >>> zc.async.interfaces.IUUID.implementedBy(uuid.UUID)
        True
        >>> zc.async.interfaces.IUUID.providedBy(zc.async.instanceuuid.UUID)
        True

    That's a bit invasive, but now you can register the instance UUID as
    a utility and get it back out as something that provides
    zc.async.interfaces.IUUID.
    
        >>> import zope.component
        >>> zope.component.provideUtility(
        ...     zc.async.instanceuuid.UUID, name='instance')
        >>> id = zope.component.getUtility(
        ...     zc.async.interfaces.IUUID, 'instance')
        >>> id is zc.async.instanceuuid.UUID
        True

    (Unfortunately you can't register a utility to provide a class, or I
    would have done that...though maybe that's not unfortunate :-) )

    """

def test_suite():
    return unittest.TestSuite((
        doctest.DocTestSuite(setUp=uuidSetUp, tearDown=uuidTearDown),
        doctest.DocFileSuite(
            'partial.txt',
            'partials_and_transactions.txt',
            'datamanager.txt',
            setUp=modSetUp, tearDown=modTearDown,
            optionflags=doctest.INTERPRET_FOOTNOTES),
        doctest.DocFileSuite(
            'README.txt',
            setUp=readmeSetUp, tearDown=readmeTearDown,
            optionflags=doctest.INTERPRET_FOOTNOTES),
        doctest.DocFileSuite(
            'README.txt',
            setUp=altReadmeSetUp, tearDown=readmeTearDown,
            optionflags=doctest.INTERPRET_FOOTNOTES),
        ))


if __name__ == '__main__':
    unittest.main(defaultTest='test_suite')
