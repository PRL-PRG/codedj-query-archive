import lsst.dps.Stage
import Detection

__all__ = ["DetectionStage"]

class DetectionStage(lsst.dps.Stage.Stage):
    def process(self):
        print 'Python DetectionStage process : _rank %i stageId %d' % (self._rank, self.stageId)
        activeClipboard = self.inputQueue.getNextDataset()

        ###########
        #
        # Get objects from clipboard
        #
        differenceImageExposure = activeClipboard.get('DifferenceExposure')
        print "DetectionStage.process clipboard contains:"
        for key in activeClipboard.getKeys():
            print "* %s: %r" % (key, activeClipboard.get(key))

        diaSourceCollection = Detection.detection(
            differenceImageExposure = differenceImageExposure,
            policy = self._policy,
            footprintList = None,
        )

        ###########
        #
        # Post results to clipboard
        #
        activeClipboard.put('DiaSources', diaSourceCollection)
        
        self.outputQueue.addDataset(activeClipboard)
