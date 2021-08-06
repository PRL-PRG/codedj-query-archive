import py.test
import hype as he

def test_docid():
    db = he.Database(str(py.test.ensuretemp('test.db')))
    try:
        doc = he.Document()
        doc['@uri'] = 'uri'
        assert doc.id == -1
        db.put_doc(doc)
        assert doc.id != -1
    finally:
        db.close()

def test_attr():
    doc = he.Document()
    doc['@uri'] = 'uri'
    doc['@title'] = 'title'
    assert doc['@uri'] == 'uri'
    assert doc['@title'] == 'title'
    assert doc.get('@uri') == 'uri'
    assert doc.get('@title') == 'title'
    assert doc.get('missing') is None
    assert doc.get('missing', 123) == 123
    py.test.raises(he.DocModifyImmutableError, doc.__setitem__, '@uri', 'bla')
    py.test.raises(KeyError, doc.__getitem__, 'missing')

def test_commit_remove():
    db = he.Database(str(py.test.ensuretemp('test.db1')))
    try:
        doc = he.Document()
        py.test.raises(he.DocNeverAddedError, doc.remove)
        py.test.raises(he.DocNeverAddedError, doc.commit)
        doc['@uri'] = 'test test'
        db.put_doc(doc)
        assert doc.id
        id = doc.id
        py.test.raises(KeyError, doc.__getitem__, '@title')
        doc['@title'] = 'title'
        assert doc.commit()
        assert db.get_doc(id)['@title'] == doc['@title']
        assert len(db) == 1
        assert doc.remove()
        assert len(db) == 0
    finally:
        db.close()
