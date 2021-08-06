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
import types
import datetime
import logging

import BTrees.OOBTree
import ZODB.POSException
import ZEO.Exceptions
import transaction.interfaces
import persistent
import persistent.list
import persistent.mapping
import twisted.internet.defer
import twisted.python.failure
import zope.interface
import zc.queue
import zc.twist
import rwproperty
import pytz

import zc.async.interfaces
import zc.async.utils
import zc.async

def _repr(obj):
    if isinstance(obj, persistent.Persistent):
        dbname = "?"
        if obj._p_jar is not None:
            dbname = getattr(obj._p_jar.db(), 'database_name', "?")
            if dbname != '?':
                dbname = repr(dbname)
        if obj._p_oid is not None:
            oid = ZODB.utils.u64(obj._p_oid)
        else:
            oid = '?'
        return '%s.%s (oid %s, db %s)' % (
            obj.__class__.__module__,
            obj.__class__.__name__,
            oid,
            dbname)
    elif isinstance(obj, types.FunctionType):
        return '%s.%s' % (obj.__module__, obj.__name__)
    else:
        return repr(obj)

# this is kept so that legacy databases can keep their references to this
# function
def success_or_failure(success, failure, res):
    callable = None
    if isinstance(res, twisted.python.failure.Failure):
        if failure is not None:
            callable = failure
    elif success is not None:
        callable = success
    if callable is None:
        return res
    return callable(res)

class RetryCommonFourTimes(persistent.Persistent): # default
    zope.component.adapts(zc.async.interfaces.IJob)
    zope.interface.implements(zc.async.interfaces.IRetryPolicy)

    # exceptions, data_cache key, max retry, initial backoff seconds,
    # incremental backoff seconds, max backoff seconds
    internal_exceptions = (
        ((ZEO.Exceptions.ClientDisconnected,), 'zeo_disconnected',
         None, 5, 5, 60),
        ((ZODB.POSException.TransactionError,), 'transaction_error',
         5, 0, 0, 0),
    )
    transaction_exceptions = internal_exceptions
    max_interruptions = 9
    log_every = 5

    def __init__(self, job):
        self.parent = self.__parent__ = job
        self.data = BTrees.family32.OO.BTree()

    def updateData(self, data_cache):
        if 'first_active' in self.data and 'first_active' in data_cache:
            data_cache.pop('first_active')
        self.data.update(data_cache)

    def jobError(self, failure, data_cache):
        return self._process(failure, data_cache, self.internal_exceptions)

    def commitError(self, failure, data_cache):
        return self._process(failure, data_cache, self.transaction_exceptions)

    def _process(self, failure, data_cache, exceptions):
        for (exc, key, max_count, init_backoff,
             incr_backoff, max_backoff) in exceptions:
            if failure.check(*exc) is not None:
                count = data_cache.get(key, 0) + 1
                if max_count is not None and count >= max_count:
                    zc.async.utils.tracelog.warning(
                        'Retry policy for job %r is not retrying after %d '
                        'counts of %s occurrences', self.parent, count, key)
                    return False
                elif count==1 or not count % self.log_every:
                    zc.async.utils.tracelog.warning(
                        'Retry policy for job %r requests another attempt '
                        'after %d counts of %s occurrences', self.parent,
                        count, key, exc_info=True)
                backoff = min(max_backoff,
                              (init_backoff + (count-1) * incr_backoff))
                if backoff:
                    time.sleep(backoff)
                data_cache[key] = count
                data_cache['last_' + key] = failure
                if 'first_active' not in data_cache:
                    data_cache['first_active'] = self.parent.active_start
                return True
        return False

    def interrupted(self):
        if 'first_active' not in self.data:
            self.data['first_active'] = self.parent.active_start
        count = self.data['interruptions'] = self.data.get('interruptions', 0) + 1
        if self.max_interruptions is None or count <= self.max_interruptions:
            if count==1 or not count % self.log_every:
                zc.async.utils.tracelog.info(
                    'Retry policy for job %r requests another attempt '
                    'after %d interrupts', self.parent, count)
            return True
        else:
            zc.async.utils.tracelog.info(
                'Retry policy for job %r is not retrying after %d '
                'interrupts', self.parent, count)
            return False


