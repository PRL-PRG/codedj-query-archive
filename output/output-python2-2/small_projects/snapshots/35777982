import os.path, shutil, struct, sys, unittest
from cStringIO import StringIO

import test_base
from pyclene import lucene


def getFullTestSuite():
    suite = unittest.TestSuite()

    if lucene.SUMO_BUILD:
        suite.addTest( unittest.makeSuite(InputStreamTest) )
        suite.addTest( unittest.makeSuite(FSInputStreamTest) )

        suite.addTest( unittest.makeSuite(OutputStreamTest) )
        suite.addTest( unittest.makeSuite(FSOutputStreamTest) )

    suite.addTest( unittest.makeSuite(DirectoryTest) )
    suite.addTest( unittest.makeSuite(FSDirectoryTest) )
    suite.addTest( unittest.makeSuite(RAMDirectoryTest) )

    if lucene.SUMO_BUILD:
        suite.addTest( unittest.makeSuite(LockTest) )
        suite.addTest( unittest.makeSuite(LockWithTest) )
        suite.addTest( unittest.makeSuite(FSLockTest) )

    return suite


class InputStreamTest(test_base.CommonBaseTest):
    def testConstructor(self):
        # Ensure that this abstract class can't be instantiated.
        # Conventionally, NotImplementedError would be raised to indicate
        # abstraction, but SWIG raises RuntimeError.
        self.assertRaises(RuntimeError, lucene.InputStream)


