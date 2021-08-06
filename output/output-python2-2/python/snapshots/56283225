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
import logging
import sys
import time

import ZEO.Exceptions
import ZODB.POSException
import rwproperty
import persistent
import zope.minmax
import zc.dict
import pytz
import zope.bforest.periodic


EXPLOSIVE_ERRORS = (SystemExit, KeyboardInterrupt)

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

class Periodic(persistent.Persistent):
    # sorts on begin_after from newest to oldest

    __parent__ = None

    def __init__(self, period, buckets):
        self._data = zope.bforest.periodic.LOBForest(period, count=buckets)

    def clear(self):
        self._data.clear()

    @property
    def period(self):
        return self._data.period
    @rwproperty.setproperty
    def period(self, value):
        self._data.period = value

    def add(self, item):
        key = zc.async.utils.dt_to_long(datetime.datetime.utcnow()) + 15
        while key in self._data:
            key -= 1
        self._data[key] = item
        assert self.__parent__ is not None
        item.parent = self.__parent__
        item.key = key

    def iter(self, start=None, stop=None):
        sources = []
        if start is not None:
            start = zc.async.utils.dt_to_long(start)
        if stop is not None:
            stop = zc.async.utils.dt_to_long(stop)
        for b in self._data.buckets:
            i = iter(b.items(start, stop))
            try:
                n = i.next()
            except StopIteration:
                pass
            else:
                sources.append([n, i])
        sources.sort()
        length = len(sources)
        while length > 1:
            src = sources.pop(0)
            yield src[0][1]
            try:
                src[0] = src[1].next()
            except StopIteration:
                length -= 1
            else:
                bisect.insort(sources, src)
        if sources:
            yield sources[0][0][1]
            for k, v in sources[0][1]:
                yield v

    def __iter__(self):
        return self._data.itervalues() # this takes more memory but the pattern
        # is typically faster than the custom iter above (for relatively
        # complete iterations of relatively small sets).  The custom iter
        # has the advantage of the start and stop code.

    def first(self, start=None):
        original = start
        if start is not None:
            start = zc.async.utils.dt_to_long(start)
            minKey = lambda bkt: bkt.minKey(start)
        else:
            minKey = lambda bkt: bkt.minKey()
        i = iter(self._data.buckets)
        bucket = i.next()
        try:
            key = minKey(bucket)
        except ValueError:
            key = None
        for b in i:
            try:
                k = minKey(b)
            except ValueError:
                continue
            if key is None or k < key:
                bucket, key = b, k
        if key is None:
            raise ValueError(original)
        return bucket[key]

    def last(self, stop=None):
        original = stop
        if stop is not None:
            stop = zc.async.utils.dt_to_long(stop)
            maxKey = lambda bkt: bkt.maxKey(stop)
        else:
            maxKey = lambda bkt: bkt.maxKey()
        i = iter(self._data.buckets)
        bucket = i.next()
        try:
            key = maxKey(bucket)
        except ValueError:
            key = None
        for b in i:
            try:
                k = maxKey(b)
            except ValueError:
                continue
            if key is None or k > key:
                bucket, key = b, k
        if key is None:
            raise ValueError(original)
        return bucket[key]

    def __nonzero__(self):
        for b in self._data.buckets:
            for ignore in b:
                return True
        return False

    def __len__(self):
        return len(self._data)

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
                # import pdb; pdb.set_trace()
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

def try_transaction_five_times(call, identifier, tm):
    ct = 0
    res = None
    while 1:
        try:
            res = call()
            tm.commit()
        except ZODB.POSException.TransactionError:
            tm.abort()
            ct += 1
            if ct >= 5:
                log.error('Five consecutive transaction errors while %s',
                          identifier, exc_info=True)
                res = zc.twist.Failure()
            else:
                continue
        except EXPLOSIVE_ERRORS:
            tm.abort()
            raise
        except:
            tm.abort()
            log.error('Error while %s', identifier, exc_info=True)
            res = zc.twist.Failure()
        return res