class RetryCommonForever(RetryCommonFourTimes):
    # retry on ZEO failures and Transaction errors during the job forever
    # retry on commitErrors and interrupteds forever.
    internal_exceptions = (
        ((ZEO.Exceptions.ClientDisconnected,), 'zeo_disconnected',
         None, 5, 5, 60),
        ((ZODB.POSException.TransactionError,), 'transaction_error',
         None, 0, 0, 0),
    )

    max_interruptions = None
    other_commit_initial_backoff = 0
    other_commit_incremental_backoff = 1
    other_commit_max_backoff = 60

    def commitError(self, failure, data_cache):
        res = super(RetryCommonForever, self).commitError(failure, data_cache)
        if not res:
            # that just means we didn't record it.  We actually are going to
            # retry.  However, we are going to back these off.
            key = 'other'
            count = data_cache['other'] = data_cache.get('other', 0) + 1
            data_cache['last_other'] = failure
            if 'first_active' not in data_cache:
                data_cache['first_active'] = self.parent.active_start
            backoff = min(self.other_commit_max_backoff,
                          (self.other_commit_initial_backoff +
                           (count-1) * self.other_commit_incremental_backoff))
            if count==1 or not count % self.log_every:
                # this is critical because it is unexpected.  Someone probably
                # needs to see this. We can't move on until it is dealt with.
                zc.async.utils.log.critical(
                    'Retry policy for job %r requests another attempt in %d '
                    'seconds after %d counts of %s occurrences',
                    self.parent, backoff, count, key, exc_info=True)
            if backoff:
                time.sleep(backoff)
        return True # always retry

class NeverRetry(persistent.Persistent):
    zope.component.adapts(zc.async.interfaces.IJob)
    zope.interface.implements(zc.async.interfaces.IRetryPolicy)

    def __init__(self, job):
        self.parent = self.__parent__ = job

    def updateData(self, data_cache):
        pass

    def jobError(self, failure, data_cache):
        return False

    def commitError(self, failure, data_cache):
        return False

    def interrupted(self):
        return False

def callback_retry_policy_factory(job):
    res = zope.component.queryAdapter(
        job, zc.async.interfaces.IRetryPolicy, 'callback')
    if res is None:
        res = RetryCommonForever(job)
    return res

def isFailure(value):
    return isinstance(value, twisted.python.failure.Failure)

def _prepare_callback(callback, failure_log_level=None,
                      retry_policy_factory=None, parent=None):
    if not zc.async.interfaces.ICallbackProxy.providedBy(callback):
        callback = zc.async.interfaces.IJob(callback)
        if failure_log_level is not None:
            callback.failure_log_level = failure_log_level
        elif callback.failure_log_level is None:
            callback.failure_log_level = logging.CRITICAL
        if retry_policy_factory is not None:
            callback.retry_policy_factory = retry_policy_factory
        elif callback.retry_policy_factory is None:
            callback.retry_policy_factory = callback_retry_policy_factory
    callback.parent = parent
    return callback

class ConditionalCallbackProxy(zc.async.utils.Base):

    zope.interface.implements(zc.async.interfaces.ICallbackProxy)

    job = None

    @property
    def status(self):
        # NEW -> (PENDING -> ASSIGNED ->) ACTIVE -> CALLBACKS -> COMPLETED
        if self.job is None:
            ob = self.parent
            while (ob is not None and
                   zc.async.interfaces.IJob.providedBy(ob)):
                ob = ob.parent
            if zc.async.interfaces.IAgent.providedBy(ob):
                return zc.async.interfaces.ASSIGNED
            elif zc.async.interfaces.IQueue.providedBy(ob):
                return zc.async.interfaces.PENDING
            return zc.async.interfaces.NEW
        return self.job.status

    @property
    def result(self):
        if self.job is None:
            return None
        return self.job.result

    def __init__(self, *args, **kwargs):
        kwargs['parent'] = self
        default = None
        if not args:
            pass
        elif args[-1] is None:
            args = args[:-1]
        elif getattr(args[-1], '__len__', None) is None:
            default = _prepare_callback(args[-1], **kwargs)
            args = args[:-1]
        self.default = default
        self.conditionals = persistent.list.PersistentList()
        for condition, job in args:
            if job is not None:
                job = _prepare_callback(job, **kwargs)
            self.conditionals.append((condition, job))
        self.callbacks = zc.queue.PersistentQueue()

    def getJob(self, result):
        if self.job is None:
            for condition, callable in self.conditionals:
                if condition(result):
                    break
            else:
                callable = self.default
            if callable is None:
                callable = _prepare_callback(_transparent, None, None, self)
            self.job = callable
        else:
            callable = self.job
        while self.callbacks:
            callable.addCallback(self.callbacks.pull())
        return callable

    def addCallbacks(self, success=None, failure=None,
                     failure_log_level=None, retry_policy_factory=None):
        return self.addCallback(SuccessFailureCallbackProxy(
            success, failure,
            failure_log_level=failure_log_level,
            retry_policy_factory=retry_policy_factory))

    def addCallback(self, callback, failure_log_level=None,
                    retry_policy_factory=None):
        callback = _prepare_callback(
            callback, failure_log_level, retry_policy_factory, self)
        if self.job is None:
            self.callbacks.put(callback)
        else:
            self.job.addCallback(callback)
        return callback


