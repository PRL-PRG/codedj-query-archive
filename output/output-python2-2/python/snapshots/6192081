#!/usr/bin/env python
'''DOCSTRING'''

import ConfigParser
import os.path
from random import expovariate
from agent import Agent

config = ConfigParser.SafeConfigParser()
config.read(os.path.join('agents','conf','agents','defaults.ini'))
config.read(os.path.join('agents','conf','agents','overrides.ini'))

# dev config values
TRACING = config.getboolean('dev', 'tracing')

# runtime config values
MEAN_FARE_GENERATION_RATE = config.getint('runtime', 'meanFareGenerationRate')

# This is out here because it looks like without if I don't use a global
# variable, when the fare count reaches storeCapacity (STORE_CAP in agent.py),
# the count resets to 0, rather than continuing to increment.  That doesn't
# account for why the fare count behaves this way, but it seems to be acting
# like a class variable.
numFaresCreated = 0

if TRACING:
    from SimPy.SimulationTrace import *
else:
    from SimPy.Simulation import *


class Fare(Agent):
    '''DOCSTRING'''
    # TODO [hipri] Monitor the time between when the fare was requested and
    # when the Fare was dropped off.  ylab should be ...
    waitMon=Monitor('All Fares total wait time')
    def __init__(self, name):
        Agent.__init__(self, name)
        # Fare maintains its own SimEvent, but Taxi uses it (look for
        # fareBeingDriven.doneSignal.signal(self.name) in the Taxi's
        # cooperate() and compete() methods.)
        self.doneSignal = SimEvent()

    def run(self):
        self.ts['mkreq'] = now()

        # request Taxi [add self to waitingFares queue]
        yield put, self, Agent.waitingFares, [self]
        self.ts['pickup'] = now()

        # TODO get the name of the Taxi that picked up this Fare.  Add to the
        # final report for this Fare.
        #
        # Also, if this is a compete simulation, it would be nice (but maybe
        # not easy) to collect the identifiers for all Taxis that are
        # competing for a given Fare.

        # picked up [received signal]
        yield waitevent, self, self.doneSignal
        self.ts['dropoff'] = now()

        # dropped off [received signal]
        whichTaxi = self.doneSignal.signalparam
        print '%.4f\t(f) %s was taken by %s' % (now(), self.name, whichTaxi)

        # WAIT MONITOR
        #Fare.waitMon.observe((self.ts['dropoff'] - self.ts['mkreq']), now())
        Fare.waitMon.observe(now() - self.ts['mkreq'])

        print "%s -- I'm outta here!" % self.name


class FareFactory(Process):
    def generate(self):
        # TODO instead of saying 'while True:', I may want to pass in (via the
        # config) a specific number of Fares to be created.
        global numFaresCreated
        while True:
            f = Fare(name="Fare-"+str(numFaresCreated))
            activate(f, f.run())
            numFaresCreated+=1
            t = expovariate(1.0/MEAN_FARE_GENERATION_RATE)
            yield hold, self, t


if __name__ == '__main__':
    # TODO try FareFactory too
    f = Fare('Filip')
    f.run()
