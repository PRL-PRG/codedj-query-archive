#!/usr/bin/env python
'''DOCSTRING'''

import ConfigParser
import os.path
from random import expovariate
from agent import Agent

config = ConfigParser.SafeConfigParser()
config.read(os.path.join('agents', 'defaults.ini'))
config.read(os.path.join('agents', 'overrides.ini'))

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
    waitMon = Monitor('All Fares total wait time')
    def __init__(self, name):
        Agent.__init__(self, name)
        # Fare maintains its own SimEvent, but Taxi uses it (look for
        # fareBeingDriven.doneSignal.signal(self.name) in the Taxi's
        # cooperate() method.)
        self.doneSignal = SimEvent()
        #self.loc['dest'] = self.mkcoords()
        self.loc['dest'] = self.map.get_location()
        # This list is used with the Taxi's compete() method.  All Taxis that
        # are competing for this Fare get dropped here temporarily.
        self.competeQ = []
	# This variable is set after a Taxi interrupts all other competitors,
	# and the Fare is basically out of the game.  All competition for this
	# Fare is OVER.
	self.pickedUp=False
        print '%.4f Fare %s activated' % (self.ts['activation'], self.name)
        print '.. Fare %s location: %s' % (self.name, self.loc)

    def run(self):
        self.ts['mkreq'] = now()
        yield put, self, Agent.waitingFares, [self]
        # TODO [hipri] Should this be yield and then self.ts?  Maybe better, I
        # should just drop self.ts['pickup'] altogether.
        self.ts['pickup'] = now()
        yield waitevent, self, self.doneSignal

#	# TESTING
#	#
#	# There is a BUG with interrupted Taxis going back and picking up the
#	# same Fare on their next time thru the queue.  Potentially the fix is
#	# to have the Fare mark itself as no longer eligible, and have the
#	# Taxi poll state before adding itself to that Fare's (closed)
#	# competeQ.
#	print ".. %.4f setting %s.pickedUp=True in fare.py ..." % (now(), self.name)
#	self.pickedUp=True

        self.ts['dropoff'] = now()
        whichTaxi = self.doneSignal.signalparam
        # TODO [hipri] This is being reported out of order.  It shows up in
        # the simulation output after the Taxi is on to the next Fare.
        # Regardless, the Fare is the right "place" to report drop off time.
        # I'll probably remove the other print statements in Taxi.py anyway.
        # They are there for development, not for the final product.
        print 'Time %s Fare %s taken by Taxi %s' % (now(), self.name,
                whichTaxi)

        # WAIT MONITOR
        #Fare.waitMon.observe((self.ts['dropoff'] - self.ts['mkreq']), now())
        Fare.waitMon.observe(now() - self.ts['mkreq'])

class FareFactory(Process):
    def generate(self):
        # TODO instead of saying 'while True:', I may want to pass in (via the
        # config) a specific number of Fares to be created.
        global numFaresCreated
        while True:
            # TODO [very lopri] f = Fare(name='Fare-%s' % numFaresCreated)?
            f = Fare(name=numFaresCreated)
            activate(f, f.run())
            numFaresCreated+=1
            t = expovariate(1.0/MEAN_FARE_GENERATION_RATE)
            yield hold, self, t

if __name__ == '__main__':
    # TODO try FareFactory too
    f = Fare('Filip')
    f.run()

