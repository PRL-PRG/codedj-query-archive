#!/usr/bin/env python
'''DOCSTRING'''

# agents/Graph is polymorphic with agents/Grid

import tigerutils


class Graph(object):
    '''DOCSTRING'''
    def __init__(self):
        pass


    # TODO
    def get_location(self):
        '''DOCSTRING'''
	pass


    # set_location?  This and get_location are not very Pythonic.  Maybe find
    # a Python for Java programmers guide?
    def update_location(self):
        '''DOCSTRING'''
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


    # TODO
    def get_distance(self, here, there):
        '''Return the distance between two points'''
	pass

