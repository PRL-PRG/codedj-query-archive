#!/usr/bin/env python
'''DOCSTRING'''

import ConfigParser
import os.path
from grid import Grid
from graph import Graph

config = ConfigParser.SafeConfigParser()
config.read(os.path.join('agents','conf','agents','defaults.ini'))
config.read(os.path.join('agents','conf','agents','overrides.ini'))

# runtime config values
STORECAP = config.getint('runtime', 'storeCapacity')
MAP_TYPE=config.get('runtime', 'mapType')

# dev config values
TRACING = config.getboolean('dev', 'tracing')

if TRACING:
    from SimPy.SimulationTrace import *
else:
    from SimPy.Simulation import *

class Agent(Process):
    '''DOCSTRING'''
    # Static (class) attributes!  Do NOT declare these in __init__ (or they
    # apparently won't be static).
    waiting=[]
    waitingFares=Store(capacity=STORECAP, initialBuffered=waiting)
    # I think this Monitor is used for ...
#    waitingFares = Store(capacity=STORECAP, initialBuffered=waiting,
#            monitored=True, monitorType=Monitor)

    # These are class attributes so that the SimPy filter functions
    # closestfare_cooperate and mixedmode_cooperate in Taxi.py can use them
    if MAP_TYPE=='grid':
        map=Grid()
    elif MAP_TYPE=='graph':
        map=Graph()

    print "MAP_TYPE:%s" % MAP_TYPE

    def __init__(self, name):
        Process.__init__(self, name)
#	if MAP_TYPE=='grid':
#            self.map=Grid()
#        elif MAP_TYPE=='graph':
#	    self.map=Graph()
#
#        print "self.map:%s" % self.map

        self.loc={}
#        self.loc['curr']=self.map.get_location()[0]
#        self.loc['dest']=self.map.get_location()[1]
        self.loc['curr']=Agent.map.get_location()[0]
        self.loc['dest']=Agent.map.get_location()[1]
        self.ts = {}    # timestamps
        self.ts['activation'] = now()
	print ('%.4f\tactivated: [(Agent %s), (location %s)]' %
			(self.ts['activation'], self.name, self.loc))

if __name__ == '__main__':
    a=Agent('Smith')
    print a.map.get_location()

