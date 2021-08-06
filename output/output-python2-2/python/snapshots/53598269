#!/usr/bin/env python
"""
daymopsutils.py: 

Functions used to set up the database for nightMops.  DayMOPS as a
system is actually written in Perl and lives someplace else; these
functions are just utility code (i.e.: throwaway code) to be used by
Francesco and jmyers to get the DayMOPS DB set up correctly for DC2.

See the python/lsst/nightmops/ directory for more standard code and
use that file whenever possible.

"""

import MySQLdb as dbi
import numpy

import auton
import ephem



# Constants
DB_HOST = ''
DB_USER = 'mops'
DB_PASS = 'mops'
#DB_DB = 'psmops_fpierfed' 
#DB_DB = 'psmops_firsttry_improved'
#DB_DB = 'psmops_jontest0'
DB_DB = 'psmops_denneau_modified'
#DB_DB = 'psmops_dc2_improved'
#DB_DB = 'psmops_dc2' #for The Big One
OBS_CODE = 568
        

# Exceptions
class DataSetError(Exception): pass



class DBCursor(object):
    """
    Implement simple connection pooling.
    """
    _cursor = None
    _user = None
    _passwd = None
    _host = None
    _port = None
    _db = None


    class IterCursor(object):
        """
        Simple layer on top of a database cursor object (i.e.
        dbi.cursors.Cursor) that supports returning an iterator to
        fetch results one at the time.

        The reason for not inheriting from dbi.cursors.Cursor is that
        that would be very database engine specific, which is not
        necesarily a good idea at this stage.
        """
        def __init__(self, cursor):
            self._cursor = cursor
            return

        def fetchiter(self):
            """
            Return an iterator that repeatetly execute a fetchone()
            cursor method until there are no more rows to fetch in
            which case raise a StopIteration exception.
            """
            res = self._cursor.fetchone()
            while(res != None):
                yield res
                res = self._cursor.fetchone()
            # <-- end while
            raise(StopIteration('Fetched all records.'))

        def __getattr__(self, methodName):
            """
            This internal method gets called when a lookup for
            methodName has failed which means that the calling code is
            trying to call a method of the real cursor object. This is
            where we fake class inheritance.

            If methodName is not a database cursor method, the lookup
            will fail and the usual AttributeError exception will be
            risen.
            """
            return(getattr(self._cursor, methodName))

    
    @classmethod
    def connect(cls, user, password, db, host='localhost', port=None):
        """
        Class method: re-use the same cursor all the time. Return the
        cursor object.
        """
        if(not cls._cursor or
           not user == cls._user or
           not password == cls._passwd or
           not db == cls._db or
           not host == cls._host or
           not port == cls._port):
            # First connection of this type. Raise exception if this fails.
            if(port):
                connection = dbi.connect(user=user,
                                         passwd=password,
                                         host=host,
                                         port=port,
                                         db=db)
            else:
                connection = dbi.connect(user=user,
                                         passwd=password,
                                         host=host,
                                         db=db)
            cls._cursor = cls.IterCursor(connection.cursor())
            cls._user=user
            cls._passwd=password
            cls._host=host
            cls._port=port
            cls._db=db
        # <-- end if
        return(cls._cursor)


class Orbit(object):
    def __init__(self, orbitID, elements, mag, epoch, covariance):
        """
        orbitID: integer
        elements: cometary elements:
                  q (AU)
                  e
                  i (deg)
                  node (deg)
                  arg_peri (deg)
                  time_peri (UTC MJD)
        epoch: orbit epoch (UTC MJD)
        covariance: 21 element array (covariance matrix in diagonal form).
        """
        self.orbitID = orbitID
        self.elements = numpy.array([float(e) for e in elements])
        self.mag = float(mag)
        self.epoch = float(epoch)
        self.covariance = self._setupCovariance(covariance)
        return

    def _setupCovariance(self, covariance):
        """
        If all elements of the covariance list ore non None, then cast that
        list into a numpy.array. Return the casted array or None in case the
        covariance is invalid (i.e. has null elements).
        """
        if(None in covariance):
            return(None)
        try:
            cov = numpy.array([float(e) for e in covariance])
        except TypeError:
            print(covariance)
            raise(TypeError())
        return(cov)



