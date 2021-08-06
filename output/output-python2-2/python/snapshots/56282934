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
import zope.interface
import zope.interface.common.mapping
import zope.interface.common.sequence
import zope.component.interfaces
import zc.queue.interfaces
from zc.async.i18n import _

# this is our only direct dependency on anything in zope.app, which is
# only used by our convenience subscribers.  Since we don't really need this,
# or zope.app, we make this import optional and provide some replacements if
# necessary.
try:
    from zope.app.appsetup.interfaces import (IDatabaseOpenedEvent,
                                              DatabaseOpened)
except ImportError:
    class IDatabaseOpenedEvent(zope.interface.Interface):
        """The main database has been opened."""

        database = zope.interface.Attribute("The main database.")

    class DatabaseOpened(object):
        zope.interface.implements(IDatabaseOpenedEvent)

        def __init__(self, database):
            self.database = database

# TODO: these interfaces are not particularly complete.  The other
# documentation is more accurate at the moment.

KEY = 'zc.async'

NEW = _('new-status', 'New')
PENDING = _('pending-status', 'Pending')
ASSIGNED = _('assigned-status', 'Assigned')
ACTIVE = _('active-status', 'Active')
CALLBACKS = _('callback-status', 'Performing Callbacks')
COMPLETED = _('completed-status', 'Completed')

class IReactor(zope.interface.Interface):
    """This describes what the dispatcher expects of the reactor.

    The reactor does not need to actually provide this interface."""

    def callFromThread(callable, *args, **kw):
        """have callable run in reactor's thread, by reactor, ASAP.

        Intended to be called from a thread other than the reactor's main
        loop.
        """

    def callInThread(callable, *args, **kw):
        """have callable run in a separate thread, ASAP.

        Must be called in same thread as reactor's main loop.
        """

    def callLater(seconds, callable, *args, **kw):
        """have callable run in reactor at least <seconds> from now

        Must be called in same thread as reactor's main loop.
        """

    def addSystemEventTrigger(phase, event, callable, *args, **kw):
        """Install a callable to be run in phase of event.

        must support phase 'before', and event 'shutdown'.
        """

    def callWhenRunning(self, _callable, *args, **kw):
        """run callable now if running, or when started.
        """

class IRetryPolicy(zope.interface.Interface):
    def jobError(failure, data_cache):
        """whether and how to retry after an error while performing job.

        return boolean as to whether to retry, or a datetime or timedelta to
        reschedule the job in the queue.  An empty timedelta means to rescedule
        for immediately, before any pending calls in the queue."""

    def commitError(failure, data_cache):
        """whether to retry after trying to commit a job's successful result.

        return boolean as to whether to retry, or a datetime or timedelta to
        reschedule the job in the queue.  An empty timedelta means to rescedule
        for immediately, before any pending calls in the queue."""

    def interrupted():
        """whether to retry after a dispatcher dies when job was in progress.

        return boolean as to whether to retry, or a datetime or timedelta to
        reschedule the job in the queue.  An empty timedelta means to rescedule
        for immediately, before any pending calls in the queue."""

    def updateData(data_cache):
        """right before committing a job, retry is given a chance to stash
        information it has saved in the data_cache."""

class AbstractObjectEvent(object):
    def __init__(self, object):
        self.object = object

class IDispatcherRegistered(zope.component.interfaces.IObjectEvent):
    """Dispatcher was registered"""

class DispatcherRegistered(AbstractObjectEvent):
    zope.interface.implements(IDispatcherRegistered)

class IDispatcherUnregistered(zope.component.interfaces.IObjectEvent):
    """Dispatcher was unregistered"""

class DispatcherUnregistered(AbstractObjectEvent):
    zope.interface.implements(IDispatcherUnregistered)

class IDispatcherActivated(zope.component.interfaces.IObjectEvent):
    """Dispatcher was activated"""

class DispatcherActivated(AbstractObjectEvent):
    zope.interface.implements(IDispatcherActivated)

