#!/usr/bin/env python
import os
import pdb
import sys
import unittest
import math

import lsst.pex.logging as logging
import lsst.daf.data as dafData
import lsst.pex.policy
import lsst.afw.Core.afwLib as afw
import lsst.afw.Core.afwCatalog as afwCat
import lsst.detection.detectionLib as det

from lsst.pex.logging import Log, ScreenLog
from lsst.pex.logging import LogRec
from lsst.daf.base import DataProperty

__all__ = ["detection"]

def detection(differenceImageExposure, policy, filterId, useLog=None, footprintList=None):
    """Detect and measure objects in an incoming difference image Exposure
    
    Inputs:
    - differenceImageExposure: an lsst.afw.Exposure containing a difference MaskedImage and WCS
    - policy: the policy; required elements are...?
    - footprintList: a sequence of detection footprints on which to force measurement
        
    Returns:
    - an lsst.afw.DiaSourceVector
    """

    if not(useLog):
        useLog = ScreenLog()
        useLog.setScreenVerbose(True)

    logging.Trace("lsst.detection.detection", 3,
        "filterId = %d" % (filterId))

    ###########
    #
    # Get directives from policy
    #
    thresh = policy.get('thresholdSigma')
    nPixMin = policy.get('numPixMinFootprint')

    ###########
    #
    # Unpack the MaskedImage from the Exposure
    #

    img = differenceImageExposure.getMaskedImage()

    ###########
    #
    # Crudely estimate noise from mean of variance image - should do sigma clipping
    #

    varImg = img.getVariance()
    noise = math.sqrt(afw.mean_channel_value(varImg))

    logging.Trace("lsst.detection.detection", 3,
        "thresholdSigma = %r; noise = %r PixMin = %r" % (thresh, noise, nPixMin))

    LogRec(useLog, Log.INFO) \
                   <<  "Threshold computation" \
                   << DataProperty("thresholdSigma", thresh) \
                   << DataProperty("noise", noise) \
                   << DataProperty("threshold", thresh*noise) \
                   << LogRec.endr

    ###########
    #
    # Build the DetectionSet for positive sources
    #

    dsPositive = det.DetectionSetF(img, det.Threshold(thresh*noise, det.Threshold.VALUE, True), "FP+", nPixMin)
    fpVecPositive = dsPositive.getFootprints()
    print "Positive detections: ", len(fpVecPositive)

    LogRec(useLog, Log.INFO) \
                   <<  "Positive detections" \
                   << DataProperty("nPositive", len(fpVecPositive)) \
                   << LogRec.endr

    ###########
    #
    # Build the DetectionSet for negative sources
    #

    dsNegative = det.DetectionSetF(img, det.Threshold(thresh*noise, det.Threshold.VALUE, False), "FP-", nPixMin)
    fpVecNegative = dsNegative.getFootprints()
    print "Negative detections: ", len(fpVecNegative)

    LogRec(useLog, Log.INFO) \
                   <<  "Negative detections" \
                   << DataProperty("nNegative", len(fpVecNegative)) \
                   << LogRec.endr


    ###########
    #
    # Measure the FootPrints
    #

    imgWCS = differenceImageExposure.getWcs()

    outputDiaSources = afwCat.DiaSourceVec()

    imgMeasure = det.MeasureF(img, "FP+")

    id = 0
    for i in range(len(fpVecPositive)):
        diaPtr = afwCat.DiaSourcePtr()
        diaPtr.setId(id)
        diaPtr.setFilterId(filterId);
        imgMeasure.measureSource(diaPtr, fpVecPositive[i], 0.0)   # NOTE explicit background of zero used for difference image
        pixCoord = afw.Coord2D(diaPtr.getColc(), diaPtr.getRowc())
        skyCoord = imgWCS.colRowToRaDec(pixCoord)
        diaPtr.setRa(skyCoord.x())
        diaPtr.setDec(skyCoord.y())
        outputDiaSources.push_back(diaPtr.get())
        id += 1
 
    imgMeasure = det.MeasureF(img, "FP-")

    for i in range(len(fpVecNegative)):
        diaPtr = afwCat.DiaSourcePtr()
        diaPtr.setId(id)
        diaPtr.setFilterId(filterId);
        imgMeasure.measureSource(diaPtr, fpVecNegative[i], 0.0)   # NOTE explicit background of zero used for difference image
        pixCoord = afw.Coord2D(diaPtr.getColc(), diaPtr.getRowc())
        skyCoord = imgWCS.colRowToRaDec(pixCoord)
        diaPtr.setRa(skyCoord.x())
        diaPtr.setDec(skyCoord.y())
        outputDiaSources.push_back(diaPtr.get())
        id += 1

    ###########
    #
    # Return the DiaSources
    #


    return outputDiaSources
