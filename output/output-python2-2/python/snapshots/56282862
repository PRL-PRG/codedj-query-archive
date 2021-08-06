import datetime
import fnmatch
import re
from uuid import UUID # we use this non-standard import spelling because
# ``uuid`` is frequently an argument

import pytz
import twisted.python.failure
import ZODB.interfaces
import ZODB.utils
import zope.component

import zc.async.dispatcher
import zc.async.interfaces
import zc.async.monitor

_available_states = frozenset(
    ('pending', 'assigned', 'active', 'callbacks', 'completed', 'succeeded',
     'failed'))

def _get_date_filter(name, value):
    since = before = None
    for o in value.split(','):
        if o.startswith('since'):
            if since is not None:
                raise ValueError('only provide "since" once (%s)' % (name,))
            since = zc.async.monitor._dt(o[5:]).replace(tzinfo=pytz.UTC)
        elif o.startswith('before'):
            if before is not None:
                raise ValueError('only provide "before" once (%s)' % (name,))
            before = zc.async.monitor._dt(o[5:]).replace(tzinfo=pytz.UTC)
    return lambda j: ((since is None or getattr(j, name) > since) and
                      (before is None or getattr(j, name) < before))

def _jobs(context, states,
         callable=None, queue=None, agent=None, requested_start=None,
         start=None, end=None, callbacks_completed=None,
         uuid=None):
    conn = ZODB.interfaces.IConnection(context)
    states = set(states)
    unknown = states - _available_states
    if unknown:
        raise ValueError('Available states are %s (unknown: %s)' %
                         (', '.join(sorted(_available_states)),
                          ', '.join(sorted(unknown))))
    completed = set(['completed', 'succeeded', 'failed']) & states
    if len(completed) > 1:
        raise ValueError(
            'can only include zero or one of '
            '"completed", "succeeded," or "failed" states.')
    elif completed:
        completed = iter(completed).next()
    if not states:
        raise ValueError('Specify at least one of the available states: %s' %
                         (', '.join(sorted(_available_states)),))
    pending = 'pending' in states
    assigned = 'assigned' in states
    active = 'active' in states
    callbacks = 'callbacks' in states
    agent_states = []
    if assigned:
        agent_states.append(zc.async.interfaces.ASSIGNED)
    if active:
        agent_states.append(zc.async.interfaces.ACTIVE)
    if callbacks:
        agent_states.append(zc.async.interfaces.CALLBACKS)
    if uuid is not None:
        if uuid.upper() == 'THIS':
            uuid = zope.component.getUtility(zc.async.interfaces.IUUID)
        else:
            uuid = UUID(uuid)
    filters = []
    if callable is not None:
        regex = fnmatch.translate(callable)
        if '.' not in callable:
            regex = r'(.*\.)?%s$' % (regex,)
        callable = re.compile(regex).match
        filters.append(
            lambda j: callable(zc.async.utils.custom_repr(j.callable)))
    if requested_start:
        filters.append(_get_date_filter('begin_after', requested_start))
    if start:
        pending = False
        filters.append(_get_date_filter('active_start', start))
    if end:
        pending = assigned = active = False
        filters.append(_get_date_filter('active_end', end))
    if callbacks_completed:
        pending = assigned = active = callbacks = False
        filters.append(
            _get_date_filter('initial_callbacks_end', callbacks_completed))
    if queue is not None:
        queue = re.compile(fnmatch.translate(queue)).match
    if agent is not None:
        agent = re.compile(fnmatch.translate(agent)).match
    queues = conn.root()[zc.async.interfaces.KEY]
    for q_name, q in queues.items():
        if queue and not queue(q_name):
            continue
        agent_filters = []
        ignore_agent_filters = agent is None and uuid is None
        if (assigned or active or callbacks or completed or
            pending and not ignore_agent_filters):
            if uuid is None:
                das = q.dispatchers.values()
            else:
                das = (q.dispatchers[uuid],)
            for da in das:
                for a_name, a in da.items():
                    if agent:
                        if not agent(a_name):
                            continue
                    if agent or uuid is not None:
                        if pending and not ignore_agent_filters:
                            if zc.async.interfaces.IFilterAgent.providedBy(a):
                                agent_filters.append(a.filter)
                                ignore_agent_filters = (
                                    ignore_agent_filters or a.filter is None)
                            else:
                                raise ValueError(
                                    'can only find pending jobs for agent if '
                                    'agent provides '
                                    'zc.async.interfaces.IFilterAgent '
                                    '(%s : %s : %s)' %
                                    (q_name, da.UUID, a_name))
                    if agent_states:
                        for j in a:
                            if j.status not in agent_states:
                                continue
                            for f in filters:
                                if not f(j):
                                    break
                            else:
                                yield j
                    if completed:
                        for j in a.completed:
                            if completed!='completed':
                                is_failure = isinstance(
                                    j.result, twisted.python.failure.Failure)
                                if (completed=='succeeded' and is_failure or
                                    completed=='failed' and not is_failure):
                                    continue
                            for f in filters:
                                if not f(j):
                                    break
                            else:
                                yield j
        if pending:
            if not agent or agent_filters:
                for j in q:
                    if not ignore_agent_filters:
                        for f in agent_filters:
                            if f(j):
                                break # this is a positive match
                        else:
                            continue
                    for f in filters:
                        if not f(j):
                            break # this is a negative match
                    else:
                        yield j

