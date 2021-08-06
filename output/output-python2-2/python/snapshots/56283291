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
import datetime
import bisect
import pytz
import persistent
import persistent.interfaces
import ZODB.interfaces
import BTrees.OOBTree
import BTrees.Length
import zope.interface
import zope.component
import zope.event
import zope.bforest
import zc.queue
import zc.dict

import zc.async.interfaces
import zc.async.utils

_marker = object()

# purely optional
@zope.interface.implementer(zc.async.interfaces.IQueue)
@zope.component.adapter(persistent.interfaces.IPersistent)
def getDefaultQueue(obj):
    return ZODB.interfaces.IConnection(obj).root()[zc.async.interfaces.KEY]['']


class DispatcherAgents(zc.async.utils.Dict):
    zope.interface.implements(zc.async.interfaces.IDispatcherAgents)

    UUID = None
    activated = None
    
    def __init__(self, uuid):
        super(DispatcherAgents, self).__init__()
        self.UUID = uuid
        self.__class__.last_ping.initialize(self)
    
    zc.async.utils.createAtom('last_ping', None)
    
    ping_interval = datetime.timedelta(seconds=30)
    ping_death_interval = datetime.timedelta(seconds=60)

    @property
    def dead(self):
        last_ping = self.last_ping
        if self.activated and (
            self.last_ping is None or self.activated > self.last_ping):
            last_ping = self.activated
        elif last_ping is None:
            return False
        return ((last_ping + self.ping_death_interval) <
                datetime.datetime.now(pytz.UTC))
        return False

    def __setitem__(self, key, value):
        if not zc.async.interfaces.IAgent.providedBy(value):
            raise ValueError('value must be IAgent')
        if len(value):
            raise ValueError('cannot add an agent with active jobs')
        current = self.get(key)
        if current is not None and len(current):
            raise ValueError('cannot remove an agent with active jobs')
        super(DispatcherAgents, self).__setitem__(key, value)

    def pop(self, key, *args):
        current = self.get(key)
        if current is not None and len(current):
            raise ValueError('cannot remove an agent with active jobs')
        return super(DispatcherAgents, self).pop(key, *args)

    def activate(self):
        if self.activated:
            raise ValueError('Already activated')
        self.activated = datetime.datetime.now(pytz.UTC)
        zope.event.notify(
            zc.async.interfaces.DispatcherActivated(self))

    def deactivate(self):
        if not self.activated:
            raise ValueError('Not activated')
        self.activated = None
        queue = self.parent
        assert zc.async.interfaces.IQueue.providedBy(queue)
        for agent in self.values():
            try:
                job = agent.pull()
            except IndexError:
                pass
            else:
                while job is not None:
                    status = job.status
                    if status == zc.async.interfaces.ASSIGNED:
                        tmp = job.assignerUUID
                        job.assignerUUID = None
                        job.parent = None
                        queue.put(job)
                        job.assignerUUID = tmp
                    elif job.status == zc.async.interfaces.ACTIVE:
                        queue.put(job.fail)
                    elif job.status == zc.async.interfaces.CALLBACKS:
                        queue.put(job.resumeCallbacks)
                    elif job.status == zc.async.interfaces.COMPLETED:
                        # huh, that's odd.
                        agent.completed.add(job)
                    try:
                        job = agent.pull()
                    except IndexError:
                        job = None
        zope.event.notify(
            zc.async.interfaces.DispatcherDeactivated(self))
        

class Queues(zc.async.utils.Dict):

    def __setitem__(self, key, value):
        if not zc.async.interfaces.IQueue.providedBy(value):
            raise ValueError('value must be IQueue')
        super(Queues, self).__setitem__(key, value)


class Dispatchers(zc.dict.Dict):
    zope.interface.implements(zc.async.interfaces.IDispatchers)

    __setitem__ = update = pop = __delitem__ = copy = None # simple hide

    def register(self, uuid):
        if uuid in self:
            raise ValueError('UUID already registered')
        da = DispatcherAgents(uuid)
        da.parent = self.__parent__ # __parent__ should be queue
        super(Dispatchers, self).__setitem__(uuid, da)
        zope.event.notify(
            zc.async.interfaces.DispatcherRegistered(da))

    def unregister(self, uuid):
        da = self[uuid]
        if da.activated:
            raise ValueError('UUID is activated.')
        da = super(Dispatchers, self).pop(uuid)
        da.parent = da.name = None
        zope.event.notify(
            zc.async.interfaces.DispatcherUnregistered(da, self.__parent__))
        return da

    def ping(self, uuid):
        da = self[uuid]
        if not da.activated:
            raise ValueError('UUID is not activated.')
        now = datetime.datetime.now(pytz.UTC)
        if (da.last_ping is None or
            da.last_ping + da.ping_interval <= now):
            da.last_ping = now
        next = self._getNextActiveSibling(uuid)
        if next is not None and next.dead:
            # `next` seems to be a dead dispatcher.
            next.deactivate()

    def _getNextActiveSibling(self, uuid):
        for da in self._data.values(min=uuid, excludemin=True):
            if da.activated:
                return da
        for da in self._data.values(max=uuid, excludemax=True):
            if da.activated:
                return da


