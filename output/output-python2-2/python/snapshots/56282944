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
import ZODB.POSException
import threading
import bisect
from time import sleep as time_sleep # this import style is intentional, so
# that test monkeypatching of time.sleep does not affect the usage in this
# module
import datetime
import logging
import sys

import pytz
import transaction
import zc.async.interfaces


_now = None

old_datetime = datetime.datetime

def set_now(dt):
    global _now
    _now = _datetime(*dt.__reduce__()[1])


class _datetime(old_datetime):
    @classmethod
    def now(klass, tzinfo=None):
        if tzinfo is None:
            return _now.replace(tzinfo=None)
        else:
            return _now.astimezone(tzinfo)
    @classmethod
    def utcnow(klass):
        return _now.replace(tzinfo=None)
    def astimezone(self, tzinfo):
        return _datetime(
            *super(_datetime,self).astimezone(tzinfo).__reduce__()[1])
    def replace(self, *args, **kwargs):
        return _datetime(
            *super(_datetime,self).replace(
                *args, **kwargs).__reduce__()[1])
    def __repr__(self):
        raw = super(_datetime, self).__repr__()
        return "datetime.datetime%s" % (
            raw[raw.index('('):],)
    def __add__(self, other):
        return _datetime(
            *super(_datetime,self).__add__(other).__reduce__()[1])
    def __reduce__(self):
        return (argh, super(_datetime, self).__reduce__()[1])
def argh(*args, **kwargs):
    return _datetime(*args, **kwargs)

_datetime.max = _datetime(*old_datetime.max.__reduce__()[1])

def setUpDatetime():
    datetime.datetime = _datetime
    set_now(datetime.datetime(2006, 8, 10, 15, 44, 22, 211, pytz.UTC))

def tearDownDatetime():
    datetime.datetime = old_datetime


class Reactor(object):

    def __init__(self):
        self.started = False
        self.calls = []
        self.triggers = []
        self._lock = threading.Lock()
        self._threads = []

    # necessary reactor methods

    def callLater(self, delay, callable, *args, **kw):
        if not self.started:
            raise ValueError('not started')
        res = (datetime.timedelta(seconds=delay) + _now, callable, args, kw)
        self._lock.acquire()
        try:
            bisect.insort(self.calls, res)
        finally:
            self._lock.release()
        # normally we're supposed to return something but not needed

    def callFromThread(self, callable, *args, **kw):
        if not self.started:
            raise ValueError('not started')
        self._lock.acquire()
        try:
            bisect.insort(
                self.calls,
                (_now, callable, args, kw))
        finally:
            self._lock.release()

    def addSystemEventTrigger(self, _when, _event, _callable, *args, **kwargs):
        assert _when == 'before' and _event == 'shutdown', (
            'unsupported trigger')
        self.triggers.append((_when, _event, _callable, args, kwargs))

    def callInThread(self, _callable, *args, **kw):
        # very naive should be fine...
        thread = threading.Thread(target=_callable, args=args, kwargs=kw)
        self._threads.append(thread)
        thread.start()

    def callWhenRunning(self, _callable, *args, **kw):
        self._lock.acquire()
        try:
            bisect.insort(self.calls, (_now, _callable, args, kw))
        finally:
            self._lock.release()

    # end reactor methods

    def start(self):
        setUpDatetime()
        self.started = True

    def stop(self):
        for when, event, callable, args, kwargs in self.triggers:
            callable(*args, **kwargs)
        self.started = False
        tearDownDatetime()

    # these are for tests

    def _get_next(self, end):
        self._lock.acquire()
        try:
            if self.calls and self.calls[0][0] <= end:
                return self.calls.pop(0)
        finally:
            self._lock.release()

    def time_flies(self, seconds):
        if not self.started:
            raise ValueError('not started')
        end = _now + datetime.timedelta(seconds=seconds)
        ct = 0
        next = self._get_next(end)
        while next is not None:
            now, callable, args, kw = next
            set_now(now)
            callable(*args, **kw) # normally this would get try...except
            ct += 1
            next = self._get_next(end)
        set_now(end)
        return ct

    def time_passes(self):
        if not self.started:
            raise ValueError('not started')
        next = self._get_next(_now)
        if next is not None:
            discard, callable, args, kw = next
            callable(*args, **kw)
            return True
        return False

    def wait_for(self, *jobs, **kwargs):
        poll_interval = kwargs.get('poll_interval', 5)
        self.time_flies(poll_interval) # starts thread
        # now we wait for the thread
        for i in range(kwargs.get('attempts', 10)):
            while self.time_passes():
                pass
            transaction.begin()
            for j in jobs:
                if j.status != zc.async.interfaces.COMPLETED:
                    break
            else:
                break
            time_sleep(0.1)
        else:
            print 'TIME OUT'

# helper functions convenient for tests

def get_poll(dispatcher, count=None, seconds=6):
    if count is None:
        count = len(dispatcher.polls)
    for i in range(seconds * 10):
        if len(dispatcher.polls) > count:
            return dispatcher.polls.first()
        time_sleep(0.1)
    else:
        assert False, 'no poll!'

def wait_for_start(job, seconds=6):
    for i in range(seconds * 10):
        t = transaction.begin()
        if job.status == zc.async.interfaces.ACTIVE:
            break
        time_sleep(0.1)
    else:
        assert False, 'job never started (%s)' % (job.status,)

def wait_for_deactivation(dispatcher, seconds=6):
    for i in range(seconds * 10):
        if dispatcher.activated == False:
            break
        time_sleep(0.1)
    else:
        assert False, 'dispatcher never deactivated'

def wait_for_death(da, seconds=6):
    for i in range(seconds * 10):
        _ = transaction.begin()
        if da.dead:
            break
        time_sleep(0.1)
    else:
        assert False, 'dispatcher agent never died'

def wait_for_result(job, seconds=6):
    for i in range(seconds * 10):
        t = transaction.begin()
        try:
            if job.status == zc.async.interfaces.COMPLETED:
                return job.result
        except ZODB.POSException.ReadConflictError:
            # storage does not have MVCC
            pass
        time_sleep(0.1)
    else:
        assert False, 'job never completed'

def wait_for_annotation(job, name):
    for i in range(60):
        t = transaction.begin()
        try:
            if name in job.annotations:
                return job.annotations[name]
        except ZODB.POSException.ReadConflictError:
            # storage does not have MVCC
            pass
        time_sleep(0.1)
    else:
        assert False, 'annotation never found'

def print_logs(log_file=sys.stdout, log_level=logging.CRITICAL):
    # really more of a debugging tool
    logger = logging.getLogger('zc.async')
    # stashing this on the dispatcher is a hack, but at least we're doing
    # it on code from the same package.
    handler = logging.StreamHandler(log_file)
    logger.setLevel(log_level)
    logger.addHandler(handler)
    return handler
