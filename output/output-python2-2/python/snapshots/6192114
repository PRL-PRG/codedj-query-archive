#!/usr/bin/env python
'''DOCSTRING'''

from operator import itemgetter # itemgegger is new in Python 2.4
from agent import Agent
import ConfigParser
import os.path

config = ConfigParser.SafeConfigParser()
config.read(os.path.join('agents','conf','agents','defaults.ini'))
config.read(os.path.join('agents','conf','agents','overrides.ini'))

# dev config values
TRACING = config.getboolean('dev', 'tracing')
DEBUG = config.getboolean('dev', 'debug')
SET_TRACE = config.getboolean('dev', 'setTrace') # for interactive debugging

# runtime config values
SIMTIME = config.getint('runtime', 'simulationTime')
TAXI_RANGE_LOW = config.getfloat('runtime', 'taxiRangeLow')
TAXI_RANGE_MID = config.getfloat('runtime', 'taxiRangeMedium')
TAXI_RANGE_HI = config.getfloat('runtime', 'taxiRangeHigh')
GRID_MIN = config.getint('runtime', 'gridMin')
GRID_MAX = config.getint('runtime', 'gridMax')
SIMTYPE = config.get('runtime', 'simType')

if TRACING:
    from SimPy.SimulationTrace import *
else:
    from SimPy.Simulation import *

if SET_TRACE:
    import pdb

# This is a 2-tuple used by the filter functions.  Looks like (23, 61).
taxi_loc = ()


class Taxi(Agent):
    '''DOCSTRING'''
    def __init__(self, name, np): # negotiation protocol
        Agent.__init__(self, name)
        self.np=np
	self.lostFares=[]

    def cooperate(self):
        '''
Coordinate pickups with other Taxis.  A SimPy PEM.

This is the PEM for cooperative negotiation.  In this simulation, Taxis choose
a Fare for pickup, and take a reference to the Fare at acknowledgment.  The
Taxi effectively locks out other Taxis from competing for that Fare by
removing it from the queue of waiting Fares just before yield'ing for the ride
to the Fare.

Contrast with the compete() method.
        '''
        global taxi_loc
        while True:
            if len(Agent.waitingFares.theBuffer) > 0:
                if DEBUG:
                    print
		    print("%.4f %s is looking for an eligible Fare:" %
				    self.name)
		    print(".. waitingFares (pre) ", [x.name for x in
			    Agent.waitingFares.theBuffer])
                taxi_loc = self.loc['curr']

                # Choose a Fare
                if self.np == 'FIFO':
                    yield get, self, Agent.waitingFares, 1
                elif self.np == 'closestfare':
                    numAgents=len(Agent.waitingFares.theBuffer)
                    yield get, self, Agent.waitingFares, closestfare_cooperate
                    #yield get, self, Agent.waitingFares, self.closestfare_cooperate
                elif self.np == 'mixedmode':
                    yield get, self, Agent.waitingFares, mixedmode_cooperate
                    #yield get, self, Agent.waitingFares, self.mixedmode_cooperate
                else:
                    print "Something broke in the negotiation protocol!"
                if DEBUG:
                    print(".. waitingFares (post)", [x.name for x in
			    Agent.waitingFares.theBuffer])
                    assert len(self.got) == 1
		print("%.4f\t%s chose " % (now(), self.name), [x.name for x
			in self.got])