def jobs(context, *states, **kwargs):
    """Return jobs in one or more states.

    Jobs are identified by integer OID and database name.  These identifiers
    can be used with the "asyncdb job" command to get details about the jobs.
    The integer OIDs can be used in a database connection to get the job with
    ``connection.get(ZODB.utils.p64(INTEGER_OID))``.

    The asyncdb commands "jobs," "count," "jobstats," and "firstjob" all share
    the same arguments, which are described below; after which usage examples
    for this command are listed.

    Arguments
    =========
    
    States
    ------

    You must provide at least one of the following states.

    - "pending": the job is in a queue, waiting to be started.
    
    - "assigned": a dispatcher has claimed the job and assigned it to one of
      its worker threads.  Work has not yet begun.  Jobs are in this state very
      briefly.
    
    - "active": A worker thread is performing this job.
    
    - "callbacks": the job's work is ended, and the thread is performing the
      callbacks, if any.
    
    - "completed": the job and its callbacks are completed.  Completed jobs
      stay in the database for only a certain amount of time--between seven and
      eight days in the default agent implementation.

    - "succeeded": the job completed successfully (that is, without raising an
      unhandled exception, and without returning an explicit
      twisted.python.failure.Failure).  This is a subset of "completed,"
      described above.

     - "failed": the job completed by raising an unhandled exception or by
       explicitly returning a twisted.python.failure.Failure.  This is a subset
       of "completed," described above.

    You may use no more than one of the states "completed," "succeeded," and
    "failed".

    Optional Arguments
    ------------------

    You can further filter your results with a number of optional arguments.

    "Shell-style glob wildcards," as referred to in this list, are "?", "*",
    "[SEQUENCE]", and "[!SEQUENCE]", as described in
    http://docs.python.org/lib/module-fnmatch.html .
    
    A "duration-based filter" described in this list accepts an argument that
    is of the form "sinceINTERVAL", "beforeINTERVAL", or
    "sinceINTERVAL,beforeINTERVAL" (no space!).  The "INTERVAL"s are of the
    form ``[nD][nH][nM][nS]``, where "n" should be replaced with a positive
    integer, and "D," "H," "M," and "S" are literals standing for "days,"
    "hours," "minutes," and "seconds." For instance, you might use ``5M`` for
    five minutes, ``20S`` for twenty seconds, or ``1H30M`` for an hour and a
    half.  Thus "before30M" would mean "thirty minutes ago or older." 
    "since45S" would mean "45 seconds ago or newer."  "since1H,before30M" would
    mean "between thirty minutes and an hour ago."  Note that reversing the two
    components in the last example--"before30M,since1H"--is equivalent.
    

    - "callable": filters by callable name.  Supports shell-style glob
      wildcards.  If you do not include a "." in the string, it matches only on
      the callable name.  If you include a ".", it matches on the
      fully-qualified name (that is, including the module).
    
    - "queue": filters by queue name.  Supports shell-style glob wildcards.
    
    - "agent": filters by agent name.  Supports shell-style glob wildcards.

      When applied to jobs in the "pending" state, this restricts the jobs to
      the ones that the agent could perform, according to its filter.
    
    - "uuid": filters by UUID string, or the special marker "THIS", indicating
      the UUID of the current process' dispatcher.  Supports shell-style glob
      wildcards.

      When applied to jobs in the "pending" state, this restricts the jobs to
      the ones that the agents for that dispatcher could perform, according to
      their filters.
    
    - "requested_start": a duration-based filter for when the job was requested
      to start.
      
      Note that, if a job is not given an explicit start time, the time when it
      was added to a queue is used.  This is based on a job's ``begin_after``
      attribute.
    
    - "start": a duration-based filter for when the job started work.
    
      Note that, if a job is restarted because of problems such as an
      interruption or a conflict error, this is the  most recent time that the
      job started work.  This is based on a job's ``active_start`` attribute.
      
      To see the first time a job started work ever, the default retry policies
      store a 'first_active' value in their ``data`` attribute
      (``job.getRetryPolicy().data.get('first_active')``).  Other information
      about retries can also be found in this data dictionary.
    
    - "end": a duration-based filter for when the job ended work (but not
      callbacks).
    
      This is based on a job's ``active_end`` attribute.
    
    - "callbacks_completed":  a duration-based filter for when the job
      finished the callbacks it had just after it performed the job.
      
      If subsequent callbacks are added, they are performed immediately, and
      will not affect the value that this filter uses.
    
      This is based on a job's ``initial_callbacks_end`` attribute.
    
    Usage Examples
    ==============
    
    Here are some examples of the command.

        asyncdb job pending
        (lists the job identifiers for pending jobs)
        
        asyndb job active agent:instance5
        (lists the job identifiers of all jobs that any agent named instance5
        is working on)
        
        asyndb job pending agent:instance5
        (lists the job identifiers of all pending jobs that agents named
        "instance5" could perform)

        asyncdb job completed end:since1H callable:import_*
        (lists the job identifiers of completed jobs that ended within the
        last hour that called a function or method that began with the string
        "import_")

    Here are some examples of how the duration-based filters work.
    
    * If you used "start:since5s" then that could be read as "jobs that
      started five seconds ago or sooner."  
    
    * "requested_start:before1M" could be read as "jobs that were supposed to
      begin one minute ago or longer". 
    
    * "end:since1M,before30S" could be read as "jobs that ended their
      primary work (that is, not including callbacks) between thirty seconds
      and one minute ago."
    
    * "callbacks_completed:before30S,since1M" could be read as "jobs that
      completed the callbacks they had when first run between thirty seconds
      and one minute ago."  (This also shows that the order of "before" and
      "since" do not matter.)
    """
    return _jobs(context, states, **kwargs)

