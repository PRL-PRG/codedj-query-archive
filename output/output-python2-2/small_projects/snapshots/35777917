import datetime, os.path, sets, shutil, sys, time, unittest
from cStringIO import StringIO

import test_base
from pyclene import lucene


_THIS_FILENAME = test_base.getFilenameOfThisPythonTestModule(__file__)


def getFullTestSuite():
    suite = unittest.TestSuite()

    suite.addTest( unittest.makeSuite(FieldTest) )
    suite.addTest( unittest.makeSuite(DateFieldTest) )
    suite.addTest( unittest.makeSuite(DocumentTest) )

    return suite


class FieldTest(test_base.CommonBaseTest):
    def testConstructor_AND_properties(self):
        # Field(String name, String string, boolean store, boolean index, boolean token)

        # Construct from string:
        f = lucene.Field('theName', 'theValue', True, True, True)
        self.assertEqual(f.name, 'theName')
        self.assertEqual(f.value, 'theValue')
        self.assertEqual(f.isStored, True)
        self.assertEqual(f.isIndexed, True)
        self.assertEqual(f.isTokenized, True)

        # Construct from lucene.Reader instance:

        #_ABANDONED_:
        # PythonFileReader was eliminated because it was 90x slower than the
        # C++ FileReader (under a non-unicode build, at least).
        #pyFile = file(_THIS_FILENAME, 'rb')
        #rIn = lucene.PythonFileReader(pyFile)

        rIn = lucene.FileReader(_THIS_FILENAME)
        f = lucene.Field('theName', rIn, True, True, True)
        self.assertEqual(f.name, 'theName')
        self.assertEqual(f.value, rIn)
        # _ABANDONED_:PythonFileReader eliminated:
        #self.assertEqual(f.reader.read(), file(pyFile.name, 'rb').read())
        self.assertEqual(f.isStored, True)
        self.assertEqual(f.isIndexed, True)
        self.assertEqual(f.isTokenized, True)

        # Verify the immutability of the following properies:
        self.assertRaises(AttributeError, setattr, f, 'name', 'other')
        self.assertRaises(AttributeError, setattr, f, 'value', 'other')
        self.assertRaises(AttributeError, setattr, f, 'isStored', False)
        self.assertRaises(AttributeError, setattr, f, 'isIndexed', False)
        self.assertRaises(AttributeError, setattr, f, 'isTokenized', False)


    def test_staticFactories(self):
        fields = FieldTest._createFromStaticFactories()

        for f in fields[:-1]:
            self.assertEqual(f.name, 'mime-type')
            self.assertEqual(f.value, 'text/plain')

        f = fields[-1] # Reader-based rather than materialized-string-based.
        self.assertEqual(f.name, 'mime-type')
        self.assertNotEqual(f.value, None)


    def _createFromStaticFactories():
        fields = []
        for factoryName in (
            'Keyword', 'Unindexed', 'Text', 'Unstored'
          ):
            f = getattr(lucene.Field, factoryName)('mime-type', 'text/plain')
            fields.append(f)

        # The variant of lucene.Field.Text that takes a Reader value rather
        # than a string value.
        # _ABANDONED_:PythonFileReader eliminated:
        # pyFile = file(_THIS_FILENAME, 'rb')
        # rIn = lucene.PythonFileReader(pyFile)
        rIn = lucene.FileReader(_THIS_FILENAME)
        f = lucene.Field.Text('mime-type', rIn)
        fields.append(f)

        return fields
    _createFromStaticFactories = staticmethod(_createFromStaticFactories)


    def test___str__(self):
        # CLucene 0.8.9 overflowed a buffer if Field::toString was called on
        # a field with a large value.  Exercise the Field::toString method
        # (aliased as __str__ in the Python wrapper) to ensure that it behaves
        # reponsibly.
        f = lucene.Field('theName', 'theValue' * 500000, True, True, True)
        str(f); repr(f)
        f = lucene.Field('theName', '', True, True, True)
        str(f); repr(f)
        f = lucene.Field('', '', True, True, True)
        str(f); repr(f)


