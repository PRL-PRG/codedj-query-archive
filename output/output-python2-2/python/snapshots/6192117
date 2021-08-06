# getFips.py (AKA graphs_driver.py)

# Note: Made this change to permit moving getFips.py to the top level
# directory along with rename to graphs_driver.py.
import agents.tigerutils as tigerutils

def run():
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

    # [TODO] show the user all the ZIP codes for the chosen county and query
    # the user for which to use (or None for all)
    query=tigerutils.QueryDatabase()
    query.chooseGraphArea()

    # TESTING
    gp1=query.get_point()
    print 'The start query.get_point() is',gp1
    start_frlong,start_frlat,tolong,tolat=gp1[2:] # skip id and tlid

    gp2=query.get_point()
    print 'The end query.get_point() is',gp2
    frlong,frlat,end_tolong,end_tolat=gp2[2:] # skip id and tlid

#    for dat in gp:
#        print dat, #float(dat)

#    print 'The query.tuptotup() is',query.tuptotup()
#    tup=query.tuptotup(gp)
#    print 'The query.tuptotup() is',tup

    # [TODO] plot the chosen area
    mg=tigerutils.MakeGraph()
    mg.makeGraph()
    #mg.makeGraphFromTLID()
#    frlong.lstrip('+')
#    frlat.lstrip('+')
#    tolong.lstrip('+')
#    tolat.lstrip('+')

    # This is not useful.  These two points are adjacent to one another.
    #
    # mg.shortest_path: [(-149171250, 64285006), (-149169441, 64289333)]
    # -- and --
    # sqlite> SELECT * FROM tiger_01 WHERE frlong='-149171250' AND frlat='64285006';
    # 627|1|1006|191177659|||||||||||-149171250|64285006|-149169441|64289333
    # 628|1|1006|191177626|||||||||||-149171250|64285006|-149174638|64285261
    # 
    # The second row is on the "other side" of the same point.  In other
    # words:
    # (-149169441,64289333)->(-149171250,64285006)->(-149174638,64285261)
    # -- and --
    # (-149174638,64285261)->(-149171250,64285006)->(-149169441,64289333)
    #
    #print "mg.shortest_path:", mg.shortest_path((frlong,frlat),(tolong,tolat))
    print("mg.shortest_path:",
		    mg.shortest_path((start_frlong,start_frlat),(end_tolong,end_tolat)))

    print("calling connected_components")
    print mg.get_connected()

if __name__ == '__main__':
    run()

# vim: tw=78
