#!/usr/bin/env python
import os
import pdb
import sys
import unittest
import math

import lsst.mwi.utils as mwiu
import lsst.mwi.data as mwiData
import lsst.mwi.policy
import lsst.fw.Core.fwLib as fw
import lsst.fw.Core.fwCatalog as fwCat
import lsst.detection.detectionLib as det

__all__ = ["detection"]

def detection(differenceImageExposure, policy, filterId, footprintList=None):
    """Detect and measure objects in an incoming difference image Exposure
    
    Inputs:
    - differenceImageExposure: an lsst.fw.Exposure containing a difference MaskedImage and WCS
    - policy: the policy; required elements are...?
    - footprintList: a sequence of detection footprints on which to force measurement
        
    Returns:
    - an lsst.fw.DiaSourceVector
    """

    mwiu.Trace("lsst.detection.detection", 3,
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
    noise = math.sqrt(fw.mean_channel_value(varImg))

    mwiu.Trace("lsst.detection.detection", 3,
        "thresholdSigma = %r; noise = %r onPixMin = %r" % (thresh, noise, nPixMin))

    ###########
    #
    # Build the DetectionSet for positive sources
    #

    dsPositive = det.DetectionSetF(img, det.Threshold(thresh*noise, det.Threshold.VALUE, True), "FP+", nPixMin)
    fpVecPositive = dsPositive.getFootprints()
    print "Positive detections: ", len(fpVecPositive)

    ###########
    #
    # Build the DetectionSet for negative sources
    #

    dsNegative = det.DetectionSetF(img, det.Threshold(thresh*noise, det.Threshold.VALUE, False), "FP-", nPixMin)
    fpVecNegative = dsNegative.getFootprints()
    print "Negative detections: ", len(fpVecNegative)


    ###########
    #
    # Measure the FootPrints
    #

    imgWCS = differenceImageExposure.getWcs()

    outputDiaSources = fwCat.DiaSourceVec()

    imgMeasure = det.MeasureF(img, "FP+")

    id = 0
    for i in range(len(fpVecPositive)):
        diaPtr = fwCat.DiaSourcePtr()
        diaPtr.setId(id)
        diaPtr.setFilterId(filterId);
        imgMeasure.measureSource(diaPtr, fpVecPositive[i], 0.0)   # NOTE explicit background of zero used for difference image
        pixCoord = fw.Coord2D(diaPtr.getColc(), diaPtr.getRowc())
        skyCoord = imgWCS.colRowToRaDec(pixCoord)
        diaPtr.setRa(skyCoord.x())
        diaPtr.setDec(skyCoord.y())
        outputDiaSources.push_back(diaPtr.get())
        id += 1
 
    imgMeasure = det.MeasureF(img, "FP-")

    for i in range(len(fpVecNegative)):
        diaPtr = fwCat.DiaSourcePtr()
        diaPtr.setId(id)
        diaPtr.setFilterId(filterId);
        imgMeasure.measureSource(diaPtr, fpVecNegative[i], 0.0)   # NOTE explicit background of zero used for difference image
        pixCoord = fw.Coord2D(diaPtr.getColc(), diaPtr.getRowc())
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
