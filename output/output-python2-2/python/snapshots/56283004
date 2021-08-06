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
import logging
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
import zope.minmax
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
        self.last_ping = zope.minmax.Maximum()

    ping_interval = datetime.timedelta(seconds=30)
    ping_death_interval = datetime.timedelta(seconds=60)

    @property
    def dead(self):
        last_ping = self.last_ping.value
        if self.activated and (
            last_ping is None or self.activated > last_ping):
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
                    if status in (zc.async.interfaces.PENDING,
                                  zc.async.interfaces.ASSIGNED):
                        # odd
                        zc.async.log.warning(
                            'unexpected job status %s for %r; treating as NEW',
                            status, job)
                        status = zc.async.interfaces.NEW
                    if status == zc.async.interfaces.NEW:
                        tmp = job.assignerUUID
                        job.assignerUUID = None
                        job.parent = None
                        queue.put(job)
                        job.assignerUUID = tmp
                    elif job.status == zc.async.interfaces.ACTIVE:
                        j = queue.put(
                            job.handleInterrupt,
                            retry_policy_factory=zc.async.job.RetryCommonForever,
                            failure_log_level=logging.CRITICAL)
                        # we don't make job's parent j because it shouldn't
                        # really be needed and it would be a pain to clean up
                    elif job.status == zc.async.interfaces.CALLBACKS:
                        j = queue.put(
                            job.resumeCallbacks,
                            retry_policy_factory=zc.async.job.RetryCommonForever,
                            failure_log_level=logging.CRITICAL)
                        # make job's parent j so that ``queue`` references work
                        # in callbacks
                        job.parent = j
                    elif job.status == zc.async.interfaces.COMPLETED:
                        # huh, that's odd.
                        agent.completed.add(job)
                        zc.async.utils.log.warning(
                            'unexpectedly had to inform agent of completion '
                            'of %r', job)
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
            zc.async.utils.log.critical(
                "Dispatcher %r not activated prior to ping. This can indicate "
                "that the dispatcher's ping_death_interval is set too short, "
                "or that some transactions in the system are taking too long "
                "to commit. Activating, to correct the current problem, but "
                "if the dispatcher was inappropriately viewed as ``dead`` and "
                "deactivated, you should investigate the cause.",
                uuid)
            da.activate()
        now = datetime.datetime.now(pytz.UTC)
        last_ping = da.last_ping.value
        if (last_ping is None or
            last_ping + da.ping_interval <= now):
            da.last_ping.value = now
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
    # this implementation is reasonable for relatively small (say, size<100)
    # quotas.

    zope.interface.implements(zc.async.interfaces.IQuota)

    _data = ()

    def __init__(self, name, size):
        self.name = name
        self.size = size

    def clean(self):
        now = datetime.datetime.now(pytz.UTC)
        changed = False
        new = []
        for job in self._data:
            status = job.status
            if status in (zc.async.interfaces.CALLBACKS,
                          zc.async.interfaces.COMPLETED) or (
                status == zc.async.interfaces.PENDING and
                job.begin_after > now): # for a rescheduled task
                changed = True # remove from quota
            else:
                new.append(job)
        if changed:
            self._data = tuple(new)

    @property
    def filled(self):
        return len(self._data) >= self.size

    def __contains__(self, item):
        for i in self:
            if i is item:
                return True
        return False

    def add(self, item):
        if item in self:
            return
        if not zc.async.interfaces.IJob.providedBy(item):
            raise ValueError('must be IJob')
        if self.name not in item.quota_names:
            raise ValueError('quota name must be in quota_names')
        if self.filled:
            raise ValueError('Quota is filled')
        # casting self._data to tuple for legacy instances; no-op for tuples
        self._data = tuple(self._data) + (item,)

    for nm in ('__len__', '__iter__', '__getitem__', '__nonzero__', 'get'):
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

    _putback_queue = None

    def __init__(self):
        self._queue = zc.queue.CompositeQueue()
        self._held = BTrees.OOBTree.OOBTree()
        self.quotas = Quotas()
        self.quotas.__parent__ = self
        self._length = BTrees.Length.Length(0)
        self.dispatchers = Dispatchers()
        self.dispatchers.__parent__ = self

    def put(self, item, begin_after=None, begin_by=None,
            failure_log_level=None, retry_policy_factory=None):
        item = zc.async.interfaces.IJob(item)
        if failure_log_level is not None:
            item.failure_log_level = failure_log_level
        if retry_policy_factory is not None:
            item.retry_policy_factory = retry_policy_factory
        if item.status != zc.async.interfaces.NEW:
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
        if item.assignerUUID is not None: # rescheduled job keeps old UUID
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

    def putBack(self, item):
        # an agent has claimed a job, but now the job needs to be returned. the
        # only current caller for this is a job's ``handleInterrupt`` method.
        # The scenario for this is that the agent's dispatcher died while the
        # job was active, interrupting the work; and the job's retry policy
        # asks that the job be put back on the queue to be claimed immediately.
        # This method puts the job in a special internal queue that ``_iter``
        # looks at first. This allows jobs to maintain their order, if needed,
        # within a quota.
        assert zc.async.interfaces.IJob.providedBy(item)
        assert item.status == zc.async.interfaces.NEW, item.status
        assert item.begin_after is not None
        assert item._p_jar is not None
        # to support legacy instances of the queue that were created before
        # this functionality and its separate internal data structure were
        # part of the code, we instantiate the _putback_queue when we first
        # need it, here.
        if self._putback_queue is None:
            self._putback_queue = zc.queue.CompositeQueue()
        self._putback_queue.put(item)
        item.parent = self
        self._length.change(1)

    def _iter(self):
        putback_queue = self._putback_queue
        if putback_queue: # not None and not empty
            dq_pop = putback_queue.pull
            for dq_ix, dq_next in enumerate(putback_queue):
                yield dq_pop, dq_ix, dq_next
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

    def remove(self, item):
        for pop, ix, job in self._iter():
            if job is item:
                assert pop(ix) is job
                self._length.change(-1)
                job.assignerUUID = None
                job.parent = None
                break
        else:
            raise LookupError('item not in queue', item)

    def claim(self, filter=None, default=None):
        now = datetime.datetime.now(pytz.UTC)
        if not self._length():
            return default
        uuid = None
        quotas_cleaned = set()
        for pop, ix, job in self._iter():
            if job.begin_after > now:
                break
            res = None
            quotas = []
            if (job.begin_by is not None and
                (job.begin_after + job.begin_by) < now):
                res = zc.async.interfaces.IJob(job.fail)
                res.args.append(zc.async.interfaces.TimeoutError())
                res.begin_after = now
                res.parent = self
                if uuid is None:
                    uuid = zope.component.getUtility(zc.async.interfaces.IUUID)
                res.assignerUUID = uuid
            else:
                for name in job.quota_names:
                    quota = self.quotas.get(name)
                    if quota is not None:
                        if name not in quotas_cleaned:
                            quota.clean()
                            quotas_cleaned.add(name)
                        if quota.filled and job not in quota:
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
