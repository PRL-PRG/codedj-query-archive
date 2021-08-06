import lsst.mwi.persistence as mwiper
import lsst.mwi.utils as mwiu
import lsst.fw.Core.fwLib as fw
#import lsst.movingobject.nightmops.ephemeris as eph
import ephemeris as eph
import numpy

def fetchAllEphems(dbLogicalLocation):
    
    db = mwiper.DbStorage()
    loc = mwiper.LogicalLocation(dbLogicalLocation)
    db.setRetrieveLocation(loc)
    db.startTransaction()
    db.setTableForQuery("ephem")
    db.outColumn("orbit_id")
    db.outColumn("mjd")
    db.outColumn("ra_deg")
    db.outColumn("dec_deg")
##     db.outColumn("mag")
    db.outColumn("smaa")
    db.outColumn("smia")
    db.outColumn("pa")
    db.orderBy("orbit_id")
    db.orderBy("mjd")

    db.query()

    numRows = 0
    res = []
    while db.next():
       ++numRows
       ephem = eph.Ephemeris( \
           db.getColumnByPosInt64(0), \
           db.getColumnByPosDouble(1), \
           db.getColumnByPosDouble(2), \
           db.getColumnByPosDouble(3), \
##         db.getColumnByPosDouble(4), \
##  Mag not set in current db
           0.0, \
           db.getColumnByPosDouble(4), \
           db.getColumnByPosDouble(5), \
           db.getColumnByPosDouble(6) )
       res.append(ephem)

    db.finishQuery()

    return res

def fetchRangeOfEphems(dbLogicalLocation, mjdMin, mjdMax):
    
    db = mwiper.DbStorage()
    loc = mwiper.LogicalLocation(dbLogicalLocation)
    db.setRetrieveLocation(loc)
    db.startTransaction()
    db.setTableForQuery("ephem")
    db.setQueryWhere("mjd >= " + str(mjdMin) + " and mjd <= " + str(mjdMax))
    db.outColumn("orbit_id")
##     db.outColumn("mjd")
    db.outColumn("ra_deg")
    db.outColumn("dec_deg")
    db.outColumn("mag")
    db.outColumn("smaa")
    db.outColumn("smia")
    db.outColumn("pa")
    db.orderBy("orbit_id")
    db.orderBy("mjd")

    db.query()

    numRows = 0
    res = []
    while db.next():
       ++numRows
       ephem = eph.Ephemeris( \
           db.getColumnByPosInt64(0), \
           db.getColumnByPosDouble(1), \
           db.getColumnByPosDouble(2), \
           db.getColumnByPosDouble(3), \
##         db.getColumnByPosDouble(4), \
##  Mag not set in current db
           0.0, \
           db.getColumnByPosDouble(4), \
           db.getColumnByPosDouble(5), \
           db.getColumnByPosDouble(6) )

    db.finishQuery()

    return res

 
def fetchCandidateEphems(dbLogicalLocation, sliceId, numSlices, mjd, deltaMjd=1.0):
    
    db = mwiper.DbStorage()
    loc = mwiper.LogicalLocation(dbLogicalLocation)
    db.setRetrieveLocation(loc)
    db.startTransaction()
    db.setTableForQuery("ephem")
    db.setQueryWhere("orbit_id % " + str(numSlices) + "=" + str(sliceId-1) + " and abs(mjd-" +
                     str(mjd) + ") < " + str(deltaMjd))
    db.outColumn("orbit_id")
    db.outColumn("mjd")
    db.outColumn("ra_deg")
    db.outColumn("dec_deg")