class DateFieldTest(test_base.CommonBaseTest):
    def test_staticMembers(self):
        self.assert_(lucene.DateField.MAX_DATE_STRING >= lucene.DateField.MIN_DATE_STRING)

        # timeToString should accept both ticks and datetime.datetime
        # instances.
        ticks = int(time.time())
        stringFromTicks = lucene.DateField.timeToString(ticks)
        ticksDateTime = datetime.datetime.fromtimestamp(ticks)
        stringFromDateTime = lucene.DateField.timeToString(ticksDateTime)
        self.assertEqual(stringFromTicks, stringFromDateTime)

        # stringToTicks (returns integer ticks):
        self.assertEqual(
            lucene.DateField.stringToTicks(stringFromTicks), ticks
          )

        # stringToTime (returns datetime.datetime object):
        self.assertEqual(
            lucene.DateField.stringToTime(stringFromTicks), ticksDateTime
          )


class DocumentTest(test_base.CommonBaseTest):
    def testConstructor(self):
        d = lucene.Document()
        return d


    def test_fieldMembershipAndRetrieval(self):
        # This case tests numerous Document methods, including the pyclene
        # equivalents of CLucene's Document::add, ::getField, and ::fields.

        def requireFieldSet(doc, reqFields):
            reqFields = sets.ImmutableSet(reqFields) # Order is undefined
            # Materialized:
            self.failIf(reqFields - sets.ImmutableSet(doc.fields()))
            # Iterated:
            self.failIf(reqFields - sets.ImmutableSet([field for field in doc]))

        d = self.testConstructor()
        requireFieldSet(d, [])

        # Make sure trying to add None raises a TypeError.
        self.assertRaises(TypeError, d.add, None)

        f1 = lucene.Field.Keyword('blah1', 'value200')
        d.add(f1)
        requireFieldSet(d, [f1])

        # Unlike CLucene's Document, pyclene's does not accept multiple fields
        # with the same name.
        f1_5 = lucene.Field.Keyword('blah1', 'value')
        self.assertRaises(KeyError, d.add, f1_5)

        f2 = lucene.Field.Keyword('blah2', 'value100')
        d.add(f2)
        requireFieldSet(d, [f1, f2])

        # _ABANDONED_:PythonFileReader eliminated:
        #pyFile = file(_THIS_FILENAME, 'rb')
        #rIn = lucene.PythonFileReader(pyFile)
        rIn = lucene.FileReader(_THIS_FILENAME)
        f3 = lucene.Field.Text('other', rIn)
        d.add(f3)
        requireFieldSet(d, [f1, f2, f3])

        f4 = lucene.Field.Keyword('blah3', 'value300')
        d.add(f4)
        requireFieldSet(d, [f1, f2, f3, f4])

        f5 = lucene.Field.Keyword('aardvark', 'value500')
        d.add(f5)
        requireFieldSet(d, [f1, f2, f3, f4, f5])


        self.assertEqual(d['aardvark'], 'value500')
        self.assertEqual(d['blah1'], 'value200')
        self.assertEqual(d['blah2'], 'value100')
        self.assertEqual(d['blah3'], 'value300')
        self.assertEqual(d['other'], rIn)

        self.assertEqual(
            sets.ImmutableSet([f for f in d.fields()]),
            sets.ImmutableSet([f1,f2,f3,f4,f5])
          )

        # A Document is immutable; a field cannot be removed once it's added.
        # Ensure that a Document does not support item deletion.
        try:
            del d['aardvark']
        except TypeError:
            pass
        else:
            self.fail('Attempt to del item from Document should not have succeeded.')


    def test___str__(self):
        # Exercise CLucene's Document::toString method.
        d = self.testConstructor()
        f = lucene.Field('theName', 'theValue' * 500000, True, True, True)
        d.add(f)
        str(d); repr(d)
        for f in FieldTest._createFromStaticFactories():
            str(d); repr(d)


if __name__ == '__main__':
    import test
    test.main(suite=getFullTestSuite())
