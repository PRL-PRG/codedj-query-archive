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
import types

import zc.twist
import zope.component
import zope.event
import zope.component.event # yuck; as of this writing, this import causes the
                            # zope.component hook to be installed in
                            # zope.event.
import ZODB.interfaces

import zc.async.interfaces
import zc.async.job
import zc.async.queue
import zc.async.instanceuuid
import zc.async.subscribers

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
    zope.component.provideAdapter( # optional, rarely used
        zc.async.job.Job,
        adapts=(types.BuiltinFunctionType,),
        provides=zc.async.interfaces.IJob)

    # UUID for this instance
    zope.component.provideUtility(
        zc.async.instanceuuid.UUID, zc.async.interfaces.IUUID)

def base():
    # see comment in ``minimal``, above
    minimal()
    zope.component.provideAdapter(zc.twist.connection)

# this function installs a queue named '' (empty string), starts the
# dispatcher, and installs an agent named 'main', with default values.
# It is a convenience for quick starts.
def start(db, poll_interval=5, db_name=None, agent_chooser=None, agent_size=3,
          twisted=False):
    zope.component.provideAdapter(zc.async.queue.getDefaultQueue)
    zope.component.provideAdapter(zc.async.queue.getDefaultQueue,
                                  adapts=(ZODB.interfaces.IConnection,))
    zope.component.provideHandler(
        zc.async.subscribers.QueueInstaller(db_name=db_name))
    if twisted:
        zope.component.provideHandler(
            zc.async.subscribers.TwistedDispatcherInstaller(
                poll_interval=poll_interval))
    else:
        zope.component.provideHandler(
            zc.async.subscribers.ThreadedDispatcherInstaller(
                poll_interval=poll_interval))
    zope.component.provideHandler(
        zc.async.subscribers.AgentInstaller('main',
                                            chooser=agent_chooser,
                                            size=agent_size))
    zope.event.notify(zc.async.interfaces.DatabaseOpened(db))
