import lsst.pex.harness.Stage
import lsst.pex.policy as policy
from lsst.pex.logging import Trace
from lsst.pex.logging import Trace_setVerbosity
from lsst.pex.logging import Log
from lsst.pex.logging import LogRec
from lsst.daf.base import DataProperty


class DayMopsStage(lsst.pex.harness.Stage.Stage):
    """
    Single stage pipeline whose only job is to execute DayMOPS
    """
    
    def __init__(self, stageId=-1, policy=None):
        """
        Constructor
        
        Initialize the logger and call the super __init__ method
        """
        super(DayMopsStage, self).__init__(stageId, policy)
        self.mopsLog = Log(Log.getDefaultLog(), "daymops.stage")
        return

    def preprocess(self): 
        """
        Execute DayMOPS as a black box. This involves setting up the appropriate
        environment variables and passing control to mopper (part of the DayMOPS
        distribution). DayMOPS handles its processing and parallelism strategy 
        independently, so we do now spawn any slices. This is why mopper is 
        invoked in the Stage preprocess method and not in process().
        """
        Trace_setVerbosity("lsst.mops", 5)
        
        # Get the environment variables from policy. We need:
        #  MOPS_DBINSTANCE: name of the MOPS database.
        #  MOPS_HOME: root of the MOPS installation.
        #  CAET_DATA: path of the JPL ephemeris files.
        mopsHome = self._policy.get('MOPS_HOME')
        mopsDBInstance = self._policy.get('MOPS_DBINSTANCE')
        mopsCaetData = self._policy.get('MOPS_CAET_DATA')

        # Log the beginning of Mops stage for this slice
        LogRec(self.mopsLog, Log.INFO) \
                                  <<  "Began DayMOPS stage" \
                                  << LogRec.endr

        # Start mopper
        raise(NotImplementedError('Write the tuff that calls mopper!'))
        
        self.mopsLog.log(Log.INFO, "Mops stage processing ended")
        return
    
    def process(self):
        """
        Do nothing. This is because the whole Stage behaviour is purely serial.
        """
        return
    
    def postprocess(self):
        """
        Do any postprocessing.
        """
        return




















