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
import datetime

import BTrees.OOBTree
import ZODB.POSException
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

def completeStartedJobArguments(job, result):
    if isinstance(result, twisted.python.failure.Failure):
        for collection in (job.args, job.kwargs.values()):
            for a in collection:
                if zc.async.interfaces.IJob.providedBy(a):
                    status = a.status
                    if status == zc.async.interfaces.ACTIVE:
                        a.fail()
                    elif status == zc.async.interfaces.CALLBACKS:
                        a.resumeCallbacks()

class Job(zc.async.utils.Base):

    zope.interface.implements(zc.async.interfaces.IJob)

    _callable_root = _callable_name = _result = None
    _status = zc.async.interfaces.NEW
    _begin_after = _begin_by = _active_start = _active_end = None
    key = None
    
    assignerUUID = None
    _quota_names = ()

    def __init__(self, *args, **kwargs):
        self.args = persistent.list.PersistentList(args) # TODO: blist
        self.callable = self.args.pop(0)
        self.kwargs = persistent.mapping.PersistentMapping(kwargs)
        self.callbacks = zc.queue.PersistentQueue()
        self.annotations = BTrees.OOBTree.OOBTree()

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
        if self._status == zc.async.interfaces.NEW:
            ob = self.parent
            while (ob is not None and
                   zc.async.interfaces.IJob.providedBy(ob)):
                ob = ob.parent
            if zc.async.interfaces.IAgent.providedBy(ob):
                return zc.async.interfaces.ASSIGNED
            elif zc.async.interfaces.IQueue.providedBy(ob):
                return zc.async.interfaces.PENDING
        return self._status

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
        # add the sugar ourselves
        if self._status != zc.async.interfaces.NEW:
            raise zc.async.interfaces.BadStatusError(
                'can only set callable when a job has NEW, PENDING, or '
                'ASSIGNED status')
        if isinstance(value, types.MethodType):
            self._callable_root = value.im_self
            self._callable_name = value.__name__
        elif isinstance(value, zc.twist.METHOD_WRAPPER_TYPE):
            self._callable_root = zc.twist.get_self(value)
            self._callable_name = value.__name__
        else:
            self._callable_root, self._callable_name = value, None

    def addCallbacks(self, success=None, failure=None):
        if success is not None or failure is not None:
            if success is not None:
                success = zc.async.interfaces.IJob(success)
            if failure is not None:
                failure = zc.async.interfaces.IJob(failure)
            res = Job(success_or_failure, success, failure)
            if success is not None:
                success.parent = res
            if failure is not None:
                failure.parent = res
            self.addCallback(res)
            # we need to handle the case of callbacks on the internal success/
            # failure jobs, to be safe.
            abort_handler = zc.async.interfaces.IJob(
                completeStartedJobArguments)
            abort_handler.args.append(res)
            res.addCallback(abort_handler)
        else:
            res = self
        return res

    def addCallback(self, callback):
        callback = zc.async.interfaces.IJob(callback)
        self.callbacks.put(callback)
        callback.parent = self
        if self._status == zc.async.interfaces.COMPLETED:
            callback(self.result) # this commits transactions!
        else:
            self._p_changed = True # to try and fire conflict errors if
            # our reading of self.status has changed beneath us
        return callback

    def __call__(self, *args, **kwargs):
        if self.status not in (zc.async.interfaces.NEW,
                               zc.async.interfaces.ASSIGNED):
            raise zc.async.interfaces.BadStatusError(
                'can only call a job with NEW or ASSIGNED status')
        tm = transaction.interfaces.ITransactionManager(self)
        self._status = zc.async.interfaces.ACTIVE
        self._active_start = datetime.datetime.now(pytz.UTC)
        tm.commit()
        effective_args = list(args)
        effective_args[0:0] = self.args
        effective_kwargs = dict(self.kwargs)
        effective_kwargs.update(kwargs)
        return self._call_with_retry(
            lambda: self.callable(*effective_args, **effective_kwargs))

    def _call_with_retry(self, call):
        ct = 0
        tm = transaction.interfaces.ITransactionManager(self)
        res = None
        while 1:
            try:
                res = call()
                if zc.async.interfaces.IJob.providedBy(res):
                    res.addCallback(self._callback)
                    tm.commit()
                elif isinstance(res, twisted.internet.defer.Deferred):
                    res.addBoth(zc.twist.Partial(self._callback))
                    tm.commit()
                else:
                    res = self._complete(res, tm)
            except ZODB.POSException.TransactionError:
                tm.abort()
                ct += 1
                if ct >= 5:
                    res = self._complete(zc.twist.Failure(), tm)
                    self.resumeCallbacks()
                else:
                    continue
            except zc.twist.EXPLOSIVE_ERRORS:
                tm.abort()
                raise
            except:
                tm.abort()
                res = self._complete(zc.twist.Failure(), tm)
                self.resumeCallbacks()
            else:
                if self._status == zc.async.interfaces.CALLBACKS:
                    self.resumeCallbacks()
            return res

    def _callback(self, res):
        self._call_with_retry(lambda: res)

    def _complete(self, res, tm):
        if isinstance(res, twisted.python.failure.Failure):
            res = zc.twist.sanitize(res)
            failure = True
        else:
            failure = False
        self._result = res
        self._status = zc.async.interfaces.CALLBACKS
        self._active_end = datetime.datetime.now(pytz.UTC)
        tm.commit()
        if failure:
            zc.async.utils.tracelog.error(
                '%r failed with traceback:\n%s',
                self,
                res.getTraceback(elideFrameworkCode=True, detail='verbose'))
        else:
            zc.async.utils.tracelog.info(
                '%r succeeded with result:\n%r',
                self, res)
        return res

    def fail(self, e=None):
        if e is None:
            e = zc.async.interfaces.AbortedError()
        if self._status not in (zc.async.interfaces.NEW,
                                zc.async.interfaces.ACTIVE):
            raise zc.async.interfaces.BadStatusError(
                'can only call fail on a job with NEW, PENDING, ASSIGNED, or '
                'ACTIVE status')
        self._complete(zc.twist.Failure(e),
                       transaction.interfaces.ITransactionManager(self))
        self.resumeCallbacks()

    def resumeCallbacks(self):
        if self._status != zc.async.interfaces.CALLBACKS:
            raise zc.async.interfaces.BadStatusError(
                'can only resumeCallbacks on a job with CALLBACKS status')
        callbacks = list(self.callbacks)
        tm = transaction.interfaces.ITransactionManager(self)
        length = 0
        while 1:
            for j in callbacks:
                if j._status == zc.async.interfaces.NEW:
                    zc.async.utils.tracelog.debug(
                        'starting callback %r to %r', j, self)
                    j(self.result)
                elif j._status == zc.async.interfaces.ACTIVE:
                    zc.async.utils.tracelog.debug(
                        'failing aborted callback %r to %r', j, self)
                    j.fail()
                elif j._status == zc.async.interfaces.CALLBACKS:
                    j.resumeCallbacks()
                # TODO: this shouldn't raise anything we want to catch, right?
                # now, this should catch all the errors except EXPLOSIVE_ERRORS
                # cleaning up dead jobs should look something like the above.
            tm.commit()
            tm.begin() # syncs
            # it's possible that someone added some callbacks, so run until
            # we're exhausted.
            length += len(callbacks)
            callbacks = list(self.callbacks)[length:]
            if not callbacks:
                try:
                    self._status = zc.async.interfaces.COMPLETED
                    if zc.async.interfaces.IAgent.providedBy(self.parent):
                        self.parent.jobCompleted(self)
                    tm.commit()
                except ZODB.POSException.TransactionError:
                    tm.abort()
                    callbacks = list(self.callbacks)[length:]
                else:
                    break # and return

