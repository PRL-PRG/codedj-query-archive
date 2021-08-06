import hype as he
import py

NAME = str(py.test.ensuretemp('test_db.db'))

def setup_module(mod):
    mod.db = he.Database(NAME)
    for title, content in [(u'1', u'one one'), (u'2', u'two two'), (u'3', u'three three')]:
        doc = he.Document()
        doc['@uri'] = title
        doc['@title'] = title
        doc.add_text(content)
        db.put_doc(doc)

def test_commit_remove():
    path = str(py.test.ensuretemp('test.db'))
    db = he.Database(path)
    try:
        assert len(db) == 0
        doc = he.Document()
        py.test.raises(he.DBRemoveError, db.remove, doc)
        py.test.raises(he.DBEditError, db.commit, doc)
        doc['@uri'] = u'test'
        db.put_doc(doc)
        assert len(db) == 1
        assert db.commit(doc)
        doc['@title'] = u'commit test'
        assert db.commit(doc)
        id = doc.id
        d = db.get_doc(doc.id)

        assert d['@title'] == doc['@title']
        assert len(db) == 1

        d_copy = db.get_doc_by_uri(u'test')
        assert d_copy['@title'] == d['@title'] == doc['@title']

        d_copy2 = db.get_doc_by_uri(u'commit test')
        assert not d_copy2 # commit test is @title not @uri

        doc2 = he.Document()
        doc2['@uri'] = u'another test'
        db.put_doc(doc2)
        id2 = doc2.id

        assert id2 != id
        assert len(db) == 2
        assert db.remove(doc)
        assert len(db) == 1

        db.flush()
        db.optimize()
        db.sync()
        assert not db.get_doc(id)
    finally:
        db.close()

def test_name():
    assert db.name == NAME

def test_len():
    assert len(db) == 3

def test_size():
    assert db.size > 0

def test_cache():
    assert db.used_cache
    assert db.records_in_cache
    db.set_cache_size(2**16, 60, 60, 60)
    db.set_special_cache_size('@uri', 60)

def test_crash_and_burn():
    db.close()
    py.test.raises(Exception, db.search)
    py.test.raises(Exception, db.__len__)

def teardown_module(mod):
    mod.db.close()

