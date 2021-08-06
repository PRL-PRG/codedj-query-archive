import py.test
import hype as he

def test_docid():
    db = he.Database(str(py.test.ensuretemp('test.db')))
    try:
        doc = he.Document(u'uri')
        assert doc.is_empty()
        assert doc.id == -1
        db.put_doc(doc)
        assert doc.id != -1
        doc_copy = doc.copy()
        assert doc_copy.id == doc.id
        assert doc_copy != doc
        doc = he.Document(u'uri2')
        doc.id = 4
        db.put_doc(doc)
        assert db.get_doc(4)['@uri'] == u'uri2'
    finally:
        db.close()

def test_attr():
    doc = he.Document(u'uri')
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
        doc = he.Document(u'title 1')
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
        doc = he.Document(u'test test')
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
        doc = he.Document(u'fooo')
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

def test_keywords():
    db = he.Database(str(py.test.ensuretemp('test.db123456')))
    try:
        TEXT = u'yoyo'
        doc = he.Document(u'1')
        doc.set_keywords({u'key1': 1, u'key2': 1})
        doc.add_text(TEXT)
        db.put_doc(doc)
        db.flush()
        db.sync()
        db.optimize()
        dbdoc = db.get_doc(doc.id)
        assert dbdoc.id == doc.id
        assert doc.get_keywords() == {u'key1': 1, u'key2': 1}
        assert dbdoc.get_keywords() == {u'key1': 1, u'key2': 1}
        assert dbdoc.get_keywords() == doc.get_keywords()
        db.remove_keywords_from(dbdoc.id)
        dbdoc = db.get_doc(dbdoc.id)
        assert dbdoc.get_keywords() == {}
    finally:
        db.close()

def test_etch():
    db = he.Database(str(py.test.ensuretemp('test.db_unique')))
    try:
        TEXT = u"I'm your baby and I want to be etched. What do you think? be be be be"
        doc = he.Document(u'1')
        doc.add_text(TEXT)
        d = db.etch(doc, 1)
        assert d == {u'be': 8246}
    finally:
        db.close()