def count(context, *states, **kwargs):
    """Count jobs in one or more states.

    The asyncdb commands "jobs," "count," "jobstats," and "firstjob" all share
    the same arguments, which are described below; after which usage examples
    for this command are listed.

    Arguments
    =========
    
    States
    ------

    You must provide at least one of the following states.

    - "pending": the job is in a queue, waiting to be started.
    
    - "assigned": a dispatcher has claimed the job and assigned it to one of
      its worker threads.  Work has not yet begun.  Jobs are in this state very
      briefly.
    
    - "active": A worker thread is performing this job.
    
    - "callbacks": the job's work is ended, and the thread is performing the
      callbacks, if any.
    
    - "completed": the job and its callbacks are completed.  Completed jobs
      stay in the database for only a certain amount of time--between seven and
      eight days in the default agent implementation.

    - "succeeded": the job completed successfully (that is, without raising an
      unhandled exception, and without returning an explicit
      twisted.python.failure.Failure).  This is a subset of "completed,"
      described above.

     - "failed": the job completed by raising an unhandled exception or by
       explicitly returning a twisted.python.failure.Failure.  This is a subset
       of "completed," described above.

    You may use no more than one of the states "completed," "succeeded," and
    "failed".

    Optional Arguments
    ------------------

    You can further filter your results with a number of optional arguments.

    "Shell-style glob wildcards," as referred to in this list, are "?", "*",
    "[SEQUENCE]", and "[!SEQUENCE]", as described in
    http://docs.python.org/lib/module-fnmatch.html .
    
    A "duration-based filter" described in this list accepts an argument that
    is of the form "sinceINTERVAL", "beforeINTERVAL", or
    "sinceINTERVAL,beforeINTERVAL" (no space!).  The "INTERVAL"s are of the
    form ``[nD][nH][nM][nS]``, where "n" should be replaced with a positive
    integer, and "D," "H," "M," and "S" are literals standing for "days,"
    "hours," "minutes," and "seconds." For instance, you might use ``5M`` for
    five minutes, ``20S`` for twenty seconds, or ``1H30M`` for an hour and a
    half.  Thus "before30M" would mean "thirty minutes ago or older." 
    "since45S" would mean "45 seconds ago or newer."  "since1H,before30M" would
    mean "between thirty minutes and an hour ago."  Note that reversing the two
    components in the last example--"before30M,since1H"--is equivalent.
    

    - "callable": filters by callable name.  Supports shell-style glob
      wildcards.  If you do not include a "." in the string, it matches only on
      the callable name.  If you include a ".", it matches on the
      fully-qualified name (that is, including the module).
    
    - "queue": filters by queue name.  Supports shell-style glob wildcards.
    
    - "agent": filters by agent name.  Supports shell-style glob wildcards.

      When applied to jobs in the "pending" state, this restricts the jobs to
      the ones that the agent could perform, according to its filter.
    
    - "uuid": filters by UUID string, or the special marker "THIS", indicating
      the UUID of the current process' dispatcher.  Supports shell-style glob
      wildcards.

      When applied to jobs in the "pending" state, this restricts the jobs to
      the ones that the agents for that dispatcher could perform, according to
      their filters.
    
    - "requested_start": a duration-based filter for when the job was requested
      to start.
      
      Note that, if a job is not given an explicit start time, the time when it
      was added to a queue is used.  This is based on a job's ``begin_after``
      attribute.
    
    - "start": a duration-based filter for when the job started work.
    
      Note that, if a job is restarted because of problems such as an
      interruption or a conflict error, this is the  most recent time that the
      job started work.  This is based on a job's ``active_start`` attribute.
      
      To see the first time a job started work ever, the default retry policies
      store a 'first_active' value in their ``data`` attribute
      (``job.getRetryPolicy().data.get('first_active')``).  Other information
      about retries can also be found in this data dictionary.
    
    - "end": a duration-based filter for when the job ended work (but not
      callbacks).
    
      This is based on a job's ``active_end`` attribute.
    
    - "callbacks_completed":  a duration-based filter for when the job
      finished the callbacks it had just after it performed the job.
      
      If subsequent callbacks are added, they are performed immediately, and
      will not affect the value that this filter uses.
    
      This is based on a job's ``initial_callbacks_end`` attribute.
    
    Usage Examples
    ==============
    
    Here are some examples of the command.

        asyncdb count pending
        (counts pending jobs)
        
        asyndb count active agent:instance5
        (counts the jobs that all agents named instance5 are working on)
        
        asyndb count pending agent:instance5
        (counts the pending jobs that all agents named "instance5" could
        perform)

        asyncdb count completed end:since1H callable:import_*
        (counts the completed jobs that ended within the last hour that called
        a function or method that began with the string "import_")

    Here are some examples of how the duration-based filters work.
    
    * If you used "start:since5s" then that could be read as "jobs that
      started five seconds ago or sooner."  
    
    * "requested_start:before1M" could be read as "jobs that were supposed to
      begin one minute ago or longer". 
    
    * "end:since1M,before30S" could be read as "jobs that ended their
      primary work (that is, not including callbacks) between thirty seconds
      and one minute ago."
    
    * "callbacks_completed:before30S,since1M" could be read as "jobs that
      completed the callbacks they had when first run between thirty seconds
      and one minute ago."  (This also shows that the order of "before" and
      "since" do not matter.)
    """
    res = 0
    for j in _jobs(context, states, **kwargs):
        res += 1
    return res

