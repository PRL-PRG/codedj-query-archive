import os
import transaction
import transaction.interfaces
import ZODB.interfaces
import twisted.internet.reactor
import zope.component
import zope.event
import zope.app.appsetup.interfaces
import zc.twist

import zc.async.datamanager
import zc.async.interfaces
import zc.async.engine

NAME = 'zc.async.datamanager'

class InstallerAndNotifier(object):

    def __init__(self, name=NAME,
                 factory=lambda *args: zc.async.datamanager.DataManager(),
                 get_folder=lambda r: r):
        zope.component.adapter(
            zope.app.appsetup.interfaces.IDatabaseOpenedEvent)(self)
        self.name = name
        self.factory = factory
        self.get_folder = get_folder

    def __call__(self, ev):
        db = ev.database
        tm = transaction.TransactionManager()
        conn = db.open(transaction_manager=tm)
        tm.begin()
        try:
            try:
                root = conn.root()
                folder = self.get_folder(root)
                tm.commit()
                if self.name not in folder:
                    folder[self.name] = self.factory(conn, folder)
                    if folder[self.name]._p_jar is None:
                        conn.add(folder[self.name])
                elif not zc.async.interfaces.IDataManager.providedBy(
                    folder[self.name]):
                    raise RuntimeError(
                        'IDataManager not found') # TODO better error
                zope.event.notify(
                    zc.async.interfaces.DataManagerAvailable(folder[self.name]))
                tm.commit()
            except:
                tm.abort()
                raise
        finally:
            conn.close()

basicInstallerAndNotifier = InstallerAndNotifier()

class SeparateDBCreation(object):
    def __init__(self, db_name='zc.async', name=NAME,
                 factory=zc.async.datamanager.DataManager,
                 get_folder=lambda r:r):
        self.db_name = db_name
        self.name = name
        self.factory = factory
        self.get_folder = get_folder

    def __call__(self, conn, folder):
        conn2 = conn.get_connection(self.db_name)
        tm = transaction.interfaces.ITransactionManager(conn)
        root = conn2.root()
        folder = self.get_folder(root)
        tm.commit()
        if self.name in folder:
            raise ValueError('data manager already exists in separate database',
                             self.db_name, folder, self.name)
        dm = folder[self.name] = self.factory()
        conn2.add(dm)
        tm.commit()
        return dm

installerAndNotifier = InstallerAndNotifier(factory=SeparateDBCreation())

@zope.component.adapter(zc.async.interfaces.IDataManagerAvailableEvent)
def installTwistedEngine(ev):
    engine = zc.async.engine.Engine(
        zope.component.getUtility(
            zc.async.interfaces.IUUID, 'instance'),
        zc.async.datamanager.Worker)
    dm = ev.object
    twisted.internet.reactor.callLater(
        0,
        zc.twist.Partial(engine.poll, dm))
    twisted.internet.reactor.addSystemEventTrigger(
        'before', 'shutdown', zc.twist.Partial(
            engine.tearDown, dm))