class FSInputStreamTest(test_base.CommonBaseTest):
    def _getPythonFileObjectReadingThisFile(self):
        return file(test_base.getFilenameOfThisPythonTestModule(__file__), 'rb')

    def testConstructor_String(self):
        filename = test_base.getFilenameOfThisPythonTestModule(__file__)
        fsIn = lucene.FSInputStream(filename)
        return fsIn


    def test_clone(self):
        orig = self.testConstructor_String()
        clone = orig.clone()
        self.failUnless( isinstance(clone, lucene.InputStream) )
        return orig, clone


    def test_isClone(self):
        for (orig, clone) in (
            self.test_clone(),
          ):
            self.failIf(orig.isClone)
            self.failUnless(clone.isClone)


    def test_close(self):
        fsIn = self.testConstructor_String()
        fsIn.close()
        self.assertRaises(IOError, fsIn.close) # Already closed.


    def test_readByte(self):
        thisFile = self._getPythonFileObjectReadingThisFile()
        firstByteFromPythonFileObject = thisFile.read(1)
        thisFile.close()

        fsIn = self.testConstructor_String()
        firstByteFromCLuceneStream = fsIn.readByte()
        fsIn.close()

        self.assertEqual(firstByteFromPythonFileObject, firstByteFromCLuceneStream)

        self.assertRaises(IOError, fsIn.readByte) # Already closed.


    def test_readBytes(self):
        thisFile = self._getPythonFileObjectReadingThisFile()
        fromPyFile = thisFile.read(11)
        thisFile.close()

        fsIn = self.testConstructor_String()

        fromCLStream = fsIn.readBytes(5)
        self.assertEqual(fromPyFile[:5], fromCLStream)

        fromCLStream = fsIn.readBytes(6)
        self.assertEqual(fromPyFile[5:], fromCLStream)

        # Trying to read past the end of the stream should raise IOError
        # (rather than crashing).  Even raising an IOError for read-overshooot
        # is unpythonic, but pyclene adds a Python-style read method that
        # behaves in a pythonic way (see test_read_PythonFileLike).
        self.assertRaises(IOError, fsIn.readBytes, 60000000)

        fsIn.close()
        self.assertRaises(IOError, fsIn.readBytes, 100) # Already closed.


    def test_read_PythonFileLike(self):
        thisFile = self._getPythonFileObjectReadingThisFile()
        pyFileBytes = thisFile.read()
        thisFile.close()

        # Test the no-arg version of read first.
        fsIn = self.testConstructor_String()
        fromCLStream = fsIn.read() # No nBytes arg.
        fsIn.close()
        self.assertEqual(pyFileBytes, fromCLStream)

        # Now test the nBytes-arg version of read.
        sizeOfThisFile = len(pyFileBytes)
        chunkSize = sizeOfThisFile / 2
        fsIn = self.testConstructor_String()
        fromCLStream1 = fsIn.read(chunkSize) # nBytes arg provided.
        # Using this Pythonic version of the read method, trying to read past
        # the end of the file should not cause a problem.
        fromCLStream2 = fsIn.read(chunkSize * 2) # nBytes arg provided.
        fsIn.close()
        self.assertEqual(pyFileBytes[:chunkSize], fromCLStream1)
        self.assertEqual(pyFileBytes[chunkSize:], fromCLStream2)

        self.assertRaises(IOError, fsIn.read) # Already closed.


    def test_readInt(self):
        thisFile = self._getPythonFileObjectReadingThisFile()
        # Lucene index format requires unsigned, big-endian storage:
        first4BytesAsInt_Py = struct.unpack('>I', thisFile.read(4))[0]
        thisFile.close()

        fsIn = self.testConstructor_String()
        first4BytesAsInt_CL = fsIn.readInt()
        fsIn.close()

        self.assertEqual(first4BytesAsInt_Py, first4BytesAsInt_CL)

        self.assertRaises(IOError, fsIn.readInt) # Already closed.


    def _test_readVariableInt(self, readMethod):
        # "VInt" refers to "variable integer" (a defined part of the Lucene
        # File Format).
        # In this test case, we use lucene.FSInputStream.readVInt() to read a
        # couple of files containing binary representations Lucene variable-
        # length integers.  We verify that readVInt() yields the correct value.
        vint128Filename = os.path.join('data', 'vint-128.bin')
        fsIn = lucene.FSInputStream(vint128Filename)
        vint128 = readMethod(fsIn)
        fsIn.close()
        self.assertEqual(vint128, 128)

        vint16385Filename = os.path.join('data', 'vint-16385.bin')
        fsIn = lucene.FSInputStream(vint16385Filename)
        vint16385 = readMethod(fsIn)
        fsIn.close()
        self.assertEqual(vint16385, 16385)

        self.assertRaises(IOError, readMethod, fsIn) # Already closed.

    def test_readVInt(self):
        self._test_readVariableInt(lucene.FSInputStream.readVInt)

    def test_readVLong(self):
        # The readVLong method is an artifact of C's type system.  From
        # Python's perspective, there's no essential difference between
        # readVInt and readVLong.  The underlying file format is the same.
        self._test_readVariableInt(lucene.FSInputStream.readVLong)


    def test_readLong(self):
        testDataFN = os.path.join('data', 'bigendian-64bit-int--123456789012345678.bin')

        # Lucene index format specified unsigned, big-endian storage.
        # Uint64: "64-bit unsigned integers are written as eight bytes,
        # high-order bytes first. "
        rawBytes_Py = file(testDataFN, 'rb').read(8)
        testInt_PyBigEndianUnsigned = struct.unpack('>Q', rawBytes_Py)[0]

        fsIn = lucene.FSInputStream(testDataFN)
        testInt_CL = fsIn.readLong()
        fsIn.close()

        if testInt_CL != testInt_PyBigEndianUnsigned:
            maxIntLen = max([len(str(i)) for i in (testInt_CL, testInt_PyBigEndianUnsigned)])
            raise IOError('Invalid interpretation of stored long.'
                '\nCLucene returned\n  [%s], but should have returned\n  [%s]'
                % (str(testInt_CL).rjust(maxIntLen),
                   str(testInt_PyBigEndianUnsigned).rjust(maxIntLen)
                  )
              )


        self.assertRaises(IOError, fsIn.readLong) # Already closed.


    def test_readString(self):
        # A "String" is a packed (32-bit length, data) pair.
        filename = os.path.join('data', 'sized-string-128.bin')

        fsIn = lucene.FSInputStream(filename)
        valueCL = fsIn.readString()
        fsIn.close()

        self.assertEqual('ABCD'*32, valueCL)

        self.assertRaises(IOError, fsIn.readString) # Already closed.


    def test_readChars(self):
        # YYY: Revisit this test case when unicode support has been added.
        thisFile = self._getPythonFileObjectReadingThisFile()
        fromPyFile = thisFile.read(200)
        thisFile.close()

        fsIn = self.testConstructor_String()
        fromCLStream = fsIn.readChars(200)
        fsIn.close()

        self.assertEqual(fromPyFile, fromCLStream)

        self.assertRaises(IOError, fsIn.readChars, 200) # Already closed.


    def test_filePointer(self):
        fsIn = self.testConstructor_String()
        fsIn.readBytes(10)
        self.assertEqual(fsIn.filePointer, 10)
        fsIn.readBytes(1025)
        self.assertEqual(fsIn.filePointer, 1035)
        fsIn.close()


    def test_seek(self):
        # To test seek, first read some data, seek backward, read some data,
        # seek forward, and read some data.  Ensure that the results of those
        # operations with a Python file object match those with an
        # FSInputStream object.
        thisFile = self._getPythonFileObjectReadingThisFile()
        fromPy_1 = thisFile.read(200)[1:]
        thisFile.seek(1)
        fromPy_2 = thisFile.read(199)
        thisFile.seek(1025)
        fromPy_3 = thisFile.read(50)
        thisFile.close()

        fsIn = self.testConstructor_String()
        fromCL_1 = fsIn.readBytes(200)[1:]
        fsIn.seek(1)
        fromCL_2 = fsIn.readBytes(199)
        fsIn.seek(1025)
        fromCL_3 = fsIn.readBytes(50)
        fsIn.close()

        self.assertEqual(fromPy_1, fromPy_2)
        self.assertEqual(fromPy_1, fromCL_1)
        self.assertEqual(fromCL_1, fromCL_2)
        self.assertEqual(fromPy_3, fromCL_3)

        self.assertRaises(IOError, fsIn.seek, 0) # Already closed.


    def test___len__(self):
        # The Length method is renamed in the Python wrapper to __len__.
        lenPy = os.path.getsize(test_base.getFilenameOfThisPythonTestModule(__file__))
        fsIn = self.testConstructor_String()
        lenCL = len(fsIn)
        fsIn.close()

        self.assertEqual(lenPy, lenCL)

        self.assertRaises(IOError, len, fsIn) # Already closed.