_status_keys = {
    zc.async.interfaces.NEW: 'new',
    zc.async.interfaces.PENDING: 'pending',
    zc.async.interfaces.ASSIGNED: 'assigned',
    zc.async.interfaces.ACTIVE: 'active',
    zc.async.interfaces.CALLBACKS: 'callbacks',
    zc.async.interfaces.COMPLETED: 'completed'}

def jobstats(context, *states, **kwargs):
    """Return statistics about jobs in one or more states.

    XXX describe statistics

    Jobs are identified by integer OID and database name.  These identifiers
    can be used with the "asyncdb job" command to get details about the jobs.

    The asyncdb commands "jobs," "count," "jobstats," and "firstjob" all share
    the same arguments, which are described below; after which usage examples
    for this command are listed.

    Arguments
    =========
    
    States
    ------

    You must provide at least one of the following states.

    - "pending": the job is in a queue, waiting to be started.
    
    - "assigned": a dispatcher has claimed the job and assigned it to one of
      its worker threads.  Work has not yet begun.  Jobs are in this state very
      briefly.
    
    - "active": A worker thread is performing this job.
    
    - "callbacks": the job's work is ended, and the thread is performing the
      callbacks, if any.
    
    - "completed": the job and its callbacks are completed.  Completed jobs
      stay in the database for only a certain amount of time--between seven and
      eight days in the default agent implementation.

    - "succeeded": the job completed successfully (that is, without raising an
      unhandled exception, and without returning an explicit
      twisted.python.failure.Failure).  This is a subset of "completed,"
      described above.

     - "failed": the job completed by raising an unhandled exception or by
       explicitly returning a twisted.python.failure.Failure.  This is a subset
       of "completed," described above.

    You may use no more than one of the states "completed," "succeeded," and
    "failed".

    Optional Arguments
    ------------------

    You can further filter your results with a number of optional arguments.

    "Shell-style glob wildcards," as referred to in this list, are "?", "*",
    "[SEQUENCE]", and "[!SEQUENCE]", as described in
    http://docs.python.org/lib/module-fnmatch.html .
    
    A "duration-based filter" described in this list accepts an argument that
    is of the form "sinceINTERVAL", "beforeINTERVAL", or
    "sinceINTERVAL,beforeINTERVAL" (no space!).  The "INTERVAL"s are of the
    form ``[nD][nH][nM][nS]``, where "n" should be replaced with a positive
    integer, and "D," "H," "M," and "S" are literals standing for "days,"
    "hours," "minutes," and "seconds." For instance, you might use ``5M`` for
    five minutes, ``20S`` for twenty seconds, or ``1H30M`` for an hour and a
    half.  Thus "before30M" would mean "thirty minutes ago or older." 
    "since45S" would mean "45 seconds ago or newer."  "since1H,before30M" would
    mean "between thirty minutes and an hour ago."  Note that reversing the two
    components in the last example--"before30M,since1H"--is equivalent.
    

    - "callable": filters by callable name.  Supports shell-style glob
      wildcards.  If you do not include a "." in the string, it matches only on
      the callable name.  If you include a ".", it matches on the
      fully-qualified name (that is, including the module).
    
    - "queue": filters by queue name.  Supports shell-style glob wildcards.
    
    - "agent": filters by agent name.  Supports shell-style glob wildcards.

      When applied to jobs in the "pending" state, this restricts the jobs to
      the ones that the agent could perform, according to its filter.
    
    - "uuid": filters by UUID string, or the special marker "THIS", indicating
      the UUID of the current process' dispatcher.  Supports shell-style glob
      wildcards.

      When applied to jobs in the "pending" state, this restricts the jobs to
      the ones that the agents for that dispatcher could perform, according to
      their filters.
    
    - "requested_start": a duration-based filter for when the job was requested
      to start.
      
      Note that, if a job is not given an explicit start time, the time when it
      was added to a queue is used.  This is based on a job's ``begin_after``
      attribute.
    
    - "start": a duration-based filter for when the job started work.
    
      Note that, if a job is restarted because of problems such as an
      interruption or a conflict error, this is the  most recent time that the
      job started work.  This is based on a job's ``active_start`` attribute.
      
      To see the first time a job started work ever, the default retry policies
      store a 'first_active' value in their ``data`` attribute
      (``job.getRetryPolicy().data.get('first_active')``).  Other information
      about retries can also be found in this data dictionary.
    
    - "end": a duration-based filter for when the job ended work (but not
      callbacks).
    
      This is based on a job's ``active_end`` attribute.
    
    - "callbacks_completed":  a duration-based filter for when the job
      finished the callbacks it had just after it performed the job.
      
      If subsequent callbacks are added, they are performed immediately, and
      will not affect the value that this filter uses.
    
      This is based on a job's ``initial_callbacks_end`` attribute.
    
    Usage Examples
    ==============
    
    Here are some examples of the command.

        asyncdb jobstats pending
        (gives statistics about the pending jobs)
        
        asyndb jobstats active agent:instance5
        (gives statistics about all jobs that any agent named instance5 is
        working on)
        
        asyndb job pending agent:instance5
        (gives statistics aboutt all pending jobs that agents named "instance5"
        could perform)

        asyncdb job completed end:since1H callable:import_*
        (gves statistics about completed jobs that ended within the last hour
        that called a function or method that began with the string "import_")

    Here are some examples of how the duration-based filters work.
    
    * If you used "start:since5s" then that could be read as "jobs that
      started five seconds ago or sooner."  
    
    * "requested_start:before1M" could be read as "jobs that were supposed to
      begin one minute ago or longer". 
    
    * "end:since1M,before30S" could be read as "jobs that ended their
      primary work (that is, not including callbacks) between thirty seconds
      and one minute ago."
    
    * "callbacks_completed:before30S,since1M" could be read as "jobs that
      completed the callbacks they had when first run between thirty seconds
      and one minute ago."  (This also shows that the order of "before" and
      "since" do not matter.)
    """
    now = datetime.datetime.now(pytz.UTC)
    d = {'pending': 0, 'assigned': 0, 'active': 0, 'callbacks': 0,
         'succeeded': 0, 'failed': 0}
    longest_wait = longest_active = None
    shortest_wait = shortest_active = None
    for j in _jobs(context, states, **kwargs):
        status = j.status 
        if status == zc.async.interfaces.COMPLETED:
            if isinstance(j.result, twisted.python.failure.Failure):
                d['failed'] += 1
            else:
                d['succeeded'] += 1
        else:
            d[_status_keys[status]] += 1
        wait = active = None
        if j.active_start:
            if j.active_end:
                active = j.active_end - j.active_start
            else:
                active = now - j.active_start
            if (longest_active is None or
                longest_active[0] < active):
                longest_active = active, j
            if (shortest_active is None or
                shortest_active[0] < active):
                shortest_active = active, j
            wait = j.active_start - j.begin_after
        else:
            wait = now - j.begin_after
        if (longest_wait is None or
            longest_wait[0] < wait):
            longest_wait = wait, j
        if (shortest_wait is None or
            shortest_wait[0] < wait):
            shortest_wait = wait, j
    d['longest wait'] = longest_wait
    d['longest active'] = longest_active
    d['shortest wait'] = shortest_wait
    d['shortest active'] = shortest_active
    return d

