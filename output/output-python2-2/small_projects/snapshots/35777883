import unittest
import gc, os, os.path, shutil, sys, tarfile, time

import test_base
from pyclene import lucene # Belongs *after* the import of test_base.

import settings

import test_prep
import test_util
import test_analysis
import test_queryparser
import test_document
import test_index
import test_store
import test_search
import test_transactional_ram_directory


DIVIDER_1 = '=' * 79
DIVIDER_2 = '-' * 79


def getFullTestSuite():
    suite = unittest.TestSuite()

    for module in (
        test_prep,
        test_util,
        test_analysis,
        test_queryparser,
        test_document,
        test_index,
        test_store,
        test_search,
        test_transactional_ram_directory,
      ):
        suite.addTest( module.getFullTestSuite() )

    return suite


def main(suite=getFullTestSuite(), createTestIndex=True):
    # Usage:
    #  'python test.py' for a single run;
    #  'python test.py inf' to run indefinitely, until KeyboardInterrupt or
    #     process kill

    if createTestIndex:
        start = time.time()
        unoptIndexTarFN, optIndexTarFN = createTestIndexArchive()
        end = time.time()
        pv('\nTest index creation took %0.03f seconds.' % (end - start))
    else:
        unoptIndexTarFN, optIndexTarFN = '', ''

    pv('\n--- RUNNING TESTS ---')

    try:
        args = sys.argv[1:]
        if len(args) == 0:
            limit = 1
        else:
            if args[0].lower() == 'inf':
                limit = None
            else:
                try:
                    limit = int(args[0])
                except ValueError:
                    pr('Iteration limit must be "inf" or an integer.')
                    sys.exit(1)
            pr('Iteration limit is %s' % limit)

        startTime = time.time()
        iterNo = 1L
        testsFailed = False
        try:
            while (not testsFailed) and (limit is None or iterNo < limit+1):
                runSecs = time.time() - startTime
                runMins, runSecs = [int(x) for x in divmod(runSecs, 60)]
                runHours, runMins = [int(x) for x in divmod(runMins, 60)]
                (runHours, runMins, runSecs) = [
                    str(x).zfill(2) for x in (runHours, runMins, runSecs)
                  ]

                pr('\n')
                pr(DIVIDER_1)
                pr('> Iteration %d (@ %s) (dur %s)' % (
                    iterNo, time.asctime(), '%s:%s:%s' % (runHours, runMins, runSecs)
                  ))
                pr(DIVIDER_1)

                iterStartTime = time.time()
                iterPassesActuallyRun = 0
                iterTestCaseCount = 0
                for passNo, (optFlag, tarFN) in enumerate((
                    ('Optimized', optIndexTarFN),
                    ('Unoptimized', unoptIndexTarFN),
                  )):
                    pr('\n')
                    pr(DIVIDER_2)
                    pr('>> [Iteration %d, Pass %d - %s Index]'
                        % (iterNo, passNo + 1, optFlag)
                      )
                    pr(DIVIDER_2)
                    test_base.CommonBaseTest.setTestArchivePath(tarFN)

                    runResults = unittest.TextTestRunner().run(suite)
                    iterPassesActuallyRun += 1

                    printDebuggingInfo()

                    iterTestCaseCount += runResults.testsRun
                    # Exit the loop as soon as a test run fails.
                    if not runResults.wasSuccessful():
                        testsFailed = True
                        break

                pr('\n')
                pr(DIVIDER_1)
                pr('> Iteration %d ran a total of %d test cases across %s in %0.2fs'
                    % (iterNo, iterTestCaseCount,
                        '%s pass%s' % (
                            iterPassesActuallyRun,
                            (iterPassesActuallyRun != 1 and 'es') or ''
                          ),
                        time.time() - iterStartTime
                      )
                  )
                pr(DIVIDER_1)

                iterNo += 1
        except KeyboardInterrupt:
            pr('\nShutting down because of user interrupt...')
    finally:
        if createTestIndex:
            removeTestIndexArchives((optIndexTarFN, unoptIndexTarFN))


