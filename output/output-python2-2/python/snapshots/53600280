import lsst.dps.Stage
import lsst.afw.Core.afwLib as afw
import lsst.daf.persistence as dafper
import lsst.pex.logging as logging
from lsst.pex.logging import Log
from lsst.pex.logging import LogRec
from lsst.daf.base import DataProperty
import Detection

__all__ = ["DetectionStage"]

class DetectionStage(lsst.dps.Stage.Stage):
    def __init__(self, stageId = -1, policy = None):

        lsst.dps.Stage.Stage.__init__(self, stageId, policy)
        self.detectionLog = Log(Log.getDefaultLog(), "detection.stage")

    def process(self):

        logging.Trace_setVerbosity("lsst.detection", 5)
        logging.Trace("lsst.detection.DetectionStage", 3, 'Python DetectionStage process : _rank %i stageId %d' % (self._rank, self.stageId))
        activeClipboard = self.inputQueue.getNextDataset()

        ###########
        #
        # Get objects from clipboard
        #
        triggerEvent = activeClipboard.get('triggerVisitEvent')
        filterNameItem = triggerEvent.findUnique('filterName')
        filterName = filterNameItem.getValueString()
        exposureIdItem = triggerEvent.findUnique('exposureId')
        exposureId = exposureIdItem.getValueInt()
        visitTimeItem = triggerEvent.findUnique('visitTime')
        visitTime = visitTimeItem.getValueDouble()
        ###########
        #
        # Log the beginning of Detection stage for this slice
        #
        LogRec(self.detectionLog, Log.INFO) \
                                  <<  "Began detection stage" \
                                  << DataProperty("exposureId", exposureId) \
                                  << DataProperty("visitTime", visitTime) \
                                  << DataProperty("filterName", filterName) \
                                  << LogRec.endr
        #
        # Instantiate a Filter object to get the id of filterName
        #
        dbLocation = dafper.LogicalLocation('mysql://lsst10.ncsa.uiuc.edu:3306/test')
        filterDB = afw.Filter(dbLocation, filterName)
        filterId = filterDB.getId()
        logging.Trace("lsst.detection.DetectionStage", 3, 'FilterName %s FilterId %d' % (filterName, filterId))
       
        differenceImageExposure = activeClipboard.get('DifferenceExposure')

        diaSourceCollection = Detection.detection(
            differenceImageExposure = differenceImageExposure,
            policy = self._policy,
            filterId = filterId,
            useLog = self.detectionLog,
            footprintList = None,
        )

        ###########
        #
        # Post results to clipboard
        #
        activeClipboard.put('DiaSources', diaSourceCollection)
        
        self.outputQueue.addDataset(activeClipboard)
