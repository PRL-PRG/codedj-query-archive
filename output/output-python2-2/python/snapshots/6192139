#!/usr/bin/env python
'''DOCSTRING'''

# TODO: edit this and put it in the module docstring.
#
# This module is activated if mapType is set to 'graph' in
# agents/conf/agent/defaults.ini or overrides.ini.  The graph simulations need
# additional preparation that the grid simulations do not.  The bulk of the
# work is done by the tigerutils module, and used by the graph module.
#
# Note 1: There winds up being a lot of activity when a Graph object is
#   created.  I may need to add an "are you sure?" dialog, or if the user
#   exits from one of the tigerutils tasks, try to recover gracefully.  Or
#   not.  There are only two mapTypes.  You either use this one, or go back to
#   grids, which are already working.
#
# Note 2: By the time a Graph object is fully instantiated, ...

# agents/Graph is polymorphic with agents/Grid

#from tigerutils import UserInput
import sys
import tigerutils

# Working out the "multiplier" for lat/long graph simulations.  Starting with
# calculating the distance from Belfield ND to Fargo ND.  I chose these
# locations because they are almost a perfectly straight line on the map.
#
# http://www.batchgeocode.com/lookup/
# Belfield ND lat:46.885885 long:-103.199379
# Fargo ND lat:46.87591 long:-96.782299
#
# Google maps says the driving distance between these two coordinates is 312
# miles.  I am using a metric of 2 "ticks" per mile, or an average Agent
# driving speed of 30 MPH.  That's 624 ticks for 312 miles.  The absolute
# value of the delta of the lat/long from Belfield to Fargo is 6.42, and
# 624/6.42 is ~100.  So that's the multiplier.
#
# >>> abs(46.885885-46.87591)+abs(-103.199379-(-96.782299))
# 6.4270550000000028
# >>> 624/6.427055
# 97.089569017224832
graphCoordinateMultiplier=100


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
        mg=tigerutils.MakeGraph()
        mg.makeGraph()

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


    # TODO
    def get_location(self):
        '''DOCSTRING'''
	# tigerutils's QueryDatabase.getPoint() returns a 6-tuple of
	# r['id'],r['tlid'],r['frlong'],r['frlat'],r['tolong'],r['tolat'].
	#    0       1         2           3          4           5
	#
	# The Agents don't need, and can't use id and tlid, so leave them out.
	return self.query.getPoint()[2:6]


    # I'm no longer using this for the regular compete methods (thanks to a
    # suggestion from Dan Struthers).  If I go on to create courtesy_compete
    # methods, and rename the regular compete methods to cutthroat_compete,
    # then I'll be able to use this.  In the meantime, I'm not going to create
    # an update_location() method in graph.py.
    def update_location(self):
        '''DOCSTRING'''
	pass


    # TODO
    def get_distance(self, here, there):
        '''Return the distance between two points'''
	pass


    # Not using get_point in Graph.  It doesn't make sense.  So it's been
    # converted to a private method.  It's implemented in Grid, and empty in
    # Graph.
    def __get_point(self):
        '''Return a single (x,y) coordinate point'''
	pass


#~~    # Here's an example in sqlite3
#~~    #
#~~    # C:\Source\hg\unified\generated\data\AK\TGR02068\sql>sqlite3 TGR02068.db
#~~    # sqlite> SELECT id, frlong, frlat, tolong, tolat FROM tiger_01 ORDER BY random() LIMIT 1;
#~~    # 438|-149.987164|64.144821|-150.002351|64.131662
#~~    # sqlite> SELECT id, frlong, frlat, tolong, tolat FROM tiger_01 ORDER BY random() LIMIT 1;
#~~    # 2164|-148.808454|63.56553|-148.811602|63.564018
#~~    # sqlite>
#~~    #
#~~    # NOTE: this method is similar to agents.Agent mkcoords()
#~~    def getPoint(self):
#~~        '''Fetch a SQLAlchemy ResultProxy for a random point on the graph.'''
#~~        randomRow=random.randint(1,self.getRecordCount())
#~~	r=self.session.execute(select([
#~~		G.tiger01_Table.c.id,
#~~		G.tiger01_Table.c.tlid,
#~~		G.tiger01_Table.c.frlong,
#~~		G.tiger01_Table.c.frlat,
#~~		G.tiger01_Table.c.tolong,
#~~		G.tiger01_Table.c.tolat
#~~		],G.tiger01_Table.c.id==randomRow)).fetchone()
#~~#	for row in result:
#~~#            return row
#~~	# This thing returns a tuple with an int and four Decimal objects
#~~	# (which I don't know what the f--k to do with).
#~~	#
#~~	# Late note: maybe it's not so bad
#~~        # >>> print decimal.Decimal("-150.330257")
#~~        # -150.330257
#~~	# 
#~~        return r['id'],r['tlid'],r['frlong'],r['frlat'],r['tolong'],r['tolat']


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
