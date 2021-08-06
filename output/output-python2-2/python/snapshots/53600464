def runDet(diaPath):
    import os
    import sys
    import optparse

    import lsst.mwi.data as mwiData
    import lsst.mwi.policy
    import lsst.fw.Core.fwLib as fw
    import lsst.mwi.utils
    import lsst.fw.Core.fwLib as fw
    import lsst.fw.Core.fwCatalog as fwCat
    import lsst.detection.detectionLib as det
    import Detection

    defInDir = os.environ.get("FWDATA_DIR", "")
    moduleDir = os.path.split(__file__)[0]
    appDir = os.path.normpath(os.path.join("../../../",moduleDir))

    defDiaPath = os.path.join(defInDir, "871034p_1_MI")
    defPolicyPath = os.path.join(appDir, "pipeline/DetectionStagePolicy.paf")
    defVerbosity = 5 # change to 0 once this all works to hide all messages

    policyPath = defPolicyPath

    diaExposure = fw.ExposureF()
    diaExposure.readFits(diaPath)
    diaMaskedImage = diaExposure.getMaskedImage()
    diaWCS = diaExposure.getWcs()

    policy = lsst.mwi.policy.Policy.createPolicy(policyPath)

    diaSources = Detection.detection(diaExposure, policy)

    for i in range(len(diaSources)):
        diaSource_i = diaSources[i]
        print diaSource_i.toString()


def testIt(diaPath):
    import lsst.mwi.data as mwiData
    
    runDet(diaPath)
    # check for memory leaks
    memId0 = 0
    if mwiData.Citizen_census(0, memId0) != 0:
        print mwiData.Citizen_census(0, memId0), "Objects leaked:"
        print mwiData.Citizen_census(mwiData.cout, memId0)
    else:
        print "No Objects leaked"