def jobsummary(job):
    now = datetime.datetime.now(pytz.UTC)
    wait = active = None
    if job.active_start:
        if job.active_end:
            active = job.active_end - job.active_start
        else:
            active = now - job.active_start
        wait = job.active_start - job.begin_after
    else:
        wait = now - job.begin_after
    if isinstance(job.result, twisted.python.failure.Failure):
        failed = True
        result = job.result.getBriefTraceback()
    else:
        failed = False
        result = zc.async.utils.custom_repr(job.result)
    a = job.agent
    if a:
        agent = job.agent.name
        dispatcher = a.parent.UUID
    else:
        agent = dispatcher = None
    q = job.queue
    if q:
        queue = q.name
    else:
        queue = None
    return {'repr': repr(job),
            'args': list(job.args),
            'kwargs': dict(job.kwargs),
            'begin after': job.begin_after,
            'active start': job.active_start,
            'active end': job.active_end,
            'initial callbacks end': job.initial_callbacks_end,
            'wait': wait,
            'active': active,
            'status': _status_keys[job.status],
            'failed': failed,
            'result': result,
            'quota names': job.quota_names,
            'agent': agent,
            'dispatcher': dispatcher,
            'queue': queue,
            'callbacks': list(job.callbacks)}

def _get_job(context, oid, database=None):
    conn = ZODB.interfaces.IConnection(context)
    if database is None:
        local_conn = conn
    else:
        local_conn = conn.get_connection(database)
    return local_conn.get(ZODB.utils.p64(int(oid)))