# Found this line in an old printout that may be "newer" than this code.  It's
# redundant in the unified project.
#                print 'taxi_loc before', taxi_loc
#                print 'setting old printout code ...'
#                taxi_loc = self.loc['curr']
#                print 'taxi_loc after', taxi_loc

                fareBeingDriven=self.got[0]

                # Drive to Fare
                #drive_dist = getdistance(fareBeingDriven.loc['curr'], taxi_loc)
                drive_dist=self.map.get_distance(fareBeingDriven.loc['curr'], taxi_loc)

                if DEBUG:
		    print("%.4f\t%s driving to %s" % (now(), self.name,
			    fareBeingDriven.name))
                yield hold, self, drive_dist

                # Pick up Fare
                self.loc=fareBeingDriven.loc     # tuple
                if DEBUG:
		    print("%.4f\t%s arrives to pick up %s" % (now(),
			    self.name, fareBeingDriven.name))

                # Drive to Fare's destination
                drive_dist=self.map.get_distance(self.loc['dest'], self.loc['curr'])

                if DEBUG:
		    print("%.4f\t%s driving to %s's destination" % (now(),
			    self.name, fareBeingDriven.name))
                yield hold, self, drive_dist

                # Drop off Fare
                self.loc['curr'] = fareBeingDriven.loc['dest']
                self.loc['dest'] = ()

                if DEBUG:
		    print("%.4f\t%s dropping off %s" % (now(), self.name,
			    fareBeingDriven.name))
                fareBeingDriven.doneSignal.signal(self.name)
            else:
		print("%.4f\tINFO: %s: There are no eligible Fares for this Taxi." %
				(now(), self.name))

                # Throttle back the flood of messages.
                #
                # NOTE: I'm using a simple 'yield hold <small-number>' here
                # because there are two main events that can occur, and the
                # Taxi should be able to respond to them promptly.  The first
                # is the arrival of a new Fare into the buffer.  The second is
                # a Fare already in the queue that becomes eligible for
                # inspection by the Taxi.
                yield hold, self, 2


    def compete(self):
        '''
Compete for Fares against other Taxis.  A SimPy PEM.

This method differs from cooperate() in a couple important ways.  First is
that the Taxis are currently competing for Fares in an "every man for himself"
sort of way.  Second is that compete uses the same negotiation protocols, but
they work a little differently.  Instead of taking a reference to a Fare, and
removing it from the queue of eligible Fares as soon as it is identified,
these Taxis need to reach the Fare before they can claim it as their own.
Also, the Taxis do not communicate with each other during the competition.
They mainly communicate with each other when one of them claims a Fare, and
alerts the others that that Fare is no longer available.

Implementation detail: Every Taxi that competes for a particular Fare goes
into that Fare's competeQ.  Then the Taxi yields for drive_dist time.  If the
yield expires, that Taxi got there first, and wins the Fare!  The winning Taxi
pulls the Fare from the queue, and interrupts all other members of that Fare's
competeQ.   The remaining Taxis call self.interruptReset(), and go off to
compete for another Fare.

Because the Fares stay in the queue until someone "wins" the right to go pick
them up, latecomers should have a shot at them too.  If they are closer, they
may get the Fare, even though others are already competing for that Fare.

Contrast with the cooperate() method.
        '''

        global taxi_loc	# maybe I should use class G

        while True:
            if len(Agent.waitingFares.theBuffer) > 0:
                if DEBUG:
                    my_curr_pre=self.loc['curr']
                    my_dest_pre=self.loc['dest']

                # Choose a Fare
                if self.np=='FIFO':
                    targetFare=Agent.waitingFares.theBuffer[0]
                elif self.np=='closestfare':
                    numAgents=len(Agent.waitingFares.theBuffer)
                    targetFare=self.closestfare_compete()
                    if DEBUG:
                        assert(numAgents==len(Agent.waitingFares.theBuffer))
			print("%s targetFare closestfare %s" % (self.name,
				targetFare.name))
                elif self.np=='mixedmode':
                    numAgents=len(Agent.waitingFares.theBuffer)
                    targetFare=self.mixedmode_compete()
                    if DEBUG:
                        assert(numAgents==len(Agent.waitingFares.theBuffer))
                    if targetFare:
                        if DEBUG:
			    print("%s targetFare closestfare %s" % (self.name,
				    targetFare.name))
                    else:
                        # Not a lot I can do here, since I don't have a Fare
                        # to compete for.  I should never get here, so this
                        # should be an Error.  For now I'll just throttle back
                        # the flood of messages and move on.
                        print("%.4f\tINFO: There are no eligible Fares for %s" %
					(now(), self.name))
                        yield hold, self, 2
                        continue
		# End choose a Fare

                if DEBUG:
                    assert(self.loc['curr']==my_curr_pre)
                    assert(self.loc['dest']==my_dest_pre)

                # update destination unconditionally
                self.loc['dest']=targetFare.loc['curr']
		print('%.4f DEBUG: %s calling get_distance [1]' % (now(),
			self.name))
                drive_dist=self.map.get_distance(self.loc['dest'], self.loc['curr'])
		# This cannot happen.  I need to figure out how to remove
		# these Graph dead spots before they are added to the
		# database.
		if drive_dist is None: # no path from curr to dest
		    print("INFO: no path from %s to %s" % (self.name,
			    targetFare.name))
		    print("%.4f\t%s is back in service" % (now(), self.name))
		    self.loc['curr']=targetFare.loc['dest']
		    self.loc['dest']=()
		    continue
