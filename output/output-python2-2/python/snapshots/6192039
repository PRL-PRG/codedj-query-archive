#!/usr/bin/env python
'''The grid module, containing the Grid class.'''

# TODO: edit this and put it in the module docstring.
#
# This module is activated if mapType is set to 'grid' in
# agents/conf/agent/defaults.ini or overrides.ini.

# agents/Grid is polymorphic with agents/Graph

import ConfigParser
import math
import os.path
import random


config = ConfigParser.SafeConfigParser()
config.read(os.path.join('agents','conf','agents','defaults.ini'))
config.read(os.path.join('agents','conf','agents','overrides.ini'))

# runtime config values
GRID_MIN = config.getint('runtime', 'gridMin')
GRID_MAX = config.getint('runtime', 'gridMax')


class Grid(object):
    '''DOCSTRING'''
    def __init__(self):
        pass


    def get_location(self):
        '''Returns a pair of points (vertices) representing a location'''
        return (self.get_point(),self.get_point())


    def get_point(self, lo=GRID_MIN, hi=GRID_MAX, length=2):
        '''Generates a two-tuple representing an (x,y) location'''
        return self.__get_vertex()


    def get_distance(self, point_a, point_b):
        '''
	Given a pair of coordinates, return the distance between them.

	The distance calculation is set in the configuration option
	distanceCalculation.  Options are straight-line distance between the
	points (the default), or driving distance.
	'''
        DC=config.get('runtime', 'distanceCalculation')
        if DC=='straightLine':    # use the hypotenuse
            return math.hypot((point_a[0]-point_b[0]), (point_a[1]-point_b[1]))
        elif DC=='drivingDistance':
            return abs(point_a[0]-point_b[0])+abs(point_a[1]-point_b[1])
        else:
            return None # error


    # private method
    def __get_vertex(self,lo=GRID_MIN,hi=GRID_MAX,length=2):
        '''[private] Returns a single (x,y) coordinate point'''
        tmp = []
        for i in range(length):
            tmp.append(random.randint(lo, hi))
        return tuple(tmp)


    # I'm no longer using this for the regular compete methods (thanks to a
    # suggestion from Dan Struthers).  If I go on to create courtesy_compete
    # methods, and rename the regular compete methods to cutthroat_compete,
    # then I'll be able to use this.  In the meantime, I'm not going to create
    # an update_location() method in graph.py.
    def update_location(self, point_a, point_b, time_delta):
        '''
	Update an Agent's current position.

	This method is usually called from Taxi.compete(), after a Taxi has
	been interrupted while en'route to a Fare.  The interruption means
	that another Taxi (the one doing the interrupting) got to the Fare
	first, and this Taxi needs to figure out where he is, so he can set
	his loc['curr'], and compete for the next Fare.
	'''
        # It doesn't happen very often, but sometimes two Taxis reach a Fare
        # at the same time.  In other words, both xdiff and ydiff are 0!  This
        # causes ZeroDivisionErrors.  But since one of them has already been
        # interrupted (which is why it's here), it is simple enough to
        # straighten out.
        #
        ax=point_a[0]
        ay=point_a[1]
        bx=point_b[0]
        by=point_b[1]
        xdiff=abs(ax-bx)
        ydiff=abs(ay-by)

        # DEBUG -- if both xdiff and ydiff are 0, then some other Taxi got to
        # the Fare just before this one did.  (Keep in mind, update_location()
        # is only called when the Taxi has been interrupted.)  So this Taxi
        # loses the Fare, and its locations are as follows:
        #
        # self.loc['curr']=self.loc['dest']
        # self.loc['dest']=()
        #
        if xdiff+ydiff==0:
            print 'This Taxi tied with another one but was interrupted!'
            return (-1,-1)

        multiplier=(time_delta*1.0)/(xdiff+ydiff)

        xdelta=round(multiplier*xdiff)
        ydelta=round(multiplier*ydiff)
        if ax>bx:
            xdelta=-xdelta
        if ay>by:
            ydelta=-ydelta

        return (int(ax+xdelta),int(ay+ydelta))


if __name__=='__main__':
    g=Grid()
