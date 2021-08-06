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
import time
import datetime
import bisect
import Queue
import thread
import threading

import twisted.python.failure
import twisted.internet.defer
import ZODB.POSException
import ZEO.Exceptions
import ZODB.utils
import BTrees
import transaction
import transaction.interfaces
import zope.component
import zope.bforest.periodic
import zc.twist

import zc.async
import zc.async.utils
import zc.async.interfaces

class PollInfo(dict):
    key = None
    @property
    def utc_timestamp(self):
        if self.key is not None:
            return zc.async.utils.long_to_dt(self.key)


class AgentThreadPool(object):

    _size = 0
    initial_backoff = 5
    incremental_backoff = 5
    maximum_backoff = 60
    jobid = None

    def __init__(self, dispatcher, name, size):
        self.dispatcher = dispatcher
        self.name = name
        self.queue = Queue.Queue(0)
        self.threads = []
        self.jobids = {}
        self.setSize(size)

    def getSize(self):
        return self._size

    def perform_thread(self):
        thread_id = thread.get_ident()
        self.jobids[thread_id] = None
        zc.async.local.dispatcher = self.dispatcher
        zc.async.local.name = self.name # this is the name of this pool's agent
        conn = self.dispatcher.db.open()
        try:
            job_info = self.queue.get()
            while job_info is not None:
                identifier, dbname, info = job_info
                self.jobids[thread_id] = (ZODB.utils.u64(identifier), dbname)
                info['thread'] = thread_id
                info['started'] = datetime.datetime.utcnow()
                zc.async.utils.tracelog.info(
                    'starting in thread %d: %s',
                    info['thread'], info['call'])
                backoff = self.initial_backoff
                conflict_retry_count = 0
                try:
                    while 1:
                        try:
                            transaction.begin()
                            if dbname is None:
                                local_conn = conn
                            else:
                                local_conn = conn.get_connection(dbname)
                            job = local_conn.get(identifier)
                            # this setstate should trigger any initial problems
                            # within the try/except retry structure here.
                            local_conn.setstate(job)
                            # this is handled in job.__call__: local.job = job
                        except ZEO.Exceptions.ClientDisconnected:
                            zc.async.utils.log.info(
                                'ZEO client disconnected while trying to '
                                'get job %d in db %s; retrying in %d seconds',
                                ZODB.utils.u64(identifier), dbname or '',
                                backoff)
                            time.sleep(backoff)
                            backoff = min(self.maximum_backoff,
                                          backoff + self.incremental_backoff)
                        except ZODB.POSException.TransactionError:
                            # continue, i.e., try again
                            conflict_retry_count += 1
                            if (conflict_retry_count == 1 or
                                not conflict_retry_count % 5):
                                zc.async.utils.log.warning(
                                    '%d transaction error(s) while trying to '
                                    'get job %d in db %s',
                                    conflict_retry_count,
                                    ZODB.utils.u64(identifier), dbname or '',
                                    exc_info=True)
                            # now ``while 1`` loop will continue, to retry
                        else:
                            break
                    try:
                        job() # this does the committing and retrying, largely
                    except zc.async.interfaces.BadStatusError:
                        transaction.abort()
                        zc.async.utils.log.error( # notice, not tracelog
                            'job already completed?', exc_info=True)
                        if job.status == zc.async.interfaces.CALLBACKS:
                            job.resumeCallbacks() # moves the job off the agent
                        else:
                            count = 0
                            while 1:
                                status = job.status
                                if status == zc.async.interfaces.COMPLETED:
                                    if zc.async.interfaces.IAgent.providedBy(
                                        job.parent):
                                        job.parent.jobCompleted(job)
                                        # moves the job off the agent
                                else:
                                    job.fail() # moves the job off the agent
                                try:
                                    transaction.commit()
                                except (ZODB.POSException.TransactionError,
                                        ZODB.POSException.POSError):
                                    if count and not count % 10:
                                        zc.async.utils.log.critical(
                                            'frequent database errors!  '
                                            'I retry forever...',
                                            exc_info=True)
                                    time.sleep(1)
                                    transaction.abort() # retry forever (!)
                                else:
                                    break
                    except zc.async.interfaces.ReassignedError:
                        transaction.abort()
                        info['reassigned'] = True
                        # will need to get next job_info and continue
                    # EXPLOSIVE_ERRORS includes Reassigned: order is important
                    except zc.async.utils.EXPLOSIVE_ERRORS:
                        transaction.abort()
                        raise
                    except:
                        # all errors should have been handled by the job at
                        # this point, so anything other than BadStatusError,
                        # SystemExit and KeyboardInterrupt are bad surprises.
                        transaction.abort()
                        zc.async.utils.log.critical(
                            'unexpected error', exc_info=True)
                        raise
                    # should come before 'completed' for threading dance
                    if isinstance(job.result, twisted.python.failure.Failure):
                        info['failed'] = True
                        info['result'] = job.result.getTraceback(
                            elideFrameworkCode=True)
                    else:
                        info['result'] = repr(job.result)
                    if len(info['result']) > 10000:
                        info['result'] = (
                            info['result'][:10000] + '\n[...TRUNCATED...]')
                    info['completed'] = datetime.datetime.utcnow()
                finally:
                    zc.async.local.job = None # also in job (here for paranoia)
                    transaction.abort() # (also paranoia)
                zc.async.utils.tracelog.info(
                    'completed in thread %d: %s',
                    info['thread'], info['call'])
                self.jobids[thread_id] = None
                job_info = self.queue.get()
        finally:
            conn.close()
            if self.dispatcher.activated:
                # this may cause some bouncing, but we don't ever want to end
                # up with fewer than needed.
                self.dispatcher.reactor.callFromThread(self.setSize)
            del self.jobids[thread_id]

    def setSize(self, size=None):
        # this should only be called from the thread in which the reactor runs
        # (otherwise it needs locks)
        old = self._size
        if size is None:
            size = old
        else:
            self._size = size
        res = []
        ct = 0
        for t in self.threads:
            if t.isAlive():
                res.append(t)
                ct += 1
        self.threads[:] = res
        if ct < size:
            for i in range(max(size - ct, 0)):
                t = threading.Thread(target=self.perform_thread)
                t.setDaemon(True)
                self.threads.append(t)
                t.start()
        elif ct > size:
            # this may cause some bouncing, but hopefully nothing too bad.
            for i in range(ct - size):
                self.queue.put(None)
        return size - old # size difference