class IDispatcherDeactivated(zope.component.interfaces.IObjectEvent):
    """Dispatcher was deactivated"""

class DispatcherDeactivated(AbstractObjectEvent):
    zope.interface.implements(IDispatcherDeactivated)

class IDispatcherReactivated(zope.component.interfaces.IObjectEvent):
    """Dispatcher was reactivated after mistaken deactivation"""

class DispatcherReactivated(AbstractObjectEvent):
    zope.interface.implements(IDispatcherReactivated)

class IObjectAdded(zope.component.interfaces.IObjectEvent):
    """Object was added to the database"""

    parent = zope.interface.Attribute(
        'container to which the object was added')

    name = zope.interface.Attribute(
        'name of the object within the container')

class ObjectAdded(AbstractObjectEvent):
    zope.interface.implements(IObjectAdded)

    def __init__(self, object, parent, name):
        super(ObjectAdded, self).__init__(object)
        self.parent = parent
        self.name = name

class AbortedError(Exception):
    """An explicit abort, as generated by the default behavior of
    IJob.handleInterrupt"""


class TimeoutError(Exception):
    """A time out caused by a ``begin_by`` value."""


class BadStatusError(Exception):
    """The job is not in the status it should be for the call being made.
    This is almost certainly a programmer error."""


class ReassignedError(Exception):
    """The job has been reassigned to another process.
    
    This should only happen when a polling timeout has made a not-dead process
    appear to be dead to a sibling."""


class IAbstractJob(zope.interface.Interface):

    parent = zope.interface.Attribute(
        """The current canonical location of the job""")

    status = zope.interface.Attribute(
        """One of constants defined in zc.async.interfaces:
        NEW, PENDING, ASSIGNED, ACTIVE, CALLBACKS, COMPLETED.

        NEW means not added to a queue and not yet called.
        PENDING means addded to a queue but not an agent, and not yet called.
        ASSIGNED means added to an agent and not yet called.
        ACTIVE means in the process of being called.
        CALLBACKS means in the process of calling callbacks.
        COMPLETED means called.""")

    result = zope.interface.Attribute(
        """The result of the call.  When state equals PENDING or ACTIVE, will
        be None.  When COMPLETED, will be a twisted.python.failure.Failure
        describing the call failure or the successful result.""")

    def addCallbacks(success=None, failure=None):
        """if success or failure is not None, adds a callback job to
        self.callbacks and returns the job.  Otherwise returns self.
        success and failure must be None or adaptable to IJob.
        addCallbacks may be called multiple times.  Each will be called
        with the result of this job.  If callback is already in COMPLETED
        state then the callback will be performed immediately."""

    def addCallback(callback):
        """callback will receive result (independent of whether it is a
        success or a failure).  callback must be adaptable to IJob.
        addCallback may be called multiple times.  Each will be called
        with the result of this job.  If callback is already in
        COMPLETED state then the callback will be performed immediately."""

    callbacks = zope.interface.Attribute(
        """A mutable persistent list of the callback jobs added by
        addCallbacks.""")


class ICallbackProxy(IAbstractJob):
    """A proxy for jobs."""

    job = zope.interface.Attribute(
        """None, before ``getJob``, then the job calculated by ``getJob``""")

    def getJob(result):
        """Get the job for the given result."""


