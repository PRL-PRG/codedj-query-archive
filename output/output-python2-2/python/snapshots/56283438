import datetime
import bisect
import pytz
import persistent
import ZODB.interfaces
import BTrees.OOBTree
import BTrees.Length
import zope.interface
import zope.component
import zope.bforest
import zc.queue

import zc.async.interfaces


def simpleWrapper(name):
    def wrapper(self, *args, **kwargs):
        return getattr(self._data, name)(*args, **kwargs)
    return wrapper

class Workers(persistent.Persistent):
    zope.interface.implements(zc.async.interfaces.IWorkers)

    def __init__(self):
        self._data = BTrees.OOBTree.OOBTree()

    for nm in ('__getitem__', 'get', '__len__', 'keys', 'values', 'items',
               '__contains__', 'maxKey', 'minKey'):
        locals()[nm] = simpleWrapper(nm)

    def __iter__(self):
        return iter(self._data)

    def add(self, value):
        value = zc.async.interfaces.IWorker(value)
        if value.UUID is None:
            raise ValueError("worker must have assigned UUID")
        self._data[value.UUID] = value
        value.__parent__ = self
        return value

    def remove(self, UUID):
        ob = self._data.pop(UUID)
        ob.__parent__ = None

def cleanDeadWorker(worker):
    dm = worker.__parent__.__parent__
    assert zc.async.interfaces.IDataManager.providedBy(dm)
    for queue, destination in (
        (worker.thread, dm.thread), (worker.reactor, dm.reactor)):
        while queue:
            p = queue[0]
            del queue[0]
            if p.state == zc.async.interfaces.PENDING:
                destination.put(p.__call__) # will wrap it
            elif p.state == zc.async.interfaces.ACTIVE:
                destination.put(p.fail)
            elif p.state == zc.async.interfaces.CALLBACKS:
                destination.put(p.resumeCallbacks)
    

class PartialQueue(persistent.Persistent):
    zope.interface.implements(zc.async.interfaces.IPartialQueue)

    def __init__(self, thread):
        self.thread = thread
        self._queue = zc.queue.CompositePersistentQueue()
        self._held = BTrees.OOBTree.OOBTree()
        self._length = BTrees.Length.Length(0)

    def put(self, item, begin_after=None, begin_by=None):
        item = zc.async.interfaces.IDataManagerPartial(item)
        if item.assignerUUID is not None:
            raise ValueError(
                'cannot add already-assigned partial')
        now = datetime.datetime.now(pytz.UTC)
        if begin_after is not None:
            item.begin_after = begin_after
        elif item.begin_after is None:
            item.begin_after = now
        if begin_by is not None:
            item.begin_by = begin_by
        elif item.begin_by is None:
            item.begin_by = datetime.timedelta(hours=1)
        item.assignerUUID = zope.component.getUtility(
            zc.async.interfaces.IUUID, 'instance')
        if item._p_jar is None:
            # we need to do this if the partial will be stored in another
            # database as well during this transaction.  Also, _held storage
            # disambiguates against the database_name and the _p_oid.
            conn = ZODB.interfaces.IConnection(self)
            conn.add(item)
        if now == item.begin_after:
            self._queue.put(item)
        else:
            self._held[
                (item.begin_after,
                 item._p_jar.db().database_name,
                 item._p_oid)] = item
        item.__parent__ = self
        self._length.change(1)
        return item

    def _iter(self):
        queue = self._queue
        tree = self._held
        q = enumerate(queue)
        t = iter(tree.items())
        q_pop = queue.pull
        t_pop = tree.pop
        def get_next(i):
            try:
                next = i.next()
            except StopIteration:
                active = False
                next = (None, None)
            else:
                active = True
            return active, next
        q_active, (q_index, q_next) = get_next(q)
        t_active, (t_index, t_next) = get_next(t)
        while q_active and t_active:
            if t_next.begin_after <= q_next.begin_after:
                yield t_pop, t_index, t_next
                t_active, (t_index, t_next) = get_next(t)
            else:
                yield q_pop, q_index, q_next
                q_active, (q_index, q_next) = get_next(q)
        if t_active:
            yield t_pop, t_index, t_next
            for (t_index, t_next) in t:
                yield t_pop, t_index, t_next
        elif q_active:
            yield q_pop, q_index, q_next
            for (q_index, q_next) in q:
                yield q_pop, q_index, q_next

    def pull(self, index=0):
        if index >= self._length():
            raise IndexError(index)
        for i, (pop, ix, next) in enumerate(self._iter()):
            if i == index:
                tmp = pop(ix)
                assert tmp is next
                self._length.change(-1)
                return next
        assert False, 'programmer error: the length appears to be incorrect.'

    def __len__(self):
        return self._length()

    def __iter__(self):
        return (next for pop, ix, next in self._iter())

    def __nonzero__(self):
        return bool(self._length())

    def __getitem__(self, index):
        if index >= len(self):
            raise IndexError(index)
        return zc.queue.getitem(self, index)

    def pullNext(self, uuid):
        now = datetime.datetime.now(pytz.UTC)
        for ix, p in enumerate(self.iterDue()):
            if uuid not in p.excludedUUIDs and (
                not p.selectedUUIDs or
                uuid in p.selectedUUIDs):
                return self.pull(ix)
            elif (p.begin_after + p.begin_by) < now:
                res = zc.async.interfaces.IDataManagerPartial(
                        self.pull(ix).fail)
                res.__parent__ = self
                res.begin_after = now
                res.begin_by = datetime.timedelta(hours=1)
                res.assignerUUID = zope.component.getUtility(
                    zc.async.interfaces.IUUID, 'instance')
                return res

    def iterDue(self):
        now = datetime.datetime.now(pytz.UTC)
        for partial in self:
            if partial.begin_after > now:
                break
            yield partial


