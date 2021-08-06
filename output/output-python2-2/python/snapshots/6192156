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
for the map, with only have 3 or 4 disconnected nodes, but it's convenient for
doing a quick check that everything works end-to-end.
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

#        # [DONE] plot the chosen area
#        self.mg=tigerutils.MakeGraph()
#        self.mg.makeGraph()


    # TODO
    def get_location(self):
        '''DOCSTRING'''
	pass


    # set_location?  This and get_location are not very Pythonic.  Maybe find
    # a Python for Java programmers guide?
    def update_location(self):
        '''DOCSTRING'''
	pass


    # TODO
    def get_distance(self, here, there):
        '''Return the distance between two points'''
	pass


    # Here's an example in sqlite3
    #
    # C:\Source\hg\unified\generated\data\AK\TGR02068\sql>sqlite3 TGR02068.db
    # sqlite> SELECT id, frlong, frlat, tolong, tolat FROM tiger_01 ORDER BY random() LIMIT 1;
    # 438|-149.987164|64.144821|-150.002351|64.131662
    # sqlite> SELECT id, frlong, frlat, tolong, tolat FROM tiger_01 ORDER BY random() LIMIT 1;
    # 2164|-148.808454|63.56553|-148.811602|63.564018
    # sqlite>
    #
    # NOTE: this method is similar to agents.Agent mkcoords()
    def getPoint(self):
        '''Fetch a SQLAlchemy ResultProxy for a random point on the graph.'''
        randomRow=random.randint(1,self.getRecordCount())
	r=self.session.execute(select([
		G.tiger01_Table.c.id,
		G.tiger01_Table.c.tlid,
		G.tiger01_Table.c.frlong,
		G.tiger01_Table.c.frlat,
		G.tiger01_Table.c.tolong,
		G.tiger01_Table.c.tolat
		],G.tiger01_Table.c.id==randomRow)).fetchone()
#	for row in result:
#            return row
	# This thing returns a tuple with an int and four Decimal objects
	# (which I don't know what the f--k to do with).
	#
	# Late note: maybe it's not so bad
        # >>> print decimal.Decimal("-150.330257")
        # -150.330257
	# 
        return r['id'],r['tlid'],r['frlong'],r['frlat'],r['tolong'],r['tolat']


    def get_point(self):
        '''Return a single (x,y) coordinate point'''
	pass


if __name__=='__main__':
    print "graph.py"
    g=Graph()

    # [DONE] plot the chosen area
#    print "Generating node data and plotting the image..."
    mg=tigerutils.MakeGraph()
    mg.makeGraph()

    print """
As a bonus, we have generated a plot of your chosen area.  It is stored in
generated/images, but if you want, I can show it to you now.  Just close the
window when you're done viewing it, and we'll continue.
"""
    print "View the generated image? "
    ui=tigerutils.UserInput()
    while True:
        confirm=ui.getDigit(1,1,"(1) yes (2) no: ")
        if confirm=='1':
	    print "TODO show the image (low priority)"
	    break
        elif confirm=='2':
            print "Exiting."
            sys.exit(0)
        else:
            continue # this is redundant but explicit


#    query=tigerutils.QueryDatabase()
#    query.getPoint()
    print "bye"
