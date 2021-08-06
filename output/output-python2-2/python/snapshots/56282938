import logging
import sys
import transaction
import zope.component

import zc.async.dispatcher
import zc.async.subscribers
import zc.async.testing

# helper functions convenient for Zope 3 functional tests

def setUp(
    connection=None,
    queue_installer=zc.async.subscribers.queue_installer,
    dispatcher_installer=zc.async.subscribers.ThreadedDispatcherInstaller(
        poll_interval=0.1),
    agent_installer=zc.async.subscribers.agent_installer,
    log_file=sys.stdout, log_level=logging.CRITICAL):
    """Set up zc.async, as is needed for Zope 3 functional tests.
    """
    if connection is None:
        connection = sys._getframe(1).f_globals['getRootFolder']()._p_jar
    db = connection.db()
    zope.component.provideHandler(agent_installer)
    event = zc.async.interfaces.DatabaseOpened(db)

    dispatcher_installer(event)
    dispatcher = zc.async.dispatcher.get()
    _ = transaction.begin()
    queue_installer(event)
    zc.async.testing.get_poll(dispatcher, count=0)
    assert "" in zc.async.testing.get_poll(dispatcher)
    assert dispatcher.activated is not None
    if log_file is not None:
        # this helps with debugging critical problems that happen in your
        # zc.async calls.  Of course, if your test
        # intentionally generates CRITICAL log messages, you may not want this;
        # pass ``log_file=None`` to setUp.
        # stashing this on the dispatcher is a hack, but at least we're doing
        # it on code from the same package.
        dispatcher._debug_handler = zc.async.testing.print_logs(
            log_file, log_level)

def tearDown():
    dispatcher = zc.async.dispatcher.get()
    if getattr(dispatcher, '_debug_handler', None) is not None:
        logger = logging.getLogger('zc.async')
        logger.removeHandler(dispatcher._debug_handler)
        del dispatcher._debug_handler
    dispatcher.reactor.callFromThread(dispatcher.reactor.stop)
    dispatcher.thread.join(3)
    for queue_pools in dispatcher.queues.values():
        for name, pool in queue_pools.items():
            pool.setSize(0)
            for thread in pool.threads:
                thread.join(3)
    zc.async.dispatcher.clear()