class DataManager(persistent.Persistent):
    zope.interface.implements(zc.async.interfaces.IDataManager)

    def __init__(self):
        self.thread = PartialQueue(True)
        self.thread.__parent__ = self
        self.reactor = PartialQueue(False)
        self.reactor.__parent__ = self
        self.workers = Workers()
        self.workers.__parent__ = self

    def _getNextActiveSibling(self, uuid):
        for worker in self.workers.values(min=uuid, excludemin=True):
            if worker.engineUUID is not None:
                return worker
        for worker in self.workers.values(max=uuid, excludemax=True):
            if worker.engineUUID is not None:
                return worker

    def checkSibling(self, uuid):
        now = datetime.datetime.now(pytz.UTC)
        next = self._getNextActiveSibling(uuid)
        if next is not None and ((
            next.last_ping + next.ping_interval + next.ping_death_interval)
            < now):
            # `next` is a dead worker.
            next.engineUUID = None
            self.thread.put(zc.async.partial.Partial(cleanDeadWorker, next))


class SizedSequence(persistent.Persistent):
    zope.interface.implements(zc.async.interfaces.ISizedSequence)

    def __init__(self, size):
        self.size = size
        self._data = zc.queue.PersistentQueue()
        self._data.__parent__ = self

    for nm in ('__len__', '__iter__', '__getitem__', '__nonzero__',
               '_p_resolveConflict'):
        locals()[nm] = simpleWrapper(nm)

    def add(self, item):
        if len(self._data) >= self.size:
            raise zc.async.interfaces.FullError(self)
        item.__parent__ = self
        item.workerUUID = self.__parent__.UUID
        self._data.put(item)
        return item

    def index(self, item):
        for ix, i in enumerate(self):
            if i is item:
                return ix
        raise ValueError("%r not in queue" % (item,))

    def remove(self, item):
        del self[self.index(item)]

    def __delitem__(self, ix):
        self._data.pull(ix)


START = datetime.datetime(2006, 1, 1, tzinfo=pytz.UTC)

def key(item):
    dt = item.begin_after
    diff = dt - START
    return (-diff.days, -diff.seconds, -diff.microseconds,
            item._p_jar.db().database_name, item._p_oid)

def code(dt):
    diff = dt - START
    return (-diff.days, -diff.seconds, -diff.microseconds)


class Completed(persistent.Persistent):
    zope.interface.implements(zc.async.interfaces.ICompletedCollection)
    # sorts on begin_after from newest to oldest

    __parent__ = None

    def __init__(self,
                 rotation_interval=datetime.timedelta(hours=2),
                 buckets=6):
        self._data = zope.bforest.OOBForest(count=buckets)
        self.rotation_interval = rotation_interval
        self.last_rotation = datetime.datetime.now(pytz.UTC)

    def add(self, item):
        self._data[key(item)] = item
        item.__parent__ = self

    def iter(self, start=None, stop=None):
        sources = []
        if start is not None:
            start = code(start)
        if stop is not None:
            stop = code(stop)
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
                bisect.insort(sources, src) # mildly interesting micro-
                # optimisation note: this approach shaves off about 1/5 of
                # an alternative approach that finds the lowest every time
                # but does not insort.
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
            start = code(start)
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
            stop = code(stop)
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
            try:
                iter(b).next()
            except StopIteration:
                pass
            else:
                return True
        return False

    def __len__(self):
        return len(self._data)

    def rotate(self):
        self._data.rotateBucket()
        self.last_rotation = datetime.datetime.now(pytz.UTC)


class Worker(persistent.Persistent):
    zope.interface.implements(zc.async.interfaces.IWorker)

    def __init__(self, UUID, reactor_size=4, thread_size=1, poll_seconds=5,
                 ping_interval=datetime.timedelta(minutes=1),
                 ping_death_interval=datetime.timedelta(seconds=30)):
        self.reactor = SizedSequence(reactor_size)
        self.reactor.__parent__ = self
        self.thread = SizedSequence(thread_size)
        self.thread.__parent__ = self
        self.engineUUID = None
        self.UUID = UUID
        self.poll_seconds = poll_seconds
        self.ping_interval = ping_interval
        self.ping_death_interval = ping_death_interval
        self.last_ping = datetime.datetime.now(pytz.UTC)
        self.completed = Completed()
        self.completed.__parent__ = self
