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
import re
import datetime
import pytz
import uuid
import simplejson

import zope.component

import zc.async.dispatcher

_marker = object()
class Encoder(simplejson.JSONEncoder):
    def default(self, obj):
        if isinstance(obj, datetime.timedelta):
            tmp = {'days': obj.days,
                   'hours': obj.seconds // (60*60),
                   'minutes': (obj.seconds % (60*60)) // 60,
                   'seconds': float(
                        obj.seconds % 60) + obj.microseconds/1000000
                  }
            res = dict((k, v) for k, v in tmp.items() if v)
            if not res:
                res['seconds'] = 0.0
            return res
        # TODO the spelling of this conditional is to support our test setup
        # shenanigans.  originally was ``isinstance(obj, datetime.datetime)``.
        # Would be nice to fix, though the duck typing is Pythonic at least.
        elif (getattr(obj, 'tzinfo', _marker) is not _marker and
              getattr(obj, 'astimezone', _marker) is not _marker):
            if obj.tzinfo is not None:
                obj = obj.astimezone(pytz.UTC).replace(tzinfo=None)
            return obj.isoformat() + "Z"
        elif isinstance(obj, uuid.UUID):
            return str(obj)
        return simplejson.JSONEncoder.default(self, obj)

encoder = Encoder(sort_keys=True, indent=4)


def status(uuid=None):
    """Get a mapping of general zc.async dispatcher information.

    'status' is one of 'STUCK', 'STARTING', 'RUNNING', or 'STOPPED', where
    'STUCK' means the poll is past due."""
    if uuid is not None:
        uuid = uuid.UUID(uuid)
    return encoder.encode(zc.async.dispatcher.get(uuid).getStatusInfo())

def jobs(queue=None, agent=None, uuid=None):
    """Show active jobs in worker threads as of the instant.

    Usage:

        jobs
        (returns active jobs as of last poll, newest to oldest)

        jobs queue:<queue name>
        (jobs are filtered to those coming from the named queue)

        jobs agent:<agent name>
        (jobs are filtered to those coming from agents with given name)

    "queue:" and "agent:" modifiers may be combined.

    Example:

        async jobs queue: agent:main
        (results filtered to queue named '' and agent named 'main')"""
    if uuid is not None:
        uuid = uuid.UUID(uuid)
    return encoder.encode(
        zc.async.dispatcher.get(uuid).getActiveJobIds(queue, agent))

def job(OID, database=None, uuid=None):
    """Local information about a job as of last poll, if known.

    Does not consult ZODB, but in-memory information.

    Usage:

        job <job id>
        (returns information about the job)

        job <job id> database:<database name>
        (returns job information, with job id disambiguated by database name)

    The job id in this case is an integer such as those returned by the
    ``async jobs`` command or in the ``longest ...`` and ``shortest ...``
    values of the ``async jobstats`` command.  It is the integer version of the
    oid of the job, and can be converted to an oid with ``ZODB.utils.p64``, and
    converted back to an integer with ``ZODB.utils.u64``.
    """
    if uuid is not None:
        uuid = uuid.UUID(uuid)
    return encoder.encode(
        zc.async.dispatcher.get(uuid).getJobInfo(long(OID), database))

_find = re.compile(r'\d+[DHMS]').findall
def _dt(s):
    if s is None:
        res = s
    else:
        try:
            res = int(s)
        except ValueError:
            vals = {}
            for val in _find(s.upper()):
                vals[val[-1]] = int(val[:-1])
            res = datetime.timedelta(
                days=vals.get('D', 0),
                hours=vals.get('H', 0),
                minutes=vals.get('M', 0),
                seconds=vals.get('S', 0)) + datetime.datetime.utcnow()
    return res


def jobstats(at=None, before=None, since=None, queue=None, agent=None,
             uuid=None):
    """Statistics on historical jobs as of last poll.

    Usage:

        jobstats
        (returns statistics on historical jobs as of last poll)

        jobstats queue:<queue name>
        (statistics are filtered to those coming from the named queue)

        jobstats agent:<agent name>
        (statistics are filtered to those coming from agents with given name)

        jobstats at:<poll key or interval>
        (statistics are collected at or before the poll key or interval)

        jobstats before:<pollkey or interval>
        (statistics are collected before the poll key or interval)

        jobstats since:<pollkey or interval>
        (statistics are collected since poll key or interval, inclusive)

    The modifiers "queue:", "agent:", "since:", and one of "at:" or "before:"
    may be combined.

    Intervals are of the format ``[nD][nH][nM][nS]``, where "n" should
    be replaced with a positive integer, and "D," "H," "M," and "S" are
    literals standing for "days," "hours," "minutes," and "seconds."
    For instance, you might use ``5M`` for five minutes, ``20S`` for
    twenty seconds, or ``1H30M`` for an hour and a half.

    Poll keys are the values shown as "key" from the ``poll`` or ``polls``
    command.

    Example:

        async jobstats queue: agent:main since:1H
        (results filtered to queue named '' and agent named 'main' from now
         till one hour ago)"""
    if uuid is not None:
        uuid = uuid.UUID(uuid)
    return encoder.encode(
        zc.async.dispatcher.get(uuid).getStatistics(
            _dt(at), _dt(before), _dt(since), queue, agent))