class OutputStreamTest(test_base.CommonBaseTest):
    def testConstructor(self):
        # Ensure that this abstract class can't be instantiated.
        # Conventionally, NotImplementedError would be raised to indicate
        # abstraction, but SWIG raises RuntimeError.
        self.assertRaises(RuntimeError, lucene.OutputStream)


class FSOutputStreamTest(test_base.CommonBaseTest):
    def _createFSO(self):
        filename = self.getTempFilename()
        return lucene.FSOutputStream(filename), filename

    def _enforceFileContents(self, filename, contents):
        self.assertEqual(file(filename, 'rb').read(), contents)


    def testConstructor(self):
        o, filename = self._createFSO()
        o.close()


    def testTruncation(self):
        # When opened by an FSOutputStream, an existing file should be
        # truncated.
        filename = self.getTempFilename()

        pyF = file(filename, 'wb')
        pyF.write('0123456789')
        pyF.close()

        oldSize = os.path.getsize(filename)
        self.assertEqual(oldSize, 10)

        o = lucene.FSOutputStream(filename)
        o.close()

        newSize = os.path.getsize(filename)
        self.assertEqual(newSize, 0)


    def test_close(self):
        o, filename = self._createFSO()
        o.close()
        self.assertRaises(IOError, o.close) # Already closed.


    def test_writeByte(self):
        o, filename = self._createFSO()
        o.writeByte('\x00')
        o.writeByte('\xff')
        self.assertRaises(TypeError, o.writeByte, 'ab') # string of length > 1
        self.assertRaises(TypeError, o.writeByte, 0) # int
        o.close()

        self._enforceFileContents(filename, '\x00\xff')

        self.assertRaises(IOError, o.writeByte, '\x00') # Already closed.


    def test_writeBytes(self):
        s = 'abcd' * 8192
        o, filename = self._createFSO()
        o.writeBytes(s)
        o.close()

        self._enforceFileContents(filename, s)

        self.assertRaises(IOError, o.writeBytes, 'abcd') # Already closed.


    def test_writeInt(self):
        ints = -2**31, -2, -1, 0, 1, 2, 100, 1023, 1024, 1025, 2**31-1
        reqContents = ''.join([struct.pack('>i', i) for i in ints])

        o, filename = self._createFSO()
        for i in ints:
            try:
                o.writeInt(i)
            except Exception, e:
                self.fail('Failed while trying to writeInt %d, with %s (%s)'
                    % (i, e.__class__.__name__, str(e))
                  )

        # On x86, raises OverflowError; on AMD64, TypeError.
        self.assertRaises((OverflowError, TypeError), o.writeInt, -sys.maxint-2)
        self.assertRaises((OverflowError, TypeError), o.writeInt, sys.maxint+1)

        o.close()

        self._enforceFileContents(filename, reqContents)

        self.assertRaises(IOError, o.writeInt, 1) # Already closed.


    def _test_writeVariableInt(self, writeMethod):
        o, filename = self._createFSO()
        writeMethod(o, 128) # Python int
        writeMethod(o, 16385) # Python int
        writeMethod(o, 128L) # Python long
        writeMethod(o, 16385L) # Python long
        self.assertRaises(TypeError, writeMethod, o, '1')
        o.close()

        vint128 = file(os.path.join('data', 'vint-128.bin'), 'rb').read()
        vint16385 = file(os.path.join('data', 'vint-16385.bin'), 'rb').read()
        reqContents = vint128 + vint16385 + vint128 + vint16385
        self._enforceFileContents(filename, reqContents)

        self.assertRaises(IOError, writeMethod, o, '1') # Already closed.

    def test_writeVInt(self):
        self._test_writeVariableInt(lucene.FSOutputStream.writeVInt)

    def test_writeVLong(self):
        # The writeVLong method is an artifact of C's type system.  From
        # Python's perspective, there's no essential difference between
        # writeVInt and writeVLong.  The underlying file format is the same.
        self._test_writeVariableInt(lucene.FSOutputStream.writeVLong)


    def test_writeLong(self):
        ints = -2**63, -2, -1, 0, 1, 2, 100, 1023, 1024, 1025, 2**63-1
        reqContents = ''.join([struct.pack('>q', i) for i in ints])

        o, filename = self._createFSO()
        for i in ints:
            o.writeLong(i)

        self.assertRaises(OverflowError, o.writeLong, -2**63-2)
        self.assertRaises(OverflowError, o.writeLong,  2**63  )
        o.close()

        self._enforceFileContents(filename, reqContents)

        self.assertRaises(IOError, o.writeLong, 1) # Already closed.


    def test_writeString(self):
        # A "String" is a packed (32-bit length, data) pair.
        s = 'ABCD'*32

        o, filename = self._createFSO()
        o.writeString(s)
        o.close()

        reqContents = file(os.path.join('data', 'sized-string-128.bin'), 'rb').read()
        self._enforceFileContents(filename, reqContents)

        self.assertRaises(IOError, o.writeString, s) # Already closed.


    def test_writeChars(self):
        # YYY: Revisit this test case when unicode support has been added.

        # (The pyclene.lucene.FSOutputStream.writeChars already implicitly
        # UTF8-encodes the incoming string.)
        s = 'ABCD'*32

        o, filename = self._createFSO()
        o.writeChars(s)
        o.close()

        self._enforceFileContents(filename, s)

        self.assertRaises(IOError, o.writeChars, s) # Already closed.


    def test_filePointer(self):
        o, filename = self._createFSO()
        self.assertEqual(o.filePointer, 0)
        o.writeBytes('X' * 10)
        self.assertEqual(o.filePointer, 10)
        o.writeBytes('X' * 1025)
        self.assertEqual(o.filePointer, 1035)
        o.close()


    def test_seek(self):
        # YYY:
        # The lucene::store::OutputStream seems to raise an exception on any
        # seek attempt, so it's currently disabled in the Python wrapper.
        o = lucene.FSOutputStream(self.getTempFilename())
        self.assertRaises(NotImplementedError, o.seek, -1)
        self.assertRaises(NotImplementedError, o.seek, 0)
        self.assertRaises(NotImplementedError, o.seek, 1)
        o.close()

        self.assertRaises(IOError, o.seek, 0) # Already closed.

        # for (streamConstructor, constructorArgs) in (
            # ( file,                  (self.getTempFilename(), 'wb') ),
            # ( lucene.FSOutputStream, (self.getTempFilename(),     ) ),
          # ):
            # stream = streamConstructor(*constructorArgs)
            # stream.write('ABCDEFGHIJ')
            # secondPass = 'KLMNOPQRST'
            # #for i in range(8, -2, -2): # [8, 6, 4, 2, 0]
            # for i in range(0, 10, 2): # [8, 6, 4, 2, 0]
                # stream.seek(i)
                # stream.write(secondPass[i])
            # self.assertRaises(IOError, stream.seek, -1)
            # stream.close()
            # self._enforceFileContents(constructorArgs[0], 'KBMDOFQHSJ')


    def test___len__(self):
        # Apparently length is always zero for output streams.
        o, filename = self._createFSO()
        self.assertEqual(len(o), 0)
        o.writeChars('blah')
        self.assertEqual(len(o), 0)
        o.close()

        self.assertRaises(IOError, len, o) # Already closed.