#                    print "TEMP DEBUG Something's broken [1]!"
#		    import sys; sys.exit(1)

                # DEBUG
#		print "DEBUG (drive_dist)", drive_dist
#		print "DEBUG (self.loc)", self.loc['dest'], self.loc['curr']

                # Drive to Fare, try to get there first
#		print("%.4f\t%s competing for %s (drive time %.4f)" %
		print("%.4f\t%s competing for %s (drive time %s)" %
				(now(), self.name, targetFare.name, drive_dist))
                yield hold, self, drive_dist

		print("%.4f\t%s arrives at %s's location (drive time %.4f)" %
				(now(), self.name, targetFare.name, drive_dist))

		# Taxi has now driven to the Fare's pickup location.  Update
		# its location to that of the Fare its competing for.  If Fare
		# is still here, self.loc['dest'] is accurate.  Otherwise,
		# need to reset it after querying the filter function for its
		# new Fare.  Set the global taxi_loc so we can use this value
		# in fare_is_here().
                self.loc['curr']=taxi_loc=targetFare.loc['curr']
		self.loc['dest']=()

                # TEMP DEBUG
		print("%.4f\t%s trying to get %s" % (now(), self.name,
			targetFare.name))

		# HACK HACK - choose a random small float wait time for
		# reneging.  This is a bit of a hack to get around the fact
		# that ...
		yield_time=random.random()/100
                yield (get, self, Agent.waitingFares, fare_is_here), (hold, self, yield_time)

		# HACK HACK - "absorb" the renege (if it occurs) from the
		# yield above.  The idea is to yield for some semi-random time
		# longer than the length of that yield, to ensure that the
		# renege time has elapsed.
		yield hold, self, yield_time+random.random()/100

                # Got the Fare
		if len(self.got)>0:
		    print("%.4f\t%s picked up %s" % (now(), self.name,
			    self.got[0].name))
		    self.loc['dest']=targetFare.loc['dest']
		    print('%.4f DEBUG: %s calling get_distance [2]' % (now(),
			    self.name))
                    drive_dist=self.map.get_distance(self.loc['dest'], self.loc['curr'])
		    if drive_dist is None: # no path from curr to dest
			print("INFO: no path from %s to %s" % (self.name,
				self.got[0].name))
		        print("%.4f\t%s is back in service" % (now(), self.name))
#		        self.loc['curr']=self.got[0].loc['dest']
#		        self.loc['dest']=()
			continue
#                        print "TEMP DEBUG Something's broken [2]!"
#		        import sys; sys.exit(1)

                    # Drive to Fare's destination, then continue
