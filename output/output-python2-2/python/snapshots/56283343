import time
import datetime
import bisect
import Queue
import thread
import threading

import twisted.python.failure
import twisted.internet.defer
import ZODB.POSException
import BTrees
import transaction
import transaction.interfaces
import zope.component
import zope.bforest.periodic
import zc.twist

import zc.async.utils
import zc.async.interfaces

def _get(reactor, job, name, default, timeout, poll, deferred, start=None):
    now = time.time()
    if start is None:
        start = now
    if name in job.annotations:
        res = job.annotations[name]
    elif start + timeout < now:
        res = default
    else:
        partial = zc.twist.Partial(
            _get, reactor, job, name, default, timeout, poll, deferred,
            start)
        partial.setReactor(reactor)
        reactor.callLater(min(poll, start + timeout - now), partial)
        return
    deferred.setResult(res)

class Result(object):

    result = None

    def __init__(self):
        self._event = threading.Event()
    
    def setResult(self, value):
        self.result = value
        self._event.set()

    def wait(self, *args):
        self._event.wait(*args)

class Local(threading.local):

    job = None
    dispatcher = None

    def getJob(self):
        return self.job

    def getDispatcher(self):
        return self.dispatcher

    def getReactor(self):
        return self.dispatcher.reactor

    def setLiveAnnotation(self, name, value, job=None):
        if self.job is None or self.dispatcher.reactor is None:
            raise ValueError('not initialized')
        if job is None:
            job = self.job
        partial = zc.twist.Partial(
            job.annotations.__setitem__, name, value)
        partial.setReactor(self.dispatcher.reactor)
        self.dispatcher.reactor.callFromThread(partial)

    def getLiveAnnotation(self, name, default=None, timeout=0,
                          poll=1, job=None):
        if self.job is None or self.dispatcher.reactor is None:
            raise ValueError('not initialized')
        if job is None:
            job = self.job
        deferred = Result()
        partial = zc.twist.Partial(
            _get, self.dispatcher.reactor, job, name, default, timeout, poll,
            deferred)
        partial.setReactor(self.dispatcher.reactor)
        self.dispatcher.reactor.callFromThread(partial)
        deferred.wait(timeout+2)
        return deferred.result

local = Local()


class PollInfo(dict):
    key = None
    @property
    def utc_timestamp(self):
        if self.key is not None:
            return zc.async.utils.long_to_dt(self.key)


class AgentThreadPool(object):

    _size = 0

    def __init__(self, dispatcher, name, size):
        self.dispatcher = dispatcher
        self.name = name
        self.queue = Queue.Queue(0)
        self._threads = []
        self.setSize(size)

    def getSize(self):
        return self._size

    def perform_thread(self):
        local.dispatcher = self.dispatcher
        conn = self.dispatcher.db.open()
        try:
            job = self.queue.get()
            while job is not None:
                identifier, dbname, info = job
                info['thread'] = thread.get_ident()
                info['started'] = datetime.datetime.utcnow()
                zc.async.utils.tracelog.info(
                    'starting in thread %d: %r',
                    info['thread'], info['call'])
                try:
                    transaction.begin()
                    if dbname is None:
                        local_conn = conn
                    else:
                        local_conn = conn.get_connection(dbname)
                    job = local_conn.get(identifier)
                    local.job = job
                    try:
                        job() # this does the committing and retrying, largely
                    except ZODB.POSException.TransactionError:
                        transaction.abort()
                        while 1:
                            try:
                                job.fail()
                                transaction.commit()
                            except ZODB.POSException.TransactionError:
                                transaction.abort() # retry forever (!)
                            else:
                                break
                    # should come before 'completed' for threading dance
                    if isinstance(job.result, twisted.python.failure.Failure):
                        info['failed'] = True
                        info['result'] = job.result.getTraceback(
                            elideFrameworkCode=True, detail='verbose')
                    else:
                        info['result'] = repr(job.result)
                    info['completed'] = datetime.datetime.utcnow()
                finally:
                    local.job = None
                    transaction.abort()
                if info['failed']:
                    zc.async.utils.tracelog.error(
                        '%s failed in thread %d with traceback:\n%s',
                        info['call'], info['thread'], info['result'])
                else:
                    zc.async.utils.tracelog.info(
                        '%s succeeded in thread %d with result:\n%s',
                        info['call'], info['thread'], info['result'])
                job = self.queue.get()
        finally:
            conn.close()
            if self.dispatcher.activated:
                # this may cause some bouncing, but we don't ever want to end
                # up with fewer than needed.
                self.dispatcher.reactor.callFromThread(self.setSize)
    
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
        for t in self._threads:
            if t.isAlive():
                res.append(t)
                ct += 1
        self._threads[:] = res
        if ct < size:
            for i in range(max(size - ct, 0)):
                t = threading.Thread(target=self.perform_thread)
                t.setDaemon(True)
                self._threads.append(t)
                t.start()
        elif ct > size:
            # this may cause some bouncing, but hopefully nothing too bad.
            for i in range(ct - size):
                self.queue.put(None)
        return size - old # size difference

