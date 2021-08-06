#!/usr/bin/env python
'''DOCSTRING'''

# agents/Grid is polymorphic with graphs/Graph 

#from area import Area
import ConfigParser
import os.path
import random

config = ConfigParser.SafeConfigParser()
config.read(os.path.join('agents', 'defaults.ini'))
config.read(os.path.join('agents', 'overrides.ini'))

# runtime config values
GRID_MIN = config.getint('runtime', 'gridMin')
GRID_MAX = config.getint('runtime', 'gridMax')

#class Grid(Area):
class Grid(object):
    '''DOCSTRING'''
    def __init__(self):
        pass
#        Area.__init__(self)

    def get_location(self, lo=GRID_MIN, hi=GRID_MAX, length=2):
	'''Generates two-tuples representing locations'''
	# the former mkcoords from Agent class - this needs a more descriptive
	# name
	tmp = []
	for i in range(length):
            tmp.append(random.randint(lo, hi))
	return tuple(tmp)

    def update_location(self, point_a, point_b, time_delta):
        '''
Update an Agent's current position.

This method is usually called from Taxi.compete(), after a Taxi has been
interrupted while en'route to a Fare.  The interruption means that another
Taxi (the one doing the interrupting) got to the Fare first, and this Taxi
needs to figure out where he is, so he can set his loc['curr'], and compete
for the next Fare.
        '''
        # It's not likely to happen very often, but there could be a tie in one or
        # both tuples.  It doesn't matter though, since the x (or y) diff just
        # goes to zero in that case.
        ax=point_a[0]
        ay=point_a[1]
        bx=point_b[0]
        by=point_b[1]

        xdiff=abs(ax-bx)
        ydiff=abs(ay-by)
        multiplier=(time_delta*1.0)/(xdiff+ydiff)

        xdelta=round(multiplier*xdiff)
        ydelta=round(multiplier*ydiff)
        if ax>bx:
            xdelta=-xdelta
        if ay>by:
            ydelta=-ydelta

        return (int(ax+xdelta),int(ay+ydelta))

    def get_distance(self, curr, dest):
        '''
Given a pair of coordinates, return the distance between them.

The calculation is set in the configuration option distanceCalculation.
Options are straight-line distance between the points, or driving distance.
        '''
        DC = config.get('runtime', 'distanceCalculation')
        if DC == 'straightLine':    # use the hypotenuse
            return math.hypot((curr[0]-dest[0]), (curr[1]-dest[1]))
        elif DC == 'drivingDistance':
            return abs(curr[0]-dest[0]) + abs(curr[1]-dest[1])
        else:
            return None # error

    # maybe this one should not be publicly available - I don't think Grid
    # even needs it, so Graph should use it internally only
    def get_point(self):
        '''Return a single (x,y) coordinate point'''
	pass

if __name__=='__main__':
    g=Grid()
