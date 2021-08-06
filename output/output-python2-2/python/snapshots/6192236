#!/usr/bin/env python
'''DOCSTRING'''

from area import Area

class Graph(Area):
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


    def get_distance(self, here, there):
        '''Return the distance between two points'''
	pass

