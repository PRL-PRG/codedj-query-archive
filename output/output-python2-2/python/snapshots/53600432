import lsst.dps.Stage
import lsst.fw.Core.fwLib as fw
import lsst.mwi.persistence as mwiper
import lsst.mwi.utils as mwiu
import Detection

__all__ = ["DetectionStage"]

class DetectionStage(lsst.dps.Stage.Stage):
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
        #
        # Instantiate a Filter object to get the id of filterName
        #
        dbLocation = mwiper.LogicalLocation('mysql://lsst10.ncsa.uiuc.edu:3306/test')
        filterDB = fw.Filter(dbLocation, filterName)
        filterId = filterDB.getId()
        mwiu.Trace("lsst.detection.DetectionStage", 3, 'FilterName %s FilterId %d' % (filterName, filterId))
       
        differenceImageExposure = activeClipboard.get('DifferenceExposure')
        print "DetectionStage.process clipboard contains:"
        for key in activeClipboard.getKeys():
            print "* %s: %r" % (key, activeClipboard.get(key))

        diaSourceCollection = Detection.detection(
            differenceImageExposure = differenceImageExposure,
            policy = self._policy,
            filterId = filterId,
            footprintList = None,
        )

        ###########
        #
        # Post results to clipboard
        #
        activeClipboard.put('DiaSources', diaSourceCollection)
        
        self.outputQueue.addDataset(activeClipboard)