class DirectoryTest(test_base.CommonBaseTest):
    def testConstructor(self):
        # Ensure that this abstract class can't be instantiated.
        # Conventionally, NotImplementedError would be raised to indicate
        # abstraction, but SWIG raises RuntimeError.
        self.assertRaises(RuntimeError, lucene.Directory)


class FSDirectoryTest(test_base.CommonBaseTest):
    def _createFSDirectoryPointingToSysTempDir(self):
        tempName = self.getTempFilename()
        tempDir = os.path.dirname(tempName)
        d = lucene.FSDirectory(tempDir)
        return d


    def _createTempFileContaining(self, contents):
        tempName = self.getTempFilename()
        f = file(tempName, 'wb')
        try:
            f.write(contents)
        finally:
            f.close()
        return tempName


    def testConstructor_String_indexExists(self):
        indLoc = self.extractTestIndex()

        # The Python extensions disallow delete/recreate via
        # createIfDoesNotExist:
        self.assertRaises(IOError, lucene.FSDirectory, indLoc,
            createIfDoesNotExist=True
          )

        d = lucene.FSDirectory(indLoc, createIfDoesNotExist=False)
        return d, indLoc


    def testConstructor_String_indexDoesNotExist(self):
        # Valid location; createIfDoesNotExist=True:
        newLoc = self.getTempFilename()
        self.assert_(not os.path.isdir(newLoc))

        d = lucene.FSDirectory(newLoc, createIfDoesNotExist=True)
        self.assert_(os.path.isdir(newLoc))

        # Invalid location; createIfDoesNotExist=True:
        self.assertRaises(IOError, lucene.FSDirectory, 'bogus/pyclene_index',
            createIfDoesNotExist=True
          )

        # Invalid location; createIfDoesNotExist=False:
        self.assertRaises(IOError, lucene.FSDirectory, 'bogus/pyclene_index')


    def test_list(self):
        d, indLoc = self.testConstructor_String_indexExists()
        self.assertEqual(d.list(), os.listdir(indLoc))
        d.close()

        self.assertRaises(IOError, d.list) # Already closed.


    def test___iter__(self):
        d, indLoc = self.testConstructor_String_indexExists()
        self.assertEqual(d.list(), [fn for fn in d])


    def test_name(self):
        tempName = self.getTempFilename()
        tempDir = os.path.dirname(tempName)
        d = lucene.FSDirectory(tempDir)
        self.assertEqual(d.name, tempDir)
        d.close()

    # The static method getDirectory is not wrapped; I didn't see the point,
    # since the constructor accomplishes the same thing.

    if lucene.SUMO_BUILD:
        def test_fileExists(self):
            tempName = self.getTempFilename()
            tempDir = os.path.dirname(tempName)
            assert os.path.exists(tempDir)
            assert not os.path.exists(tempName) # Haven't actually created a file yet.

            d = lucene.FSDirectory(tempDir)
            self.failIf(d.fileExists(os.path.basename(tempName)))
            f = file(tempName, 'wb')
            # Now the file actually exists.
            self.failUnless(d.fileExists(os.path.basename(tempName)))
            f.close()
            os.remove(tempName)
            # Now it's gone again.
            self.failIf(d.fileExists(os.path.basename(tempName)))
            d.close()

            self.assertRaises(IOError, d.fileExists, os.path.basename(tempName)) # Already closed.


        def test_fileModified(self):
            d = self._createFSDirectoryPointingToSysTempDir()
            tempName = self._createTempFileContaining('X')
            self.assertEqual(
                d.fileModified(os.path.basename(tempName)),
                os.path.getmtime(tempName)
              )
            d.close()

            self.assertRaises(IOError, d.fileModified, os.path.basename(tempName)) # Already closed.

        # The static version of fileModified is not wrapped; I didn't see the point.

        def test_fileLength(self):
            d = self._createFSDirectoryPointingToSysTempDir()
            tempName = self._createTempFileContaining('X' * 16385)
            self.assertEqual( d.fileLength(os.path.basename(tempName)), 16385 )
            d.close()

            self.assertRaises(IOError, d.fileLength, os.path.basename(tempName)) # Already closed.


        def test_deleteFile(self):
            d = self._createFSDirectoryPointingToSysTempDir()
            tempName = self._createTempFileContaining('X')
            d.deleteFile(os.path.basename(tempName))
            self.failIf(os.path.exists(tempName))
            d.close()

            self.assertRaises(IOError, d.deleteFile, os.path.basename(tempName)) # Already closed.

        def test_renameFile(self):
            d = self._createFSDirectoryPointingToSysTempDir()
            tempName = self._createTempFileContaining('X')
            assert os.path.isfile(tempName)
            renameTo = tempName + '_RENAMED'
            self.trackThisTempFile(renameTo) # Make sure it gets deleted if it's created.
            d.renameFile(tempName, renameTo)
            self.assert_( os.path.isfile(renameTo) )
            d.close()

            self.assertRaises(IOError, d.renameFile, renameTo, tempName) # Already closed.


        def test_createFile_AND_openFile(self):
            d = self._createFSDirectoryPointingToSysTempDir()
            tempName = self.getTempFilename()

            oStream = d.createFile(tempName)
            written = 'ABCxyz'
            oStream.writeBytes(written)
            oStream.close()
            self.assertEqual(file(tempName, 'rb').read(), written)

            iStream = d.openFile(tempName)
            read = iStream.readBytes(len(written))
            iStream.close()
            self.assertEqual(read, written)
            d.close()
            self.assertRaises(IOError, d.openFile, tempName) # Already closed.

            os.remove(tempName)
            self.assertRaises(IOError, d.createFile, tempName) # Already closed.


        def test_makeLock(self):
            d = self._createFSDirectoryPointingToSysTempDir()
            tempName = self.getTempFilename()

            # lucene.Lock subclasses (such as FSLock) are tested elsewhere.
            lock = d.makeLock(tempName)
            self.assert_(isinstance(lock, lucene.Lock))

            d.close()
            self.assertRaises(IOError, d.makeLock, tempName) # Already closed.


    # No test_refInc because refInc method not wrapped.


    def test_close(self):
        d = self._createFSDirectoryPointingToSysTempDir()
        d.close()
        self.assertRaises(IOError, d.close) # Already closed.