class SuccessFailureCallbackProxy(ConditionalCallbackProxy):

    @property
    def success(self):
        return self.default

    @property
    def failure(self):
        return self.conditionals[0][1]

    def __init__(self, success, failure, failure_log_level=None,
                 retry_policy_factory=None):
        super(SuccessFailureCallbackProxy, self).__init__(
            (isFailure, failure), success,
            failure_log_level=failure_log_level,
            retry_policy_factory=retry_policy_factory)

_status_mapping = {
    0: zc.async.interfaces.NEW,
    # calculated: zc.async.interfaces.PENDING, 
    # calculated: zc.async.interfaces.ASSIGNED,
    1: zc.async.interfaces.ACTIVE,
    2: zc.async.interfaces.CALLBACKS,
    3: zc.async.interfaces.COMPLETED}


class Job(zc.async.utils.Base):

    zope.interface.implements(zc.async.interfaces.IJob)

    _callable_root = _callable_name = _result = None
    _status_id = None
    _status = None # legacy; we use _status_id now
    _begin_after = _begin_by = _active_start = _active_end = None
    key = None
    _retry_policy = None
    retry_policy_factory = None # effectively "look up IRetryPolicy adapter
    # for '' (empty string) name, and use RetryCommonFourTimes if the adapter
    # doesn't exist"
    failure_log_level = None # effectively logging.ERROR
    assignerUUID = None
    _quota_names = ()

    def __init__(self, *args, **kwargs):
        self._status_id = 0 # we do this here rather than in the class because
        # the attribute is new; if _status_id is set, we know we can ignore
        # the legacy _status value.
        self.args = persistent.list.PersistentList(args) # TODO: blist
        self.callable = self.args.pop(0)
        self.kwargs = persistent.mapping.PersistentMapping(kwargs)
        self.callbacks = zc.queue.PersistentQueue()
        self.annotations = BTrees.OOBTree.OOBTree()

    def setUp(self):
        # a hook (see z3.py, for instance) used in __call__
        pass

    def tearDown(self, setup_info):
        # a hook (see z3.py, for instance) used in __call__
        pass

    @property
    def active_start(self):
        return self._active_start

    @property
    def active_end(self):
        return self._active_end

    @property
    def initial_callbacks_end(self):
        return self.key and zc.async.utils.long_to_dt(self.key).replace(
            tzinfo=pytz.UTC)

    @property
    def quota_names(self):
        return self._quota_names
    @rwproperty.setproperty
    def quota_names(self, value):
        if isinstance(value, basestring):
            raise TypeError('provide an iterable of names')
        status = self.status
        if status != zc.async.interfaces.NEW:
            if status == zc.async.interfaces.PENDING:
                quotas = self.queue.quotas
                for name in value:
                    if name not in quotas:
                        raise ValueError('unknown quota name', name)
            else:
                raise zc.async.interfaces.BadStatusError(
                    'can only set quota_names when a job has NEW or PENDING '
                    'status')
        self._quota_names = tuple(value)

    @property
    def begin_after(self):
        return self._begin_after
    @rwproperty.setproperty
    def begin_after(self, value):
        if self.status != zc.async.interfaces.NEW:
            raise zc.async.interfaces.BadStatusError(
                'can only set begin_after when a job has NEW status')
        if value is not None:
            if value.tzinfo is None:
                raise ValueError('cannot use timezone-naive values')
            else:
                value = value.astimezone(pytz.UTC)
        self._begin_after = value

    @property
    def begin_by(self):
        return self._begin_by
    @rwproperty.setproperty
    def begin_by(self, value):
        if self.status not in (zc.async.interfaces.PENDING,
                               zc.async.interfaces.NEW):
            raise zc.async.interfaces.BadStatusError(
                'can only set begin_by when a job has NEW or PENDING status')
        if value is not None:
            if value < datetime.timedelta():
                raise ValueError('negative values are not allowed')
        self._begin_by = value

    @property
    def queue(self):
        ob = self.parent
        while (ob is not None and
               (zc.async.interfaces.IJob.providedBy(ob) or
                zc.async.interfaces.IAgent.providedBy(ob) or
                zc.async.interfaces.IDispatcherAgents.providedBy(ob))):
            ob = ob.parent
        if not zc.async.interfaces.IQueue.providedBy(ob):
            ob = None
        return ob

    @property
    def agent(self):
        ob = self.parent
        while (ob is not None and
               zc.async.interfaces.IJob.providedBy(ob)):
            ob = ob.parent
        if not zc.async.interfaces.IAgent.providedBy(ob):
            ob = None
        return ob

    @property
    def result(self):
        return self._result

    @property
    def status(self):
        # NEW -> (PENDING -> ASSIGNED ->) ACTIVE -> CALLBACKS -> COMPLETED
        if self._status_id is None: # legacy
            res = self._status
        else:
            res = _status_mapping[self._status_id]
        if res == zc.async.interfaces.NEW:
            ob = self.parent
            while (ob is not None and
                   zc.async.interfaces.IJob.providedBy(ob)):
                ob = ob.parent
            if zc.async.interfaces.IAgent.providedBy(ob):
                res = zc.async.interfaces.ASSIGNED
            elif zc.async.interfaces.IQueue.providedBy(ob):
                res = zc.async.interfaces.PENDING
        return res

    @classmethod
    def bind(klass, *args, **kwargs):
        res = klass(*args, **kwargs)
        res.args.insert(0, res)
        return res

    def __repr__(self):
        try:
            call = _repr(self._callable_root)
            if self._callable_name is not None:
                call += ' :' + self._callable_name
            args = ', '.join(_repr(a) for a in self.args)
            kwargs = ', '.join(k+"="+_repr(v) for k, v in self.kwargs.items())
            if args:
                if kwargs:
                    args += ", " + kwargs
            else:
                args = kwargs
            return '<%s ``%s(%s)``>' % (_repr(self), call, args)
        except (TypeError, ValueError, AttributeError):
            # broken reprs are a bad idea; they obscure problems
            return super(Job, self).__repr__()

    @property
    def callable(self):
        if self._callable_name is None:
            return self._callable_root
        else:
            return getattr(self._callable_root, self._callable_name)
    @rwproperty.setproperty
    def callable(self, value):
        # can't pickle/persist methods by default as of this writing, so we
        # add the sugar ourselves.  In future, would like for args to be
        # potentially methods of persistent objects too...
        if self.status != zc.async.interfaces.NEW:
            raise zc.async.interfaces.BadStatusError(
                'can only set callable when a job has NEW, PENDING, or '
                'ASSIGNED status')
        if isinstance(value, types.MethodType):
            self._callable_root = value.im_self
            self._callable_name = value.__name__
        elif isinstance(value, zc.twist.METHOD_WRAPPER_TYPE):
            self._callable_root = zc.twist.get_self(value)
            self._callable_name = value.__name__
        elif (isinstance(value, types.BuiltinMethodType) and
              getattr(value, '__self__', None) is not None):
            self._callable_root = value.__self__
            self._callable_name = value.__name__
        else:
            self._callable_root, self._callable_name = value, None
        if (zc.async.interfaces.IJob.providedBy(self._callable_root) and
            self._callable_root.parent is None):
            # if the parent is already set, that is probably an agent or
            # something like that.  Don't override, or else the agent won't
            # get cleaned out.
            self._callable_root.parent = self

    def addCallbacks(self, success=None, failure=None,
                     failure_log_level=None, retry_policy_factory=None):
        return self.addCallback(SuccessFailureCallbackProxy(
            success, failure,
            failure_log_level=failure_log_level,
            retry_policy_factory=retry_policy_factory))

    def addCallback(self, callback, failure_log_level=None,
                    retry_policy_factory=None):
        callback = _prepare_callback(
            callback, failure_log_level, retry_policy_factory, self)
        self.callbacks.put(callback)
        if self.status == zc.async.interfaces.COMPLETED:
            if zc.async.interfaces.ICallbackProxy.providedBy(callback):
                call = callback.getJob(self.result)
            else:
                call = callback
            call(self.result) # this commits transactions!
        else:
            self._p_changed = True # to try and fire conflict errors if
            # our reading of self.status has changed beneath us
        return callback

    def getRetryPolicy(self):
        if self._retry_policy is not None:
            return self._retry_policy
        if self.retry_policy_factory is None:
            # first try to look up adapter with name of ''; then if that fails
            # use RetryCommonFourTimes
            res = zope.component.queryAdapter(
                self, zc.async.interfaces.IRetryPolicy, '')
            if res is None:
                res = RetryCommonFourTimes(self)
        elif isinstance(self.retry_policy_factory, basestring):
            res = zope.component.getAdapter(
                self, zc.async.interfaces.IRetryPolicy,
                self.retry_policy_factory)
            # this may cause an error. We can't proceed because we don't know
            # what to do, and it may be *critical* to know. Therefore, in
            # _getRetry, we rely on never_fail to keep on sending critical
            # errors in the log, and never stopping.
        else:
            res = self.retry_policy_factory(self)
        self._retry_policy = res
        return res

    def _getRetry(self, call_name, tm, *args):
        # if we are after the time that we are supposed to begin_by, no retry
        if (self.begin_by is not None and self.begin_after is not None and
            self.begin_by + self.begin_after > datetime.datetime.now(pytz.UTC)):
            return False
        # we divide up the two ``never_fail`` calls so that retries in getting
        # the policy don't affect actually calling the method.
        identifier = 'getting retry policy for %r' % (self,)
        policy = zc.async.utils.never_fail(self.getRetryPolicy, identifier, tm)
        call = getattr(policy, call_name, None)
        if call is None:
            zc.async.utils.log.error(
                'retry policy %r for %r does not have required %s method',
                policy, self, call_name)
            return None
        identifier = 'getting result for %s retry for %r' % (call_name, self)
        res = zc.async.utils.never_fail(lambda: call(*args), identifier, tm)
        self._check_reassigned((zc.async.interfaces.ACTIVE,)) # will raise
        # exception if necessary
        return res

    def __call__(self, *args, **kwargs):
        statuses = (zc.async.interfaces.NEW, zc.async.interfaces.ASSIGNED)
        if self.status not in statuses:
            raise zc.async.interfaces.BadStatusError(
                'can only call a job with NEW or ASSIGNED status')
        tm = transaction.interfaces.ITransactionManager(self)
        def prepare():
            self._check_reassigned(statuses)
            self._status_id = 1 # ACTIVE
            self._active_start = datetime.datetime.now(pytz.UTC)
            effective_args = list(args)
            effective_args[0:0] = self.args
            effective_kwargs = dict(self.kwargs)
            effective_kwargs.update(kwargs)
            return effective_args, effective_kwargs
        identifier = 'preparing for call of %r' % (self,)
        effective_args, effective_kwargs = zc.async.utils.never_fail(
            prepare, identifier, tm)
        # this is the calling code.  It is complex and long because it is
        # trying both to handle exceptions reasonably, and to honor the
        # IRetryPolicy interface for those exceptions.
        data_cache = {}
        res = None
        while 1:
            zc.async.local.job = self # we do this in the loop for paranoia
            try:
                setup_info = self.setUp()
                res = self.callable(*effective_args, **effective_kwargs)
            except zc.async.utils.EXPLOSIVE_ERRORS:
                tm.abort()
                zc.async.utils.try_five_times(
                    lambda: self.tearDown(setup_info),
                    'tearDown for %r' % self, tm, commit=False)
                raise
            except:
                res = zc.twist.Failure()
                tm.abort()
                zc.async.utils.try_five_times(
                    lambda: self.tearDown(setup_info),
                    'tearDown for %r' % self, tm, commit=False)
                retry = self._getRetry('jobError', tm, res, data_cache)
                if isinstance(retry, (datetime.timedelta, datetime.datetime)):
                    identifier = (
                        'rescheduling %r as requested by '
                        'associated IRetryPolicy %r' % (
                            self, self.getRetryPolicy()))
                    if self is zc.async.utils.never_fail(
                        lambda: self._reschedule(retry, data_cache),
                        identifier, tm):
                        zc.async.local.job = None
                        return self
                elif retry:
                    continue
                # policy didn't exist or returned False or couldn't reschedule
            try:
                callback = self._set_result(res, tm, data_cache)
            except zc.async.utils.EXPLOSIVE_ERRORS:
                tm.abort()
                zc.async.utils.try_five_times(
                    lambda: self.tearDown(setup_info),
                    'tearDown for %r' % self, tm, commit=False)
                raise
            except:
                failure = zc.twist.Failure()
                tm.abort()
                zc.async.utils.try_five_times(
                    lambda: self.tearDown(setup_info),
                    'tearDown for %r' % self, tm, commit=False)
                retry = self._getRetry('commitError', tm, failure, data_cache)
                if isinstance(retry, (datetime.timedelta, datetime.datetime)):
                    identifier = (
                        'rescheduling %r as requested by '
                        'associated IRetryPolicy %r' % (
                            self, self.getRetryPolicy()))
                    if self is zc.async.utils.never_fail(
                        lambda: self._reschedule(retry, data_cache),
                        identifier, tm):
                        zc.async.local.job = None
                        return self
                elif retry:
                    continue
                # policy didn't exist or returned False or couldn't reschedule
                if isinstance(res, twisted.python.failure.Failure):
                    log_level = self.failure_log_level
                    if log_level is None:
                        log_level = logging.ERROR
                    zc.async.utils.log.log(
                        log_level,
                        'Commit failed for %r (see subsequent traceback).  '
                        'Prior to this, job failed with traceback:\n%s',
                        self,
                        res.getTraceback(
                            elideFrameworkCode=True, detail='verbose'))
                else:
                    zc.async.utils.log.info(
                        'Commit failed for %r (see subsequent traceback).  '
                        'Prior to this, job succeeded with result: %r',
                        self, res)
                res = failure
                def complete():
                    self._check_reassigned((zc.async.interfaces.ACTIVE,))
                    self._result = res
                    self._status_id = 2 # CALLBACKS
                    self._active_end = datetime.datetime.now(pytz.UTC)
                    policy = self.getRetryPolicy()
                    if data_cache and self._retry_policy is not None:
                        self._retry_policy.updateData(data_cache)
                identifier = 'storing failure at commit of %r' % (self,)
                zc.async.utils.never_fail(complete, identifier, tm)
                callback = True
            else:
                zc.async.utils.try_five_times(
                    lambda: self.tearDown(setup_info),
                    'tearDown for %r' % self, tm, commit=False)
            if callback:
                self._log_completion(res)
                identifier = 'performing callbacks of %r' % (self,)
                zc.async.utils.never_fail(self.resumeCallbacks, identifier, tm)
            zc.async.local.job = None
            return res

    def handleInterrupt(self):
        # should be called within a job that has a RetryCommonForever policy
        tm = transaction.interfaces.ITransactionManager(self)
        if self.status == zc.async.interfaces.ACTIVE:
            retry = self._getRetry('interrupted', tm)
            if isinstance(retry, (datetime.datetime, datetime.timedelta)):
                self._reschedule(retry, queue=self.queue)
            elif retry:
                self._reschedule(datetime.timedelta(), queue=self.queue)
            else:
                res = zc.twist.Failure(zc.async.interfaces.AbortedError())
                if self._set_result(res, tm):
                    self.resumeCallbacks()
                self._log_completion(res)
        elif self.status != zc.async.interfaces.CALLBACKS:
            # we have to allow CALLBACKS or else some retries will fall over,
            # because handleInterrupt may fail after a commit of the aborted
            # error
            raise zc.async.interfaces.BadStatusError(
                'can only call ``handleInterrupt`` on a job with ACTIVE '
                'status') # um...or CALLBACKS, but that's a secret :-D
        else:
            self.resumeCallbacks()

    def fail(self, e=None):
        # something may have fallen over the last time this was called, so we
        # are careful to only store the error if we're not in the CALLBACKS
        # status.
        callback = True
        status = self.status
        if status in (zc.async.interfaces.COMPLETED,
                      zc.async.interfaces.ACTIVE):
            raise zc.async.interfaces.BadStatusError(
                'can only call fail on a job with NEW, PENDING, or ASSIGNED '
                'status') # ...or CALLBACKS, but that's because of
                # retries, and is semantically incorrect
        if status != zc.async.interfaces.CALLBACKS:
            if e is None:
                e = zc.async.interfaces.TimeoutError()
            res = zc.twist.Failure(e)
            callback = self._set_result(
                res, transaction.interfaces.ITransactionManager(self))
            self._log_completion(res)
        if callback:
            self.resumeCallbacks()

    def _reschedule(self, when, data_cache=None, queue=None):
        if not isinstance(when, (datetime.datetime, datetime.timedelta)):
            raise TypeError('``when`` must be datetime or timedelta')
        in_agent = zc.async.interfaces.IAgent.providedBy(self.parent)
        if queue is None:
            # this is a reschedule from jobError or commitError
            if not in_agent:
                zc.async.utils.log.critical(
                    'error for IRetryPolicy %r on %r: '
                    'can only reschedule a job directly in an agent',
                    self.getRetryPolicy(), self)
                return None
            queue = self.queue
        if data_cache is not None and self._retry_policy is not None:
            self._retry_policy.updateData(data_cache)
        self._status_id = 0 # NEW
        self._active_start = None
        if in_agent:
            self.parent.remove(self)
        else:
            self.parent = None
        now = datetime.datetime.now(pytz.UTC)
        if isinstance(when, datetime.datetime):
            if when.tzinfo is None:
                when = when.replace(tzinfo=pytz.UTC)
            if when <= now:
                queue.putBack(self)
            else:
                queue.put(self, begin_after=when)
        elif isinstance(when, datetime.timedelta):
            if when <= datetime.timedelta():
                queue.putBack(self)
            else:
                queue.put(self, begin_after=now+when)
        return self

    def _check_reassigned(self, expected_statuses):
        agent = self.agent
        res = self.status not in expected_statuses or (
            zc.async.interfaces.IAgent.providedBy(agent) and
            not zc.async.interfaces.IJob.providedBy(self._result) and
            zc.async.local.getAgentName() is not None and
            (zc.async.local.getAgentName() != agent.name or
             zc.async.local.getDispatcher().UUID != agent.parent.UUID))
        if res:
            # the only known scenario for this to occur is the following.
            # agent took job.  dispatcher gave it to a thread.  While
            # performing the job, the poll was unable to write to the db,
            # perhaps because of a database disconnect or because of a
            # too-long commit in another process or thread.  Therefore,
            # A sibling has noticed that this agent seems to have died
            # and put this job back in the queue, where it has been claimed
            # by another process/agent.
            # It's debatable whether this is CRITICAL or ERROR level.  We'll
            # go with ERROR for now.
            zc.async.utils.log.error(
                'Job %r was reassigned.  Likely cause was that polling was '
                'unable to occur as regularly as expected, perhaps because of '
                'long commit times in the application.', self)
            raise zc.async.interfaces.ReassignedError()

    def _set_result(self, res, tm, data_cache=None):
        # returns whether to call ``resumeCallbacks``
        callback = True
        if zc.async.interfaces.IJob.providedBy(res):
            res.addCallback(self._callback)
            self._result = res # temporary
            callback = False
        elif isinstance(res, twisted.internet.defer.Deferred):
            partial = zc.twist.Partial(self._callback)
            partial.max_transaction_errors = None # retry conflicts forever
            res.addBoth(partial)
            callback = False
        else:
            if isinstance(res, twisted.python.failure.Failure):
                res = zc.twist.sanitize(res)
            self._result = res
            self._status_id = 2 # CALLBACKS
            self._active_end = datetime.datetime.now(pytz.UTC)
        if self._retry_policy is not None and data_cache:
            self._retry_policy.updateData(data_cache)
        tm.commit() # this should raise a ConflictError if the job has been
        # reassigned.
        return callback

    def _log_completion(self, res):
        if isinstance(res, twisted.python.failure.Failure):
            log_level = self.failure_log_level
            if log_level is None:
                log_level = logging.ERROR
            zc.async.utils.log.log(
                log_level,
                '%r failed with traceback:\n%s',
                self,
                res.getTraceback(
                    elideFrameworkCode=True, detail='verbose'))
        else:
            zc.async.utils.tracelog.info(
                '%r succeeded with result: %r',
                self, res)

    def _callback(self, res):
        # done within a job or partial, so we can rely on their retry bits to
        # some degree.  However, we commit transactions ourselves, so we have
        # to be a bit careful that the result hasn't been set already.
        callback = True
        if self.status == zc.async.interfaces.ACTIVE:
            callback = self._set_result(
                res, transaction.interfaces.ITransactionManager(self))
            self._log_completion(res)
        if callback:
            self.resumeCallbacks()

    def handleCallbackInterrupt(self, caller):
        if self.status != zc.async.interfaces.ACTIVE:
            raise zc.async.interfaces.BadStatusError(
                'can only handleCallbackInterrupt on a job with ACTIVE status')
        if caller.status != zc.async.interfaces.CALLBACKS:
            raise zc.async.interfaces.BadStatusError(
                'can only handleCallbackInterrupt with caller in CALLBACKS '
                'status')
        result = caller.result
        if self.result is not None:
            if not zc.async.interfaces.IJob.providedBy(self.result):
                msg = ('Callback %r is in an apparently insane state: result '
                       'has been set (%r), the result is not a job, and yet '
                       'the status is ACTIVE.  This should not be possible.  ')
                if self.result == result:
                    zc.async.utils.log.error(
                        msg + 'Stored result is equivalent to currently '
                        'received result, so will '
                        'change status to CALLBACKS and '
                        'run callbacks, for no clear "right" action.',
                        self, self.result)
                    self._status_id = 2 # CALLBACKS
                    self._active_end = datetime.datetime.now(pytz.UTC)
                    self.resumeCallbacks()
                    return
                else:
                    zc.async.utils.log.error(
                        msg + 'Stored result is not equivalent to currently '
                        'received result (%r), so will '
                        '(re?)run this job with new result, for no clear '
                        '"right" action.',
                        self, self.result, result)
                    # fall through
            elif self.result.status == zc.async.interfaces.COMPLETED:
                zc.async.utils.log.warning(
                    'Callback %r is in an apparently insane state: inner job '
                     'result has been completed, including callbacks, but '
                     'this job has not been '
                     'completed.  This should not be possible.  Will set '
                     'result and run callbacks, for no clear "right" action.')
                callback = self._set_result(self.result.result)
                self._log_completion(self.result.result)
                if callback:
                    self.resumeCallbacks()
                return
            else:
                return # we are going to hope that the job works; it should,
                # and there's no way for us to know that it won't here.
        tm = transaction.interfaces.ITransactionManager(self)
        retry = self._getRetry('interrupted', tm)
        istime = isinstance(
            retry, (datetime.timedelta, datetime.datetime))
        if istime:
            zc.async.utils.log.error(
                'error for IRetryPolicy %r on %r: '
                'cannot reschedule a callback, only retry.  '
                'We will retry now, for no clear "right" action.',
                self.getRetryPolicy(), self)
        if retry or istime:
            zc.async.utils.tracelog.debug(
                'retrying interrupted callback '
                '%r to %r', self, caller)
            self._status_id = 0 # NEW
            self._active_start = None
            self(result)
        else:
            zc.async.utils.tracelog.debug(
                'aborting interrupted callback '
                '%r to %r', self, caller)
            self.fail(zc.async.interfaces.AbortedError())

    def resumeCallbacks(self):
        # should be called within a job that has a RetryCommonForever policy
        if self.status != zc.async.interfaces.CALLBACKS:
            raise zc.async.interfaces.BadStatusError(
                'can only resumeCallbacks on a job with CALLBACKS status')
        self._check_reassigned((zc.async.interfaces.CALLBACKS,))
        callbacks = list(self.callbacks)
        tm = transaction.interfaces.ITransactionManager(self)
        length = 0
        while 1:
            for j in callbacks:
                self._check_reassigned((zc.async.interfaces.CALLBACKS,))
                if zc.async.interfaces.ICallbackProxy.providedBy(j):
                    j = j.getJob(self.result)
                status = j.status
                if status in (zc.async.interfaces.NEW,
                              zc.async.interfaces.ASSIGNED,
                              zc.async.interfaces.PENDING):
                    if (j.begin_by is not None and
                        (j.begin_after + j.begin_by) <
                        datetime.datetime.now(pytz.UTC)):
                        zc.async.utils.log.error(
                            'failing expired callback %r to %r', j, self)
                        j.fail()
                    else:
                        zc.async.utils.tracelog.debug(
                            'starting callback %r to %r', j, self)
                        j(self.result)
                elif status == zc.async.interfaces.ACTIVE:
                    j.handleCallbackInterrupt(self)
                elif status == zc.async.interfaces.CALLBACKS:
                    j.resumeCallbacks()
                # TODO: this shouldn't raise anything we want to catch, right?
                # now, this should catch all the errors except EXPLOSIVE_ERRORS
                # cleaning up dead jobs should look something like the above.
            tm.begin() # syncs
            # it's possible that someone added some callbacks, so run until
            # we're exhausted.
            length += len(callbacks)
            callbacks = list(self.callbacks)[length:]
            if not callbacks:
                # this whole method is called within a never_fail...
                self._status_id = 3 # COMPLETED
                if zc.async.interfaces.IAgent.providedBy(self.parent):
                    self.parent.jobCompleted(self)
                tm.commit()
                return

