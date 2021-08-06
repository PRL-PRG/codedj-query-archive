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
import bisect
import datetime
import logging
import sys
import time
import types

import persistent.interfaces
import BTrees
import ZEO.Exceptions
import ZODB.POSException
import ZODB.utils
import rwproperty
import persistent
import zope.minmax
import zc.dict
import pytz
import zope.bforest.periodic

import zc.async.interfaces


EXPLOSIVE_ERRORS = (SystemExit, KeyboardInterrupt,
                    zc.async.interfaces.ReassignedError)

SYSTEM_ERRORS = (ZEO.Exceptions.ClientDisconnected,
                 ZODB.POSException.POSKeyError)

INITIAL_BACKOFF = 5
MAX_BACKOFF = 60
BACKOFF_INCREMENT = 5

def simpleWrapper(name):
    # notice use of "simple" in function name!  A sure sign of trouble!
    def wrapper(self, *args, **kwargs):
        return getattr(self._data, name)(*args, **kwargs)
    return wrapper

log = logging.getLogger('zc.async.events')
tracelog = logging.getLogger('zc.async.trace')

class Base(persistent.Persistent):

    _z_parent__ = parent = None

    # we use ``parent`` for our data structures.  As a convenience, we
    # support the ``__parent__`` attribute used by most security policies so
    # that ``__parent__`` uses ``parent`` unless __parent__ is explicitly set.
    @property
    def __parent__(self):
        if self._z_parent__ is not None:
            return self._z_parent__
        return self.parent
    @rwproperty.setproperty
    def __parent__(self, value):
        self._z_parent__ = None

# for legacy databases
Atom = zope.minmax.Maximum


class Dict(zc.dict.Dict, Base):

    copy = None # mask

    def __setitem__(self, key, value):
        previous = self.get(key)
        super(Dict, self).__setitem__(key, value)
        value.name = key
        value.parent = self
        if previous is not None:
            previous.parent = previous.name = None

    def pop(self, key, *args):
        try:
            res = super(Dict, self).pop(key)
        except KeyError:
            if args:
                return args[0]
            else:
                raise
        res.parent = None
        res.name = None
        return res

def dt_to_long(dt):
    # 4 low bits, 0-15, will be discarded and can be set, if desired
    # newer dates are smaller than older, so BTrees sort from newer to older
    if dt.tzinfo is not None:
        dt = dt.astimezone(pytz.UTC).replace(tzinfo=None)
    delta = datetime.datetime.max - dt
    return (delta.days << 41 | delta.seconds << 24 | delta.microseconds << 4)

def long_to_dt(l):
    microseconds = (l >> 4) & (2**20-1)
    seconds = (l >> 24) & (2**17-1)
    days = (l >> 41)
    return (datetime.datetime.max -
            datetime.timedelta(days, seconds, microseconds))


class AbstractSet(persistent.Persistent):

    __parent__ = None

    def __init__(self):
        self._data = BTrees.family64.IO.BTree()

    def clear(self):
        self._data.clear()

    def add(self, item):
        key = dt_to_long(datetime.datetime.utcnow()) + 15
        while key in self._data:
            key -= 1
        self._data[key] = item
        assert self.__parent__ is not None
        item.parent = self.__parent__
        item.key = key

    def __iter__(self):
        return self._data.itervalues()

    def __len__(self):
        return len(self._data)

    def __nonzero__(self):
        return bool(self._data)

    def first(self, start=None):
        if start is not None:
            if isinstance(start, (int, long)):
                args = (start,)
            else:
                args = (dt_to_long(start),)
        else:
            args = ()
        return self._data[self._data.minKey(*args)]

    def last(self, stop=None):
        if stop is not None:
            if isinstance(stop, (int, long)):
                args = (stop,)
            else:
                args = (dt_to_long(stop),)
        else:
            args = ()
        return self._data[self._data.maxKey(*args)]

    def iter(self, start=None, stop=None):
        if start is not None:
            start = dt_to_long(start)
        if stop is not None:
            stop = dt_to_long(stop)
        return self._data.itervalues(start, stop)


class Periodic(AbstractSet):
    # sorts on begin_after from newest to oldest

    def __init__(self, period, buckets):
        self._data = zope.bforest.periodic.LOBForest(period, count=buckets)

    @property
    def period(self):
        return self._data.period
    @rwproperty.setproperty
    def period(self, value):
        self._data.period = value


class RollingSet(AbstractSet):

    size = 100

    def add(self, item):
        super(RollingSet, self).add(item)
        diff = len(self._data) - self.size
        while diff > 0:
            self._data.pop(self._data.maxKey())
            diff -= 1