def createTestIndexArchive():
    # Create a tar file containing the index of pyclene's own source code.
    # Place that tar file in the temp directory, and set a global reference
    # to it so that test code that requires a sample index can use it
    # (typically via one of the test_base.CommonBaseTest.extractTestIndex*
    # methods).

    pv('--- CREATING TEST INDEX OF OWN SOURCE CODE ---')

    allFilenames = test_base.listFilesDestinedForTestIndex()

    commonPrefix = os.path.commonprefix(allFilenames)
    commonPrefixLen = len(commonPrefix)

    tempIndexDir = test_base.generateTempFilename(suffix='_pyclene_test_index')
    assert not os.path.exists(tempIndexDir)
    fsDir = lucene.FSDirectory(tempIndexDir, True)
    assert os.path.isdir(tempIndexDir)

    w = lucene.IndexWriter(fsDir, lucene.StandardAnalyzer(), True)
    w.maxFieldLength = sys.maxint
    assert w.docCount() == 0
    for filename in allFilenames:
        doc = test_index.FileDocument(filename)
        pv('Indexing [%s]' % filename[commonPrefixLen:])
        w.addDocument(doc)

    assert w.docCount() == len(allFilenames)
    w.close()

    pv('\n')
    indexFNs = []
    try:
        try:
            for optFlag in ('UNOPTIMIZED', 'OPTIMIZED'):
                tarFilename = test_base.generateTempFilename(
                    '.pyclene_test_index-%s.tar' % optFlag
                  )
                indexFNs.append(tarFilename)
                tarFile = tarfile.open(tarFilename, 'w')
                for indexComponent in fsDir:
                    fullIndexComponent = os.path.join(fsDir.name, indexComponent)
                    tarFile.add(fullIndexComponent, indexComponent)
                tarFile.close()

                w = lucene.IndexWriter(fsDir, lucene.StandardAnalyzer(), False)
                w.optimize()
                w.close()

                pv('%s: %s KB' % (
                    ('Test index size (%s)' % optFlag).ljust(29),
                    str(os.path.getsize(tarFilename) / 1024).rjust(10)
                  ))
        except:
            origEx = sys.exc_info()[1]
            for tempFN in indexFNs:
                if os.path.isfile(tempFN):
                    os.remove(tempFN)

            raise origEx
    finally:
        shutil.rmtree(tempIndexDir)

    return indexFNs


def removeTestIndexArchives(filenames):
    # tiaPath = test_base.CommonBaseTest.getTestArchivePath()
    for tiaPath in filenames:
        if tiaPath is not None and os.path.exists(tiaPath):
            try:
                os.remove(tiaPath)
            except:
                pr('Unable to remove test index archive at "%s"' % tiaPath)
            else:
                test_base.CommonBaseTest.setTestArchivePath(None)


def printDebuggingInfo():
    # Don't try to print the refcount info if this build of Python wasn't
    # configured with sufficient debugosity (see Misc/SpecialBuilds.txt in the
    # Python source distribution).
    if not (hasattr(sys, 'gettotalrefcount') and hasattr(sys, 'getcounts')):
        return

    collectCount = gc.collect()
    pr(DIVIDER_1)
    pr('EXTREME_DEBUG Report')
    pr(DIVIDER_1)
    pr('collectCount: %d' % collectCount)
    pr('sys.gettotalrefcount(): %d' % sys.gettotalrefcount())

    pr(DIVIDER_1)
    counts = sys.getcounts()

    typeStats = {}
    for (tp_name, tp_allocs, tp_frees, tp_maxalloc) in counts:
        typeStats[tp_name] = {'allocs': tp_allocs, 'frees': tp_frees, 'maxalloc': tp_maxalloc}

    for (tp_name, tp_allocs, tp_frees, tp_maxalloc) in counts:
        if 'Test' in tp_name:
            continue
        if tp_allocs != tp_frees:
            def doStatsOffset(companionStats):
                return (
                       (abs(tp_allocs - tp_frees) == companionStats['frees'])
                    or (abs(tp_frees - tp_allocs) == companionStats['allocs'])
                  )

            if tp_name.endswith('Ptr') and tp_name[:-3] in typeStats:
                companionTypeName = tp_name[:-3]
                companionStats = typeStats[companionTypeName]
                if doStatsOffset(companionStats):
                    pr('%s offset by companion %s' % (
                        tp_name.ljust(40), companionTypeName
                      ))
                    continue
            elif tp_name + 'Ptr' in typeStats:
                companionTypeName = tp_name + 'Ptr'
                companionStats = typeStats[companionTypeName]
                if doStatsOffset(companionStats):
                    pr('%s offset by companion %s' % (
                        tp_name.ljust(40), companionTypeName
                      ))
                    continue


            pr('%s (%s)' % (tp_name.ljust(50), str(tp_allocs - tp_frees).rjust(7)))
    pr(DIVIDER_1)

    sys.stderr.flush()


# pr prints unconditionally; pv only if verbosity is on.
def pr(obj):
    print >> sys.stderr, obj

if settings.isVerbose():
    def pv(obj):
        print >> sys.stderr, obj
else:
    def pv(obj):
        pass


if __name__ == '__main__':
    main()