# conveniences for serial and parallel jobs

def _transparent(*results):
    return results

def _serial_or_parallel(scheduler, jobs, kw):
    if kw and (len(kw) > 1 or kw.keys()[0] != 'postprocess'):
        raise TypeError('only accepts one keyword argument, ``postprocess``')
    postprocess = zc.async.interfaces.IJob(kw.get('postprocess', _transparent))
    result = Job(scheduler,
                 *(zc.async.interfaces.IJob(j) for j in jobs),
                 **dict(postprocess=postprocess))
    return result

def _queue_next(main_job, ix=0, ignored_result=None):
    jobs = main_job.args
    queue = main_job.queue
    if ix < len(jobs):
        next = jobs[ix]
        queue.put(next)
        next.addCallback(Job(_queue_next, main_job, ix+1))
    else:
        postprocess = main_job.kwargs['postprocess']
        if postprocess.status == zc.async.interfaces.NEW:
            # will not be NEW if this is a retry
            postprocess.args.extend(jobs)
            queue.put(postprocess)

def _schedule_serial(*jobs, **kw):
    for ix, job in enumerate(jobs): # important for interrupts
        if job.status == zc.async.interfaces.NEW:
            break
    else:
        ix += 1
    _queue_next(zc.async.local.getJob(), ix)
    return kw['postprocess']

def serial(*jobs, **kw):
    return _serial_or_parallel(_schedule_serial, jobs, kw)

def _queue_all(main_job, ignored_result=None):
    jobs = main_job.args
    queue = main_job.queue
    complete = True
    for job in jobs:
        status = job.status
        if status == zc.async.interfaces.NEW:
            queue.put(job)
            job.addCallback(Job(_queue_all, main_job))
            complete = False
        elif status not in (zc.async.interfaces.COMPLETED,
                            zc.async.interfaces.CALLBACKS):
            complete = False
    if complete:
        postprocess = main_job.kwargs['postprocess']
        if postprocess.status == zc.async.interfaces.NEW:
            # will not be NEW if this is a retry
            postprocess.args.extend(jobs)
            queue.put(postprocess)

def _schedule_parallel(*jobs, **kw):
    _queue_all(zc.async.local.getJob())
    return kw['postprocess']

def parallel(*jobs, **kw):
    return _serial_or_parallel(_schedule_parallel, jobs, kw)
