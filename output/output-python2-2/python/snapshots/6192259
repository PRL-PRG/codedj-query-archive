#!/usr/bin/env python

import ConfigParser
import math
import os.path
import random
from agents.fare import Fare, FareFactory
from agents.taxi import Taxi
from agents.agent import Agent

from SimPy.Simulation import *

SIMTIME=500

# The cooperate NPs
def model_cooperate_FIFO():
    initialize()
    random.seed(333777)
    NP = 'FIFO'

    for j in range(1,5):
        f=Fare(name=-j)
        activate(f, f.run())

    for i in range(5):
        tx = Taxi('Yellow-%s' % i, NP)
        activate(tx, tx.cooperate())

    ff = FareFactory()
    activate(ff, ff.generate())
    simulate(until=SIMTIME)
    print 'waitingFares', [x.name for x in Agent.waitingFares.theBuffer]

def model_cooperate_closestfare():
    initialize()
    random.seed(333777)
    NP = 'closestfare'

    for j in range(1,5):
        f=Fare(name=-j)
        activate(f, f.run())

    for i in range(5):
        tx = Taxi('Yellow-%s' % i, NP)
        activate(tx, tx.cooperate())

    ff = FareFactory()
    activate(ff, ff.generate())
    simulate(until=SIMTIME)
    print 'waitingFares', [x.name for x in Agent.waitingFares.theBuffer]

def model_cooperate_mixedmode():
    initialize()
    random.seed(333777)
    NP = 'mixedmode'

    for j in range(1,5):
        f=Fare(name=-j)
        activate(f, f.run())

    for i in range(5):
        tx = Taxi('Yellow-%s' % i, NP)
        activate(tx, tx.cooperate())

    ff = FareFactory()
    activate(ff, ff.generate())
    simulate(until=SIMTIME)
    print 'waitingFares', [x.name for x in Agent.waitingFares.theBuffer]


# The compete NPs
def model_compete_FIFO():
    initialize()
    random.seed(333777)
    NP = 'FIFO'

    for j in range(1,5):
        f=Fare(name=-j)
        activate(f, f.run())

    for i in range(5):
        tx = Taxi('Yellow-%s' % i, NP)
        activate(tx, tx.compete())

    ff = FareFactory()
    activate(ff, ff.generate())
    simulate(until=SIMTIME)
    print 'waitingFares', [x.name for x in Agent.waitingFares.theBuffer]

def model_compete_closestfare():
    initialize()
    random.seed(333777)
    NP = 'closestfare'

    for j in range(1,5):
        f=Fare(name=-j)
        activate(f, f.run())

    for i in range(5):
        tx = Taxi('Yellow-%s' % i, NP)
        activate(tx, tx.compete())

    ff = FareFactory()
    activate(ff, ff.generate())
    simulate(until=SIMTIME)
    print 'waitingFares', [x.name for x in Agent.waitingFares.theBuffer]

def model_compete_mixedmode():
    initialize()
    random.seed(333777)
    NP = 'mixedmode'

    for j in range(1,5):
        f=Fare(name=-j)
        activate(f, f.run())

    for i in range(5):
        tx = Taxi('Yellow-%s' % i, NP)
        activate(tx, tx.compete())

    ff = FareFactory()
    activate(ff, ff.generate())
    simulate(until=SIMTIME)
    print 'waitingFares', [x.name for x in Agent.waitingFares.theBuffer]


if __name__ == '__main__':
    reply = raw_input('Run cooperate/FIFO? (y/n): ')
    if reply == 'y':
        model_cooperate_FIFO(); print
    reply = raw_input('Run cooperate/closestfare? (y/n) ')
    if reply == 'y':
        model_cooperate_closestfare(); print
    reply = raw_input('Run cooperate/mixedmode? (y/n) ')
    if reply == 'y':
        model_cooperate_mixedmode(); print
    reply = raw_input('Run compete/FIFO? (y/n) ')
    if reply == 'y':
        model_compete_FIFO(); print
    reply = raw_input('Run compete/closestfare? (y/n) ')
    if reply == 'y':
        model_compete_closestfare(); print
    reply = raw_input('Run compete/mixedmode? (y/n) ')
    if reply == 'y':
        model_compete_mixedmode(); print