#		    # Use %s instead of %.4f for drive_dist in case it's None
#		    print("%.4f\t%s driving to %s's destination (drive time %s)" %
		    print("%.4f\t%s driving to %s's destination (drive time %.4f)" %
				    (now(), self.name, targetFare.name, drive_dist))
                    yield hold, self, drive_dist

                    # BUGBUG this was missing from compete() !!
                    targetFare.doneSignal.signal(self.name)

		    print("%.4f\t%s is back in service" % (now(), self.name))
		    self.loc['curr']=targetFare.loc['dest']
		    self.loc['dest']=()
		    continue

                # Too late, Fare already picked up
		else:

                    # DEBUG
		# HACK HACK - "absorb" the renege (if it occurs) from the
		# yield above.  The idea is to yield for some semi-random time
		# longer than the length of that yield, to ensure that the
		# renege time has elapsed.
#		    yield hold, self, yield_time+random.random()/100

		    print("%.4f\t%s lost %s" % (now(), self.name, targetFare.name))
		    print("%.4f\t%s back in service" % (now(), self.name))
		    self.loc['dest']=()

            else:
		print("%.4f\tINFO: There are no eligible Fares for %s" %
				(now(), self.name))
                # Throttle back the flood of messages.
                #
                # NOTE: I'm using a simple 'yield hold <small-number>' here
                # because there are two main events that can occur, and the
                # Taxi should be able to respond to them promptly.  The first
                # is the arrival of a new Fare into the buffer.  The second is
                # a Fare already in the queue that becomes eligible for
                # inspection by the Taxi.
                yield hold, self, 2


    def closestfare_compete(self, not_a_magic_buffer=None):
        '''
        TODO UPDATE DOCSTRING

Filter: return the Fare that is geographically closest to the calling Taxi.

NOTE: The Fare is returned as a single-element list, because that (a list) is
what SimPy's yield is expecting.  This is a filter function for the Store, and
should not be called directly.  This is the second of the Taxi's three
negotiation protocols.
        '''
        tmp=[]
        if not not_a_magic_buffer:
            not_a_magic_buffer=Agent.waitingFares.theBuffer
        if DEBUG:
            if buffer==Agent.waitingFares.theBuffer:
                print 'the buffers are equal'
            if buffer is Agent.waitingFares.theBuffer:
                print 'the buffers are the same'
        if not len(not_a_magic_buffer)>0:
            print 'Buffer is empty!'
            return
        for fare in not_a_magic_buffer:

            # DEBUG
	    #
	    # Try to figure out why graph get_distance() sometimes triggers a
	    # TypeError.  It's here:
	    #   for lon,lat in self.mkgraph.shortest_path(here,there):
            # TypeError: 'int' object is not iterable
	    #
	    # HEY!  Look what jumped out!  The Taxi and Fare current locations
	    # are the same!  Zzzzzap!!!  Something's broken!  (And my note
	    # about forgetting to reset a loc seems to be pointing in the
	    # right direction.)
	    #
            #((u'-149169424', u'64331706'), (u'-149169424', u'64331706'))
            #Too short?  Maybe I forgot to reset a loc?
	    #
            print(self.name, self.loc['curr'], fare.loc['curr'])

#	    print('%.4f DEBUG: (%s, %s) calling get_distance [3]' % (now(),
#		    self.name, fare.name))
            d=self.map.get_distance(fare.loc['curr'], self.loc['curr'])

	    # TODO need to make sure all Fares are inspected, even if some
	    # have to be skipped or removed from the simulation.
	    #
	    # Late note: this should no longer happen, since I added
	    # get_connected() to tigerutils.  Confirmed?
            if d is None: # no path from curr to dest
                print("INFO: no path from %s to %s" % (self.name, fare.name))
		print("%s is an invalid Fare and %s will not compete for it" %
				(fare.name, self.name))
#		d=-1
#                print("%.4f\t%s is back in service" % (now(), self.name))
#		        self.loc['curr']=self.got[0].loc['dest']
#		        self.loc['dest']=()
#                continue
            else:
                if DEBUG:
		    print("Distance from %s to %s: %.4f" % (self.name,
			    fare.name, d))
                tmp.append((fare, d))
