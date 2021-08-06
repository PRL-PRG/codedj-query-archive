import lsst.pex.harness.Stage
import lsst.daf.base as datap
import lsst.pex.policy as policy
from lsst.pex.logging import Trace
from lsst.pex.logging import Trace_setVerbosity
from lsst.pex.logging import Log
from lsst.pex.logging import LogRec
from lsst.daf.base import DataProperty

import lsst.mops.mopsLib
import lsst.mops.nightmops.ephemeris as eph
import lsst.mops.nightmops.ephemDB as ephDB




class MopsStage(lsst.pex.harness.Stage.Stage):
    def __init__(self, stageId=-1, policy=None):
        """
        Standard Stage initializer.
        """
        lsst.pex.harness.Stage.Stage.__init__(self, stageId, policy)
        self.mopsLog = Log(Log.getDefaultLog(), 'mops.stage')
        return


    def process(self): 
        """
        Execute the needed processing code for this Stage


        psuedocode: 
        -determine rank (done)
        -get fov, ra, time, and FOVID from clipboard (done)

        - check whether current mjd range is still valid
        - if not, load orbit_id's for our slice (orbit_id%universe_size == rank)
          and current mjd
        
        -get a python list of all orbits (use allOrbits function, which 
         interrogates the DB)
        -use rank to determine this slice's section of the orbits list
        -use propogateOrbit to interpolate those orbits to a known location
        -write those orbits out to a known database table so AP can read them
        """
        Trace_setVerbosity('lsst.mops', 5)
        
        # Get our slice ID  and tot number of slices(for simple parallelism 
        # purposes).        
        sliceId = self.getRank()
        numSlices = self.getUniverseSize() - 1  # want only real slices

        # Get needed params from policy
        ephemDbFromPolicy = self._policy.get('ephemDB')
        fovDiamFromPolicy = self._policy.get('fovDiam')
        obscodeFromPolicy = self._policy.get('obscode')

        # Get objects from clipboard
        self.activeClipboard = self.inputQueue.getNextDataset()
        triggerEvent = self.activeClipboard.get('mops1Event')
        
        fovRA = triggerEvent.findUnique('FOVRA').getValueDouble()
        fovDec = triggerEvent.findUnique('FOVDec').getValueDouble()
        visitId = triggerEvent.findUnique('visitId').getValueInt()
        mjd = triggerEvent.findUnique('visitTime').getValueDouble()

        # Log the beginning of Mops stage for this slice
        LogRec(self.mopsLog, Log.INFO) \
                                  <<  'Began mops stage' \
                                  << DataProperty('visitId', visitId) \
                                  << DataProperty('MJD', mjd) \
                                  << LogRec.endr

        # get this Slice's set of potential objects in the FOV
        candidateOrbits = ephDB.selectOrbitsForFOV(ephemDbFromPolicy, 
                                                   sliceId, 
                                                   numSlices, 
                                                   fovRA,
                                                   fovDec,
                                                   fovDiamFromPolicy,
                                                   mjd)

        # Propagate each orbit to mjd.
        ephems = [ephDB.propagateOrbit(o, mjd, obscodeFromPolicy) 
                  for o in candidateOrbits]
                  
        # Try and reduce the list even further by discarding positions that are
        # entirely outside of the Fov.
        ephems = [e for e in ephems if ephDB._isinside(e, fovRA, fovDec, fovR)]

        Trace('lsst.mops.MopsStage', 3, 
              'Number of orbits in fov: %d' % len(candidateOrbits))

        # Log the number of predicted ephems
        LogRec(self.mopsLog, Log.INFO) \
              <<  'Candidate orbits' \
              << DataProperty('predicted objects in the FOV', len(candidateOrbits)) \
              << DataProperty('predicted ephems in the FOV', len(ephems)) \
              << LogRec.endr

         # build a MopsPredVec for our Stage output
        mopsPreds = mopsLib.MopsPredVec()

        for e in ephems:
            mopsPred = afwCat.MopsPred()
            mopsPred.setId(e.orbitId)
            mopsPred.setMjd(e.MJD)
            mopsPred.setRa(e.RA)
            mopsPred.setDec(e.Dec)
            mopsPred.setSemiMinorAxisLength(e.SMIA)
            mopsPred.setSemiMajorAxisLength(e.SMAA)
            mopsPred.setPositionAngle(e.PA)
            mopsPred.setMagnitude(e.Mag)
            mopsPreds.push_back(mopsPred)
        
        # put output on the clipboard
        self.activeClipboard.put('MopsPreds', mopsPreds)
        self.outputQueue.addDataset(self.activeClipboard)
        self.mopsLog.log(Log.INFO, 'Mops stage processing ended')
        return