class Quota(zc.async.utils.Base):

    zope.interface.implements(zc.async.interfaces.IQuota)

    def __init__(self, name, size):
        self._data = zc.queue.Queue()
        self.name = name
        self.size = size

    def clean(self):
        for i, job in enumerate(reversed(self._data)):
            if job.status in (
                zc.async.interfaces.CALLBACKS,
                zc.async.interfaces.COMPLETED):
                self._data.pull(-1-i)

    @property
    def filled(self):
        return len(self._data) >= self.size

    def add(self, item):
        if not zc.async.interfaces.IJob.providedBy(item):
            raise ValueError('must be IJob')
        if self.name not in item.quota_names:
            raise ValueError('quota name must be in quota_names')
        # self.clean()
        if self.filled:
            raise ValueError('Quota is filled')
        self._data.put(item)

    for nm in ('__len__', '__iter__', '__getitem__', '__nonzero__', 'get', 
               '__contains__'):
        locals()[nm] = zc.async.utils.simpleWrapper(nm)


class Quotas(zc.dict.Dict):

    __setitem__ = update = pop = __delitem__ = copy = None # simple hide

    def create(self, name, size=1):
        res = Quota(name, size)
        super(Quotas, self).__setitem__(name, res)
        res.parent = self

    def remove(self, name):
        super(Quotas, self).pop(name)


class Queue(zc.async.utils.Base):
    zope.interface.implements(zc.async.interfaces.IQueue)

    def __init__(self):
        self._queue = zc.queue.CompositeQueue()
        self._held = BTrees.OOBTree.OOBTree()
        self.quotas = Quotas()
        self.quotas.__parent__ = self
        self._length = BTrees.Length.Length(0)
        self.dispatchers = Dispatchers()
        self.dispatchers.__parent__ = self

    def put(self, item, begin_after=None, begin_by=None):
        item = zc.async.interfaces.IJob(item)
        if item.assignerUUID is not None:
            raise ValueError(
                'cannot add already-assigned job')
        for name in item.quota_names:
            if name not in self.quotas:
                raise ValueError('unknown quota name', name)
        now = datetime.datetime.now(pytz.UTC)
        if begin_after is not None:
            item.begin_after = begin_after
        elif item.begin_after is None:
            item.begin_after = now
        if begin_by is not None:
            item.begin_by = begin_by
        elif item.begin_by is None:
            item.begin_by = datetime.timedelta(hours=1) # good idea?
        item.assignerUUID = zope.component.getUtility(
            zc.async.interfaces.IUUID)
        if item._p_jar is None:
            # we need to do this if the job will be stored in another
            # database as well during this transaction.  Also, _held storage
            # disambiguates against the database_name and the _p_oid.
            conn = ZODB.interfaces.IConnection(self)
            conn.add(item)
        if now >= item.begin_after:
            self._queue.put(item)
        else:
            self._held[
                (item.begin_after,
                 item._p_jar.db().database_name,
                 item._p_oid)] = item
        item.parent = self
        self._length.change(1)
        return item

    def _iter(self):
        queue = self._queue
        tree = self._held
        q = enumerate(queue)
        t = iter(tree.items())
        q_pop = queue.pull
        t_pop = tree.pop
        def get_next(i):
            try:
                next = i.next()
            except StopIteration:
                active = False
                next = (None, None)
            else:
                active = True
            return active, next
        q_active, (q_index, q_next) = get_next(q)
        t_active, (t_index, t_next) = get_next(t)
        while q_active and t_active:
            if t_next.begin_after <= q_next.begin_after:
                yield t_pop, t_index, t_next
                t_active, (t_index, t_next) = get_next(t)
            else:
                yield q_pop, q_index, q_next
                q_active, (q_index, q_next) = get_next(q)
        if t_active:
            yield t_pop, t_index, t_next
            for (t_index, t_next) in t:
                yield t_pop, t_index, t_next
        elif q_active:
            yield q_pop, q_index, q_next
            for (q_index, q_next) in q:
                yield q_pop, q_index, q_next

    def pull(self, index=0):
        length = len(self)
        if index < 0:
            index += length
            if index < 0:
                raise IndexError(index + length)
        if index >= length:
            raise IndexError(index)
        for i, (pop, ix, job) in enumerate(self._iter()):
            if i == index:
                tmp = pop(ix)
                assert tmp is job
                self._length.change(-1)
                job.assignerUUID = None
                job.parent = None
                return job
        assert False, 'programmer error: the length appears to be incorrect.'

    def claim(self, filter=None, default=None):
        now = datetime.datetime.now(pytz.UTC)
        if not self._length():
            return default
        uuid = None
        for pop, ix, job in self._iter():
            if job.begin_after > now:
                break
            res = None
            quotas = []
            if (job.begin_after + job.begin_by) < now:
                res = zc.async.interfaces.IJob(
                        job.fail) # specify TimeoutError?
                res.begin_after = now
                res.begin_by = datetime.timedelta(hours=1)
                res.parent = self
                if uuid is None:
                    uuid = zope.component.getUtility(zc.async.interfaces.IUUID)
                res.assignerUUID = uuid
            else:
                for name in job.quota_names:
                    quota = self.quotas.get(name)
                    if quota is not None:
                        quota.clean()
                        if quota.filled:
                            break
                        quotas.append(quota)
                else:
                    res = job
            if res is not None and (filter is None or filter(res)):
                tmp = pop(ix)
                assert tmp is job
                self._length.change(-1)
                for quota in quotas:
                    quota.add(job)
                job.parent = None
                return res
        return default

    def __len__(self):
        return self._length()

    def __iter__(self):
        return (next for pop, ix, next in self._iter())

    def __nonzero__(self):
        return bool(self._length())

    def __getitem__(self, index):
        length = len(self)
        if index < 0:
            index += length
            if index < 0:
                raise IndexError(index + length)
        if index >= length:
            raise IndexError(index)
        for i, (pop, ix, job) in enumerate(self._iter()):
            if i == index:
                return job
        assert False, 'programmer error: the length appears to be incorrect.'
