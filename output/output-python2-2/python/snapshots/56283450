import uuid
import Queue
import thread
import threading
import datetime
import logging
import pytz
import twisted.internet.reactor
import ZODB.POSException
import transaction
import transaction.interfaces

import zc.twist
    
def remove(container, partial, result=None):
    container.remove(partial)
    container.__parent__.completed.add(partial)
    
def perform(p):
    p()
    p.addCallback(zc.async.partial.Partial(remove, p.__parent__, p))

class Engine(object):
    # this intentionally does not have an interface.  It would be nicer if this
    # could be a Twisted service, part of the main Zope service, but that does
    # not appear easy to arrange at the moment.  Therefore we have a subscriber
    # in subscribers.py that does custom set-up, using raw reactor code.
    # Eventually I'd like to move this to a service interface, and tie it to
    # the Zope service in the subscriber.

    _needed = 0
    alive = True

    def __init__(self, UUID, factory):
        self.workerUUID = UUID
        self.factory = factory
        self.thread_queue = Queue.Queue(0)
        self._threads = []
        self.UUID = uuid.uuid4() # this is supposed to distinguish this engine
        # instance from any others potentially wanting to work on the worker.

    def perform_thread(self):
        try:
            job = self.thread_queue.get()
            while job is not None:
                db, identifier = job
                conn = db.open()
                removal = None
                try:
                    transaction.begin()
                    p = conn.get(identifier)
                    p.thread = thread.get_ident()
                    transaction.commit()
                    removal = zc.twist.Partial(remove, p.__parent__, p)
                    try:
                        p() # this does the committing and retrying, largely
                    except ZODB.POSException.TransactionError:
                        transaction.abort()
                        while 1:
                            try:
                                p.fail()
                                transaction.commit()
                            except ZODB.POSException.TransactionError:
                                transaction.abort() # retry forever (!)
                            else:
                                break
                finally:
                    conn.close()
                    if removal is not None:
                        twisted.internet.reactor.callFromThread(removal)
                job = self.thread_queue.get()
        finally:
            # this may cause some bouncing, but we don't ever want to end
            # up with fewer than needed.
            twisted.internet.reactor.callFromThread(self.set_threads)
    
    def set_threads(self, needed=None):
        # this should only be called from the main thread (otherwise it needs
        # locks)
        if needed is None:
            needed = self._needed
        else:
            self._needed = needed
        res = []
        ct = 0
        for t in self._threads:
            if t.isAlive():
                res.append(t)
                ct += 1
        self._threads[:] = res
        if ct < needed:
            for i in range(max(needed - ct, 0)):
                t = threading.Thread(target=self.perform_thread)
                self._threads.append(t)
                t.start()
        elif ct > needed:
            # this may cause some bouncing, but hopefully nothing too bad.
            for i in range(ct - needed):
                self.thread_queue.put(None)
    
    def poll(self, datamanager):
        if not self.alive:
            return
        poll_seconds = 0.25
        call = zc.twist.Partial(self.poll, datamanager)
        try:
            tm = transaction.interfaces.ITransactionManager(datamanager)
            tm.begin()
            now = datetime.datetime.now(pytz.UTC)
            worker = datamanager.workers.get(self.workerUUID)
            if worker is not None:
                if (worker.engineUUID is not None and
                    worker.engineUUID != self.UUID):
                    # uh-oh.  Maybe another engine is in on the action?
                    time_of_death = (worker.last_ping + worker.ping_interval
                                     + worker.ping_death_interval)
                    if time_of_death < now:
                        # hm.  Looks like it's dead.
                        zc.async.datamanager.cleanDeadWorker(worker)
                        worker.engineUUID = self.UUID
                    else:
                        # this is some other engine's UUID,
                        # and it isn't dead (yet?).  Houston, we have a problem.
                        interval = time_of_death - now
                        logging.warning(
                            'Another engine instance, %s, has claimed worker '
                            '%s.  This engine instance, %s, is '
                            "deferring.  The other engine will be "
                            "regarded dead and scheduled for removal after "
                            '%d days, %d seconds, and %d microseconds',
                            worker.engineUUID, worker.UUID, self.UUID,
                            interval.days, interval.seconds,
                            interval.microseconds)
                        return # which will call the finally clause
                else:
                    worker.engineUUID = self.UUID
                try:
                    tm.commit()
                except ZODB.POSException.TransactionError:
                    tm.abort()
                    # uh-oh.  Somebody else may be adding a worker for the
                    # same UUID.  we'll just return for now, and figure that
                    # the next go-round will report the problem.
                    return # will call finally clause
            else:
                worker = self.factory(self.workerUUID)
                datamanager.workers.add(worker)
                worker.engineUUID = self.UUID
                try:
                    tm.commit()
                except ZODB.POSException.TransactionError:
                    tm.abort()
                    # uh-oh.  Somebody else may be adding a worker for the
                    # same UUID.  we'll just return for now, and figure that
                    # the next go-round will report the problem.
                    return # will call finally clause
            poll_seconds = worker.poll_seconds
            datamanager.checkSibling(worker.UUID)
            try:
                tm.commit()
            except ZODB.POSException.TransactionError:
                tm.abort()
                # we'll retry next poll.
            if (worker.completed.last_rotation +
                worker.completed.rotation_interval) <= now:
                worker.completed.rotate()
                try:
                    tm.commit()
                except ZODB.POSException.TransactionError:
                    tm.abort()
                    # we'll retry next poll.
            if worker.last_ping + worker.ping_interval <= now:
                worker.last_ping = now
                try:
                    tm.commit()
                except ZODB.POSException.TransactionError:
                    # uh-oh: are there two engines working with the same worker?
                    tm.abort() # and retry next time
                    logging.error(
                        "Transaction error for worker %s.  This should not "
                        "happen.", self.workerUUID)
                    return
            def thread_size():
                if len(datamanager.workers) == 1:
                    return 1
                else:
                    return worker.thread_size
            self.set_threads(thread_size())
            while len(worker.thread) < thread_size():
                p = datamanager.thread.pullNext(uuid)
                if p is not None:
                    worker.thread.add(p)
                    try:
                        tm.commit()
                    except ZODB.POSException.TransactionError:
                        tm.abort()
                    else:
                        self.thread_queue.put((p._p_jar.db(), p._p_oid))
                else:
                    break
            self.set_threads(thread_size())
            while len(worker.reactor) < worker.reactor.size:
                p = datamanager.reactor.pullNext(uuid)
                if p is not None:
                    worker.reactor.add(p)
                    try:
                        tm.commit()
                    except ZODB.POSException.TransactionError:
                        tm.abort()
                    else:
                        twisted.internet.reactor.callLater(
                            0, zc.twist.Partial(perform, p))
                else:
                    break
            now = datetime.datetime.now(pytz.UTC)
            if worker.last_ping + worker.ping_interval <= now:
                worker.last_ping = now
                try:
                    tm.commit()
                except ZODB.POSException.TransactionError:
                    # uh-oh: are there two engines working with the same worker?
                    tm.abort() # and retry next time
                    logging.error(
                        "Transaction error for worker %s.  This should not "
                        "happen.", self.workerUUID)
                    return
        finally:
            tm.abort()
            if self.alive:
                twisted.internet.reactor.callLater(poll_seconds, call)

    def tearDown(self, datamanager):
        self.alive = False
        self.set_threads(0)
        try:
            tm = transaction.interfaces.ITransactionManager(datamanager)
            tm.begin()
            worker = datamanager.workers.get(self.workerUUID)
            if worker is not None:
                worker.engineUUID = None
                datamanager.thread.put(
                    zc.async.partial.Partial(
                        zc.async.datamanager.cleanDeadWorker, worker))
                tm.commit()
        finally:
            tm.abort()
        
