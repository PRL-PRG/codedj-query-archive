import unittest
import os, os.path, shutil, sys, tarfile, tempfile

import distutils.util

import settings

# If the user has specified a custom temp dir, switch to it.
customTempDir = settings.tempDir()
if customTempDir:
    tempfile.tempdir = customTempDir

# Add the compiled pyclene directory that corresponds to this version of Python
# (assuming such a directory can be found) to Python's import path.
pycleneLibPath = None

# Bubble upward until we find the 'build' directory created by distutils.
parDir = os.path.abspath(os.pardir)
lastParDir = None
while True:
    buildDir = os.path.abspath(os.path.join(parDir, 'build'))
    if os.path.isdir(buildDir):
        break
    parDir = os.path.dirname(parDir)
    if parDir == lastParDir:
        break # Couldn't find a 'build' directory.
    lastParDir = parDir

# If not build dir was found, assume that the user has explicitly added the
# pyclene to Python's path.
if os.path.isdir(buildDir):
    libDir = 'lib.%s-%s' % (
        distutils.util.get_platform(),
        '.'.join( [str(n) for n in sys.version_info[:2]] )
      )
    pycleneLibPath = os.path.join(buildDir, libDir)
    if not os.path.exists(pycleneLibPath):
        # If our initial guess didn't match a directory, revert to the first
        # 'build/lib*' directory available.
        libDirs = [d for d in os.listdir(buildDir) if os.path.isdir(d) and d.startswith('lib')]
        if len(libDirs) > 0:
            pycleneLibPath = os.path.join(buildDir, libDirs[0])

    if pycleneLibPath:
        sys.path.insert(0, pycleneLibPath)

from pyclene import lucene


class CommonBaseTest(unittest.TestCase):
    _textIndexTarFN = None

    def __init__(self, *args, **kwargs):
        unittest.TestCase.__init__(self, *args, **kwargs)

        self.trackedTempFiles = []


    def __del__(self):
        self.removeTrackedTempFiles()


    def setUp(self):
        if settings.isVerbose():
            print >> sys.stderr, '\n>>> BEGIN: %s <<<' % str(self)
        unittest.TestCase.setUp(self)


    def tearDown(self):
        self.removeTrackedTempFiles()

        if settings.isVerbose():
            print >> sys.stderr, '\n>>> END: %s <<<' % str(self)
        unittest.TestCase.tearDown(self)


    def getTestArchivePath():
        return CommonBaseTest._textIndexTarFN
    getTestArchivePath = staticmethod(getTestArchivePath)

    def setTestArchivePath(filename):
        CommonBaseTest._textIndexTarFN = filename
    setTestArchivePath = staticmethod(setTestArchivePath)

    def extractTestIndex(self, toDir=None):
        if toDir is None:
            toDir = self.getTempFilename()
            os.makedirs(toDir)
        if not os.path.isdir(toDir):
            raise IOError('toDir (%s) is not a directory.' % toDir)

        tarF = tarfile.open(CommonBaseTest.getTestArchivePath(), 'r')
        for indexComponent in tarF.getmembers():
            tarF.extract(indexComponent, toDir)
        tarF.close()

        return toDir


    def extractTestIndexToRAMDirectory(self):
        ramDir = lucene.RAMDirectory()
        w = lucene.IndexWriter(ramDir, lucene.StandardAnalyzer(), True)
        w.addIndexes( [ lucene.FSDirectory(self.extractTestIndex()) ] )
        w.close()
        del w
        assert len(ramDir.list()) > 0
        return ramDir


    def getTempFilename(self, ext='.pyclene_test_file', automaticallyDelete=True):
        filename = generateTempFilename(suffix=ext)
        if automaticallyDelete:
            self.trackThisTempFile(filename)
        return filename

    def trackThisTempFile(self, filename):
        self.trackedTempFiles.append(filename)

    def removeTrackedTempFiles(self):
        for filename in self.trackedTempFiles:
            try:
                if os.path.isfile(filename):
                    os.remove(filename)
                elif os.path.isdir(filename):
                    shutil.rmtree(filename)
            except (IOError, OSError), e:
                print >> sys.stderr, e
                print >> sys.stderr, 'Unable to remove temp file %s' % filename
        self.trackedTempFiles = []


def getFilenameOfThisPythonTestModule(rawFilename):
    # Given a test module's __file__, sanitize it as follows:
    # 1. Absolutize filename.
    # 2. Prefer the '.py' file over its bytecoded variants '.pyc'/'.pyo'.
    fn = os.path.abspath(rawFilename)
    ext = os.path.splitext(fn)[1]
    if ext.startswith('.py') and not fn.endswith('.py'):
        fn = fn[:-(len(ext)-3)]
    assert fn.endswith('.py')
    return fn


def generateTempFilename(suffix=''):
    tf = tempfile.NamedTemporaryFile(suffix=suffix)
    tempFilename = tf.name
    tf.close()
    return tempFilename


def listFilesDestinedForTestIndex():
    # Provide a common point at which
    allFilenames = []
    for root, subdirs, filenames in os.walk(os.path.abspath(
        os.path.join(*([os.pardir] * 2))
      )):
        filenames = [
            os.path.join(root, fn)
            for fn in filenames if testIndexFilenameFilter(fn)
          ]
        allFilenames.extend(filenames)

    allFilenames.sort()
    return allFilenames


def testIndexFilenameFilter(fn):
    return (
            os.path.splitext(fn)[1].lower() in (
                '.txt', '.py', '.h', '.hpp', '.c', '.cpp', '.cxx',
              )
        and not os.path.basename(fn).lower().startswith('vgerr')
      )
