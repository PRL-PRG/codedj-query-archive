#!/usr/bin/env python
'''
The graph module, containing the logic to run geographic simulations.

This module is activated if mapType is set to 'graph' in
agents/conf/agent/defaults.ini or overrides.ini.  The graph simulations need
additional preparation that the grid simulations do not.  The bulk of the work
is done by the tigerutils module, and used by the graph module.
'''

# agents/Graph is polymorphic with agents/Grid

import sys
import tigerutils


class Graph(object):
    '''DOCSTRING'''
    def __init__(self):

        # greetings and confirmation
        greeting="""
Greetings!

Before running a geographical Agent simulation, we need to choose an area.
I'll present a list of states and territories from the U.S. Census Bureau, and
ask you to choose one.  Then I'll present a list of counties in that state,
and again ask you to choose one of them.  At that time, you can choose whether
to use the entire county for the simulation, or a specific ZIP code, which is
generally a smaller area.

Note 1: The first time a new county is chosen, the file will be downloaded
  from www.census.gov.  Subsequent runs of the simulation on the same county
  will reuse the downloaded data to save bandwidth and preparation time.

Note 2: The smallest data file may be Denali county, Alaska: state code 02,
  county code 068.  Some of the ZIP codes in this county are too small to use
  for the map, with only have 3 or 4 disconnected nodes, but it's convenient
  for doing a quick check that everything works end-to-end.
"""

        print greeting
        print "Run the geographic simulation?"

        ui=tigerutils.UserInput()
        while True:
            confirm=ui.getDigit(1,1,"(1) continue (2) quit: ")
            if confirm=='1':
                break
            elif confirm=='2':
                print "Exiting."
                sys.exit(0)
            else:
                continue # this is redundant but explicit

        # The rest of this is right out of graphs_driver.py.  Many of them are
        # not intended for use outside of this "constructor", so they are not
        # instance variables (prepended with 'self.').
        #
        # [DONE] choose the FIPS county file
        fips=tigerutils.GetFips()
        fips.getSelection()

        # [DONE] download it
        fips.getFipsZipFile()

        # [DONE] extract it, copy the parts we need, and clean up the rest
        pff=tigerutils.ProcessFipsFiles()
        pff.unzip()
        pff.export()
        pff.cleanup()

        # [DONE] munge the raw data into a more database-friendly format
        rm=tigerutils.RunMungers()
        rm.process()

        # [DONE] query the user for which database engine to use
        ui=tigerutils.UserInput()
        ui.getDbEngine()

        # [RT1 DONE] create the database and add schema
        db=tigerutils.CreateDatabase()

        # [RT1 DONE] parse munged file and create record data from it
        loaddb=tigerutils.LoadDatabase()

        # [DONE] show the user all the ZIP codes for the chosen county and
        # query the user for which to use (or None for all)
        self.query=tigerutils.QueryDatabase()
        self.query.chooseGraphArea()

        # [DONE] plot the chosen area
        self.mkgraph=tigerutils.MakeGraph()
        self.mkgraph.makeGraph()

        print """
As a bonus, we have generated a plot of your chosen area.  It is stored in
generated/images, but if you want, you can view it now.  Just close the window
when you're done, and we'll continue.
"""
        print "View the generated image? "
        ui=tigerutils.UserInput()
        while True:
            confirm=ui.getDigit(1,1,"(1) yes (2) no: ")
            if confirm=='1':
                print "TODO show the image (low priority)"
                break
            elif confirm=='2':
                print "Skip the image viewing, and continue with the demo"
                break
            else:
                continue # this is redundant but explicit
    # end __init__ (finally)


    def get_location(self):
        '''Returns a pair of points (vertices) representing a location'''
        point_a=self.get_point()
        point_b=self.get_point()
        while point_a==point_b:
            point_b=self.get_point()
        return (point_a,point_b)


    def get_point(self):
        '''Generates a two-tuple representing an (x,y) location'''
        return self.__get_vertex()


    def get_distance(self, here, there):
        '''
	Given a pair of coordinates, return the driving distance between them.

	The distance calculation is set in the configuration option
	distanceCalculation.  Options are straight-line distance between the
	points (the default), or driving distance.
	'''
        # This distance is subject to the graphCoordinateMultiplier, to bring
        # it approximately in line with the grid simulation.  The multiplier
        # is used here in order to make this thing behave as much as possible
        # like get_distance() from Grid.py.
        #
        # Q: Should the multiplier be added to the INI files?
        # A: Yes.  It should be variable with the varying size of the region
        #    (ZIP or county).
        #
        # Working out the "multiplier" for lat/long graph simulations.
        # Starting with calculating the distance from Belfield ND to Fargo ND.
        # I chose these locations because they are almost a perfectly straight
        # line on the map.
        #
        # http://www.batchgeocode.com/lookup/
        # Belfield ND lat:46.885885 long:-103.199379
        # Fargo ND lat:46.87591 long:-96.782299
        #
        # Google maps says the driving distance between these two coordinates
        # is 312 miles.  I am using a metric of 2 "ticks" per mile, or an
        # average Agent driving speed of 30 MPH.  That's 624 ticks for 312
        # miles.  The absolute value of the delta of the lat/long from
        # Belfield to Fargo is 6.42, and 624/6.42 is ~100.  So that's the
        # multiplier.  The idea isn't to be an exact counterpart to the Grid
        # class.  I'm not trying to compare apples to apples.  But it's nice
        # to be in the ballpark.
        #
        # >>> abs(46.885885-46.87591)+abs(-103.199379-(-96.782299))
        # 6.4270550000000028
        # >>> 624/6.427055
        # 97.089569017224832
        #
        #coordinateMultiplier=1
        coordinateMultiplier=10
        #coordinateMultiplier=100
        coordinateDivisor=1e-6
        coordinateNormalization=coordinateMultiplier*coordinateDivisor

        #
        #Traceback (most recent call last):
        #	...
        #  File "C:\Source\hg\unified\agents\graph.py", line 229, in get_distance
        #    for lon,lat in self.mkgraph.shortest_path(here,there):
        #TypeError: 'int' object is not iterable
        #
        #C:\Source\hg\unified>python
        #>>> for lon,lat in (1.0,'two'):
        #...   print lon, lat
        #...
        #Traceback (most recent call last):
        #  File "<stdin>", line 1, in <module>
        #TypeError: 'float' object is not iterable
        #
        lon_dist=lat_dist=0
        for lon,lat in self.mkgraph.shortest_path(here,there):
            try:
                lastlon=currlon
                lastlat=currlat
                lon_dist+=abs(lon-lastlon)
                lat_dist+=abs(lat-lastlat)
            except NameError: # first time thru
                currlon=lastlon=lon
                currlat=lastlat=lat
        norm=lon_dist*coordinateNormalization+lat_dist*coordinateNormalization
        return norm


    def __get_vertex(self,connected=True):
        '''
	[private] Returns a single (x,y) coordinate point.

	Parameter connected specifies whether this vertex should come from the
	first and largest list of nodes.  This is important for the simulation
	to function properly, since Agents located on unconnected vertices are
	unreachable.
	'''
        tmp=self.query.get_point()
        if connected is True:
            connected_vertices=self.mkgraph.get_connected()
            # I'm not sure what's going on here ...
            fr=(int(tmp[2]),int(tmp[3]))
            while fr not in connected_vertices:
                tmp=self.query.get_point()
                # ... and here
                fr=(int(tmp[2]),int(tmp[3]))
        else:
            # tmp[0:2] are id and tlid that the Agents don't need
            # tmp[2:4] are (frlong,frlat)
            # tmp[4:6] are (tolong,tolat) which are not needed here
            fr=(tmp[2:4])
        return fr


    # I'm no longer using this for the regular compete methods (thanks to a
    # suggestion from Dan Struthers).  If I go on to create courtesy_compete
    # methods, and rename the regular compete methods to cutthroat_compete,
    # then I'll be able to use this.  In the meantime, I'm not going to create
    # an update_location() method in graph.py.
    def update_location(self):
        '''DOCSTRING'''
        pass


if __name__=='__main__':
    print "graph.py"
    g=Graph()

    print "trying g.get_location()..."
    location=g.get_location()
    loc={}
    loc['curr']=(location[0],location[1])
    loc['dest']=(location[2],location[3])
    print "loc['curr']=", loc['curr']
    print "loc['dest']=", loc['dest']

    print "bye"
