#!/usr/bin/env python
'''DOCSTRING'''

from area import Area

class Grid(Area):
    '''DOCSTRING'''
    def __init__(self):
        Area.__init__(self)

    def getPoint(self):
        '''Return a single (x,y) coordinate point'''
	pass

    def getDistance(self, here, there):
        '''Return the distance between two points'''
	pass

def getdistance(dest, currentLocation=None):
    '''
Given a pair of coordinates, return the distance between them (float).

Returns the straight-line distance between the points ("as the crow
flies") if hypotenuse is True, (the default).  Otherwise, returns the
driving distance.
    '''
    # CAUTION: the compete methods do not use taxi_loc, so this is a hazard.
    curr = currentLocation or taxi_loc
    if not curr:
        print 'What am I supposed to do with an empty current location tuple??'
        print 'dest:', dest, 'curr:', curr
        stopSimulation()
        print 'more stubby!'

    DC = config.get('runtime', 'distanceCalculation')
    if DC == 'straightLine':    # use the hypotenuse
        return math.hypot((curr[0]-dest[0]), (curr[1]-dest[1]))
    elif DC == 'drivingDistance':
        return abs(curr[0]-dest[0]) + abs(curr[1]-dest[1])

    def updateLocation(self):
        '''DOCSTRING'''
	# Since Area is a separate class, I'm not sure how this method will
	# work yet.  The idea is simple enough: an Agent is travelling from
	# one place to another, and for some reason needs to calculate their
	# current location.
	#
	# Inputs include at least starting point and travel time; maybe also
	# ending point and travelling speed.
	pass

