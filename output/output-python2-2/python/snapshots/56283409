
import threading
import bisect
import datetime

import pytz


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

    def _get_next(self, end):
        self._lock.acquire()
        try:
            if self.calls and self.calls[0][0] <= end:
                return self.calls.pop(0)
        finally:
            self._lock.release()

    def start(self):
        setUpDatetime()
        self.started = True

    def stop(self):
        for when, event, callable, args, kwargs in self.triggers:
            callable(*args, **kwargs)
        self.started = False
        tearDownDatetime()

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
