#!/usr/bin/env python
'''DOCSTRING'''

# agents/Grid is polymorphic with graphs/Graph 

#from area import Area
import ConfigParser
import math
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
	# It doesn't happen very often, but sometimes two Taxis reach a Fare
	# at the same time.  In other words, both xdiff and ydiff are 0!  This
	# causes ZeroDivisionErrors.  But since one of them has already been
	# interrupted (which is why it's here), it is simple enough to
	# straighten out.

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

    def get_distance(self, point_a, point_b):
        '''
Given a pair of coordinates, return the distance between them.

The calculation is set in the configuration option distanceCalculation.
Options are straight-line distance between the points, or driving distance.
        '''
        DC=config.get('runtime', 'distanceCalculation')
        if DC=='straightLine':    # use the hypotenuse
            dist=math.hypot((point_a[0]-point_b[0]), (point_a[1]-point_b[1]))
	    print 'dist: ', dist
	    return dist
#            return math.hypot((point_a[0]-point_b[0]), (point_a[1]-point_b[1]))
        elif DC=='drivingDistance':
            return abs(point_a[0]-point_b[0])+abs(point_a[1]-point_b[1])
        else:
            return None # error

    # maybe this one should not be publicly available - I don't think Grid
    # even needs it, so Graph should use it internally only
    def get_point(self):
        '''Return a single (x,y) coordinate point'''
	pass

if __name__=='__main__':
    g=Grid()