#            if DEBUG:
#                print("Distance from %s to %s: %.4f" % (self.name, fare.name, d))
#            tmp.append((fare, d))
        tmp2=sorted(tmp, key=itemgetter(1))

	# TODO Arrgh!  It looks like sometimes there is NOTHING in the tmp
	# queue.
	print("DEBUG len(tmp): %d, len(tmp2): %d" % (len(tmp), len(tmp2)))
        result=map(itemgetter(0), tmp2)[0]

        # Critical difference between the competition and cooperative
        # closestfares: cooperative cf returns the Fare, and removes it from
        # waitingFares.theBuffer immediately.  Competition cf just returns the
        # Fare, without taking it out of the buffer until later.  Note also,
        # this method returns a single Fare, not (a list containing) a single
        # Fare.
        return result


    # Third of the Taxi's three negotiation protocols.  See notes in
    # ~/finalproject/agents/docs/daily_status/2007/04/08.txt
    def mixedmode_compete(self, not_a_magic_buffer=None):
        '''
Find the Fare with the lowest aggregate score.

The Taxi uses a combination of the Fare's time in the waitingFares buffer plus
their distance from the Taxi to determine which Fares to inspect.  If there
are any Fares in this list, then the one with the lowest score (cost) is
returned to the caller.

If the list is empty, in other words, if there are no Fares which meet the
time and space (distance) requirements of this Taxi, the Taxi goes into a
getQ, and stays there until at least one suitable Fare comes along.
Fortunately, there don't seem to be any restrictions from SimPy on when a Taxi
can get out of the queue.  It's just a matter of satisfying the filter
function.

NOTE: This method is explicitly not a SimPy filter function.  It behaves
similarly, but is used for competition only, which has different requirements,
since only the first Taxi to reach the Fare may remove the Fare from the
queue, and all others have to renege out.
        '''

        def __printFareDetails(taxiRange):
            '''DOCSTRING'''
            PRETTY_PRINT = config.getboolean('runtime', 'prettyPrint')
            print
            if PRETTY_PRINT:
                # TODO [lopri] This is unclear.  For example, what does range
                # even mean in this context?  It should refer to the range
                # that a Taxi is looking for Fares.  I am using it to mean the
                # distance (reach) of the Fare's broadcast.  Sort this out.
                #
                # I need to use as many words as it takes
                # to_make_things_clear.  I can shorten them later, but not
                # until things are simpler.
                print "  %.4f\tFare %s broadcast stats:" % (now(), fare.name)
                print "    range: %s (based on Fare's time in queue)" % broadcastRange
                print "    time in queue: %.4f" % TIQ
                print "    distance from Taxi: %.4f" % d
                print("    Taxi's range: %.2f (= TAXI_RANGE_XXX * GRID_MAX)" %
				(taxiRange*GRID_MAX))
                print "    weight: %.4f\t(= SIMTIME - TIQ)" % weight
                print "    score: %.4f\t(= weight + distance)" % score
            else:
                print("  %.4f\t%s's broadcast stats: range: %s, time in queue: %.4f, Taxi's range: %.2f, distance from Taxi: %.4f, weight: %.4f, score: %.4f" %
                        (now(), fare.name, broadcastRange, TIQ,
				taxiRange*GRID_MAX, d, weight, score))

        # start of mixedmode_compete()
        tmp = []
        if not not_a_magic_buffer:
            not_a_magic_buffer = Agent.waitingFares.theBuffer
        VERBOSE = config.getboolean('runtime', 'verbose')
        # I should never hit this, but it can't hurt to leave it in.
        if not len(Agent.waitingFares.theBuffer) > 0:
            print 'Buffer is empty!'
            return
        for fare in not_a_magic_buffer:
            TIQ = (now() - fare.ts['mkreq'])
            d = self.map.get_distance(fare.loc['curr'], self.loc['curr'])

            # TODO [eventually] put the weight and scoring routines into a
            # config file.  Major TK.
            weight = SIMTIME - TIQ
            score = weight + d
            f_time_ratio = TIQ/(SIMTIME*1.0)    # force integer division

            # Figure out which category the Fare goes in.  The first part is
            # all about TIME!
            #
            # If Fare has been in the queue long enough for a Global
            # broadcast, calculate score and append to list for further
            # consideration.
            if TAXI_RANGE_MID < f_time_ratio <= TAXI_RANGE_HI:
                broadcastRange = 'GLOBAL'
                if VERBOSE: __printFareDetails(TAXI_RANGE_HI)
		print(".. Pushing (%s, score %.4f) onto list" % (fare.name,
			score))
                tmp.append((fare, score))

            # Has the Fare been in the queue long enough to be a Regional?
            elif TAXI_RANGE_LOW < f_time_ratio <= TAXI_RANGE_MID:
                broadcastRange = 'REGIONAL'
                if VERBOSE: __printFareDetails(TAXI_RANGE_MID)

                # Is Fare close enough for Taxi to pickup?
                #
                # If distance from Taxi to Fare is less than or equal to
                # (TAXI_RANGE_MID * GRID_MAX), then broadcast is received by
                # Taxi, and Fare gets added to the queue.
                if d <= (TAXI_RANGE_MID * GRID_MAX):
		    print(".. Pushing (%s, score %.4f) onto list" %
				    (fare.name, score))
                    tmp.append((fare, score))
                else:
                    # Fare's been around long enough for it's broadcast to be
                    # Regional, but this Taxi is not in range.  Break out of
                    # the loop, and evaluate the next Fare.
                    if DEBUG:
                        print '  %s:' % fare.name,
                        print 'regional broadcast, but Fare is out of range:',
			print("(distance) %.1f > (range) %.1f" % (d,
				TAXI_RANGE_MID * GRID_MAX))
                    if (d < (TAXI_RANGE_LOW * GRID_MAX)):
                        print 'Regional broadcast is broken!'
                    continue

            # It's a local broadcast
            else:
                broadcastRange = 'LOCAL'
                if VERBOSE: __printFareDetails(TAXI_RANGE_LOW)
                # The Fare has only been in the queue long enough to be a Local
                if d <= (TAXI_RANGE_LOW  * GRID_MAX):
		    print(".. Pushing (%s, score %.4f) onto list" %
				    (fare.name, score))
                    tmp.append((fare, score))
                else:
                    # Local broadcast, but this Taxi is not in range.  Break
                    # out of the loop, and evaluate the next Fare.
                    if DEBUG:
                        print '  Fare %s:' % fare.name,
                        print 'local broadcast, but Fare is out of range:',
			print("(distance) %.1f > (range) %.1f" % (d,
				TAXI_RANGE_LOW * GRID_MAX))
                    if (d < TAXI_RANGE_LOW * GRID_MAX):
                        print 'Local broadcast is broken!'
                    continue

        # If I hit the continue every time, it's easy to get here and have
        # nothing in tmp2.  I'm not sure this is right, but it seems to me
        # that if there's nothing here, the only thing left to do is return.
        if len(tmp) == 0:
            if DEBUG:
                print "%.4f\tINFO:" % now(),
                print 'There are no eligible Fares for this Taxi.  Entering getQ...'