# this is mostly for testing

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

class Dispatcher(object):

    activated = False
    conn = None

    def __init__(self, db, reactor, poll_interval=5, uuid=None):
        if uuid is None:
            uuid = zope.component.getUtility(zc.async.interfaces.IUUID)
        if uuid in _dispatchers:
            raise ValueError('dispatcher for this UUID is already registered')
        _dispatchers[uuid] = self
        self.db = db
        self.reactor = reactor # we may allow the ``reactor`` argument to be
        # None at some point, to default to the installed Twisted reactor.
        self.poll_interval = poll_interval
        self.UUID = uuid
        # we keep these small so that memory usage doesn't balloon too big.
        # for polls, about 10 minutes at 5 seconds a poll with a fairly large
        # poll size of maybe 300 bytes means 12 polls/minute, or 120 polls,
        # * 300 == 36000, about 36K.  Not too bad.  Jobs can take much more
        # memory depending on the result--a failure takes a lot of memory, for
        # instance--and there's no real way to guess how many we would get in
        # a given period of time.  With a wild guess of an average of a K per
        # job, and storage of 20 minutes, we would get 240K for 12 jobs a
        # minute, or 1.2M for a job a second, and so on.  That's much bigger,
        # but we still have a long way to go before we have noticeable memory
        # consumption on typical production machines.
        # We keep jobs longer than polls because you may want to find out
        # about active jobs in a given poll, and jobs will begin their
        # timeout period when they are begun, so we give a bit of cushion.
        self.polls = zc.async.utils.Periodic(
            period=datetime.timedelta(minutes=10), buckets=5) # max of 12.5 min
        self.jobs = zope.bforest.periodic.OOBForest(
            period=datetime.timedelta(minutes=20), count=9) # max of 22.5 min
        self._activated = set()
        self.queues = {}
        self.dead_pools = []

    def _getJob(self, agent):
        try:
            job = agent.claimJob()
        except zc.twist.EXPLOSIVE_ERRORS:
            transaction.abort()
            raise
        except:
            transaction.abort()
            zc.async.utils.log.error(
                'Error trying to get job for UUID %s from '
                'agent %s (oid %s) in queue %s (oid %s)', 
                self.UUID, agent.name, agent._p_oid,
                agent.queue.name,
                agent.queue._p_oid, exc_info=True)
            return zc.twist.Failure()
        res = self._commit(
            'Error trying to commit getting a job for UUID %s from '
            'agent %s (oid %s) in queue %s (oid %s)' % (
            self.UUID, agent.name, agent._p_oid,
            agent.queue.name,
            agent.queue._p_oid))
        if res is None:
            # Successful commit
            res = job
        return res

    def _commit(self, debug_string=''):
        retry = 0
        while 1:
            try:
                transaction.commit()
            except ZODB.POSException.TransactionError:
                transaction.abort()
                if retry >= 5:
                    zc.async.utils.log.error(
                        'Repeated transaction error trying to commit in '
                        'zc.async: %s', 
                        debug_string, exc_info=True)
                    return zc.twist.Failure()
                retry += 1
            except zc.twist.EXPLOSIVE_ERRORS:
                transaction.abort()
                raise
            except:
                transaction.abort()
                zc.async.utils.log.error(
                    'Error trying to commit: %s', 
                    debug_string, exc_info=True)
                return zc.twist.Failure()
            else:
                break

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
                    if da.activated:
                        if da.dead:
                            da.deactivate()
                        else:
                            zc.async.utils.log.error(
                                'UUID %s already activated in queue %s '
                                '(oid %s): another process?  To stop '
                                'poll attempts in this process, set '
                                '``zc.async.dispatcher.get().activated = '
                                "False``.  To stop polls permanently, don't "
                                'start a zc.async.dispatcher!',
                                self.UUID, queue.name, queue._p_oid)
                            continue
                    da.activate()
                    self._activated.add(queue._p_oid)
                    # removed below if transaction fails
                    res = self._commit(
                        'Error trying to commit activation of UUID %s in '
                        'queue %s (oid %s)' % (
                            self.UUID, queue.name, queue._p_oid))
                    if res is not None:
                        self._activated.remove(queue._p_oid)
                        continue
                queue_info = poll_info[queue.name] = {}
                pools = self.queues.get(queue.name)
                if pools is None:
                    pools = self.queues[queue.name] = {}
                for name, agent in da.items():
                    job_info = []
                    active_jobs = [
                        (job._p_oid,
                         getattr(job._p_jar.db(), 'database_name', None))
                         for job in agent]
                    agent_info = queue_info[name] = {
                        'size': None, 'len': None, 'error': None,
                        'new jobs': job_info, 'active jobs': active_jobs}
                    try:
                        agent_info['size'] = agent.size
                        agent_info['len'] = len(agent)
                    except zc.twist.EXPLOSIVE_ERRORS:
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
                            try:
                                agent.failure = res
                            except zc.twist.EXPLOSIVE_ERRORS:
                                raise
                            except:
                                transaction.abort()
                                zc.async.utils.log.error(
                                    'error trying to stash failure on agent')
                            else:
                                # TODO improve msg
                                self._commit('trying to stash failure on agent')
                        else:
                            info = {'result': None,
                                    'failed': False,
                                    'poll id': None,
                                    'quota names': job.quota_names,
                                    'call': repr(job),
                                    'started': None,
                                    'completed': None,
                                    'thread': None}
                            started_jobs.append(info)
                            dbname = getattr(
                                job._p_jar.db(), 'database_name', None)
                            jobid = (job._p_oid, dbname)
                            self.jobs[jobid] = info
                            job_info.append(jobid)
                            pool.queue.put(
                                (job._p_oid, dbname, info))
                            job = self._getJob(agent)
                queue.dispatchers.ping(self.UUID)
                self._commit('trying to commit ping')
                if len(pools) > len(queue_info):
                    conn_delta = 0
                    for name, pool in pools.items():
                        if name not in agent_info:
                            conn_delta += pool.setSize(0)
                            self.dead_pools.append(pools.pop(name))
                    if conn_delta:
                        db = queues._p_jar.db()
                        # this is a bit premature--it should really happen
                        # when all threads are complete--but since the pool just
                        # complains if the size is not honored, and this approach
                        # is easier, we're doing this.
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
        try:
            self.poll()
        finally:
            self.reactor.callFromThread(deferred.callback, None)

    def threadedPoll(self):
        if not self.activated:
            return
        deferred = twisted.internet.defer.Deferred()
        self.reactor.callInThread(self._inThreadPoll, deferred)
        deferred.addCallback(
            lambda result: self.reactor.callLater(
                self.poll_interval, self.threadedPoll))

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
        self.activated = False
        transaction.begin()
        try:
            queues = self.conn.root().get(zc.async.interfaces.KEY)
            if queues is not None:
                for queue in queues.values():
                    da = queue.dispatchers.get(self.UUID)
                    if da is not None and da.activated:
                        da.deactivate()
                self._commit('trying to tear down')
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
            maxKey = self.jobs.maxKey((oid,))
            if minKey != maxKey:
                raise ValueError('ambiguous database name')
            else:
                database_name = minKey[1]
        return self.jobs[(oid, database_name)]

    def getActiveJobIds(self, queue=None, agent=None):
        """returns active jobs from newest to oldest"""
        res = []
        try:
            poll = self.polls.first()
        except ValueError:
            pass
        else:
            old = []
            unknown = []
            for info in _iter_info(poll, queue, agent):
                res.extend(info['new jobs'])
                for job_id in info['active jobs']:
                    job_info = self.jobs.get(job_id)
                    if job_info is None:
                        unknown.append(job_id)
                    else:
                        bisect.insort(old, (job_info['poll id'], job_id))
            res.extend(i[1] for i in old)
            res.extend(unknown)
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
        for bucket in tuple(self.polls._data.buckets): # freeze order
            try:
                if at is None:
                    key = bucket.minKey()
                else:
                    key = bucket.minKey(at)
                return bucket[key]
            except (ValueError, KeyError):
                # ValueError because minKey might not have a value
                # KeyError because bucket might be cleared in another thread
                # between minKey and __getitem__
                pass
        raise ValueError('no poll matches')

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
