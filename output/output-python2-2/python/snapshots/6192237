#!/usr/bin/env python
'''DOCSTRING'''

from area import Area

class Grid(Area):
    '''DOCSTRING'''
    def __init__(self):
        Area.__init__(self)

    def get_location(self):
        '''DOCSTRING'''
	pass

    # set_location?  This and get_location are not very Pythonic.  Maybe find
    # a Python for Java programmers guide?
    def update_location(self):
        '''DOCSTRING'''
	# Since Area is a separate class, I'm not sure how this method will
	# work yet.  The idea is simple enough: an Agent is travelling from
	# one place to another, and for some reason needs to calculate their
	# current location.
	#
	# Inputs include at least starting point and travel time; maybe also
	# ending point and travelling speed.
	pass

    def get_point(self):
        '''Return a single (x,y) coordinate point'''
	pass

    def get_distance(dest, currentLocation=None):
    (self, here, there):
        '''
Given a pair of coordinates, return the distance between them (float).

Returns the straight-line distance between the points if hypotenuse is
True (default).  Otherwise, returns the driving distance.
        '''
	pass

def getdistance(dest, currentLocation=None):
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
        '''
Update the Taxi's current position.

This method is normally only called from compete(), after a Taxi has been
interrupted while en'route to a Fare.  The interruption means that another
Taxi (the one doing the interrupting) got to the Fare first, and this Taxi
needs to figure out where he is, so he can set his loc['curr'], and compete
for the next Fare.

Implementation detail: to keep things simple, I am just putting the Taxi near
the halfway point between their former current location and their destination.

NOTE: This method works under the assumption that the Taxi travels 1 unit of
the grid for each tick of the simulation's clock.  This may eventually become
a configuration setting, but it's low priority.
        '''
#        print '%s self.loc:' % self.name, self.loc
        assert(self.loc['curr'])
        assert(self.loc['dest'])

        curr_tmp = {}
        curr_tmp['x'] = ((self.loc['curr'][0] + self.loc['dest'][0])/2)
        curr_tmp['y'] = ((self.loc['curr'][1] + self.loc['dest'][1])/2)

        self.loc['curr'] = (curr_tmp['x'], curr_tmp['y'])
        self.loc['dest'] = ()
        curr_tmp.clear()
        return

