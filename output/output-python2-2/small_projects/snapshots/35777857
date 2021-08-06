# This is a loose translation (a bit more elaborate) of the Java-Lucene demo
# SearchFiles.java

import os, os.path, sys, time
from pyclene import lucene


def main():
    if len(sys.argv) != 2:
        print >> sys.stderr, 'Usage:\n  python %s index-directory' % sys.argv[0]
        sys.exit(1)

    indexDir = sys.argv[1]

    searcher = lucene.IndexSearcher(indexDir)
    analyzer = lucene.StandardAnalyzer()

    indexSize = sum([os.path.getsize(os.path.join(indexDir, f)) for f in os.listdir(indexDir)])
    print 'Searching %d-document, %0.1f MB index at\n  [%s]' % (
        searcher.maxDoc(), indexSize / 1024.0**2, indexDir
      )

    try:
        searchLoop(searcher, analyzer)
    except (EOFError, KeyboardInterrupt):
        pass


def searchLoop(searcher, analyzer):
    for rawQuery in userSuppliedRawQueries():
        query = lucene.QueryParser.parse(rawQuery, 'contents', analyzer)
        print 'Searching for: [%s]' % query.toString('contents')

        start = timer()
        hits = searcher.search(query)
        stop = timer()

        print '-' * 70
        print 'Search took %.06f seconds.' % (stop-start)
        print '%d total matching documents' % len(hits)
        print '-' * 70

        for i, doc in enumerate(hits):
            if not userWantsAnotherResultPage(i):
                print '-' * 70
                break
            try:
                docTitle = doc['path']
            except KeyError:
                docTitle = str(doc)
            print '%d. %s' % (i + 1, docTitle)


def userSuppliedRawQueries():
    while True:
        rawQuery = raw_input('Query: ').strip()
        if rawQuery == '':
            raise EOFError
        yield rawQuery


def userWantsAnotherResultPage(resultPos):
    if resultPos == 0 or resultPos % 10 != 0:
        return True
    else:
        more = None
        while more not in ('y','n','yes','no',''):
            more = raw_input('More (y/n) ? ').strip().lower()
        return more not in ('n', 'no')


def timer():
    if sys.platform.lower().startswith('win'):
        return time.clock()
    else:
        return time.time()


if __name__ == '__main__':
    main()
