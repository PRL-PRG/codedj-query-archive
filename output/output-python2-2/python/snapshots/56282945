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
import persistent
import datetime

import zope.interface
import zope.component

import zc.async.interfaces
import zc.async.utils


def chooseFirst(agent):
    return agent.queue.claim()


class Agent(zc.async.utils.Base):

    zope.interface.implements(zc.async.interfaces.IAgent)

    def __init__(self, chooser=None, size=3):
        if chooser is None:
            chooser = chooseFirst
        self.chooser = chooser
        self.size = size
        self._data = zc.queue.PersistentQueue()
        self._data.__parent__ = self
        self.completed = zc.async.utils.Periodic(
            period=datetime.timedelta(days=7),
            buckets=7)
        zope.interface.alsoProvides(
            self.completed, zc.async.interfaces.ICompletedCollection)
        self.completed.__parent__ = self

    @property
    def queue(self):
        if self.parent is not None:
            return self.parent.parent

    for nm in ('__len__', '__iter__', '__getitem__', '__nonzero__'):
        locals()[nm] = zc.async.utils.simpleWrapper(nm)

    def index(self, item):
        for ix, i in enumerate(self):
            if i is item:
                return ix
        raise ValueError("%r not in %s" % (item, self.__class__.__name__))

    def remove(self, item):
        self.pull(self.index(item))

    def __delitem__(self, ix):
        self.pull(ix)

    def pull(self, index=0):
        res = self._data.pull(index)
        res.parent = None
        return res

    def claimJob(self):
        if not self.parent.activated or self.parent.dead:
            # we don't want to claim a job unless we are activated.
            # Normally, this should be the case, but in unusual
            # circumstances, such as very long commits causing the
            # ping to not be able to commit, we might get in this
            # unfortunate circumstance.
            # TODO: we would like to have a read conflict error if we read
            # activated but it changed beneath us.  If the ZODB grows a gesture
            # to cause this, use it.
            return None
        if len(self._data) < self.size:
            res = self.chooser(self)
            if res is not None:
                res.parent = self
                self._data.put(res)
        else:
            res = None
        return res

    def jobCompleted(self, job):
        self.remove(job)
        self.completed.add(job)

@zope.component.adapter(zc.async.interfaces.IDispatcherActivated)
def addMainAgentActivationHandler(event):
    da = event.object
    if 'main' not in da:
        da['main'] = Agent()