#                print ("%.4f\tINFO: There are no eligible Fares for %s" %
#				(now(), self.name))
            return
        tmp2 = sorted(tmp, key=itemgetter(1))
        result = map(itemgetter(0), tmp2)[0]
        # Borrowed from the bottom of closestfare.  Applies here as well.
        #
        # Critical difference between the competition and cooperative
        # closestfares: cooperative cf returns the Fare, and removes it from
        # waitingFares.theBuffer immediately.  Competition cf just returns the
        # Fare, without taking it out of the buffer until later.  Note also,
        # this method returns a single Fare, not (a list containing) a single
        # Fare.
        return result


def closestfare_cooperate(buffer):
    '''
Filter: return the Fare that is geographically closest to the calling Taxi.

Implementation detail: I think the name 'buffer' is mentioned in the
documentation as being a special trigger for SimPy, to ensure "magic
behavior".

NOTE: The Fare is returned as a single-element list, because that (a list) is
what SimPy's yield is expecting.  This is a filter function for the Store, and
should not be called directly.  This is the second of the Taxi's three
negotiation protocols.
    '''
    tmp=[]
    if not len(Agent.waitingFares.theBuffer) > 0:
        print 'Buffer is empty!'
        return
    if DEBUG:
        if buffer==Agent.waitingFares.theBuffer:
            print 'the buffers are equal'
        if buffer is Agent.waitingFares.theBuffer:
            print 'the buffers are the same'
    for fare in buffer:
        d=Agent.map.get_distance(fare.loc['curr'], taxi_loc)
        if DEBUG:
            print("Distance from Taxi to Fare %s: %.4f" % (fare.name, d))
        tmp.append((fare, d))
    tmp2=sorted(tmp, key=itemgetter(1))
    result=map(itemgetter(0), tmp2)[0]
    return [result]


