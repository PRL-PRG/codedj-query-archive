import os, os.path, sys, time
from pyclene import lucene

runningAsMainProgram = __name__ == '__main__'
def log(s):
    if runningAsMainProgram:
        print s

def main():
    if len(sys.argv) != 3:
        print >> sys.stderr, 'Usage:\n  python %s source-dir index-dir' % sys.argv[0]
        sys.exit(1)

    sourceDir, indexDir = sys.argv[1:]
    prepareAndIndex(sourceDir, indexDir)


def index(sourceDir, indexDir):
    if not os.path.isdir(sourceDir):
        raise IOError('%s is not a directory.' % sourceDir)

    if os.path.exists(indexDir):
        raise IOError('%s already exists.' % indexDir)

    if not os.path.isdir(os.path.dirname(indexDir)):
        raise IOError('Parent directory of index dir %s does not exist.' % indexDir)

    start = time.time()
    nIndexed, totalFileSize = _performTheIndexOp(sourceDir, indexDir, ('.txt',))
    stop = time.time()

    log('-' * 70)
    log('Indexed %d documents (%d KB) in %.02f secs.' % (
        nIndexed, totalFileSize / 1024, stop-start
      ))
    log('-' * 70)


def _performTheIndexOp(sourceDir, indexDir, extensions):
    extensions = [ext.lower() for ext in extensions]
    w = lucene.IndexWriter(indexDir, lucene.StandardAnalyzer(), True)

    totalFileSize = 0
    for root, subdirs, filenames in os.walk(os.path.abspath(sourceDir)):
        filenames = [os.path.join(root, fn) for fn in filenames
            if os.path.splitext(fn)[1].lower() in extensions
          ]
        for filename in filenames:
            log('Indexing %s' % filename[len(sourceDir)+len(os.sep):])
            totalFileSize += os.path.getsize(filename)
            doc = FileDocument(filename)
            w.addDocument(doc)

    nIndexed = w.docCount()
    log('Optimizing index...')
    w.optimize()
    w.close()

    return nIndexed, totalFileSize


def FileDocument(filename):
    doc = lucene.Document()

    # Path:
    doc.add(lucene.Field.Text('path', filename))

    # Modtime:
    doc.add(lucene.Field.Keyword('modified',
        lucene.DateField.timeToString(os.path.getmtime(filename))
      ))

    # Contents:
    data = file(filename, 'rb').read()
    data = data.replace('\0', ' ')
    contentsField = lucene.Field.Text('contents', data)
    doc.add(contentsField)

    return doc


if __name__ == '__main__':
    main()
