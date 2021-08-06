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
"""Thread-local values and features.

``local``, below, is reexported as zc.async.local
"""
import time
import threading

import zc.twist

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
    name = None

    def getJob(self):
        return self.job

    def getQueue(self):
        return self.job.queue

    def getDispatcher(self):
        return self.dispatcher

    def getReactor(self):
        return self.dispatcher.reactor

    def getAgentName(self):
        return self.name

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
