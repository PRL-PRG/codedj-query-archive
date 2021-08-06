def runDet(diaPath):
    import os
    import sys
    import optparse

    import lsst.daf.data as dafBase
    import lsst.pex.policy
    import lsst.pex.logging
    import lsst.afw.image as afwImage
    import lsst.afw.detection as afwDet
    import lsst.detection as det
    import Detection

    defInDir = os.environ.get("FWDATA_DIR", "")
    moduleDir = os.path.split(__file__)[0]
    appDir = os.path.normpath(os.path.join("../../../",moduleDir))

    defDiaPath = os.path.join(defInDir, "871034p_1_MI")
    defPolicyPath = os.path.join(appDir, "pipeline/DetectionStagePolicy.paf")
    defVerbosity = 5 # change to 0 once this all works to hide all messages

    policyPath = defPolicyPath

    diaExposure = afwImage.ExposureF()
    diaExposure.readFits(diaPath)
    diaMaskedImage = diaExposure.getMaskedImage()
    diaWCS = diaExposure.getWcs()

    policy = lsst.pex.policy.Policy.createPolicy(policyPath)

    diaSources = Detection.detection(diaExposure, policy)

    for i in range(len(diaSources)):
        diaSource_i = diaSources[i]
        print diaSource_i.toString()


def testIt(diaPath):
    import lsst.daf.data as dafBase
    
    runDet(diaPath)
    # check for memory leaks
    memId0 = 0
    if dafBase.Citizen_census(0, memId0) != 0:
        print dafBase.Citizen_census(0, memId0), "Objects leaked:"
        print dafBase.Citizen_census(dafBase.cout, memId0)
    else:
        print "No Objects leaked"