class RAMDirectoryTest(test_base.CommonBaseTest):
    def testContructor(self):
        d = lucene.RAMDirectory()
        return d

    def test_close(self):
        d = self.testContructor()
        d.close()
        self.assertRaises(IOError, d.close)


    def test_fileExists_AND_createFile_AND_openFile_AND_fileLength(self):
        d = self.testContructor()
        # ZZZ_DECIDE_WHETHER_TO_WRAP_THESE:
        # These methods are no longer wrapped, even in a sumo build.
        self.assertRaises(NotImplementedError, d.fileExists, 'a')
        self.assertRaises(NotImplementedError, d.createFile, 'a')
        self.assertRaises(NotImplementedError, d.openFile, 'a')
        self.assertRaises(NotImplementedError, d.fileLength, 'a')

        return # ZZZ_DECIDE_WHETHER_TO_WRAP_THESE

        # d.createFile('do-not-close-so-as-to-check-from-mem-leaks')
        #
        # filename = 'file1'
        # # First, try to openFile a file that isn't there.
        # self.assertRaises(IOError, d.openFile, filename)
        #
        # thisModulesBytes = file(os.path.abspath(__file__), 'rb').read()
        # fThisModule = StringIO(thisModulesBytes)
        # fThisModuleBack = StringIO()
        #
        # self.failIf(d.fileExists(filename))
        # oStream = d.createFile(filename)
        # self.failUnless(d.fileExists(filename))
        # # Copy the contents of this test module (Python source file) into the
        # # RAMOutputStream from a StringIO (which "acts like" a file object).
        # shutil.copyfileobj(fThisModule, oStream)
        # fThisModule.close()
        # oStream.close()
        # # Retrieve the contents of this test module from the RAMDirectory into
        # # a StringIO (which "acts like" a file object).
        # iStream = d.openFile(filename)
        # shutil.copyfileobj(iStream, fThisModuleBack)
        # iStream.close()
        # self.assertEqual(thisModulesBytes, fThisModuleBack.getvalue())
        #
        # # Test the fileLength method.
        # self.assertEqual(d.fileLength(filename), len(thisModulesBytes))
        # filename2 = 'file2'
        # d.createFile(filename2).close()
        # self.assert_(d.fileExists(filename2))
        # self.assertEqual(d.fileLength(filename2), 0)
        #
        # d.close()
        # # Already closed:
        # self.assertRaises(IOError, d.close)
        # self.assertRaises(IOError, d.fileExists, filename)
        # self.assertRaises(IOError, d.createFile, filename)
        # self.assertRaises(IOError, d.openFile, filename)
        # self.assertRaises(IOError, d.fileLength, filename)


    def test_fileModified(self):
        d = self.testContructor()
        # ZZZ_DECIDE_WHETHER_TO_WRAP_THESE:
        # These methods are no longer wrapped, even in a sumo build.
        self.assertRaises(NotImplementedError, d.fileModified, 'a')
        return # ZZZ_DECIDE_WHETHER_TO_WRAP_THESE

        # filename = 'file1'
        # d.createFile(filename).close()
        # firstModTime = d.fileModified(filename)
        # d.deleteFile(filename)
        # d.createFile(filename).close()
        # secondModTime = d.fileModified(filename)
        #
        # self.assert_(firstModTime <= secondModTime)
        #
        # d.close()
        # self.assertRaises(IOError, d.fileModified, filename) # Already closed.


    def test_deleteFile(self):
        d = self.testContructor()
        # ZZZ_DECIDE_WHETHER_TO_WRAP_THESE:
        # These methods are no longer wrapped, even in a sumo build.
        self.assertRaises(NotImplementedError, d.deleteFile, 'a')
        return # ZZZ_DECIDE_WHETHER_TO_WRAP_THESE

        # filename = 'file1'
        #
        # d.createFile(filename).close()
        # self.failUnless(d.fileExists(filename))
        # self.assertRaises(IOError, d.createFile, filename)
        # d.deleteFile(filename)
        # self.failIf(d.fileExists(filename))
        # self.assertRaises(IOError, d.deleteFile, filename)
        #
        # d.close()
        # self.assertRaises(IOError, d.deleteFile, filename) # Already closed.


    def test_renameFile(self):
        d = self.testContructor()
        # ZZZ_DECIDE_WHETHER_TO_WRAP_THESE:
        # These methods are no longer wrapped, even in a sumo build.
        self.assertRaises(NotImplementedError, d.renameFile, 'a', 'b')
        return # ZZZ_DECIDE_WHETHER_TO_WRAP_THESE

        # filename1 = 'file1'
        # filename2 = 'file2'
        #
        # self.assertRaises(IOError, d.renameFile, filename1, filename2)
        # self.assertRaises(IOError, d.renameFile, filename2, filename1)
        # self.assertRaises(IOError, d.renameFile, filename1, filename1)
        #
        # d.createFile(filename1).close()
        # self.assertRaises(IOError, d.renameFile, filename1, filename1)
        # self.failUnless(d.fileExists(filename1))
        # self.failIf(d.fileExists(filename2))
        #
        # d.renameFile(filename1, filename2)
        # self.failIf(d.fileExists(filename1))
        # self.failUnless(d.fileExists(filename2))
        #
        # d.renameFile(filename2, filename1)
        # self.failUnless(d.fileExists(filename1))
        # self.failIf(d.fileExists(filename2))
        #
        # d.close()
        # self.assertRaises(IOError, d.renameFile, filename1, filename2) # Already closed.


    def test_makeLock(self):
        d = self.testContructor()
        # ZZZ_DECIDE_WHETHER_TO_WRAP_THESE:
        # These methods are no longer wrapped, even in a sumo build.
        self.assertRaises(NotImplementedError, d.makeLock, 'a')
        return # ZZZ_DECIDE_WHETHER_TO_WRAP_THESE

        # self.failUnless( isinstance(d.makeLock('file1'), lucene.Lock) )
        # d.close()
        #
        # self.assertRaises(IOError, d.makeLock) # Already closed.


    def test_list(self):
        d = self.testContructor()
        self.assertEqual(d.list(), [])

        # ZZZ_DECIDE_WHETHER_TO_WRAP_THESE:
        # createFile/deleteFile are no longer wrapped, even in a sumo build.
        #
        # filename1 = 'file1'
        # filename2 = 'file2'
        # d.createFile(filename1).close()
        # self.assertEqual(d.list(), [filename1])
        # d.createFile(filename2).close()
        # self.assert_( filename1 in d.list() and filename2 in d.list() )
        # d.deleteFile(filename1)
        # self.assertEqual(d.list(), [filename2])
        # d.deleteFile(filename2)
        # self.assertEqual(d.list(), [])

        d.close()
        self.assertRaises(IOError, d.list) # Already closed.

        # Not-empty:
        d = self.extractTestIndexToRAMDirectory()
        self.assert_(len(d.list()) > 0)

        d.close()
        self.assertRaises(IOError, d.list) # Already closed.


    # ZZZ_DECIDE_WHETHER_TO_WRAP_THESE:
    # These methods are no longer wrapped, even in a sumo build.
    # def test_opsOnNonexistentFiles(self):
        # # Ensure that attempting to operate on a file that doesn't exist does
        # # not cause memory corruption.
        # d = self.testContructor()
        # self.assertRaises(IOError, d.fileModified, 'nonexistent-file')
        # self.assertRaises(IOError, d.fileLength, 'nonexistent-file')
        # self.assertRaises(IOError, d.deleteFile, 'nonexistent-file')
        # self.assertRaises(IOError, d.renameFile, 'nonexistent-file1', 'nonexistent-file1')
        # self.assertRaises(IOError, d.renameFile, 'nonexistent-file1', 'nonexistent-file2')
        # self.assertRaises(IOError, d.openFile, 'nonexistent-file')
        # d.close()


