import os
import random
import math

import ZEO.ClientStorage
import ZODB
import transaction
import twisted.internet.reactor
import zc.monitor
import zc.monitor.interfaces
import zope.component

import zc.async.configure
import zc.async.queue
import zc.async.instanceuuid
import zc.async.agent
import zc.async.monitor
import zc.async.monitordb

def generate_sample(size=100000):
    count = 0
    for i in range(size):
        if math.hypot(random.random(), random.random()) < 1:
            count += 1
    return count, size

def process_samples(*sample_jobs):
    count = 0 
    size = 0
    for j in sample_jobs:
        count += j.result[0]
        size += j.result[1]
    return 4.0 * count / size

def choose_generate_sample(agent):
    return agent.queue.claim(
        lambda j: j.callable.__name__ == 'generate_sample')

def choose_another(agent):
    return agent.queue.claim(
        lambda j: j.callable.__name__ != 'generate_sample')

def install_agent(db):
    conn = db.open()
    try:
        q = zc.async.queue.getDefaultQueue(conn)
        try:
            dispatcher = q.dispatchers[zc.async.instanceuuid.UUID]
        except KeyError:
            twisted.internet.reactor.callLater(0.05, install_agent, db)
        else:
            if 'generate_sample' not in dispatcher:
                agent = dispatcher['main']
                agent.chooser = choose_another
                dispatcher['generate_sample'] = zc.async.agent.Agent(
                    choose_generate_sample, 1)
                transaction.commit()
    finally:
        transaction.abort()
        conn.close()

if __name__ == '__main__':
    monitor_port = os.environ.get('MONITOR_PORT')
    if monitor_port:
        for f in (zc.monitor.interactive, zc.monitor.quit, zc.monitor.help,
                  zc.async.monitor.async, zc.async.monitordb.asyncdb):
            zope.component.provideUtility(
                f, zc.monitor.interfaces.IMonitorPlugin, f.__name__)
        zc.monitor.start(int(monitor_port))
    storage = ZEO.ClientStorage.ClientStorage(
        ('127.0.0.1', 9999))
    db = ZODB.DB(storage)
    zc.async.configure.base()
    zc.async.configure.start(
        db, poll_interval=0.1, twisted=True)
    twisted.internet.reactor.callWhenRunning(install_agent, db)
    twisted.internet.reactor.run()