def fetchAllOrbits(user=DB_USER, password=DB_PASS, host=DB_HOST, db=DB_DB):
    """
    Fetch the orbits of all known moving objects from day-MOPS.
    """
    # Get a cursor to the database.
    cursor = DBCursor.connect(user=user,
                              password=password,
                              host=host,
                              db=db)

    # Select all orbits associated to a derived object.
    sql = 'select orbit_id, q, e, i, node, arg_peri, time_peri, epoch, ' + \
          'h_v, ' + \
          'cov_01, cov_02, cov_03, cov_04, cov_05, cov_06, cov_07, cov_08, ' + \
          'cov_09, cov_10, cov_11, cov_12, cov_13, cov_14, cov_15, cov_16, ' + \
          'cov_17, cov_18, cov_19, cov_20, cov_21 ' + \
          'from orbits'
    # Execute the SQl statement and then fetch the results.
    numRows = cursor.execute(sql)
    res = []
    if(numRows):
        # We got results, fetch them all and morph them into Orbit objects.
        res = [Orbit(orbitID=record[0],
                     elements=record[1:7],
                     epoch=record[7],
                     mag=record[8],
                     covariance=record[9:]) for record in cursor.fetchiter()]
    return(res)

def fetchAllOrbitsAndEphems(mjd, deltaMJD=1., user=DB_USER, password=DB_PASS,
                            host=DB_HOST, db=DB_DB):
    """
    Fetch the orbits of all known moving objects from day-MOPS together with
    their precomputed ephemerides at int(mjd)-deltaMJD, int(mjd) and
    int(mjd)+deltaMJD.

    Return
        {orbitId: [OrbitInstance, (RA, Dec, int(mjd)-deltaMJD),
                   (RA, Dec, int(mjd)),(RA, Dec, int(mjd)+deltaMJD)]}
    """
    # Get a cursor to the database.
    cursor = DBCursor.connect(user=user,
                              password=password,
                              host=host,
                              db=db)

    # Select all orbits associated to a derived object.
    sql = 'select o.orbit_id, o.q, o.e, o.i, o.node, o.arg_peri, ' + \
          'o.time_peri, o.epoch, o.h_v, ' + \
          'o.cov_01, o.cov_02, o.cov_03, o.cov_04, o.cov_05, o.cov_06, ' + \
          'o.cov_07, o.cov_08, o.cov_09, o.cov_10, o.cov_11, o.cov_12, ' + \
          'o.cov_13, o.cov_14, o.cov_15, o.cov_16, o.cov_17, o.cov_18, ' + \
          'o.cov_19, o.cov_20, o.cov_21, e.ra_deg, e.dec_deg, e.mjd ' +\
          'from orbits o,  ephem e ' +\
          'where o.orbit_id = e.orbit_id and ' +\
          'e.mjd in (%f, %f, %f) order by o.orbit_id, e.mjd' %(int(mjd)-deltaMJD,
                                                        int(mjd),
                                                        int(mjd)+deltaMJD)

    # Execute the SQl statement and then fetch the results.
    numRows = cursor.execute(sql)
    res = {}
    if(numRows):
        # We got results, fetch them all and morph them into Orbit objects.
        # Keep in mind that we wil have three entries per orbit.
        for i in range(0, numRows, 3):
            # Get the three entries.
            recordPre = cursor.fetchone()
            record = cursor.fetchone()
            recordPost = cursor.fetchone()

            orbitID = record[0]
            o = Orbit(orbitID=record[0],
                      elements=record[1:7],
                      epoch=record[7],
                      mag=record[8],
                      covariance=record[9:30])
            ephemPre = (recordPre[30], recordPre[31], recordPre[32])
            ephem = (record[30], record[31], record[32])
            ephemPost = (recordPost[30], recordPost[31], recordPost[32])
            res[orbitID] = [o, ephemPre, ephem, ephemPost]
        # <-- end for
    # <-- end if
    print "got ", len(res), " results in FetchAllOrbitsAndEphem"
    return(res)


def _sameOrbit(*records):
    """
    Internal function. Check whether the orbits described by the input database
    records are infact the same.

    Input records can have different structure. The only important thing is that
    the orbit_id is their first element.
    """
    refID = records[0][0]
    return(len([r[0] for r in records if r[0] != refID]) == 0)


def propagateOrbit(orbit, mjd, obscode=OBS_CODE):
    """
    Compute the ephemerides for orbit orbit at time mjd from obscode.

    Return
        numpy.array([RA, Dec, mag, mjd, smaa, smia, pa])

        RA: Right Ascension (deg).
        Dec: Declination (deg).
        mag: apparent magnitude (mag).
        mjd: input ephemerides date time (UTC MJD).
        smaa: error ellipse semi major axis (deg).
        smia: error ellipse semi minor axis (deg).
        pa: error ellipse position angle (deg).
    """
    if(orbit.covariance != None):
        return(ephem.ephemerides(elementsType='COM',
                                 orbitElements=orbit.elements,
                                 orbitEpoch=orbit.epoch,
                                 absMag=orbit.mag,
                                 obsCode=obscode,
                                 times=numpy.array([mjd, ]))[0])
    return(ephem.ephemerides(elementsType='COM',
                             orbitElements=orbit.elements,
                             covariance=orbit.covariance,
                             orbitEpoch=orbit.epoch,
                             absMag=orbit.mag,
                             obsCode=obscode,
                             times=numpy.array([mjd, ]))[0])


