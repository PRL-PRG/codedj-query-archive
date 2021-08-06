import random
import math

import ZEO.ClientStorage
import ZODB
import twisted.internet.reactor

import zc.async.configure

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

if __name__ == '__main__':
    storage = ZEO.ClientStorage.ClientStorage(
        ('127.0.0.1', 9999))
    db = ZODB.DB(storage)
    zc.async.configure.base()
    zc.async.configure.start(
        db, poll_interval=0.1, twisted=True)
    twisted.internet.reactor.run()
