"""
Simple example that indexes all .txt files it finds in the current directory
and provides a simple search interface to them.

You can download the entire RFC collection from a number of locations, i.e.
<http://www.rfc-editor.org/download.html>, and unpack it here.
"""

import os
import shutil
import sys
import hype as he

SOURCES = './sources'

def rfcs():
    for rfc in os.listdir(SOURCES):
        if os.path.splitext(rfc)[1] == '.txt':
            yield rfc

def rebuild():
    try:
        shutil.rmtree('db')
    except OSError:
        pass
    db = he.Database('db')
    for rfc in rfcs():
        print 'Indexing', rfc, '...'
        doc = he.Document()
        doc['@uri'] = rfc
        doc.add_text(open(os.path.join(SOURCES,rfc)).read())
        db.put_doc(doc)
    db.close()

def search(query):
    db = he.Database('db')
    search = db.search(query, simple=True)
    print 'Best match:', search[0]['@uri']
    print 'Top 10:', [doc['@uri'] for doc in search[:10]]
    print '%d of %d documents found' % (len(search), len(db))
    db.close()

if __name__ == '__main__':
    if sys.argv[1] == 'rebuild':
        rebuild()
    else:
        search(' '.join(sys.argv[1:]))