def getId(obj):
    dbname = getattr(obj._p_jar.db(), 'database_name', None)
    return (ZODB.utils.u64(obj._p_oid), dbname)

# this is mostly for testing, though ``get`` comes in handy generally

_dispatchers = {}

def get(uuid=None, default=None):
    if uuid is None:
        uuid = zope.component.getUtility(zc.async.interfaces.IUUID)
    return _dispatchers.get(uuid, default)

def pop(uuid=None):
    if uuid is None:
        uuid = zope.component.getUtility(zc.async.interfaces.IUUID)
    return _dispatchers.pop(uuid)

clear = _dispatchers.clear

# end of testing bits

class Dispatcher(object):

    activated = False
    conn = None
    thread = None # this is just a placeholder that other code can use the
    # way that zc.async.subscribers.ThreadedDispatcherInstaller.__call__ does.

    def __init__(self, db, reactor=None, poll_interval=5, uuid=None,
                 jobs_size=200, polls_size=400):
        if uuid is None:
            uuid = zope.component.getUtility(zc.async.interfaces.IUUID)
        if uuid in _dispatchers:
            raise ValueError('dispatcher for this UUID is already registered')
        _dispatchers[uuid] = self
        self.db = db
        self._reactor = reactor
        self.poll_interval = poll_interval
        self.UUID = uuid
        # Let's talk about jobs_size and polls_size.
        #
        # Let's take a random guess that data for a job might be about 1K on
        # average.  That would mean that the default value (keep 200 jobs)
        # would mean about 200K.
        #
        # Let's randomly guess that a poll record averages 300 bytes on
        # average.  That would mean that the default value (keep 400 polls)
        # would mean (400*300 bytes == 120000 bytes == ) about 120K.  That
        # would cover (400 polls * 5 seconds/poll * 1 min/60 seconds == )
        # just over 33 minutes of polling at the default poll_interval.
        #
        # These memory usages should be not really noticeable on typical
        # production machines. On the other hand, if this is causing you memory
        # problems, reduce these values when you instantiate your dispatcher.
        self.polls = zc.async.utils.RollingSet()
        self.polls.size = polls_size
        self.polls.__parent__ = self
        self.jobs = zc.async.utils.RollingMapping()
        self.jobs.size = jobs_size
        self.jobs.__parent__ = self
        self._activated = set()
        self.queues = {}
        self.dead_pools = []

    @property
    def reactor(self):
        res = self._reactor
        if res is None:
            # importing this the first time is kinda slow so we're lazy
            import twisted.internet.reactor
            res = self._reactor = twisted.internet.reactor
        return res

    def _getJob(self, agent):
        identifier = (
            'getting job for UUID %s from agent %s (oid %d) '
            'in queue %s (oid %d)' % (
                self.UUID, agent.name, ZODB.utils.u64(agent._p_oid),
                agent.queue.name, ZODB.utils.u64(agent.queue._p_oid)))
        res = zc.async.utils.try_five_times(
            agent.claimJob, identifier, transaction)
        if isinstance(res, twisted.python.failure.Failure):
            identifier = 'stashing failure on agent %s (oid %s)' % (
                agent.name, ZODB.utils.u64(agent._p_oid))
            def setFailure():
                agent.failure = res
            zc.async.utils.try_five_times(
                setFailure, identifier, transaction)
        return res

    def poll(self):
        poll_info = PollInfo()
        started_jobs = []
        transaction.begin() # sync and clear
        try:
            queues = self.conn.root().get(zc.async.interfaces.KEY)
            if queues is None:
                transaction.abort()
                return
            for queue in queues.values():
                poll_info[queue.name] = None
                if self.UUID not in queue.dispatchers:
                    queue.dispatchers.register(self.UUID)
                da = queue.dispatchers[self.UUID]
                if queue._p_oid not in self._activated:
                    identifier = (
                        'activating dispatcher UUID %s in queue %s (oid %d)' %
                        (self.UUID, queue.name, ZODB.utils.u64(queue._p_oid)))
                    def activate():
                        if da.activated:
                            if da.dead:
                                da.deactivate()
                            else:
                                zc.async.utils.log.error(
                                    'UUID %s already activated in queue %s '
                                    '(oid %d): another process?  (To stop '
                                    'poll attempts in this process, set '
                                    '``zc.async.dispatcher.get().activated = '
                                    "False``.  To stop polls permanently, don't "
                                    'start a zc.async.dispatcher!)',
                                    self.UUID, queue.name,
                                    ZODB.utils.u64(queue._p_oid))
                                return False
                        da.activate()
                        return True
                    if zc.async.utils.try_five_times(
                        activate, identifier, transaction) is True:
                        self._activated.add(queue._p_oid)
                    else:
                        continue
                identifier = 'committing ping for UUID %s' % (self.UUID,)
                zc.async.utils.try_five_times(
                    lambda: queue.dispatchers.ping(self.UUID), identifier,
                    transaction)
                queue_info = poll_info[queue.name] = {}
                pools = self.queues.get(queue.name)
                if pools is None:
                    pools = self.queues[queue.name] = {}
                for name, agent in da.items():
                    job_info = []
                    active_jobs = [getId(job) for job in agent]
                    agent_info = queue_info[name] = {
                        'size': None, 'len': None, 'error': None,
                        'new jobs': job_info, 'active jobs': active_jobs}
                    try:
                        agent_info['size'] = agent.size
                        agent_info['len'] = len(agent)
                    except zc.async.utils.EXPLOSIVE_ERRORS:
                        raise
                    except:
                        agent_info['error'] = zc.twist.Failure()
                        transaction.abort()
                        continue
                    pool = pools.get(name)
                    if pool is None:
                        pool = pools[name] = AgentThreadPool(
                            self, name, agent_info['size'])
                        conn_delta = agent_info['size']
                    else:
                        conn_delta = pool.setSize(agent_info['size'])
                    if conn_delta:
                        db = queues._p_jar.db()
                        db.setPoolSize(db.getPoolSize() + conn_delta)
                    job = self._getJob(agent)
                    while job is not None:
                        if isinstance(job, twisted.python.failure.Failure):
                            agent_info['error'] = job
                            job = None
                        else:
                            info = {'result': None,
                                    'failed': False,
                                    'agent': name,
                                    'queue': queue.name,
                                    'poll id': None,
                                    'quota names': job.quota_names,
                                    'call': repr(job),
                                    'started': None,
                                    'completed': None,
                                    'thread': None,
                                    'reassigned': False}
                            started_jobs.append(info)
                            jobid = uoid, dbname = getId(job)
                            self.jobs[jobid] = info
                            job_info.append(jobid)
                            pool.queue.put(
                                (job._p_oid, dbname, info))
                            job = self._getJob(agent)
                if len(pools) > len(queue_info):
                    conn_delta = 0
                    for name, pool in pools.items():
                        if name not in agent_info:
                            conn_delta += pool.setSize(0)
                            self.dead_pools.append(pools.pop(name))
                    if conn_delta:
                        db = queues._p_jar.db()
                        # this is a bit premature--it should really happen when
                        # all threads are complete--but since the pool just
                        # complains if the size is not honored, and this
                        # approach is easier, we're doing this.
                        db.setPoolSize(db.getPoolSize() + conn_delta)
            if len(self.queues) > len(poll_info):
                conn_delta = 0
                for queue_pools in self.queues.values():
                    if name not in poll_info:
                        for name, pool in queue_pools.items():
                            conn_delta += pool.setSize(0)
                            self.dead_pools.append(queue_pools.pop(name))
                if conn_delta:
                    # this is a bit premature--it should really happen
                    # when all threads are complete--but since the pool just
                    # complains if the size is not honored, and this approach
                    # is easier, we're doing this.
                    self.db.setPoolSize(self.db.getPoolSize() + conn_delta)
        finally:
            transaction.abort()
            try:
                last = self.polls.first()
            except ValueError:
                last = None
            self.polls.add(poll_info)
            for info in started_jobs:
                info['poll id'] = poll_info.key
            if last is None or last != poll_info:
                zc.async.utils.tracelog.debug(
                    'poll %s: %r', poll_info.key, poll_info)

    def directPoll(self):
        if not self.activated:
            return
        try:
            self.poll()
        finally:
            self.reactor.callLater(self.poll_interval, self.directPoll)

    def _inThreadPoll(self, deferred):
        self.conn = self.db.open()
        try:
            self.poll()
        finally:
            self.conn.close()
            self.reactor.callFromThread(deferred.callback, None)

    def threadedPoll(self):
        if not self.activated:
            return
        deferred = twisted.internet.defer.Deferred()
        deferred.addCallback(
            lambda result: self.reactor.callLater(
                self.poll_interval, self.threadedPoll))
        self.reactor.callInThread(self._inThreadPoll, deferred)

    def activate(self, threaded=False):
        if self.activated:
            raise ValueError('already activated')
        zc.async.utils.log.info('attempting to activate dispatcher %s',
                                self.UUID)
        self.activated = datetime.datetime.utcnow()
        # in case this is a restart, we clear old data
        self.polls.clear()
        self.jobs.clear()
        # increase pool size to account for the dispatcher poll
        self.db.setPoolSize(self.db.getPoolSize() + 1)
        if not threaded:
            self.conn = self.db.open() # we keep the same connection for all
            # polls as an optimization
        if threaded:
            self.reactor.callWhenRunning(self.threadedPoll)
        else:
            self.reactor.callWhenRunning(self.directPoll)
        self.reactor.addSystemEventTrigger(
            'before', 'shutdown', self.deactivate)

    def deactivate(self):
        if not self.activated:
            raise ValueError('not activated')
        self.activated = None # "in progress"
        try:
            transaction.begin()
            try:
                identifier = 'cleanly deactivating UUID %s' % (self.UUID,)
                def deactivate_das():
                    queues = self.conn.root().get(zc.async.interfaces.KEY)
                    if queues is not None:
                        for queue in queues.values():
                            da = queue.dispatchers.get(self.UUID)
                            if da is not None and da.activated:
                                da.deactivate()
                zc.async.utils.try_five_times(
                    deactivate_das, identifier, transaction)
            finally:
                transaction.abort()
                self.conn.close()
            conn_delta = 0
            for queue_pools in self.queues.values():
                for name, pool in queue_pools.items():
                    conn_delta += pool.setSize(0)
                    self.dead_pools.append(queue_pools.pop(name))
            conn_delta -= 1
            self.db.setPoolSize(self.db.getPoolSize() + conn_delta)
            zc.async.utils.log.info('deactivated dispatcher %s',
                                    self.UUID)
        finally:
            self.activated = False # "completed" (can distinguish for tests)

    # these methods are used for monitoring and analysis

    STOPPED = 'STOPPED'
    RUNNING = 'RUNNING'
    STUCK = 'STUCK'
    STARTING = 'STARTING'

    def getStatusInfo(self):
        res = {'time since last poll': None, 'uptime': None, 'uuid': self.UUID}
        poll_interval = res['poll interval'] = datetime.timedelta(
                    seconds=self.poll_interval)
        if not self.activated:
            res['status'] = self.STOPPED
        else:
            now = datetime.datetime.utcnow()
            try:
                poll = self.polls.first()
            except ValueError:
                # no polls
                next = self.activated + poll_interval
                if next < now:
                    res['status'] = self.STUCK
                else:
                    res['status'] = self.STARTING
                res['time since last poll'] = now - self.activated
            else:
                next = poll.utc_timestamp + poll_interval
                if next < now:
                    res['status'] = self.STUCK
                else:
                    res['status'] = self.RUNNING
                res['time since last poll'] = now - poll.utc_timestamp
                res['uptime'] = now - self.activated
        return res

    def getJobInfo(self, oid, database_name=None):
        if database_name is None:
            # these will raise ValueErrors for unknown oids.  We'll let 'em.
            minKey = self.jobs.minKey((oid,))
            maxKey = self.jobs.maxKey((oid+1,))
            if minKey != maxKey:
                raise ValueError('ambiguous database name')
            else:
                database_name = minKey[1]
        res = self.jobs[(oid, database_name)]
        if res['completed'] is None:
            jobid = (oid, database_name)
            info = self.polls.first()[res['queue']][res['agent']]
            if (jobid not in info['active jobs'] and
                jobid not in info['new jobs']):
                res = res.copy()
                res['reassigned'] = True
        return res

    def getActiveJobIds(self, queue=None, agent=None):
        """returns active jobs from newest to oldest"""
        res = []
        for queue_name, agents in self.queues.items():
            if queue is None or queue_name == queue:
                for agent_name, pool in agents.items():
                    if agent is None or agent_name == agent:
                        res.extend(val for val in pool.jobids.values()
                                   if val is not None)
        return res

    def getPollInfo(self, at=None, before=None):
        if at is not None:
            if before is not None:
                raise ValueError('may only provide one of `at` and `before`')
            if isinstance(at, datetime.datetime):
                at = zc.async.utils.dt_to_long(at)
        elif before is not None:
            if isinstance(before, datetime.datetime):
                at = zc.async.utils.dt_to_long(before) + 16
            else:
                at = before + 1
        return self.polls.first(at)

    def iterPolls(self, at=None, before=None, since=None, count=None):
        # `polls` may be mutated during iteration so we don't iterate over it
        if at is not None and before is not None:
            raise ValueError('may only provide one of `at` and `before`')
        if isinstance(since, datetime.datetime):
            since = zc.async.utils.dt_to_long(since) + 15
        ct = 0
        while 1:
            if count is not None and ct >= count:
                break
            try:
                info = self.getPollInfo(at=at, before=before)
            except ValueError:
                break
            else:
                if since is None or before <= since:
                    yield info
                    ct += 1
                    before = info.key
                    at = None
                else:
                    break

    def getStatistics(self, at=None, before=None, since=None, queue=None,
                      agent=None):
        if at is not None and before is not None:
            raise ValueError('may only provide one of `at` and `before`')
        res = {
            'started': 0,
            'successful': 0,
            'failed': 0,
            'unknown': 0
            }
        started = successful = failed = unknown = 0
        _pair = (None, None)
        successful_extremes = [_pair, _pair]
        failed_extremes = [_pair, _pair]
        active_extremes = [_pair, _pair]
        now = datetime.datetime.utcnow()
        first = True
        poll = first_poll = None
        def process(jobs):
            for jobid in jobs:
                jobinfo = self.jobs.get(jobid)
                if jobinfo is None:
                    res['unknown'] += 1
                    continue
                if jobinfo['completed']:
                    if jobinfo['failed']:
                        pair = failed_extremes
                        res['failed'] += 1
                    else:
                        pair = successful_extremes
                        res['successful'] += 1
                else:
                    pair = active_extremes
                start = jobinfo['started'] or poll_time
                stop = jobinfo['completed'] or now
                duration = stop - start
                if pair[0][0] is None or pair[0][0] > duration:
                    pair[0] = (duration, jobid)
                if pair[1][0] is None or pair[1][0] < duration:
                    pair[1] = (duration, jobid)
        for poll in self.iterPolls(at=at, before=before, since=since):
            poll_time = poll.utc_timestamp
            for agent_info in _iter_info(poll, queue, agent):
                res['started'] += len(agent_info['new jobs'])
                process(agent_info['new jobs'])
            if first:
                first = False
                first_poll = poll
        if poll is not None:
            for agent_info in _iter_info(poll, queue, agent):
                process(agent_info['active jobs'])
        if first_poll is not None:
            stat_start = first_poll.utc_timestamp
            stat_end = poll.utc_timestamp
        else:
            start_start = None
            stat_end = None
        res.update({
            'shortest successful': successful_extremes[0][1],
            'longest successful': successful_extremes[1][1],
            'shortest failed': failed_extremes[0][1],
            'longest failed': failed_extremes[1][1],
            'shortest active': active_extremes[0][1],
            'longest active': active_extremes[1][1],
            'statistics start': stat_start,
            'statistics end': stat_end,
            })
        return res

def _iter_info(poll, queue, agent):
    if queue is None:
        queues = poll.values()
    elif queue not in poll:
        queues = []
    else:
        queues = [poll[queue]]
    for q in queues:
        if agent is None:
            for i in q.values():
                yield i
        elif agent in q:
            yield q[agent]
