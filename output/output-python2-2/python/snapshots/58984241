import random, shutil, sys, time
import hype as he
import xapwrap.index
from xapwrap.document import Document, TextField

#Results on my ibook G4 1.3Ghz with 512MB.
#indexing 50000 documents
#hyperestraier:
#    real   2m47.859s
#    user   1m35.070s
#    sys    0m40.958s
#xapwrap:
#    real  14m07.248s
#    user   5m42.155s
#    sys    0m34.266s

#Searching 100 times in 50000 documents:
#hyperestraier:
#    Timing 100 searches, repeating each search 1 times
#    Total search time:  7.49640250206
#    Time per search: 0.074964
#    Average hit rate: 818
#xapwrap:
#    Timing 100 searches, repeating each search 1 times
#    Total search time:  39.0512754917
#    Time per search: 0.390513
#    Average hit rate: 734
#The difference in hit rate is due to the documents and the queries being
#randomly generated.

#space occupation on disk
#hyperestraier:
#    dialtone@aiolia/Volumes/dati/Sviluppo/mw/3rd-party/hyperestraier/benchmark.db$ du
#    3624    ./_attr/0001
#    3608    ./_attr/0002
#    3624    ./_attr/0003
#    10864   ./_attr
#    58088   ./_idx
#    6952    ./_text/0001
#    6960    ./_text/0002
#    6952    ./_text/0003
#    6968    ./_text/0004
#    6976    ./_text/0005
#    6944    ./_text/0006
#    6952    ./_text/0007
#    48712   ./_text
#    118680  .

#xapwrap:
#    dialtone@aiolia/Volumes/dati/Sviluppo/mw/3rd-party/hyperestraier/benchmark.xap.db$ du
#    468664  .

#Summary:
#             hyperestraier          xapwrap         % variation (hypererstraier = 100)
#indexing   167s (~300 doc/s)    847s (59 doc/s)      19.71% as fast (or 507% slower)
#searching  7.49s (13.3 src/s)   39s (2.5 src/s)      19.20% as fast (or 520% slower)
#disk space      58MB                 229MB           25.30% smaller (or 394% bigger)

DB = 'benchmark.db'
XAPDB = 'benchmark.xap.db'

WORDS = open('benchmark.words').read().split()
WORDS_PER_DOC = (50, 100)
DOCS = 50000

SEARCHES = 100
SEARCH_REPEAT = 1 # Increase this if your computer is insanely fast.

def init():
    try:
        shutil.rmtree(DB)
    except OSError:
        pass
    db = he.Database(DB)
    try:
        for i in xrange(0, DOCS):
            if i%(DOCS/10) == 0:
                print 'Adding %d documents ...' % (DOCS/10)
            doc = he.Document()
            doc['@uri'] = str(i)
            text = ' '.join(random.sample(WORDS, random.randint(*WORDS_PER_DOC)))
            doc.add_text(text)
            db.put_doc(doc)
    finally:
        db.close()

def xap_init():
    try:
        shutil.rmtree(XAPDB)
    except OSError:
        pass
    db = xapwrap.index.SmartIndex(XAPDB, True)
    try:
        for i in xrange(0, DOCS):
            if i%(DOCS/10) == 0:
                print 'Adding %d documents ...' % (DOCS/10)
            text = ' '.join(random.sample(WORDS, random.randint(*WORDS_PER_DOC)))
            doc = Document(textFields = TextField('text', text))
            db.index(doc)
    finally:
        db.flush()
        db.close()

def _search(db):
    times = []
    results = []
    print 'Timing %s searches, repeating each search %d times'%(SEARCHES, SEARCH_REPEAT)
    for i in xrange(SEARCHES):
        phrase = ' AND '.join(random.sample(WORDS, random.randint(2,4)))
        start = time.time()
        for x in xrange(SEARCH_REPEAT):
            search = db.search(phrase)
            # This is needed to ensure the search is actually run
            len(search)
        end = time.time()
        times.append((end-start))
        results.append(len(search))
    total = sum(times)/SEARCH_REPEAT
    print 'Total search time: ', total
    print 'Time per search: %f' % (total/SEARCHES)
    print 'Average hit rate: %d' % (sum(results)/SEARCHES)


def search():
    db = he.Database(DB)
    try:
        _search(db)
    finally:
        db.close()

    db = xapwrap.index.SmartIndex(XAPDB)
    try:
        _search(db)
    finally:
        db.close()

def searchIter(db):
    phrase = ' AND '.join(random.sample(WORDS, 2))
    search = db.search(phrase)
    print len(search)

def main(args):
    if len(args):
        if args[0] == 'xapian':
            print args[0]
            xap_init()
        else:
            init()
    else:
        search()

if __name__ == '__main__':
    main(sys.argv[1:])