def traceback(context, oid, database=None, detail='default'):
    """Return the traceback for the job identified by integer oid."""
    detail = detail.lower()
    if detail not in ('brief', 'default', 'verbose'):
        raise ValueError('detail must be one of "brief," "default," "verbose"')
    job = _get_job(context, oid, database)
    if not isinstance(job.result, twisted.python.failure.Failure):
        return None
    return job.result.getTraceback(detail=detail)

def job(context, oid, database=None):
    """Return summary of job identified by integer oid."""
    return jobsummary(_get_job(context, oid, database))

def firstjob(context, *states, **kwargs):
    """Return summary of first job found matching given filters.

    XXX describe what "first" means for different states.

    The asyncdb commands "jobs," "count," "jobstats," and "firstjob" all share
    the same arguments, which are described below; after which usage examples
    for this command are listed.

    Arguments
    =========
    
    States
    ------

    You must provide at least one of the following states.

    - "pending": the job is in a queue, waiting to be started.
    
    - "assigned": a dispatcher has claimed the job and assigned it to one of
      its worker threads.  Work has not yet begun.  Jobs are in this state very
      briefly.
    
    - "active": A worker thread is performing this job.
    
    - "callbacks": the job's work is ended, and the thread is performing the
      callbacks, if any.
    
    - "completed": the job and its callbacks are completed.  Completed jobs
      stay in the database for only a certain amount of time--between seven and
      eight days in the default agent implementation.

    - "succeeded": the job completed successfully (that is, without raising an
      unhandled exception, and without returning an explicit
      twisted.python.failure.Failure).  This is a subset of "completed,"
      described above.

     - "failed": the job completed by raising an unhandled exception or by
       explicitly returning a twisted.python.failure.Failure.  This is a subset
       of "completed," described above.

    You may use no more than one of the states "completed," "succeeded," and
    "failed".

    Optional Arguments
    ------------------

    You can further filter your results with a number of optional arguments.

    "Shell-style glob wildcards," as referred to in this list, are "?", "*",
    "[SEQUENCE]", and "[!SEQUENCE]", as described in
    http://docs.python.org/lib/module-fnmatch.html .
    
    A "duration-based filter" described in this list accepts an argument that
    is of the form "sinceINTERVAL", "beforeINTERVAL", or
    "sinceINTERVAL,beforeINTERVAL" (no space!).  The "INTERVAL"s are of the
    form ``[nD][nH][nM][nS]``, where "n" should be replaced with a positive
    integer, and "D," "H," "M," and "S" are literals standing for "days,"
    "hours," "minutes," and "seconds." For instance, you might use ``5M`` for
    five minutes, ``20S`` for twenty seconds, or ``1H30M`` for an hour and a
    half.  Thus "before30M" would mean "thirty minutes ago or older." 
    "since45S" would mean "45 seconds ago or newer."  "since1H,before30M" would
    mean "between thirty minutes and an hour ago."  Note that reversing the two
    components in the last example--"before30M,since1H"--is equivalent.
    

    - "callable": filters by callable name.  Supports shell-style glob
      wildcards.  If you do not include a "." in the string, it matches only on
      the callable name.  If you include a ".", it matches on the
      fully-qualified name (that is, including the module).
    
    - "queue": filters by queue name.  Supports shell-style glob wildcards.
    
    - "agent": filters by agent name.  Supports shell-style glob wildcards.

      When applied to jobs in the "pending" state, this restricts the jobs to
      the ones that the agent could perform, according to its filter.
    
    - "uuid": filters by UUID string, or the special marker "THIS", indicating
      the UUID of the current process' dispatcher.  Supports shell-style glob
      wildcards.

      When applied to jobs in the "pending" state, this restricts the jobs to
      the ones that the agents for that dispatcher could perform, according to
      their filters.
    
    - "requested_start": a duration-based filter for when the job was requested
      to start.
      
      Note that, if a job is not given an explicit start time, the time when it
      was added to a queue is used.  This is based on a job's ``begin_after``
      attribute.
    
    - "start": a duration-based filter for when the job started work.
    
      Note that, if a job is restarted because of problems such as an
      interruption or a conflict error, this is the  most recent time that the
      job started work.  This is based on a job's ``active_start`` attribute.
      
      To see the first time a job started work ever, the default retry policies
      store a 'first_active' value in their ``data`` attribute
      (``job.getRetryPolicy().data.get('first_active')``).  Other information
      about retries can also be found in this data dictionary.
    
    - "end": a duration-based filter for when the job ended work (but not
      callbacks).
    
      This is based on a job's ``active_end`` attribute.
    
    - "callbacks_completed":  a duration-based filter for when the job
      finished the callbacks it had just after it performed the job.
      
      If subsequent callbacks are added, they are performed immediately, and
      will not affect the value that this filter uses.
    
      This is based on a job's ``initial_callbacks_end`` attribute.
    
    Usage Examples
    ==============
    
    Here are some examples of the command.

        asyncdb firstjob pending
        (describes the first (next-to-be-processed) pending jobs)
        
        asyndb job active agent:instance5
        (describes the first job that any agent named instance5
        is working on; "first" doesn't mean much here.)
        
        asyndb job pending agent:instance5
        (describes the first (next-to-be-processed) pending jobs that agents
        named "instance5" could perform)

        asyncdb job completed end:since1H callable:import_*
        (describes the first (most recently completed) completed job that ended
        within the last hour that called a function or method that began with
        the string "import_")

    Here are some examples of how the duration-based filters work.
    
    * If you used "start:since5s" then that could be read as "jobs that
      started five seconds ago or sooner."  
    
    * "requested_start:before1M" could be read as "jobs that were supposed to
      begin one minute ago or longer". 
    
    * "end:since1M,before30S" could be read as "jobs that ended their
      primary work (that is, not including callbacks) between thirty seconds
      and one minute ago."
    
    * "callbacks_completed:before30S,since1M" could be read as "jobs that
      completed the callbacks they had when first run between thirty seconds
      and one minute ago."  (This also shows that the order of "before" and
      "since" do not matter.)
    """
    for j in _jobs(context, states, **kwargs):
        return jobsummary(j)
    return None