class RollingMapping(zc.dict.OrderedDict):

    size = 100

    def __setitem__(self, key, value):
        super(RollingMapping, self).__setitem__(key, value)
        diff = len(self) - self.size
        if diff > 0:
            for key in self._order[:diff]:
                self._data.pop(key)
            del self._order[:diff]
            self._len.change(-diff)

    def maxKey(self, key=None):
        if key is None:
            args = ()
        else:
            args = (key,)
        return self._data.maxKey(*args)

    def minKey(self, key=None):
        if key is None:
            args = ()
        else:
            args = (key,)
        return self._data.minKey(*args)


def never_fail(call, identifier, tm):
    # forever for TransactionErrors; forever, with backoff, for anything else
    trans_ct = 0
    backoff_ct = 0
    backoff = INITIAL_BACKOFF
    res = None
    while 1:
        try:
            res = call()
            tm.commit()
        except ZODB.POSException.TransactionError:
            tm.abort()
            trans_ct += 1
            if not trans_ct % 5:
                log.warning(
                    '%d consecutive transaction errors while %s',
                    trans_ct, identifier, exc_info=True)
                res = None
        except EXPLOSIVE_ERRORS:
            tm.abort()
            raise
        except Exception, e:
            if isinstance(e, SYSTEM_ERRORS):
                level = logging.ERROR
            else:
                level = logging.CRITICAL
            tm.abort()
            backoff_ct += 1
            if backoff_ct == 1:
                log.log(level,
                        'first error while %s; will continue in %d seconds',
                        identifier, backoff, exc_info=True)
            elif not backoff_ct % 10:
                log.log(level,
                        '%d consecutive errors while %s; '
                        'will continue in %d seconds',
                        backoff_ct, identifier, backoff, exc_info=True)
            res = None
            time.sleep(backoff)
            backoff = min(MAX_BACKOFF, backoff + BACKOFF_INCREMENT)
        else:
            return res

def wait_for_system_recovery(call, identifier, tm):
    # forever for TransactionErrors; forever, with backoff, for SYSTEM_ERRORS
    trans_ct = 0
    backoff_ct = 0
    backoff = INITIAL_BACKOFF
    res = None
    while 1:
        try:
            res = call()
            tm.commit()
        except ZODB.POSException.TransactionError:
            tm.abort()
            trans_ct += 1
            if not trans_ct % 5:
                log.warning(
                    '%d consecutive transaction errors while %s',
                    ct, identifier, exc_info=True)
                res = None
        except EXPLOSIVE_ERRORS:
            tm.abort()
            raise
        except SYSTEM_ERRORS:
            tm.abort()
            backoff_ct += 1
            if backoff_ct == 1:
                log.error('first error while %s; will continue in %d seconds',
                          identifier, backoff, exc_info=True)
            elif not backoff_ct % 5:

                log.error('%d consecutive errors while %s; '
                          'will continue in %d seconds',
                          backoff_ct, identifier, backoff, exc_info=True)
            res = None
            time.sleep(backoff)
            backoff = min(MAX_BACKOFF, backoff + BACKOFF_INCREMENT)
        except:
            log.error('Error while %s', identifier, exc_info=True)
            tm.abort()
            return zc.twist.Failure()
        else:
            return res

def try_five_times(call, identifier, tm, commit=True):
    ct = 0
    res = None
    while 1:
        try:
            res = call()
            if commit:
                tm.commit()
        except ZODB.POSException.TransactionError:
            tm.abort()
            ct += 1
            if ct >= 5:
                log.critical('Five consecutive transaction errors while %s',
                          identifier, exc_info=True)
                res = zc.twist.Failure()
            else:
                continue
        except EXPLOSIVE_ERRORS:
            tm.abort()
            raise
        except:
            tm.abort()
            log.critical('Error while %s', identifier, exc_info=True)
            res = zc.twist.Failure()
        return res

def custom_repr(obj):
    if persistent.interfaces.IPersistent.providedBy(obj):
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
    elif isinstance(obj, (types.FunctionType, types.BuiltinFunctionType)):
        return '%s.%s' % (obj.__module__, obj.__name__)
    else:
        return repr(obj)

def sortedmerge(sources, key=None):
    if key is None:
        key = lambda item: item
    sorted_sources = []
    for src in sources:
        iterator = iter(src)
        try:
            first = iterator.next()
        except StopIteration:
            pass
        else:
            sorted_sources.append((key(first), first, iterator))
    sorted_sources.sort()
    while sorted_sources:
        ignore, result, iterator = sorted_sources.pop(0)
        yield result
        try:
            next = iterator.next()
        except StopIteration:
            pass
        else:
            bisect.insort(sorted_sources, (key(next), next, iterator))
