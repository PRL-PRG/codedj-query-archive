#!/usr/bin/env python
"""Run the image subtraction and detection pipeline on a series of images
and send the event to trigger the association pipeline
"""
from __future__ import with_statement

import os
import sys
import optparse
import subprocess
import socket

import lsst.mwi.data as mwiData
import lsst.mwi.utils
import lsst.dps.startPipeline

def main():
    pipelineDir = os.path.dirname(os.path.abspath(__file__))
    parentDir = os.path.dirname(pipelineDir)

    defSubtractionPolicyPath = os.path.join(parentDir, "imageSubtraction.paf")
    defDetectionPolicyPath = os.path.join(parentDir, "detection.paf")
    defVerbosity = 0
    
    usage = """usage: %%prog [options] runId

Notes:
- runId is an informative string; for test runs include your initials
- policy paths are relative to %s
- default --subpolicy=%s
- default --detpolicy=%s""" % (pipelineDir, defSubtractionPolicyPath, defDetectionPolicyPath)
    
    parser = optparse.OptionParser(usage)
    parser.add_option("-c", "--create", action="store_true", default=False,
        help="create DC2 I/O directories and database tables?")
    parser.add_option("-s", "--subpolicy", default=defSubtractionPolicyPath, help="image subtract policy file")
    parser.add_option("-d", "--detpolicy", default=defDetectionPolicyPath, help="detection policy file")
    parser.add_option("-v", "--verbosity", type=int, default=defVerbosity,
        help="verbosity of diagnostic trace messages; default=%s" % (defVerbosity,))
    (options, args) = parser.parse_args()
    
    if len(args) < 1:
        print "Error: runId required"
        sys.exit(0)
        
    runId = args[0]
    subtractionPolicyPath = options.subpolicy
    detectionPolicyPath = options.detpolicy
    doCreate = options.create

    print "Image Subtraction Policy file:", subtractionPolicyPath
    print "Detection Policy file:", detectionPolicyPath
    print "RunId:", runId
    
    def copyTemplatedConfigFile(templateName, templateDict):
        """Read a templated configuration file, fill it in and write it out.
        templateName is a path relative to pipelineDir
        templateDict contains the items to substitute in the template file
        """
        #print "copyTemplatedConfigFile(%r, %r)" % (templateName, templateDict)
        templateBaseName, templateExt = os.path.splitext(templateName)
        if not templateBaseName.endswith("_template"):
            raise RuntimeError("Invalid templateName %r; must end with _template" % (templateName,))
        inPath = os.path.join(pipelineDir, templateName)
        with file(inPath, "rU") as templateFile:
            templateText = templateFile.read()
            destText = templateText % templateDict
        outName = templateBaseName[:-len("_template")] + templateExt
        outPath = os.path.join(pipelineDir, outName)
        with file(outPath, "w") as destFile:
            destFile.write(destText)
    
    # write configuration files, filling in templates as required
    copyTemplatedConfigFile(
        "nodelist_template.scr",
        dict(
            ipaddress = socket.gethostname(),
        ),
    )
    copyTemplatedConfigFile(
        "pipeline_policy_template.paf",
        dict(
            imageSubtractionPolicyPath = subtractionPolicyPath,
            detectionPolicyPath = detectionPolicyPath,
        ),
    )
    
    if options.verbosity > 0:
        print "Verbosity =", options.verbosity
        lsst.mwi.utils.Trace_setVerbosity("lsst.imageproc", options.verbosity)
    lsst.mwi.utils.Trace_setVerbosity("dps", 3)
    
    print """Starting the pipeline.
Once you see a message like:
  Python Slice handleEvents rank :  0  - waiting on receive...
then from a new shell run one of:
    eventgenerator.py < dataFile
or (for one sample event):
    triggervisitevent.py
to feed images to the image subtraction pipeline.

Control-C the pipeline when it is done (or you have had enough).
"""
    nodeList = os.path.join(pipelineDir, "nodelist.scr")
    lsst.dps.startPipeline.startPipeline(nodeList, "pipeline_policy.paf", runId, doCreate, doCreate)

if __name__ == "__main__":
    memId0 = mwiData.Citizen_getNextMemId()
    main()
    # check for memory leaks
    if mwiData.Citizen_census(0, memId0) != 0:
        print mwiData.Citizen_census(0, memId0), "Objects leaked:"
        print mwiData.Citizen_census(mwiData.cout, memId0)