def batchPropagateOrbit(orbit, mjds, obscode=OBS_CODE):
    """
    Compute the ephemerides for orbit orbit from obscode at each of the MJDs
    in the mjds list.

    Return
        numpy.array([[RA, Dec, mag, mjd[i], smaa, smia, pa], ])

        RA: Right Ascension (deg).
        Dec: Declination (deg).
        mag: apparent magnitude (mag).
        mjd[i]: input ephemerides i-th date time (UTC MJD).
        smaa: error ellipse semi major axis (deg).
        smia: error ellipse semi minor axis (deg).
        pa: error ellipse position angle (deg).
    """
    if(orbit.covariance != None):
        return(ephem.ephemerides(elementsType='COM',
                                 orbitElements=orbit.elements,
                                 covariance=orbit.covariance,
                                 orbitEpoch=orbit.epoch,
                                 absMag=orbit.mag,
                                 obsCode=obscode,
                                 times=numpy.array(mjds)))
    return(ephem.ephemerides(elementsType='COM',
                             orbitElements=orbit.elements,
                             orbitEpoch=orbit.epoch,
                             absMag=orbit.mag,
                             obsCode=obscode,
                             times=numpy.array(mjds)))


def _buildEphemTable(mjd, deltaMJD, user=DB_USER, password=DB_PASS,
                     host=DB_HOST, db=DB_DB):
    """
    Fetch all the orbits from the orbits table and then, propagate those orbits
    to int(mjd)-deltaMJD, int(mjd) and int(mjd)+deltaMJD. Write the results in a
    table called ephem in the form
        orbit_id, ra, dec, mjd
    If the table is there already, drop it and recreate it.
    """
    # Get a cursor to the database.
    cursor = DBCursor.connect(user=user,
                              password=password,
                              host=host,
                              db=db)
    # Create the table.
    cursor.execute('drop table if exists ephem')
    cursor.execute('create table ephem (orbit_id bigint not null,' + \
                   'ra_deg double not null,' + \
                   'dec_deg double not null,' + \
                   'mjd double not null, ' + \
                       'smia double, smaa double, pa double);')

    # Fetch all the known orbits.
    orbits = fetchAllOrbits()

    # Propagate each of them to  int(mjd)-deltaMJD, int(mjd), int(mjd)+deltaMJD
    times = (int(mjd)-deltaMJD, int(mjd), int(mjd)+deltaMJD)
    for orbit in orbits:
        res = batchPropagateOrbit(orbit, mjds=times)
        sql = 'insert into ephem ' + \
            '(orbit_id, ra_deg, dec_deg, mjd, smaa, smia, pa) values ' + \
            '(%d, %f, %f, %f, %f, %f, %f), (%d, %f, %f, %f, %f, %f, %f), (%d, %f, %f, %f, %f, %f, %f)' \
            %(orbit.orbitID, res[0][0], res[0][1], res[0][3], 
              res[0][4], res[0][5], res[0][6],
              orbit.orbitID, res[1][0], res[1][1], res[1][3],
              res[1][4], res[1][5], res[1][6],
              orbit.orbitID, res[2][0], res[2][1], res[2][3],
              res[2][4], res[2][5], res[2][6])
        cursor.execute(sql)
    # <-- end for
    return

#TBD: implement some tools for adding fields to the DB, pruning useless 
# orbits from the DB, and updating the ephem table's shape, etc.


if(__name__ == '__main__'):
    # import unittest
    import time


    #fovRA = 63.
    #fovDec = 19.
    #fovR = 5.
    mjd = 54006.


    #for Serge's test data:
    #271.45819107331246544944 <= ra <= 275.50313489353718455056
    #-28.875 <= dec <= -25.375 
    fovRA = 273.48066298342485
    fovR = 2.2611923367975386 #this one is correct, NOT the other!
    fovDec = -27.125
    
    print DB_DB
    

    elapsedTime = time.time()
    elapsedTime = 0.
    t0 = time.time()
    # Create the ephem table.
    _buildEphemTable(mjd, 1.) 
    elapsedTime += time.time() - t0
        
    print('Propagation of took %.02f s' % (elapsedTime))
    
    
