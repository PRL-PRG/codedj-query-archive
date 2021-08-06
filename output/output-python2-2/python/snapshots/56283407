import types

import zc.twist
import zope.component
import ZODB.interfaces

import zc.async.interfaces
import zc.async.job
import zc.async.instanceuuid

# These functions accomplish what configure.zcml does; you don't want both
# to be in play (the component registry will complain).

def minimal():
    # use this ``minimal`` function if you have the
    # zope.app.keyreference.persistent.connectionOfPersistent adapter
    # installed in your zope.component registry.  Otherwise use ``base``
    # below.

    # persistent object and connection -> transaction manager
    zope.component.provideAdapter(zc.twist.transactionManager)
    zope.component.provideAdapter(zc.twist.transactionManager,
                                  adapts=(ZODB.interfaces.IConnection,))

    # function and method -> job
    zope.component.provideAdapter(
        zc.async.job.Job,
        adapts=(types.FunctionType,),
        provides=zc.async.interfaces.IJob)
    zope.component.provideAdapter(
        zc.async.job.Job,
        adapts=(types.MethodType,),
        provides=zc.async.interfaces.IJob)
    zope.component.provideAdapter( # optional, rarely used
        zc.async.job.Job,
        adapts=(zc.twist.METHOD_WRAPPER_TYPE,),
        provides=zc.async.interfaces.IJob)

    # UUID for this instance
    zope.component.provideUtility(
        zc.async.instanceuuid.UUID, zc.async.interfaces.IUUID)

def base():
    # see comment in ``minimal``, above
    minimal()
    zope.component.provideAdapter(zc.twist.connection)