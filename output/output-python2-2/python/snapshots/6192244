#!/usr/bin/env python
'''DOCSTRING'''

import ConfigParser
import os.path
import random

config = ConfigParser.SafeConfigParser()
config.read(os.path.join('agents', 'defaults.ini'))
config.read(os.path.join('agents', 'overrides.ini'))

# dev config values
TRACING = config.getboolean('dev', 'tracing')

# runtime config values
GRID_MIN = config.getint('runtime', 'gridMin')
GRID_MAX = config.getint('runtime', 'gridMax')
STORECAP = config.getint('runtime', 'storeCapacity')

if TRACING:
    from SimPy.SimulationTrace import *
else:
    from SimPy.Simulation import *


class Agent(Process):
    '''DOCSTRING'''
    # Static (class) attributes!  Do NOT declare these in __init__ (or they
    # apparently won't be static).
    waiting=[]
    waitingFares = Store(capacity=STORECAP, initialBuffered=waiting)
    # I think this Monitor is used for ...
#    waitingFares = Store(capacity=STORECAP, initialBuffered=waiting,
#            monitored=True, monitorType=Monitor)

    def __init__(self, name):
        Process.__init__(self, name)
        self.loc = {}
        self.loc['curr'] = self.mkcoords()
        self.loc['dest'] = ()
        self.ts = {}    # timestamps
        self.ts['activation'] = now()

    # TODO this might belong in its own class, so we can choose a coordinate
    # system (grid or graph) at config time.
    #
    # Late note: a TK for either grid or graph might be the better choice.
    def mkcoords(self, lo=GRID_MIN, hi=GRID_MAX, length=2):
        '''Generates two-tuples representing locations'''
        tmp = []
        for i in range(length):
            tmp.append(random.randint(lo, hi))
        return tuple(tmp)

if __name__ == '__main__':
    a = Agent('Agent Smith')
    print a.mkcoords()
