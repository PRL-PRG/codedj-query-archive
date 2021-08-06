
import types

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

import zc.async.interfaces
import zc.twist
from zc.async import rwproperty

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

def completeStartedPartialArguments(partial, result):
    if isinstance(result, twisted.python.failure.Failure):
        for collection in (partial.args, partial.kwargs.values()):
            for a in collection:
                if (zc.async.interfaces.IPartial.providedBy(a) and
                    a.state not in (
                        zc.async.interfaces.PENDING,
                        zc.async.interfaces.COMPLETED)):
                    if a.state == zc.async.interfaces.ACTIVE:
                        a.fail()
                    elif a.state == zc.async.interfaces.CALLBACKS:
                        a.resumeCallbacks()
    return result

class Partial(persistent.Persistent):

    zope.interface.implements(zc.async.interfaces.IPartial)
    zope.interface.classProvides(zc.async.interfaces.IPartialFactory)

    __parent__ = _callable_root = _callable_name = _result = None
    _state = zc.async.interfaces.PENDING

    def __init__(self, *args, **kwargs):
        self.args = persistent.list.PersistentList(args)
        self.callable = self.args.pop(0)
        self.kwargs = persistent.mapping.PersistentMapping(kwargs)
        self.callbacks = zc.queue.PersistentQueue()
        self.annotations = BTrees.OOBTree.OOBTree()

    @property
    def result(self):
        return self._result

    @property
    def state(self):
        return self._state

    @property
    def unhandled_error(self):
        if (self.state in (zc.async.interfaces.COMPLETED,
                           zc.async.interfaces.CALLBACKS) and
            isinstance(self.result, twisted.python.failure.Failure)):
            ct = 0
            for c in self.callbacks:
                if (c.state not in (zc.async.interfaces.COMPLETED,
                                    zc.async.interfaces.CALLBACKS) or
                    c.unhandled_error):
                    return True
                ct += 1
            if not ct:
                return True
        return False

    @classmethod
    def bind(klass, *args, **kwargs):
        res = klass(*args, **kwargs)
        res.args.insert(0, res)
        return res

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
        if self.state != zc.async.interfaces.PENDING:
            raise zc.async.interfaces.BadStateError(
                'can only set callable when a partial is in PENDING state')
        if isinstance(value, types.MethodType):
            self._callable_root = value.im_self
            self._callable_name = value.__name__
        else:
            self._callable_root, self._callable_name = value, None

    def addCallbacks(self, success=None, failure=None):
        if success is not None or failure is not None:
            if success is not None:
                success = zc.async.interfaces.IPartial(success)
            if failure is not None:
                failure = zc.async.interfaces.IPartial(failure)
            res = Partial(success_or_failure, success, failure)
            if success is not None:
                success.__parent__ = res
            if failure is not None:
                failure.__parent__ = res
            self.addCallback(res)
            # we need to handle the case of callbacks on the internal success/
            # failure partials, to be safe.
            abort_handler = zc.async.interfaces.IPartial(
                completeStartedPartialArguments)
            abort_handler.args.append(res)
            res = res.addCallback(abort_handler)
        else:
            res = self
        return res

    def addCallback(self, callback):
        callback = zc.async.interfaces.IPartial(callback)
        self.callbacks.put(callback)
        callback.__parent__ = self
        if self.state == zc.async.interfaces.COMPLETED:
            callback(self.result) # this commits transactions!
        else:
            self._p_changed = True # to try and fire conflict errors if
            # our reading of self.state has changed beneath us
        return callback

    def __call__(self, *args, **kwargs):
        if self.state != zc.async.interfaces.PENDING:
            raise zc.async.interfaces.BadStateError(
                'can only call a partial in PENDING state')
        tm = transaction.interfaces.ITransactionManager(self)
        self._state = zc.async.interfaces.ACTIVE
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
                if zc.async.interfaces.IPartial.providedBy(res):
                    res.addCallback(self._callback)
                elif isinstance(res, twisted.internet.defer.Deferred):
                    res.addBoth(zc.twist.Partial(self._callback))
                else:
                    if isinstance(res, twisted.python.failure.Failure):
                        res = zc.twist.sanitize(res)
                    self._result = res
                    self._state = zc.async.interfaces.CALLBACKS
                tm.commit()
            except ZODB.POSException.TransactionError:
                tm.abort()
                ct += 1
                if ct >= 5:
                    res = self._result = zc.twist.sanitize(
                        twisted.python.failure.Failure())
                    self._state = zc.async.interfaces.CALLBACKS
                    tm.commit()
                    self.resumeCallbacks()
                else:
                    continue
            except zc.twist.EXPLOSIVE_ERRORS:
                tm.abort()
                raise
            except:
                tm.abort()
                res = self._result = zc.twist.sanitize(
                    twisted.python.failure.Failure())
                self._state = zc.async.interfaces.CALLBACKS
                tm.commit()
                self.resumeCallbacks()
            else:
                if self.state == zc.async.interfaces.CALLBACKS:
                    self.resumeCallbacks()
            return res

    def _callback(self, res):
        self._call_with_retry(lambda: res)

    def fail(self, e=None):
        if e is None:
            e = zc.async.interfaces.AbortedError()
        if self.state not in (zc.async.interfaces.PENDING,
                              zc.async.interfaces.ACTIVE):
            raise zc.async.interfaces.BadStateError(
                'can only call fail on a partial in PENDING or ACTIVE states')
        tm = transaction.interfaces.ITransactionManager(self)
        self._result = zc.twist.sanitize(
            twisted.python.failure.Failure(e))
        self._state = zc.async.interfaces.CALLBACKS
        tm.commit()
        self.resumeCallbacks()

    def resumeCallbacks(self):
        if self.state != zc.async.interfaces.CALLBACKS:
            raise zc.async.interfaces.BadStateError(
                'can only resumeCallbacks on a partial in CALLBACKS state')
        callbacks = list(self.callbacks)
        tm = transaction.interfaces.ITransactionManager(self)
        length = 0
        while 1:
            for p in callbacks:
                if p.state == zc.async.interfaces.PENDING:
                    p(self.result)
                elif p.state == zc.async.interfaces.ACTIVE:
                    p.fail()
                elif p.state == zc.async.interfaces.CALLBACKS:
                    p.resumeCallbacks()
                # TODO: this shouldn't raise anything we want to catch, right?
                # now, this should catch all the errors except EXPLOSIVE_ERRORS
                # cleaning up dead partials should look something like the above.
            tm.commit()
            # it's possible that someone added some callbacks run until
            # we're exhausted.
            length += len(callbacks)
            callbacks = list(self.callbacks)[length:]
            if not callbacks:
                try:
                    self._state = zc.async.interfaces.COMPLETED
                    tm.commit()
                except ZODB.POSException.TransactionError:
                    tm.abort()
                    callbacks = list(self.callbacks)[length:]
                else:
                    break # and return
