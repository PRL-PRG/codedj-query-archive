import glob, os, os.path, sys, timeit

import find_pyclene_binaries
from pyclene import lucene

DIVIDER = '-' * 78

# timeitOrientedCallable is a "hook" in the global namespace for the sake
# of the timeit module.
timeitOrientedCallable = None


def main():
    global timeitOrientedCallable

    printPhase('Extraction and Indexing', precedingNewlines=0)

    sampleArchiveDir = os.path.abspath(os.sep.join([os.pardir] * 6))
    sampleArchiveFN = 'python-dev-parsed-text.tar.bz2'
    sampleArchivePath = os.path.join(sampleArchiveDir, sampleArchiveFN)
    if not os.path.exists(sampleArchivePath):
        print 'Sample data file not found at expected location'
        print '  %s' % sampleArchivePath
        print 'Please obtain %s and place it in' % sampleArchiveFN
        print '  %s' % sampleArchiveDir
        sys.exit(1)

    print '  Launching subprocess to create sample index from'
    print '    %s' % sampleArchivePath
    print '  (Extracting and indexing in subprocess avoids skewing performance'
    print '   stats of main process, which performs actual searches.)'
    print '\n  Please wait...',
    sys.stdout.flush()

    if sys.platform.lower().startswith('win'):
        pythonExe = '"%s"' % sys.executable
    else:
        pythonExe = 'python'
    indexDir = os.popen(
        '%s prepare_sample_data.py %s' % (pythonExe, sampleArchivePath)
      ).read().strip()

    print '\n' # Two newlines, since last print statement didn't include trailing newline.
    indexCreationSucceeded = os.path.isdir(indexDir)
    if indexCreationSucceeded:
        print '  Sample index is located at:\n    %s' % indexDir
    else:
        print '  Sample index creation failed.  Test process output:'
        print '  [%s]' % indexDir
        sys.exit(1)


    # Search phase:
    analyzer = lucene.StandardAnalyzer()
    queryText = '+"guido van rossum" -"tim peters" +compile +gcc'
    query = lucene.QueryParser.parse(queryText, 'contents', analyzer)

    def _performSearch():
        hits = searcher.search(query)
    timeitOrientedCallable = _performSearch

    timer = timeit.Timer(
        stmt='timeitOrientedCallable()',
        setup="from __main__ import timeitOrientedCallable"
      )

    reader = lucene.IndexReader.open(indexDir)
    for title in ('Disk', 'RAM'):
        printPhase('%s-Search' % title)
        print '  Test query is: [%s]' % query.toString('contents')

        searcher = lucene.IndexSearcher(reader)

        runs = 10
        searchesPerRun = 1000
        print '\n  Executing search test; please wait...',
        sys.stdout.flush()
        times = timer.repeat(repeat=runs, number=searchesPerRun)
        print
        searcher.close()

        printPhase('%s-Reporting' % title, precedingNewlines=1)

        print '  Executed %s runs (%s searches per run)' % (
            str(runs).rjust(3), str(searchesPerRun).rjust(5)
          )
        printCPUStats(runs, searchesPerRun, times)
        printMemoryStats()

        # Now that initial memory usage figures have been collected, create
        # a memory-resident copy of the index for the second pass, so as to
        # make the performance stats less dependent on disk quirks.
        ramDir = lucene.RAMDirectory()
        writer = lucene.IndexWriter(ramDir, analyzer, True)
        writer.addIndexes( [ lucene.FSDirectory(indexDir) ] )
        writer.close()
        # Replace disk-reader with RAM-reader.
        reader = lucene.IndexReader.open(ramDir)
        del ramDir, writer


    printPhase('Cleanup')

    print '  To facilitate analysis, this program intentionally does not'
    print '  delete the directories created by prepare_sample_data.py, namely:'
    for dirLoc in glob.glob(os.path.join(os.path.dirname(indexDir), 'pyclene__*')):
        print '    - %s' % dirLoc
    print
    print '  prepare_sample_data.py will, however, delete and recreate those'
    print '  directories each time it is run.'


def printPhase(phaseTitle, precedingNewlines=2):
    if precedingNewlines > 0:
        print '\n' * (precedingNewlines - 1)
    print DIVIDER
    print '%s Phase' % phaseTitle
    print DIVIDER


def printCPUStats(runs, searchesPerRun, times):
    def displayTime(t):
        return ('%0.5f' % t).rjust(12)

    print
    print '  CPU usage stats:'
    print '    Fastest run:  %s s ' % displayTime(min(times))
    print '    Slowest run:  %s s ' % displayTime(max(times))
    print '    Average run:  %s s ' % displayTime(sum(times) / len(times))


def printMemoryStats():
    if sys.platform.lower().startswith('win'):
        # Requires very recent version of pywin32 (build 203, for example).
        try:
            import win32process
        except ImportError:
            print
            print '  The win32process module (part of pywin32) is not installed,'
            print '  so memory usage stats are not available.'
            return
        memInfo = win32process.GetProcessMemoryInfo(win32process.GetCurrentProcess())
        memPeakkB = memInfo['PeakWorkingSetSize'] / 1024
    elif sys.platform.lower().startswith('linux'):
        # XXX: Not sure which Linux status entry is comparable to the mem info
        # retrieved on Windows, so I've disabled it for the moment:
        return
        #memPeakkB = int(
        #    [
        #      line for line in file('/proc/%s/status' % os.getpid(), 'rb')
        #      if line.startswith('VmSize')
        #    ][0].split()[1]
        #  )
    else:
        # Don't try to print any mem stats on operating systems that aren't
        # explicitly supported.
        return

    print
    print '  Memory usage stats:'
    print '    Peak mem usage: %s kB' % str(memPeakkB).rjust(10)


if __name__ == '__main__':
    main()
