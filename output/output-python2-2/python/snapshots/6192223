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
	# the former mkcoords from Agent class
	tmp = []
	for i in range(length):
            tmp.append(random.randint(lo, hi))
	return tuple(tmp)

    def update_location(self, start_loc, end_loc, time_delta):
        '''
Update an Agent's current position.

This method is usually called from Taxi.compete(), after a Taxi has been
interrupted while en'route to a Fare.  The interruption means that another
Taxi (the one doing the interrupting) got to the Fare first, and this Taxi
needs to figure out where he is, so he can set his loc['curr'], and compete
for the next Fare.
        '''

#        print '%s self.loc:' % self.name, self.loc
#        assert(self.loc['curr'])
#        assert(self.loc['dest'])

        print '[DEBUG] start_loc:', start_loc
        print '[DEBUG] end_loc:', end_loc
	print '[DEBUG] time_delta:', time_delta
	print '[DEBUG] abs(end_loc[0]-start_loc[0]):', abs(end_loc[0]-start_loc[0])
	print '[DEBUG] abs(end_loc[1]-start_loc[1]):', abs(end_loc[1]-start_loc[1])

	# Find the location of the Agent, given the data above: start (7,132),
	# end (129,180), diff (122,48), delta (104).  I'm thinking maybe
	# something like a ratio of (diff[0]+diff[1])/delta, but what good is
	# that?
	#
	# The only difficulty is figuring out how far to move in the
	# x-direction, and how far to move in the y-direction.  And even that
	# shouldn't be too hard.
	return

#        curr_tmp = {}
#        curr_tmp['x'] = ((self.loc['curr'][0] + self.loc['dest'][0])/2)
#        curr_tmp['y'] = ((self.loc['curr'][1] + self.loc['dest'][1])/2)
#
#        self.loc['curr'] = (curr_tmp['x'], curr_tmp['y'])
#        self.loc['dest'] = ()
#        curr_tmp.clear()
#        return

    def get_point(self):
        '''Return a single (x,y) coordinate point'''
	pass

    def get_distance(self, curr, dest):
        '''
Given a pair of coordinates, return the distance between them (float).

Returns the straight-line distance between the points if hypotenuse is
True (default).  Otherwise, returns the driving distance.
        '''
        # CAUTION: the compete methods do not use taxi_loc, so this is a hazard.
#        curr = currentLocation or taxi_loc
#        if not curr:
#            print 'What am I supposed to do with an empty current location tuple??'
#            print 'dest:', dest, 'curr:', curr
#            stopSimulation()
#            print 'more stubby!'

        DC = config.get('runtime', 'distanceCalculation')
        if DC == 'straightLine':    # use the hypotenuse
            return math.hypot((curr[0]-dest[0]), (curr[1]-dest[1]))
        elif DC == 'drivingDistance':
            return abs(curr[0]-dest[0]) + abs(curr[1]-dest[1])

if __name__=='__main__':
    g=Grid()