class IJob(IAbstractJob):

    callable = zope.interface.Attribute(
        """The callable object that should be called with *IJob.args and
        **IJob.kwargs when the IJob is called.  Mutable.""")

    args = zope.interface.Attribute(
        """a peristent list of the args that should be applied to self.call.
        May include persistent objects (though note that, if passing a method
        is desired, it will typicall need to be wrapped in an IJob).""")

    kwargs = zope.interface.Attribute(
        """a persistent mapping of the kwargs that should be applied to
        self.call.  May include persistent objects (though note that, if
        passing a method is desired, it will typicall need to be wrapped
        in an IJob).""")

    annotations = zope.interface.Attribute(
        """An OOBTree that is available for metadata use.""")

    def __call__(*args, **kwargs):
        """call the callable.  Any given args are effectively appended to
        self.args for the call, and any kwargs effectively update self.kwargs
        for the call."""

    def handleInterrupt():
        """use IRetryPolicy to decide whether to abort."""

    def resumeCallbacks():
        """Make all callbacks remaining for this job.  Any callbacks
        that are in PENDING state should be called normally; any callbacks
        in ACTIVE state should be `fail`ed; any callbacks in CALLBACKS state
        should `resumeCallback`; and any callbacks in COMPLETED state should
        be untouched.  May only be called when job is in CALLBACKS state.
        State will be COMPLETED after this call."""

    assignerUUID = zope.interface.Attribute(
        """The UUID of the software instance that was in charge when the
        IJob was put in an IJobQueue.  Should be assigned by
        IJobQueue.put.""")

#     selectedUUIDs = zope.interface.Attribute(
#         """a set of selected worker UUIDs.  If it is empty, it is
#         interpreted as the set of all available workerUUIDs.  Only
#         workers with UUIDs in the set may perform it.
#
#         If a worker would have selected this job for a run, but the
#         difference of selected_workerUUIDs and excluded_workerUUIDs
#         stopped it, it is responsible for verifying that the effective
#         set of workerUUIDs intersects with the available workers; if the
#         intersection contains no possible workers, the worker should
#         call job.fail().""")

    begin_after = zope.interface.Attribute(
        """A datetime.datetime in UTC of the first time when the
        job may run.  Cannot be set after job gets a data_manager.
        """)

    begin_by = zope.interface.Attribute(
        """A datetime.timedelta of the duration after the begin_after
        value after which the job will fail, if it has not already
        begun.  Cannot be set after job has begun.""")


class IAgent(zope.interface.common.sequence.IFiniteSequence):
    """Responsible for picking jobs and keeping track of them.

    An agent is a persistent object in a queue that is associated with a
    dispatcher and is responsible for picking jobs and keeping track of
    them. Zero or more agents within a queue can be associated with a
    dispatcher.

    Each agent for a given dispatcher is identified uniquely with a
    name.  A fully (universally) unique identifier for the agent can be
    obtained by combining the key of the agent's queue in the main queue
    mapping at the ZODB root; the UUID of the agent's dispatcher; and
    the agent's name.
    """

    size = zope.interface.Attribute(
        """The maximum number of jobs this agent should have active at a time.
        """)

    name = zope.interface.Attribute(
        """The name for this agent.  Unique within its dispatcher's jobs for
        its queue.  Can be used to obtain agent with
        queue.dispatchers[*dispatcher UUID*][*name*].""")

    completed = zope.interface.Attribute(
        """an ICompleted of recent completed jobs.""")

    parent = zope.interface.Attribute(
        """a link to parent: an IDispatcherAgents container.""")

    def get():
        """get a new item, obtained from queue; or None if there are no
        items in the queue that this agent wants to take, or the agent is
        full.  If an item is returned, it has also been added to the agent.
        """

    def remove(item):
        """remove item, or raise ValueError if item is not in queue"""

    def __delitem__(index):
        """delete item at index"""

    def index(item):
        """return index, or raise ValueError if item is not in queue"""


