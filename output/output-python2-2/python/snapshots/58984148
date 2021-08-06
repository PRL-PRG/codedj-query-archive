import py.test
import hype as he

def test_docid():
    db = he.Database(str(py.test.ensuretemp('test.db')))
    try:
        doc = he.Document()
        assert doc.is_empty()
        doc['@uri'] = u'uri'
        assert doc.id == -1
        db.put_doc(doc)
        assert doc.id != -1
        doc_copy = doc.copy()
        assert doc_copy.id == doc.id
        assert doc_copy != doc
        doc = he.Document()
        doc['@uri'] = u'uri2'
        doc.id = 4
        db.put_doc(doc)
        assert db.get_doc(4)['@uri'] == u'uri2'
    finally:
        db.close()

def test_attr():
    doc = he.Document()
    doc['@uri'] = u'uri'
    doc['@title'] = u'title'
    assert doc['@uri'] == u'uri'
    assert doc['@title'] == u'title'
    assert doc.get('@uri') == u'uri'
    assert doc.get('@title') == u'title'
    assert doc.get('missing') is None
    assert doc.get('missing', 123) == 123
    py.test.raises(he.DocModifyImmutableError, doc.__setitem__, '@uri', u'bla')
    py.test.raises(KeyError, doc.__getitem__, 'missing')

def test_python_in_python_out():
    db = he.Database(str(py.test.ensuretemp('test.db12')))
    try:
        from datetime import datetime
        dtt = he.dt_to_str
        t = datetime.now()
        doc = he.Document()
        doc['@uri'] = u'title 1'
        doc['@mdate'] = t
        doc['@cdate'] = t
        doc['@adate'] = t
        doc['@size'] = 1
        doc['@weight'] = 1
        db.put_doc(doc)
        doc1 = list(db.search().add(u'@mdate NUMEQ %s' % (dtt(t, 0),)))[0]
        doc2 = list(db.search().add(u'@mdate NUMEQ %s' % (dtt(t),)))[0]
        assert doc1.id == doc2.id
        assert dtt(doc1['@mdate'], 0) == dtt(doc['@mdate'], 0) == dtt(t, 0)
        assert dtt(doc1['@cdate'], 0) == dtt(doc['@cdate'], 0) == dtt(t, 0)
        assert dtt(doc1['@adate'], 0) == dtt(doc['@adate'], 0) == dtt(t, 0)
        assert doc1['@size'] == doc['@size'] == 1
        assert doc1['@weight'] == doc['@weight'] == 1
    finally:
        db.close()

def test_commit_remove():
    db = he.Database(str(py.test.ensuretemp('test.db1')))
    try:
        doc = he.Document()
        doc['@uri'] = u'test test'
        db.put_doc(doc)
        assert doc.id
        id = doc.id
        py.test.raises(KeyError, doc.__getitem__, '@title')
        doc['@title'] = u'title'
        assert db.commit(doc)
        assert db.get_doc(id)['@title'] == doc['@title']
        assert len(db) == 1
        assert db.remove(doc)
        assert len(db) == 0
    finally:
        db.close()

def test_text():
    db = he.Database(str(py.test.ensuretemp('test.db1234')))
    try:
        TEXT = u'yooooooo'
        doc = he.Document()
        doc['@uri'] = u'fooo'
        doc.add_text(TEXT)
        doc.add_text(TEXT)
        doc.add_hidden_text(TEXT)
        doc.add_hidden_text(TEXT)
        db.put_doc(doc)
        assert db.get_doc(doc.id).texts == [TEXT, TEXT]
        assert db.get_doc(doc.id).text == TEXT+u' '+TEXT
        assert db.get_doc(doc.id).hidden_text == TEXT+u' '+TEXT
    finally:
        db.close()

def test_text_post():
    py.test.skip("We need to implement a way to edit even the content of a document")
    db = he.Database(str(py.test.ensuretemp('test.db12345')))
    try:
        TEXT = u'yooooooo'
        doc = he.Document()
        doc['@uri'] = u'fooo'
        o = doc.id
        assert doc.id == -1
        db.put_doc(doc)
        o1 = doc.id
        assert doc.id != -1
        doc.add_text(TEXT)
        o2 = doc.id
        assert doc.id != -1
        db.commit(doc)
        db.flush()
        db.sync()
        db.optimize()
        o3 = doc.id
        assert doc.id != -1
        assert -1 == o != o1 == o2 == o3
        assert doc.texts
        assert doc.text
        assert db.get_doc(doc.id).texts == [TEXT]
    finally:
        db.close()

def test_keywords():
    db = he.Database(str(py.test.ensuretemp('test.db123456')))
    try:
        TEXT = u'yoyo'
        doc = he.Document()
        doc['@uri'] = u'1'
        doc.set_keywords({u'key1': u'value', u'key2': u'value2'})
        doc.add_text(TEXT)
        db.put_doc(doc)
        db.flush()
        db.sync()
        db.optimize()
        dbdoc = db.get_doc(doc.id)
        assert dbdoc.id == doc.id
        assert doc.get_keywords() == {u'key1': u'value', u'key2': u'value2'}
        assert dbdoc.get_keywords() == {u'key1': u'value', u'key2': u'value2'}
        assert dbdoc.get_keywords() == doc.get_keywords()
    finally:
        db.close()