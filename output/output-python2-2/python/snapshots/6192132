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
    gp=query.get_point()
    print 'The query.get_point() is',gp
    frlong,frlat,tolong,tolat=gp[2:] # skip id and tlid
    for dat in gp:
        print dat, #float(dat)

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
    mg.shortest_path((frlong,frlat),(tolong,tolat))
#    print query.__rpOne()

if __name__ == '__main__':
    run()

# vim: tw=78
