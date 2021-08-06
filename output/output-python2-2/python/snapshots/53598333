import lsst.dps.Stage
import lsst.mwi.data as datap
import lsst.mwi.policy as policy
import lsst.mwi.utils as mwiu
from lsst.mwi.logging import Log
from lsst.mwi.logging import LogRec
from lsst.mwi.data import DataProperty

import lsst.fw.Core.fwCatalog as fwCat
import lsst.movingobj.nightmops.ephemeris as eph
import lsst.movingobj.nightmops.ephemDB as ephDB

class MopsStage(lsst.dps.Stage.Stage):

    #------------------------------------------------------------------------
    def __init__(self):

        self.mopsLog = Log(Log.getDefaultLog(), "movingobj.stage")

    def process(self): 
        """
        Execute the needed processing code for this Stage


        psuedocode: 
        -determine rank (done)
        -get fov, ra, time, and FOVID from clipboard (done)

        - check whether current mjd range is still valid
        - if not, load orbit_id's for our slice (orbit_id%universe_size == rank) and current mjd
        
        -get a python list of all orbits (use allOrbits function, which interrogates the DB)
        -use rank to determine this slice's section of the orbits list
        -use propogateOrbit to interpolate those orbits to a known location
        -write those orbits out to a known database table so AP can read them
        """

        mwiu.Trace_setVerbosity("lsst.movingobj", 5)
        
        sliceId = self.getRank()
        numSlices = self.getUniverseSize() - 1  # want only real slices

        #########
        #
        # Get needed params from policy
        #
        ephemDbFromPolicy = self._policy.get('ephemDB')

        fovDiamFromPolicy = self._policy.get('fovDiam')

        
        ###########
        #
        # Get objects from clipboard
        #
        self.activeClipboard = self.inputQueue.getNextDataset()

        triggerEvent = self.activeClipboard.get('mops1Event')

        fovRAItem = triggerEvent.findUnique('FOVRA')
        fovRA = fovRAItem.getValueDouble()

        fovDecItem = triggerEvent.findUnique('FOVDec')
        fovDec = fovDecItem.getValueDouble()

        visitIdItem = triggerEvent.findUnique('visitId')
        visitId = visitIdItem.getValueInt()

        MJDItem = triggerEvent.findUnique('visitTime')
        mjd = MJDItem.getValueDouble()

        ###########
        #
        # Log the beginning of Mops stage for this slice
        #
        LogRec(self.mopsLog, Log.INFO) \
                                  <<  "Began mops stage" \
                                  << DataProperty("visitId", visitId) \
                                  << DataProperty("MJD", mjd)

        # get this Slice's set of potential objects in the FOV

        candidateEphems = ephDB.fetchCandidateEphems(ephemDbFromPolicy, sliceId, numSlices, mjd)

        mwiu.Trace("lsst.movingobj.MopsStage", 3, 'Number of candidate ephems: %d' % len(candidateEphems))
        
        # get a list of predicted ephems that actually fall in our fov

        ephPreds = ephDB.selectOrbitsForFOV(candidateEphems, mjd, fovRA, fovDec, fovDiamFromPolicy)
        mwiu.Trace("lsst.movingobj.MopsStage", 3, 'Number of ephems in fov: %d' % len(ephPreds))

        ###########
        #
        # Log the number of predicted ephems
        #
        LogRec(self.mopsLog, Log.INFO) \
                                  <<  "Predicted ephems" \
                                  << DataProperty("possible objects at this MJD", len(candidateEphems)) \
                                  << DataProperty("predicted objects in the FOV", len(ephPreds))

         # build a MopsPredVec for our Stage output
        
        mopsPreds = fwCat.MopsPredVec()

        for eph in ephPreds:
            mopsPred = fwCat.MopsPred()
            mopsPred.setId(eph.orbitId)
            mopsPred.setMjd(eph.MJD)
            mopsPred.setRa(eph.RA)
            mopsPred.setDec(eph.Dec)
            mopsPred.setSemiMinorAxisLength(eph.SMIA)
            mopsPred.setSemiMajorAxisLength(eph.SMAA)
            mopsPred.setPositionAngle(eph.PA)
            mopsPred.setMagnitude(eph.Mag)
            mopsPreds.push_back(mopsPred)
        
        # put output of selectOrbitsForFOV on the clipboard

        self.activeClipboard.put('MopsPreds', mopsPreds)
        
        self.outputQueue.addDataset(self.activeClipboard)

        self.mopsLog.log("Mops stage processing ended")