class IQueue(zc.queue.interfaces.IQueue):

    parent = zope.interface.Attribute(
        """the IDataManager of which this is a part.""")

    def put(item, begin_after=None, begin_by=None):
        """Put an IJob adapted from item into the queue.  Returns IJob.

        Rememeber that IJobs are not guaranteed to be run in order
        added to a queue.  If you need sequencing, use
        IJob.addCallbacks.

        item must be an IJob, or be adaptable to that interface.
        begin_after must be None (to leave the job's current value) or a
        datetime.datetime.  begin_by must be None (to leave it alone) or a
        datetime.timedelta of the duration after the begin_after.

        If item.begin_after is None and begin_after is None, begin_after will
        effectively be now.  If item.begin_by is None and begin_by is None,
        begin_by will effectively be datetime.timedelta(hours=1).

        datetime.datetimes are suggested to be in UTC.  Timezone-naive
        datetimes will be interpreted as in UTC.  Timezone-aware datetimes
        will be converted to UTC, and errors because of this (such as
        pytz ambiguity errors) will be raised.

        When an IJob is put in the queue, the queue puts the
        begin_after time and begin_by duration on the job,
        and the UUID of the Zope instance that put the job in the
        queue on the `assignerUUID`.
        """

    def putBack(item):
        """Return a previously claimed job to the top of the queue."""

    def pull(index=0):
        """Remove and return a job, by default from the front of the queue.

        Raise IndexError if index does not exist.

        This is the blessed way to remove an unclaimed job from the queue so
        that dispatchers will not try to perform it.
        """

    def remove(item):
        """Removes item from queue or raises LookupError if not found."""

    def claim(filter=None, default=None):
        """returns first due job that is available for the given filter,
        removing it from the queue as appropriate; or None, if none are
        available. Responsible for including jobs to fail expired
        jobs."""

class IDispatcherAgents(zope.interface.common.mapping.IMapping):
    """holds agents.  contained agents get a ``name`` and ``parent``
    associated with this mapping."""

class IDispatchers(zope.interface.common.mapping.IEnumerableMapping):

    def register(UUID):
        "register UUID"

    def unregister(UUID):
        "unregister UUID"

    def ping(UUID):
        """responsible for setting ping time if necessary for this
        dispatcher agent, and for decomissioning dead dispatchers for
        the next highest dispatcher (sorted by UUID) if its (last_ping.value +
        ping_interval + ping_death_interval) < now.  If this is the
        highest dispatcher UUID, cycles around to lowest."""

class IQuota(zope.interface.common.mapping.IEnumerableMapping):
    def clean():
        ''
    filled = zope.interface.Attribute(
        "")
    def add(item):
        "add a job"
    name = zope.interface.Attribute(
        "")
    parent = zope.interface.Attribute(
        "")

class FullError(Exception):
    """Container is full.
    """

class ISizedSequence(zope.interface.common.sequence.IFiniteSequence):
    size = zope.interface.Attribute(
        """an integer.  If the queue's len >= size, put will raise
        FullError""")

    def add(item):
        """same contract as IQueue.put, except if queue's len >= size, put will
        raise FullError, and all objects get __parent__ set to the queue;
        and it will only store jobs."""

    __parent__ = zope.interface.Attribute(
        """a link to parent: an IWorker""")

    def remove(item):
        """remove item, or raise ValueError if item is not in queue"""

    def __delitem__(index):
        """delete item at index"""

    def index(item):
        """return index, or raise ValueError if item is not in queue"""

class ICompletedCollection(zope.interface.Interface):
    def __iter__():
        """Iterate over jobs in collection, from most recent `begin_after`
        to oldest"""

    def iter(start=None, stop=None):
        """Iterate over jobs in collection, starting and stopping with
        given timezone-aware datetime values reasonably efficiently."""

    def __len__():
        """Return number of jobs in collection"""

    def add(job):
        """Add job to collection and set __parent__ to the collection."""

    __parent__ = zope.interface.Attribute(
        """an IAgent""")

    def first(start=None):
        """Return the first (most recent) job in the collection, starting
        with optional timezone-aware datetime."""

    def last(stop=None):
        """Return the last (oldest) job in the collection, stopping
        with optional timezone-aware datetime."""

    def __nonzero__():
        "whether collection contains any jobs"

class IUUID(zope.interface.Interface):
    """A marker interface for the API of Ka-Ping Yee's uuid.UUID class.
    See http://zesty.ca/python/uuid.html """
