import lsst.dps.Stage
import lsst.fw.Core.fwLib as fw
import lsst.mwi.persistence as mwiper
import lsst.mwi.utils as mwiu
from lsst.mwi.logging import Log
from lsst.mwi.logging import LogRec
from lsst.mwi.data import DataProperty
import Detection

__all__ = ["DetectionStage"]

class DetectionStage(lsst.dps.Stage.Stage):
    def __init__(self, stageId, policy):

        self.detectionLog = Log(Log.getDefaultLog(), "detection.stage")

    def process(self):

        mwiu.Trace_setVerbosity("lsst.detection", 5)
        mwiu.Trace("lsst.detection.DetectionStage", 3, 'Python DetectionStage process : _rank %i stageId %d' % (self._rank, self.stageId))
        activeClipboard = self.inputQueue.getNextDataset()

        ###########
        #
        # Get objects from clipboard
        #
        triggerEvent = activeClipboard.get('triggerVisitEvent')
        filterNameItem = triggerEvent.findUnique('filterName')
        filterName = filterNameItem.getValueString()
        exposureIdItem = triggerEvent.findUnique('exposureId')
        exposureId = exposureIdItem.getValueString()
        visitTimeItem = triggerEvent.findUnique('visitTime')
        visitTime = visitTimeItem.getValueString()
        ###########
        #
        # Log the beginning of Detection stage for this slice
        #
        LogRec(self.detectionLog, Log.INFO) \
                                  <<  "Began detection stage" \
                                  << DataProperty("exposureId", exposureId) \
                                  << DataProperty("visitTime", visitTime) \
                                  << DataProperty("filterName", filterName)
        #
        # Instantiate a Filter object to get the id of filterName
        #
        dbLocation = mwiper.LogicalLocation('mysql://lsst10.ncsa.uiuc.edu:3306/test')
        filterDB = fw.Filter(dbLocation, filterName)
        filterId = filterDB.getId()
        mwiu.Trace("lsst.detection.DetectionStage", 3, 'FilterName %s FilterId %d' % (filterName, filterId))
       
        differenceImageExposure = activeClipboard.get('DifferenceExposure')

        diaSourceCollection = Detection.detection(
            differenceImageExposure = differenceImageExposure,
            policy = self._policy,
            filterId = filterId,
            log = self.detectionLog,
            footprintList = None,
        )

        ###########
        #
        # Post results to clipboard
        #
        activeClipboard.put('DiaSources', diaSourceCollection)
        
        self.outputQueue.addDataset(activeClipboard)
