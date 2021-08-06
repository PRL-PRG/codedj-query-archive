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
    agent_installer=zc.async.subscribers.agent_installer):
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


def tearDown():
    dispatcher = zc.async.dispatcher.get()
    dispatcher.reactor.callFromThread(dispatcher.reactor.stop)
    dispatcher.thread.join(3)
