# getFips.py
import graphs.tigerutils as tigerutils

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

    # [TODO] plot the chosen area
    mg=tigerutils.MakeGraph()
    mg.makeGraph()

if __name__ == '__main__':
    run()

# vim: tw=78