class LockTest(test_base.CommonBaseTest):
    def testConstructor(self):
        # Ensure that this abstract class can't be instantiated.
        # Conventionally, NotImplementedError would be raised to indicate
        # abstraction, but SWIG raises RuntimeError.
        self.assertRaises(RuntimeError, lucene.Lock)


class LockWithTest(test_base.CommonBaseTest):
    def testConstructor(self):
        # Ensure that this abstract class can't be instantiated.
        # Conventionally, NotImplementedError would be raised to indicate
        # abstraction, but SWIG raises RuntimeError.
        self.assertRaises(RuntimeError, lucene.LockWith)


class FSLockTest(test_base.CommonBaseTest):
    def testConstructor(self, filename=None):
        if filename is None:
            filename = self.getTempFilename()
        lock = lucene.FSLock(filename)
        return lock, filename


    def test_filename(self):
        # Valid filename that could exist:
        lock, filename = self.testConstructor()
        self.assertEqual(filename, lock.filename)

        # FSLock doesn't care if it gets an invalid filename (i.e., a filename
        # that *could not* exist), but we need to ensure that the
        # FSLock.filename property returns exactly the filename that was passed
        # to the constructor.  Therefore, the Python wrapper implements some
        # controls (e.g., rejects null bytes in the filename).
        trash = 'bog\0us'
        self.assertRaises(ValueError, self.testConstructor, filename=trash)

        invalidButNotRejected = '%:$|!#$%#$U%O\\/^'
        lock, _ = self.testConstructor(filename=invalidButNotRejected)
        self.assertEqual(invalidButNotRejected, lock.filename)


    def test_obtain_AND_release(self):
        lock, filename = self.testConstructor()
        self.failUnless(lock.obtain())
        self.failUnless(os.path.exists(lock.filename))
        lock.release()
        self.failIf(os.path.exists(lock.filename))


    def test_release_invalid(self):
        # Make sure the FSLock doesn't segfault if the lock file gets deleted
        # out from under it.  It's allowed to raise an exception, though.
        lock, filename = self.testConstructor()
        self.failUnless(lock.obtain())
        self.failUnless(os.path.exists(lock.filename))
        os.remove(lock.filename)
        try:
            lock.release()
        except:
            pass


if __name__ == '__main__':
    import test
    test.main(suite=getFullTestSuite())