def UUIDs(context):
    """Return all active UUIDs."""
    conn = ZODB.interfaces.IConnection(context)
    queues = conn.root()[zc.async.interfaces.KEY]
    if not len(queues):
        return []
    queue = iter(queues.values()).next()
    return [str(UUID) for UUID, da in queue.dispatchers.items()
            if da.activated]

def status(context, queue=None, agent=None, uuid=None):
    """Return status of the agents of all queues and all active UUIDs."""
    conn = ZODB.interfaces.IConnection(context)
    if uuid is not None:
        if uuid.upper() == 'THIS':
            uuid = zope.component.getUtility(zc.async.interfaces.IUUID)
        else:
            uuid = UUID(uuid)
    if queue is not None:
        queue = re.compile(fnmatch.translate(queue)).match
    if agent is not None:
        agent = re.compile(fnmatch.translate(agent)).match
    queues = conn.root()[zc.async.interfaces.KEY]
    res ={}
    if not len(queues):
        return res
    for q_name, q in queues.items():
        if queue is None or queue(q_name):
            das = {}
            res[q_name] = {'len': len(q), 'dispatchers': das}
            for da_uuid, da in q.dispatchers.items():
                if da.activated and (uuid is None or da_uuid == uuid):
                    agents = {}
                    das[str(da_uuid)] = da_data = {
                        'last ping': da.last_ping.value,
                        'since ping': (datetime.datetime.now(pytz.UTC) -
                                       da.last_ping.value),
                        'dead': da.dead,
                        'ping interval': da.ping_interval,
                        'ping death interval': da.ping_death_interval,
                        'agents': agents
                        }
                    for a_name, a in da.items():
                        if agent is None or agent(a_name):
                            agents[a_name] = d = {
                                'size': a.size,
                                'len': len(a)
                                }
                            if zc.async.interfaces.IFilterAgent.providedBy(a):
                                d['filter'] = a.filter
                            else:
                                d['chooser'] = a.chooser
    return res

