import lsst.dps.Stage
import lsst.mwi.data as datap
import lsst.mwi.policy as policy
import lsst.fw.Core.fwCatalog as fwCat
import lsst.movingobj.nightmops.ephemeris as eph
import lsst.movingobj.nightmops.ephemDB as ephDB

class MopsStage(lsst.dps.Stage.Stage):

    #------------------------------------------------------------------------

    def __init__(self):
        """
        Do initialization for the whole pipeline run
        """


    #------------------------------------------------------------------------
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

        sliceId = self.getRank()
        numSlices = self.getUniverseSize()

        #########
        #
        # Get needed params from policy
        #
        ephemDBFromPolicy = self._policy.get('ephemDB')

        fovDiamFromPolicy = self._policy.get('fovDiam')

        
        ###########
        #
        # Get objects from clipboard
        #
        self.activeClipboard = self.inputQueue.getNextDataset()

        triggerEvent = activeClipboard.get('mops1Event')

        fovRAItem = triggerEvent.findUnique('FOVRA')
        fovRA = fovRANameItem.getValueString()

        fovDecItem = triggerEvent.findUnique('FOVDec')
        fovDec = fovRANameItem.getValueString()

        visitIdItem = triggerEvent.findUnique('visitId')
        fovRA = fovRANameItem.getValueString()

        MJDItem = triggerEvent.findUnique('visitTime')
        mjd = MJDItem.getValueString()

        # get this Slice's set of potential objects in the FOV

        candidateEphems = ephDB.fetchCandidateEphems(ephemDbFromPolicy, sliceId, numSlices, mjd)

        # get a list of predicted ephems that actually fall in our fov

        ephPreds = ephDB.selectOrbitsForFOV(orbitsAndEphems, mjd, fovRA, fovDEC, fovDiamFromPolicy)

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

        activeClipboard.put('MopsPreds', mopsPreds)
        
        self.outputQueue.addDataset(self.activeClipboard)