def poll(at=None, before=None, uuid=None):
    """Get information about a single poll, defaulting to most recent.

    Usage:

        poll
        (returns most recent poll)

        poll at:<poll key or interval>
        (returns poll at or before the poll key or interval)

        poll before:<poll key or interval>
        (returns poll before the poll key or interval)

    Intervals are of the format ``[nD][nH][nM][nS]``, where "n" should
    be replaced with a positive integer, and "D," "H," "M," and "S" are
    literals standing for "days," "hours," "minutes," and "seconds."
    For instance, you might use ``5M`` for five minutes, ``20S`` for
    twenty seconds, or ``1H30M`` for an hour and a half.

    Example:

        async poll at:5M
        (get the poll information at five minutes ago or before)"""
    # TODO: parse at and before to datetimes
    if uuid is not None:
        uuid = uuid.UUID(uuid)
    info = zc.async.dispatcher.get(uuid).getPollInfo(_dt(at), _dt(before))
    res = {'key': info.key, 'time': info.utc_timestamp.isoformat() + "Z",
           'results': info}
    return encoder.encode(res)

def polls(at=None, before=None, since=None, count=None, uuid=None):
    """Get information about recent polls, defaulting to most recent.

    Usage:

        polls
        (returns most recent 3 poll)

        polls at:<poll key or interval>
        (returns up to 3 polls at or before the poll key or interval)

        polls before:<poll key or interval>
        (returns up to 3 polls before the poll key or interval)

        polls since:<poll key or interval>
        (returns polls since the poll key or interval, inclusive)

        polls count <positive integer>
        (returns the given number of the most recent files)

    The modifiers "since:", "count:", and one of "at:" or "before:" may
    be combined.

    Intervals are of the format ``[nD][nH][nM][nS]``, where "n" should
    be replaced with a positive integer, and "D," "H," "M," and "S" are
    literals standing for "days," "hours," "minutes," and "seconds."
    For instance, you might use ``5M`` for five minutes, ``20S`` for
    twenty seconds, or ``1H30M`` for an hour and a half.

    Example:

        async polls before:5M since:10M
        (get the poll information from 5 to 10 minutes ago)"""
    if uuid is not None:
        uuid = uuid.UUID(uuid)
    if count is None:
        if since is None:
            count = 3
    else:
        count = int(count)
    return encoder.encode(
        [{'key': p.key, 'time': p.utc_timestamp.isoformat() + "Z",
          'results': p}
         for p in zc.async.dispatcher.get(uuid).iterPolls(
            _dt(at), _dt(before), _dt(since), count)])

# provide in async and separately:

def utcnow():
    """Return the current time in UTC, in ISO 8601 format."""
    return datetime.datetime.utcnow().isoformat() + "Z"

def UUID():
    """Get instance UUID in hex."""
    res = zope.component.getUtility(zc.async.interfaces.IUUID)
    if res is not None:
        return str(res)

funcs = {}

def help(cmd=None):
    """Get help on an async monitor tool.

    Usage is 'async help <tool name>' or 'async help'."""
    if cmd is None:
        res = [
            "These are the tools available.  Usage for each tool is \n"
            "'async <tool name> [modifiers...]'.  Learn more about each \n"
            "tool using 'async help <tool name>'.\n"]
        for nm, func in sorted(funcs.items()):
            res.append('%s: %s' % (
                nm, func.__doc__.split('\n', 1)[0]))
        return '\n'.join(res)
    f = funcs.get(cmd)
    if f is None:
        return 'Unknown async tool'
    return f.__doc__

for f in status, jobs, job, jobstats, poll, polls, utcnow, UUID, help:
    funcs[f.__name__] = f

def monitor(funcs, help, connection, cmd, raw):
    if cmd is None:
        res = help
    else:
        f = funcs.get(cmd)
        if f is None:
            res = '[Unknown async tool]'
        else:
            args = []
            kwargs = {}
            for val in raw:
                if ':' in val:
                    key, val = val.split(':', 1)
                    kwargs[key] = val
                else:
                    if kwargs:
                        raise ValueError(
                            'placeful modifiers must come before named '
                            'modifiers')
                    args.append(val)
            res = f(*args, **kwargs)
    connection.write(res)
    connection.write('\n')

def async(connection, cmd=None, *raw):
    """A collection of tools to monitor zc.async activity in this process.

    To see a list of async tools, use 'async help'.

    To learn more about an async monitor tool, use 'async help <tool name>'."""
    monitor(funcs, async.__doc__, connection, cmd, raw)

def asyncdb(connection, cmd=None, *raw):
    """A collection of tools to monitor zc.async activity in the database.

    To see a list of asyncdb tools, use 'asyncdb help'.

    To learn more about an asyncdb monitor tool, use 'asyncdb help <tool name>'.

    ``asyncdb`` tools differ from ``async`` tools in that ``asyncdb`` tools
    access the database, and ``async`` tools do not."""
    monitor(dbfuncs, asyncdb.__doc__, connection, cmd, raw)
