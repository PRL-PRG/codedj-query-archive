import os, os.path, shutil, sys, tarfile, tempfile

import find_pyclene_binaries

runningAsMainProgram = __name__ == '__main__'

# Add the 'demo' directory to Python's import path so we can use the
# index_text_files program to effect the indexing:
sys.path.append(os.path.abspath(os.path.join(*( [os.pardir] * 3 + ['demo'] ))))
import index_text_files


def prepareSampleIndex(archiveFN):
    if not os.path.isfile(archiveFN):
        errMsg = '"%s" is not a file.' % str(archiveFN)
        if runningAsMainProgram:
            print >> sys.stderr, errMsg
            sys.exit(1)
        else:
            raise IOError(errMsg)

    sysTempDir = tempfile.gettempdir()

    # Create a work directory under the system temp dir:
    workDir = os.path.join(sysTempDir, 'pyclene__pydevbench_work')
    if os.path.isdir(workDir):
        shutil.rmtree(workDir)
    os.mkdir(workDir)

    # If the index directory exists, delete it.
    indexDir = os.path.join(sysTempDir, 'pyclene__pydevbench_index')
    if os.path.isdir(indexDir):
        shutil.rmtree(indexDir)

    # Under the work directory, create a directory into which we can extract
    # the sample data:
    inputFileDir = os.path.join(workDir, 'input_files')
    os.mkdir(inputFileDir)

    # Extract the sample data:
    tarF = tarfile.open(archiveFN, 'r:bz2')
    for indexComponent in tarF.getmembers():
        tarF.extract(indexComponent, inputFileDir)
    tarF.close()

    # Index the sample data:
    index_text_files.index(inputFileDir, indexDir)

    if runningAsMainProgram:
        print indexDir,
    else:
        return indexDir


if runningAsMainProgram:
    if len(sys.argv) != 2:
        print >> sys.stderr, 'Usage: python %s sampleArchiveFilename' % sys.argv[0]
        sys.exit(1)

    prepareSampleIndex(sys.argv[1])