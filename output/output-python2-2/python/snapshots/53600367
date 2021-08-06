"""Support for image defects"""

import lsst.fw.Core.fwLib as fw
import lsst.mwi.policy as policy
import lsst.detection.detectionLib as detection

def policyToBadRegionList(policyFile):
    """Given a Policy file describing a CCD's bad pixels, return a vector of BadRegion::Ptr""" 

    badPixelsPolicy = policy.Policy.createPolicy(policyFile)
    badPixels = detection.DefectListT()
    
    d = badPixelsPolicy.getArray("Defects")
    for reg in d:
        col0 = reg.get("col0")
        ncol = reg.get("ncol")
        if not ncol:
            col1 = reg.get("col1")
            ncol = col1 - col0 - 1

        row0 = reg.get("row0")
        nrow = reg.get("nrow")
        if not nrow:
            row1 = reg.get("row1")
            nrow = row1 - row0 - 1

        bbox = fw.BBox2i(col0, row0, ncol, nrow)
        badPixels.push_back(detection.DefectPtrT(detection.Defect(bbox)))

    del badPixelsPolicy

    return badPixels