funcs = {}

def help(context, cmd=None):
    """Get help on an asyncdb monitor tool.

    Usage is 'asyncdb help <tool name>' or 'asyncdb help'."""
    if cmd is None:
        res = [
            "These are the tools available.  Usage for each tool is \n"
            "'asyncdb <tool name> [modifiers...]'.  Learn more about each \n"
            "tool using 'asyncdb help <tool name>'.\n"]
        for nm, func in sorted(funcs.items()):
            res.append('%s: %s' % (
                nm, func.__doc__.split('\n', 1)[0]))
        return '\n'.join(res)
    f = funcs.get(cmd)
    if f is None:
        return 'Unknown async tool'
    return f.__doc__

for f in (
    count, jobs, job, firstjob, traceback, jobstats, UUIDs, status, help):
    name = f.__name__
    funcs[name] = f

def asyncdb(connection, cmd=None, *raw):
    """Monitor and introspect zc.async activity in the database.

    To see a list of asyncdb tools, use 'asyncdb help'.

    To learn more about an asyncdb tool, use 'asyncdb help <tool name>'.

    ``asyncdb`` tools differ from ``async`` tools in that ``asyncdb`` tools
    access the database, and ``async`` tools do not."""
    zc.async.monitor.monitor(
        funcs, asyncdb.__doc__, connection, cmd, raw, needs_db_connection=True)
