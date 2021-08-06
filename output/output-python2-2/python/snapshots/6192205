#!/usr/bin/env python
'''DOCSTRING'''

import ConfigParser
import math
import os.path
import random
from agents.fare import Fare, FareFactory
from agents.taxi import Taxi
from agents.agent import Agent

# TEST
import graphs.tigerutils

config=ConfigParser.SafeConfigParser()
config.read(os.path.join('agents','defaults.ini'))
config.read(os.path.join('agents','overrides.ini'))

# dev and runtime config values
TRACING=config.getboolean('dev','tracing')
NUM_TAXIS=config.getint('runtime','numTaxis')
NUM_FARES=config.getint('runtime','numFares')
NP=config.get('runtime','negotiationProtocol')
SIMTIME=config.getint('runtime','simulationTime')
SIMTYPE=config.get('runtime','simType')
USE_GUI=config.getboolean('runtime','useGUI')
SEED=config.getint('runtime','randomSeed')

if TRACING:
    from SimPy.SimulationTrace import *
else:
    from SimPy.Simulation import *

def printHeader(verbose=False):
    # This function prints all the configuration data for this specific
    # simulation run.  Useful for ensuring we're comparing apples to apples.
    #
    # TODO [later] Flag items that are changed from the default, or
    # (conversely, but basically equivalently), flag default items.

    # What kind of leading information would be useful?  Full path to
    # driver.py?  Timestamp?
    print '---[ Useful information here ]---'

    if verbose:
        print 'Sorry, verbose is not working yet.'
    else:
        print 'Runtime configuration settings:'
        for k, v in config.items('runtime'):
            # TODO Think about how to compare the values of two dicts here, so
            # I can simply flag those that are changed from one to the other.
            print '  %s=%s' % (k, v)
        print 'Development configuration settings:'
        for k, v in config.items('dev'):
            # TODO Think about how to compare the values of two dicts here, so
            # I can simply flag those that are changed from one to the other.
            print '  %s=%s' % (k, v)
    print '---[ No more useful information here ]---'


def model():
    initialize()
    random.seed(SEED)
#    random.seed(333777)

#    # Create Fares prior to starting the simulation.  Now a TK.
#    for j in range(1,NUM_FARES):
#        # TODO Put this in place later.  grep for "Fare Fare" to see what it's
#        # doing.  It may in fact point to other problems with my strategy.
#        #f=Fare(name='Fare-' + `j`)
##        fname = 'Fare-%s' % j
##        f=Fare(fname)
#        f=Fare(name=-j)
#        activate(f, f.run())

    # Team 1 - Yellow Cab
    for i in range(NUM_TAXIS):
        tx = Taxi('Yellow-%s' % i, NP)
        if SIMTYPE == 'cooperate':
            activate(tx, tx.cooperate())
        elif SIMTYPE == 'compete':
            activate(tx, tx.compete())
        else:
            print "Error: can't set the simulation type"
            import sys; sys.exit()
    #
    # Team 2 - Checker Cab
    #for i in range(4):
    #    tx=Taxi('Checker-%s' % i)
    #    tx=Taxi('Checker-%s' % i, 'closestfare')
    #    tx=Taxi('Checker-%s' % i, 'mixedmode')
    #    activate(tx, tx.cooperate())
    #    activate(tx, tx.compete())

    ff = FareFactory()
    activate(ff, ff.generate())
#    activate(ff, ff.generate(), datacollector=)
    simulate(until=SIMTIME)
    print 'waitingFares', [x.name for x in Agent.waitingFares.theBuffer]
#    print 'allMonitors: %s' % allMonitors

    # TODO this is the average wait time for all fares.  It's okay as a
    # starting point, but I want two things.  First, I want a breakdown of the
    # two times fareRequest to pickup, and pickup to dropoff.  Second, I want
    # to break it down either by Fare, or, if that turns out to not offer
    # anything interesting, AT LEAST I want to get at the variance, because
    # that is one of the statistics that I want to optimize.
    #
    # TODO [hipri?] figure out where to put this
#    print '\nMean fare wait time %s, time average %s' % (G.fareMon.mean(),
#            G.fareMon.timeAverage(now()))

    # TODO how to get this value from the agents/fare.py?
#    print 'Total number of Fares created: %s' % agents.fare.numFaresCreated
#    print 'Number of Fares created: %s' % (-1)
#    print 'Number of Fares that have been serviced: %d' % (-1)
#    print 'Number of Fares in waiting in buffer at end of simulation: %d' % (-1)
#    print 'use asserts here!'

def reportstats():
#    print
#    print 'WTF are these stats anyway?'
#    print 'length of waiting Fares buffer at different times:', Agent.waitingFares.bufferMon
#    print 'getQ:', Agent.waitingFares.getQMon
#    print 'putQ:', Agent.waitingFares.putQMon
    print
    #print 'Fare.waitMon:', Fare.waitMon
    print 'Fare.waitMon.name:', Fare.waitMon.name
    print 'Fare.waitMon.yseries:', Fare.waitMon.yseries()
    print '  * yseries: Elapsed time between when the Fare made a request',
    print 'for pickup, and when the Taxi dropped off the Fare'
    print 'Fare.waitMon.tseries():', Fare.waitMon.tseries()
    print '  * tseries: Recorded simtimes when the Taxis dropped off the Fares'
    print 'Fare.waitMon.total:', Fare.waitMon.total()
    print '  * total: The total of all times recorded tseries times.  Not',
    print 'all that useful by itself, but used for calculating the mean.'
    print 'Fare.waitMon.count:', Fare.waitMon.count()
    print '  * count: The number of Fares that were picked up and dropped off.'
    print 'Fare.waitMon.mean:', Fare.waitMon.mean()
    print '  * mean: The mean of the values.  yseries/count'
    print 'Fare.waitMon.var:', Fare.waitMon.var()
    print '  * var: The variance of the values.  All I know for certain is',
    print 'that the variance is the square of the standard deviation, which',
    print 'is generally considered a more useful statistic.'
    print 'Fare.waitMon.stdDeviation:', math.sqrt(Fare.waitMon.var())
    print '  * stdDeviation: Not part of SimPy.Monitor, but easy to',
    print "calculate.  It's the square root of the variance."
    print 'Fare.waitMon.timeAverage:', Fare.waitMon.timeAverage()
    #print 'Fare.waitMon.:', Fare.waitMon
#    Fare.waitMon.var()

def oooh_shiny():
    # TODO [hipri] Add this into the main proggy.  Use config switch 'useGUI'
    # to decide whether to use SimPlot/plotHistogram, or just printHistogram.
    # There is no reason to leave that important data out, even though I don't
    # always want to incur the cost/hassle of producing a GUI.
    if USE_GUI:
        from SimPy.SimPlot import SimPlot
        histo = Fare.waitMon.histogram(low=0.0, high=SIMTIME, nbins=20)
        plt = SimPlot()
        plt.plotHistogram(histo, xlab='Time', ylab='Number of waiting Fares',
                title='Time waiting for Taxi', color='red', width=2)
        plt.mainloop()
    else:
#        print 'Got here'
        #Fare.waitMon.printHistogram(histo, xlab='Time', \
        #        ylab='Number of waiting Fares', \
        #        title='Time waiting for Taxi', color='red', width=2)
#        Fare.waitMon.printHistogram(histo)
        Fare.waitMon.setHistogram(low=0.0, high=SIMTIME, nbins=20)
        print Fare.waitMon.printHistogram(fmt='%6.4f')

if __name__ == '__main__':
    printHeader()
    model()
#    reportstats()
#    oooh_shiny()
