import os.path, random, shutil, sys, unittest
from cStringIO import StringIO

import test_base
from pyclene import lucene

if lucene.UNICODE_BUILD:
    import codecs


_THIS_FILENAME = test_base.getFilenameOfThisPythonTestModule(__file__)


def getFullTestSuite():
    suite = unittest.TestSuite()

    #_ABANDONED_:suite.addTest( unittest.makeSuite(BitSetTest) )

    suite.addTest( unittest.makeSuite(FileReaderTest) )
    suite.addTest( unittest.makeSuite(StringReaderTest) )

    #_ABANDONED_:suite.addTest( unittest.makeSuite(PythonFileReaderTest) )

    return suite


# CLucene's lucene::util::Reader subclasses are wrapped very opaquely (they
# duplicate functionality already in the Python standard library), so these
# tests exercise only the constructor and the close method.
class _ReaderTest(test_base.CommonBaseTest):
    def test_close(self):
        # The testConstructor method will be implemented by subclasses:
        x = self.testConstructor()

        x.close()
        self.assertRaises(IOError, x.close) # Already closed.


class FileReaderTest(_ReaderTest):
    def testConstructor(self):
        return lucene.FileReader(_nameOfTestFile())


class StringReaderTest(_ReaderTest):
    def testConstructor(self):
        return lucene.StringReader(_textOfTestFile())



class _ABANDONED_BitSetTest(test_base.CommonBaseTest):
    # BitSet is no longer exposed.
    def test(self):
        bs = lucene.BitSet(0)
        self.assertRaises(IndexError, bs.__getitem__, 0)
        self.assertRaises(IndexError, bs.__setitem__, 0, True)

        bs = lucene.BitSet(1)
        self.assert_(not bs[0])
        bs[0] = True
        self.assert_(bs[0])
        self.assertRaises(ValueError, bs.__setitem__, 0, False)

        self.assertRaises(IndexError, bs.__getitem__, 1)
        self.assertRaises(IndexError, bs.__setitem__, 1, True)


class _ABANDONED_PythonFileReaderTest(test_base.CommonBaseTest):
    def testConstructor(self):
        return self._createNonUnicodeReaderAndFile()

    def _createNonUnicodeReaderAndFile(self):
        f1 = file(_THIS_FILENAME, 'rb')
        f2 = file(_THIS_FILENAME, 'rb')
        r = lucene.PythonFileReader(f1)
        return r, f2

    def _createUnicodeReaderAndFile(self):
        unicodeFilename = os.path.join('data', 'utf8_3_byte_char.txt')
        f1 = codecs.open(unicodeFilename, encoding='UTF8')
        r = lucene.PythonFileReader(f1)
        f2 = codecs.open(unicodeFilename, encoding='UTF8')
        return r, f2

    def _chooseCreateMethod(self):
        if lucene.UNICODE_BUILD:
            return self._createUnicodeReaderAndFile
        else:
            return self._createNonUnicodeReaderAndFile


    def test_read_AND_available_AND_close(self):
        r, f = self._chooseCreateMethod()()
        self.assertEqual(r.read(), f.read()) # No length specified.
        r.close(); f.close()

        r, f = self._chooseCreateMethod()()
        thisFileSize = os.path.getsize(os.path.abspath(_THIS_FILENAME))
        totalRead = 0
        for nBytes in [2**n for n in range(10)]:
            rBytesRead, fBytesRead = r.read(nBytes), f.read(nBytes)
            self.assertEqual(rBytesRead, fBytesRead)
            totalRead += len(rBytesRead)
            self.assertEqual(r.available(), thisFileSize - totalRead)

        r.close(); f.close()
        self.assertRaises(IOError, r.close)


    def test_readChar(self):
        r, f = self._chooseCreateMethod()()
        for i in xrange(os.path.getsize(f.name)):
            self.assertEqual(r.readChar(), f.read(1))

        r.close(); f.close()
        self.assertRaises(IOError, r.readChar)


    def test_peek(self):
        r, f = self._chooseCreateMethod()()
        for i in xrange(os.path.getsize(f.name)):
            self.assertEqual(r.peek(), r.readChar())

        r.close(); f.close()
        self.assertRaises(IOError, r.peek)


    def test_seek_AND_position(self):
        r, f = self._chooseCreateMethod()()
        bytesInFile = os.path.getsize(f.name)

        for i in xrange(100):
            nextPos = random.randint(0, bytesInFile-1)
            r.seek(nextPos)
            f.seek(nextPos)
            self.assertEqual(r.position(), f.tell())

        r.close(); f.close()
        self.assertRaises(IOError, r.seek, 0)
        self.assertRaises(IOError, r.position)


    def test_readingFileContainingNullByte(self):
        # PythonFileReader should disallow the reading of null bytes from an
        # ASCII-oriented underlying stream (but it's fine from a
        # unicode-oriented underlying stream).
        f = file(os.path.join('data', 'file-containing-null-byte.bin'))
        pf = lucene.PythonFileReader(f)
        self.assertRaises(IOError, pf.read)
        f.close() ; pf.close()



# Utility functions:
def _nameOfTestFile():
    name = os.path.abspath(os.path.join(os.pardir, '_clucene_wrap.cpp'))
    assert os.path.isfile(name)
    return name

def _textOfTestFile():
    return file(_nameOfTestFile(), 'rb').read()


if __name__ == '__main__':
    import test
    test.main(suite=getFullTestSuite(), createTestIndex=False)