# Third of the Taxi's three negotiation protocols.  See notes in
# ~/finalproject/agents/docs/daily_status/2007/04/08.txt
def mixedmode_cooperate(buffer):
    '''
Filter: find the Fare with the lowest aggregate score.

The Taxi uses a combination of the Fare's time in the waitingFares buffer plus
their distance from the Taxi to determine which Fares to inspect.  If there
are any Fares in this list, then the one with the lowest score (cost) is
returned to the caller.

If the list is empty, in other words, if there are no Fares which meet the
time and space (distance) requirements of this Taxi, the Taxi goes into a
getQ, and stays there until at least one suitable Fare comes along.
Fortunately, there don't seem to be any restrictions from SimPy on when a Taxi
can get out of the queue.  It's just a matter of satisfying the filter
function.

NOTE: The Fare is returned as a single-element list, because that's what
SimPy's yield is expecting.  This is a filter function for the Store, and
should not be called directly.  This is the third of the Taxi's three
negotiation protocols.
    '''
    def __printFareDetails(taxiRange):
        '''DOCSTRING'''
        PRETTY_PRINT = config.getboolean('runtime', 'prettyPrint')
        print
        if PRETTY_PRINT:
            # TODO [lopri] This is unclear.  For example, what does range even
            # mean in this context?  It should refer to the range that a Taxi
            # is looking for Fares.  I am using it to mean the distance
            # (reach) of the Fare's broadcast.  Sort this out.
            #
            # I need to use as many words as it takes to_make_things_clear.  I
            # can shorten them later, but not until things are simpler.
            print "  %.4f\tFare %s broadcast stats:" % (now(), fare.name)
            print "    range: %s (based on Fare's time in queue)" % broadcastRange
            print "    time in queue: %.4f" % TIQ
            print "    distance from Taxi: %.4f" % d
	    print("    Taxi's range: %.2f (= TAXI_RANGE_XXX * GRID_MAX)" %
			    (taxiRange*GRID_MAX))
            print "    weight: %.4f\t(= SIMTIME - TIQ)" % weight
            print "    score: %.4f\t(= weight + distance)" % score
        else:
            print("  %.4f\tFare %s broadcast stats: range: %s, time in queue: %.4f, Taxi's range: %.2f, distance from Taxi: %.4f, weight: %.4f, score: %.4f" %
			    (now(), fare.name, broadcastRange, TIQ,
				    taxiRange*GRID_MAX, d, weight, score))

    # start of mixedmode_cooperate()
    tmp = []
    VERBOSE = config.getboolean('runtime', 'verbose')
    # I should never hit this, but it can't hurt to leave it in.
    if not len(Agent.waitingFares.theBuffer) > 0:
        print 'Buffer is empty!'
        return
    for fare in buffer:
        TIQ = (now() - fare.ts['mkreq'])
        d = Agent.map.get_distance(fare.loc['curr'], taxi_loc)
        # TODO [eventually] put the weight and scoring routines into a config
        # file.  Major TK.
        weight = SIMTIME - TIQ
        score = weight + d
        f_time_ratio = TIQ/(SIMTIME*1.0)    # force integer division

        # Figure out which category the Fare goes in.  The first part is all
        # about TIME!
        #
        # If Fare has been in the queue long enough for a Global broadcast,
        # calculate score and append to list for further consideration.
        if TAXI_RANGE_MID < f_time_ratio <= TAXI_RANGE_HI:
            broadcastRange = 'GLOBAL'
            if VERBOSE: __printFareDetails(TAXI_RANGE_HI)
	    print(".. Pushing (%s, score %.4f) onto list" % (fare.name,
		    score))
            tmp.append((fare, score))

        # Has the Fare been in the queue long enough to be a Regional?
        elif TAXI_RANGE_LOW < f_time_ratio <= TAXI_RANGE_MID:
            broadcastRange = 'REGIONAL'
            if VERBOSE: __printFareDetails(TAXI_RANGE_MID)

            # Is Fare close enough for Taxi to pickup?
            #
            # If distance from Taxi to Fare is less than or equal to
            # (TAXI_RANGE_MID * GRID_MAX), then broadcast is received by Taxi,
            # and Fare gets added to the queue.
            if d <= (TAXI_RANGE_MID * GRID_MAX):
		print(".. Pushing (%s, score %.4f) onto list" % (fare.name,
			score))
                tmp.append((fare, score))
            else:
                # Fare's been around long enough for it's broadcast to be
                # Regional, but this Taxi is not in range.  Break out of the
                # loop, and evaluate the next Fare.
                if DEBUG:
                    print '  %s:' % fare.name,
                    print 'regional broadcast, but Fare is out of range:',
		    print("(distance) %.1f > (range) %.1f" % (d,
			    TAXI_RANGE_MID * GRID_MAX))
                if (d < (TAXI_RANGE_LOW * GRID_MAX)):
                    print 'Regional broadcast is broken!'
                continue

        # It's a local broadcast
        else:
            broadcastRange = 'LOCAL'
            if VERBOSE: __printFareDetails(TAXI_RANGE_LOW)
            # The Fare has only been in the queue long enough to be a Local
            if d <= (TAXI_RANGE_LOW  * GRID_MAX):
		print(".. Pushing (%s, score %.4f) onto list" % (fare.name,
			score))
                tmp.append((fare, score))
            else:
                # Local broadcast, but this Taxi is not in range.  Break out
                # of the loop, and evaluate the next Fare.
                if DEBUG:
                    print '  %s:' % fare.name,
                    print 'local broadcast, but Fare is out of range:',
		    print("(distance) %.1f > (range) %.1f" % (d,
			    TAXI_RANGE_LOW * GRID_MAX))
                if (d < TAXI_RANGE_LOW * GRID_MAX):
                    print 'Local broadcast is broken!'
                continue

    # If I hit the continue every time, it's easy to get here and have nothing
    # in tmp2.  I'm not sure this is right, but it seems to me that if there's
    # nothing here, the only thing left to do is return.
    if len(tmp) == 0:
        if DEBUG:
            print "%.4f\tINFO:" % now(),
            print "There are no eligible Fares for this Taxi.  Entering getQ..."
        return
    tmp2 = sorted(tmp, key=itemgetter(1))
    result = map(itemgetter(0), tmp2)[0]
    return [result]


def fare_is_here(buffer):
    '''
Filter: if there is a Fare at this location, return it, else None.
    '''
    tmp=[]
    for fare in buffer:
        # return first Fare at taxi_loc or None
        if fare.loc['curr']==taxi_loc:
            tmp.append(fare)
	    break
    return tmp


if __name__ == '__main__':
    t = Taxi('Terence', 'FIFO')
    #t.cooperate()
    t.compete()
    print "updating negotiation protocol to 'closestfare' and running again"
    t.np = 'closestfare'
    #t.cooperate()
    t.compete()
    # TODO [eventually] add in the rest of the simulation runs