##     db.outColumn("mag")
    db.outColumn("smaa")
    db.outColumn("smia")
    db.outColumn("pa")
    db.orderBy("orbit_id")
    db.orderBy("mjd")

    db.query()

    numRows = 0
    res = []
    while db.next():
       ++numRows
       ephem = eph.Ephemeris( \
           db.getColumnByPosInt64(0), \
           db.getColumnByPosDouble(1), \
           db.getColumnByPosDouble(2), \
           db.getColumnByPosDouble(3), \
##         db.getColumnByPosDouble(4), \
##  Mag not set in current db
           0.0, \
           db.getColumnByPosDouble(4), \
           db.getColumnByPosDouble(5), \
           db.getColumnByPosDouble(6) )
       res.append(ephem)

    db.finishQuery()

    return res

 
def selectOrbitsForFOV(candidateEphems, mjd, fovRA, fovDEC, fovDiam):
   #
   # Create a BBox2f for the fov in ra, dec space
   #
   # Loop over candidateEphems
   #    Collect all Ephems for same orbit_id
   #    Interpolate to mjd
   #    Test wheter in BBox.  If so add to MopsPredVec
   # Return MopsPredVec

   mopsPreds = []

   fovBox = fw.BBox2f(fovRA - fovDiam/2, fovDEC - fovDiam, fovDiam, fovDiam)

   currentOrbitId = -9999   # all real orbitId's are positive
   ephInterpSet = []
   
   for eph in candidateEphems:
      thisOrbitId = eph.orbitId;
      if thisOrbitId == currentOrbitId:
         ephInterpSet.append(eph)
      else:
         if len(ephInterpSet) > 0:
            ephPred = interpolateEphemerides(ephInterpSet, mjd)
            ephInterpSet = []
            if contains(ephPred, fovBox):
               mopsPreds.append(ephPred)
            
         currentOrbitId = thisOrbitId
         ephInterpSet.append(eph)

   return mopsPreds

def contains(eph, box):

   coord = fw.Vector2f(eph.RA, eph.Dec)
   return box.contains(coord)


def interpolateEphemerides(ephemList, MJD, method="quadratic"):
    """interpolateEphemerides:

    do interpolation between ephemerides from 3 different dates.

    input: 

    -ephemList: a list of Ephemerides predictions for the same object,
       and all at different times (suggested: 1 day apart, with
       midpoint near parameter given for MJD), at least two for a
       linear interpolation and at least 3 for a quadratic.
  
    -MJD: mean julian date time to which we must interpolate.
    
    -method (optional): "linear" for linear, "quadratic" for
     quadratic.

    output:
    -a new Ephemeris object 
    """
    #uses numpy.polyfit to get estimation TBD: implement "linear"
    #approx and add some checking/exceptions, also update SMIA, SMAA
    x = [ephemList[i].RA for i in range(3)]
    y = [ephemList[i].Dec for i in range(3)]
    MJDs = [ephemList[i].MJD for i in range(3)]
        
    if method=="quadratic":
        quadratic = numpy.polyfit(x,y,2) 
        #gets quadratic fit for these 3 points quadratic is a numpy
        #array of form [a, b, c] where y = ax**2 + bx + c
        #print quadratic
        
        #now we need to correlate MJD to x (i.e., RA) and scale the
        #rates of change
        dx = x[0] - x[2]
        dMJD = MJDs[0] - MJDs[2]
        offsetMJD = MJDs[1] - MJD
        offsetX = offsetMJD*(dx/dMJD)
        #print "dx: %f, dMJD: %f, offsetMJD: %F, offsetX: %F" %(dx, 
        #                 dMJD, offsetMJD, offsetX)
        #now subtract offsetX from x[2] and use our function to get
        #estimated y-value
        newX = x[1] - offsetX
        newY = numpy.polyval(quadratic, newX)
        #print "(x,y): (%f,%f)"% (newX, newY)
        return eph.Ephemeris(ephemList[0].orbitId, MJD, newX, newY, 
                         ephemList[0].Mag, ephemList[0].SMAA,
                         ephemList[0].SMIA, ephemList[0].PA)

    else:
        raise "InterpolateEphemeris: Sorry, only quadratic mode is implemented so far."

