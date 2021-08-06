# This file was created automatically by SWIG.
# Don't modify this file, modify the SWIG interface instead.

import _cl_c

try:
    from weakref import proxy as weakref_proxy
except:
    weakref_proxy = lambda x: x


import datetime, time

# Utility functions:
def _maybeDatetimeToInt(t):
    if isinstance(t, datetime.datetime):
        t = int(time.mktime(t.timetuple()))
    return t


def _makePythonIteratorCompliantNextMethod(originalMethod):
    # $originalMethod is an unbound method that's designed to return one item
    # per call until exhaustion, when it returns None.
    # We generate a method compliant with the Python iterator protocol, which
    # requires that StopIteration be raised upon exhaustion.
    def iteratorCompliantMethod(self):
        nextItem = originalMethod(self)
        if nextItem is None:
            raise StopIteration
        return nextItem
    return iteratorCompliantMethod


def _makeSingleShotMethod(originalMethod, tag, exceptionClass=IOError):
    tagAttrName = '_%s_singleShot' % tag
    def singleShot(self):
        if getattr(self, tagAttrName, False):
            raise exceptionClass('Already %s.' % tag)
        ret = originalMethod(self)
        setattr(self, tagAttrName, True)
        return ret
    return singleShot



setGlobals = _cl_c.setGlobals
class StringReader(object):
    def __repr__(self):
        return "<C lucene::util::StringReader instance at %s>" % (self.this,)
    def __init__(self, s):
        # Retain a reference to the str or unicode object so it's guaranteed
        # not to be collected during the life of this StringReader.
        self._s = s

        # If this is a non-unicode build, we pass the C++ StringReader a
        # pointer to the str's internal buffer, so its deleteValue constructor
        # parameter is False.
        # For Unicode builds, copying is required, so the C++ StringReader's
        # deleteValue constructor is True.
        newobj = _cl_c.new_StringReader(s, len(s), UNICODE_BUILD)
        # Remember to execute the SWIG boilerplate:
        self.this = newobj.this
        self.thisown = 1
        del newobj.thisown


    def __del__(self, destroy=_cl_c.delete_StringReader):
        try:
            if self.thisown: destroy(self)
        except: pass
    close = _makeSingleShotMethod(_cl_c.StringReader_close, 'closed')



class StringReaderPtr(StringReader):
    def __init__(self, this):
        self.this = this
        if not hasattr(self,"thisown"): self.thisown = 0
        self.__class__ = StringReader
_cl_c.StringReader_swigregister(StringReaderPtr)
cvar = _cl_c.cvar
UNICODE_BUILD = cvar.UNICODE_BUILD
SUMO_BUILD = cvar.SUMO_BUILD

class FileReader(object):
    def __repr__(self):
        return "<C lucene::util::FileReader instance at %s>" % (self.this,)
    def __eq__(*args): return _cl_c.FileReader___eq__(*args)
    def __init__(self, *args):
        newobj = _cl_c.new_FileReader(*args)
        self.this = newobj.this
        self.thisown = 1
        del newobj.thisown
    def __del__(self, destroy=_cl_c.delete_FileReader):
        try:
            if self.thisown: destroy(self)
        except: pass
    close = _makeSingleShotMethod(_cl_c.FileReader_close, 'closed')



class FileReaderPtr(FileReader):
    def __init__(self, this):
        self.this = this
        if not hasattr(self,"thisown"): self.thisown = 0
        self.__class__ = FileReader
_cl_c.FileReader_swigregister(FileReaderPtr)

class InputStream(object):
    def __init__(self): raise RuntimeError, "No constructor defined"
    def __repr__(self):
        return "<C lucene::store::InputStream instance at %s>" % (self.this,)
    def readString(*args): return _cl_c.InputStream_readString(*args)
    def readByte(*args): return _cl_c.InputStream_readByte(*args)
    def readBytes(*args): return _cl_c.InputStream_readBytes(*args)
    def readChars(*args): return _cl_c.InputStream_readChars(*args)
    def read(*args): return _cl_c.InputStream_read(*args)
    filePointer = property(_cl_c.InputStream_filePointer_get, _cl_c.InputStream_filePointer_set)
    def __del__(self, destroy=_cl_c.delete_InputStream):
        try:
            if self.thisown: destroy(self)
        except: pass
    def clone(*args): return _cl_c.InputStream_clone(*args)
    def readInt(*args): return _cl_c.InputStream_readInt(*args)
    def readVInt(*args): return _cl_c.InputStream_readVInt(*args)
    def readLong(*args): return _cl_c.InputStream_readLong(*args)
    def readVLong(*args): return _cl_c.InputStream_readVLong(*args)
    def close(*args): return _cl_c.InputStream_close(*args)
    def seek(*args): return _cl_c.InputStream_seek(*args)
    def __len__(*args): return _cl_c.InputStream___len__(*args)

class InputStreamPtr(InputStream):
    def __init__(self, this):
        self.this = this
        if not hasattr(self,"thisown"): self.thisown = 0
        self.__class__ = InputStream
_cl_c.InputStream_swigregister(InputStreamPtr)

class OutputStream(object):
    def __init__(self): raise RuntimeError, "No constructor defined"
    def __repr__(self):
        return "<C lucene::store::OutputStream instance at %s>" % (self.this,)
    def writeByte(*args): return _cl_c.OutputStream_writeByte(*args)
    def writeChars(*args): return _cl_c.OutputStream_writeChars(*args)
    def write(*args): return _cl_c.OutputStream_write(*args)
    filePointer = property(_cl_c.OutputStream_filePointer_get, _cl_c.OutputStream_filePointer_set)
    def __del__(self, destroy=_cl_c.delete_OutputStream):
        try:
            if self.thisown: destroy(self)
        except: pass
    def writeBytes(*args): return _cl_c.OutputStream_writeBytes(*args)
    def writeInt(*args): return _cl_c.OutputStream_writeInt(*args)
    def writeVInt(*args): return _cl_c.OutputStream_writeVInt(*args)
    def writeLong(*args): return _cl_c.OutputStream_writeLong(*args)
    def writeVLong(*args): return _cl_c.OutputStream_writeVLong(*args)
    def writeString(*args): return _cl_c.OutputStream_writeString(*args)
    def isClosed(*args): return _cl_c.OutputStream_isClosed(*args)
    def close(*args): return _cl_c.OutputStream_close(*args)
    def __len__(*args): return _cl_c.OutputStream___len__(*args)

class OutputStreamPtr(OutputStream):
    def __init__(self, this):
        self.this = this
        if not hasattr(self,"thisown"): self.thisown = 0
        self.__class__ = OutputStream
_cl_c.OutputStream_swigregister(OutputStreamPtr)

class Lock(object):
    def __init__(self): raise RuntimeError, "No constructor defined"
    def __repr__(self):
        return "<C lucene::store::LuceneLock instance at %s>" % (self.this,)
    def obtain(*args): return _cl_c.Lock_obtain(*args)
    def release(*args): return _cl_c.Lock_release(*args)
    def __del__(self, destroy=_cl_c.delete_Lock):
        try:
            if self.thisown: destroy(self)
        except: pass

class LockPtr(Lock):
    def __init__(self, this):
        self.this = this
        if not hasattr(self,"thisown"): self.thisown = 0
        self.__class__ = Lock
_cl_c.Lock_swigregister(LockPtr)

class LockWith(object):
    def __init__(self): raise RuntimeError, "No constructor defined"
    def __repr__(self):
        return "<C lucene::store::LuceneLockWith instance at %s>" % (self.this,)
    def __del__(self, destroy=_cl_c.delete_LockWith):
        try:
            if self.thisown: destroy(self)
        except: pass
    def run(*args): return _cl_c.LockWith_run(*args)

class LockWithPtr(LockWith):
    def __init__(self, this):
        self.this = this
        if not hasattr(self,"thisown"): self.thisown = 0
        self.__class__ = LockWith
_cl_c.LockWith_swigregister(LockWithPtr)

class Directory(object):
    def __init__(self): raise RuntimeError, "No constructor defined"
    def __repr__(self):
        return "<C lucene::store::Directory instance at %s>" % (self.this,)
    def __del__(self, destroy=_cl_c.delete_Directory):
        try:
            if self.thisown: destroy(self)
        except: pass
    def list(*args): return _cl_c.Directory_list(*args)
    def fileExists(*args): return _cl_c.Directory_fileExists(*args)
    def fileModified(*args): return _cl_c.Directory_fileModified(*args)
    def fileLength(*args): return _cl_c.Directory_fileLength(*args)
    def openFile(*args): return _cl_c.Directory_openFile(*args)
    def deleteFile(*args): return _cl_c.Directory_deleteFile(*args)
    def renameFile(*args): return _cl_c.Directory_renameFile(*args)
    def createFile(*args): return _cl_c.Directory_createFile(*args)
    def makeLock(*args): return _cl_c.Directory_makeLock(*args)
    def close(*args): return _cl_c.Directory_close(*args)

class DirectoryPtr(Directory):
    def __init__(self, this):
        self.this = this
        if not hasattr(self,"thisown"): self.thisown = 0
        self.__class__ = Directory
_cl_c.Directory_swigregister(DirectoryPtr)

class FSInputStream(InputStream):
    def __repr__(self):
        return "<C lucene::store::FSInputStream instance at %s>" % (self.this,)
    def clone(*args): return _cl_c.FSInputStream_clone(*args)
    isClone = property(_cl_c.FSInputStream_isClone_get, _cl_c.FSInputStream_isClone_set)
    def __init__(self, *args):
        newobj = _cl_c.new_FSInputStream_FromFilename(*args)
        self.this = newobj.this
        self.thisown = 1
        del newobj.thisown
    def __del__(self, destroy=_cl_c.delete_FSInputStream):
        try:
            if self.thisown: destroy(self)
        except: pass
    def close(*args): return _cl_c.FSInputStream_close(*args)

class FSInputStreamPtr(FSInputStream):
    def __init__(self, this):
        self.this = this
        if not hasattr(self,"thisown"): self.thisown = 0
        self.__class__ = FSInputStream
_cl_c.FSInputStream_swigregister(FSInputStreamPtr)

class FSOutputStream(OutputStream):
    def __repr__(self):
        return "<C lucene::store::FSOutputStream instance at %s>" % (self.this,)
    def __init__(self, *args):
        newobj = _cl_c.new_FSOutputStream(*args)
        self.this = newobj.this
        self.thisown = 1
        del newobj.thisown
    def __del__(self, destroy=_cl_c.delete_FSOutputStream):
        try:
            if self.thisown: destroy(self)
        except: pass
    def flushBuffer(*args): return _cl_c.FSOutputStream_flushBuffer(*args)
    def close(*args): return _cl_c.FSOutputStream_close(*args)
    def __len__(*args): return _cl_c.FSOutputStream___len__(*args)

class FSOutputStreamPtr(FSOutputStream):
    def __init__(self, this):
        self.this = this
        if not hasattr(self,"thisown"): self.thisown = 0
        self.__class__ = FSOutputStream
_cl_c.FSOutputStream_swigregister(FSOutputStreamPtr)

class FSDirectory(Directory):
    def __repr__(self):
        return "<C lucene::store::FSDirectory instance at %s>" % (self.this,)
    name = property(_cl_c.FSDirectory_name_get, _cl_c.FSDirectory_name_set)
    def __init__(self, *args):
        newobj = _cl_c.new_FSDirectory(*args)
        self.this = newobj.this
        self.thisown = 1
        del newobj.thisown
    def __del__(self, destroy=_cl_c.delete_FSDirectory):
        try:
            if self.thisown: destroy(self)
        except: pass
    def fileExists(*args): return _cl_c.FSDirectory_fileExists(*args)
    def fileLength(*args): return _cl_c.FSDirectory_fileLength(*args)
    def deleteFile(*args): return _cl_c.FSDirectory_deleteFile(*args)
    def renameFile(*args): return _cl_c.FSDirectory_renameFile(*args)
    def createFile(*args): return _cl_c.FSDirectory_createFile(*args)
    def openFile(*args): return _cl_c.FSDirectory_openFile(*args)
    def makeLock(*args): return _cl_c.FSDirectory_makeLock(*args)
    def close(*args): return _cl_c.FSDirectory_close(*args)

class FSDirectoryPtr(FSDirectory):
    def __init__(self, this):
        self.this = this
        if not hasattr(self,"thisown"): self.thisown = 0
        self.__class__ = FSDirectory
_cl_c.FSDirectory_swigregister(FSDirectoryPtr)

class FSLock(Lock):
    def __repr__(self):
        return "<C lucene::store::FSLock instance at %s>" % (self.this,)
    filename = property(_cl_c.FSLock_filename_get)
    def __init__(self, *args):
        newobj = _cl_c.new_FSLock(*args)
        self.this = newobj.this
        self.thisown = 1
        del newobj.thisown
    def __del__(self, destroy=_cl_c.delete_FSLock):
        try:
            if self.thisown: destroy(self)
        except: pass
    def obtain(*args): return _cl_c.FSLock_obtain(*args)
    def release(*args): return _cl_c.FSLock_release(*args)

class FSLockPtr(FSLock):
    def __init__(self, this):
        self.this = this
        if not hasattr(self,"thisown"): self.thisown = 0
        self.__class__ = FSLock
_cl_c.FSLock_swigregister(FSLockPtr)

class RAMDirectory(Directory):
    def __repr__(self):
        return "<C lucene::store::RAMDirectory instance at %s>" % (self.this,)
    def list(*args): return _cl_c.RAMDirectory_list(*args)
    def __init__(self, *args):
        newobj = _cl_c.new_RAMDirectory(*args)
        self.this = newobj.this
        self.thisown = 1
        del newobj.thisown
    def __del__(self, destroy=_cl_c.delete_RAMDirectory):
        try:
            if self.thisown: destroy(self)
        except: pass
    def close(*args): return _cl_c.RAMDirectory_close(*args)

class RAMDirectoryPtr(RAMDirectory):
    def __init__(self, this):
        self.this = this
        if not hasattr(self,"thisown"): self.thisown = 0
        self.__class__ = RAMDirectory
_cl_c.RAMDirectory_swigregister(RAMDirectoryPtr)

class TransactionalRAMDirectory(RAMDirectory):
    def __repr__(self):
        return "<C lucene::store::TransactionalRAMDirectory instance at %s>" % (self.this,)
    def __init__(self, *args):
        newobj = _cl_c.new_TransactionalRAMDirectory(*args)
        self.this = newobj.this
        self.thisown = 1
        del newobj.thisown
    def transIsOpen(*args): return _cl_c.TransactionalRAMDirectory_transIsOpen(*args)
    def transStart(*args): return _cl_c.TransactionalRAMDirectory_transStart(*args)
    def transCommit(*args): return _cl_c.TransactionalRAMDirectory_transCommit(*args)
    def transAbort(*args): return _cl_c.TransactionalRAMDirectory_transAbort(*args)
    def close(*args): return _cl_c.TransactionalRAMDirectory_close(*args)
    def dummy_createFile(*args): return _cl_c.TransactionalRAMDirectory_dummy_createFile(*args)
    def dummy_deleteFile(*args): return _cl_c.TransactionalRAMDirectory_dummy_deleteFile(*args)
    def dummy_renameFile(*args): return _cl_c.TransactionalRAMDirectory_dummy_renameFile(*args)
    def dummy_readFile(*args): return _cl_c.TransactionalRAMDirectory_dummy_readFile(*args)
    def __del__(self, destroy=_cl_c.delete_TransactionalRAMDirectory):
        try:
            if self.thisown: destroy(self)
        except: pass

class TransactionalRAMDirectoryPtr(TransactionalRAMDirectory):
    def __init__(self, this):
        self.this = this
        if not hasattr(self,"thisown"): self.thisown = 0
        self.__class__ = TransactionalRAMDirectory
_cl_c.TransactionalRAMDirectory_swigregister(TransactionalRAMDirectoryPtr)


DateField_timeToString = _cl_c.DateField_timeToString

DateField_stringToTime = _cl_c.DateField_stringToTime

TermEnum__iter__ = _cl_c.TermEnum__iter__
class TermDocs(object):
    def __init__(self): raise RuntimeError, "No constructor defined"
    def __repr__(self):
        return "<C lucene::index::TermDocs instance at %s>" % (self.this,)
    def __del__(self, destroy=_cl_c.delete_TermDocs):
        try:
            if self.thisown: destroy(self)
        except: pass
    def seek(*args): return _cl_c.TermDocs_seek(*args)
    def doc(*args): return _cl_c.TermDocs_doc(*args)
    def freq(*args): return _cl_c.TermDocs_freq(*args)
    def next(*args): return _cl_c.TermDocs_next(*args)
    def skipTo(*args): return _cl_c.TermDocs_skipTo(*args)
    def close(*args): return _cl_c.TermDocs_close(*args)

class TermDocsPtr(TermDocs):
    def __init__(self, this):
        self.this = this
        if not hasattr(self,"thisown"): self.thisown = 0
        self.__class__ = TermDocs
_cl_c.TermDocs_swigregister(TermDocsPtr)

class TermEnum(object):
    def __init__(self): raise RuntimeError, "No constructor defined"
    def __repr__(self):
        return "<C lucene::index::TermEnum instance at %s>" % (self.this,)
    def term(*args): return _cl_c.TermEnum_term(*args)
    def itercall(*args): return _cl_c.TermEnum_itercall(*args)
    def next(*args): return _cl_c.TermEnum_next(*args)
    def docFreq(*args): return _cl_c.TermEnum_docFreq(*args)
    def close(*args): return _cl_c.TermEnum_close(*args)
    def __del__(self, destroy=_cl_c.delete_TermEnum):
        try:
            if self.thisown: destroy(self)
        except: pass

class TermEnumPtr(TermEnum):
    def __init__(self, this):
        self.this = this
        if not hasattr(self,"thisown"): self.thisown = 0
        self.__class__ = TermEnum
_cl_c.TermEnum_swigregister(TermEnumPtr)

class TermPositions(TermDocs):
    def __init__(self): raise RuntimeError, "No constructor defined"
    def __repr__(self):
        return "<C lucene::index::TermPositions instance at %s>" % (self.this,)
    def nextPosition(*args): return _cl_c.TermPositions_nextPosition(*args)
    def __del__(self, destroy=_cl_c.delete_TermPositions):
        try:
            if self.thisown: destroy(self)
        except: pass

class TermPositionsPtr(TermPositions):
    def __init__(self, this):
        self.this = this
        if not hasattr(self,"thisown"): self.thisown = 0
        self.__class__ = TermPositions
_cl_c.TermPositions_swigregister(TermPositionsPtr)

class IndexReader(object):
    def __init__(self): raise RuntimeError, "No constructor defined"
    def __repr__(self):
        return "<C lucene::index::IndexReader instance at %s>" % (self.this,)
    directory = property(_cl_c.IndexReader_directory_get)
    def __del__(self, destroy=_cl_c.delete_IndexReader):
        try:
            if self.thisown: destroy(self)
        except: pass
    openFromDirName = staticmethod(_cl_c.IndexReader_openFromDirName)
    openFromDir = staticmethod(_cl_c.IndexReader_openFromDir)
    lastModified = staticmethod(_cl_c.IndexReader_lastModified)
    lastModified = staticmethod(_cl_c.IndexReader_lastModified)
    indexExists = staticmethod(_cl_c.IndexReader_indexExists)
    indexExists = staticmethod(_cl_c.IndexReader_indexExists)
    def numDocs(*args): return _cl_c.IndexReader_numDocs(*args)
    def maxDoc(*args): return _cl_c.IndexReader_maxDoc(*args)
    def document(*args): return _cl_c.IndexReader_document(*args)
    def isDeleted(*args): return _cl_c.IndexReader_isDeleted(*args)
    def termsAll(*args): return _cl_c.IndexReader_termsAll(*args)
    def termsSpecific(*args): return _cl_c.IndexReader_termsSpecific(*args)
    def docFreq(*args): return _cl_c.IndexReader_docFreq(*args)
    def termDocsContaining(*args): return _cl_c.IndexReader_termDocsContaining(*args)
    def termPositionsContaining(*args): return _cl_c.IndexReader_termPositionsContaining(*args)
    def termPositionsAll(*args): return _cl_c.IndexReader_termPositionsAll(*args)
    def termDocsAll(*args): return _cl_c.IndexReader_termDocsAll(*args)
    def deleteAt(*args): return _cl_c.IndexReader_deleteAt(*args)
    def deleteWithTerm(*args): return _cl_c.IndexReader_deleteWithTerm(*args)
    def close(*args): return _cl_c.IndexReader_close(*args)
    isLocked = staticmethod(_cl_c.IndexReader_isLocked)
    isLocked = staticmethod(_cl_c.IndexReader_isLocked)
    unlock = staticmethod(_cl_c.IndexReader_unlock)

class IndexReaderPtr(IndexReader):
    def __init__(self, this):
        self.this = this
        if not hasattr(self,"thisown"): self.thisown = 0
        self.__class__ = IndexReader
_cl_c.IndexReader_swigregister(IndexReaderPtr)

IndexReader_openFromDirName = _cl_c.IndexReader_openFromDirName

IndexReader_openFromDir = _cl_c.IndexReader_openFromDir

IndexReader_lastModified = _cl_c.IndexReader_lastModified

IndexReader_indexExists = _cl_c.IndexReader_indexExists

IndexReader_isLocked = _cl_c.IndexReader_isLocked

IndexReader_unlock = _cl_c.IndexReader_unlock

class IndexWriter(object):
    def __repr__(self):
        return "<C lucene::index::IndexWriter instance at %s>" % (self.this,)
    def addIndexes(*args): return _cl_c.IndexWriter_addIndexes(*args)
    def __del__(self, destroy=_cl_c.delete_IndexWriter):
        try:
            if self.thisown: destroy(self)
        except: pass
    maxFieldLength = property(_cl_c.IndexWriter_maxFieldLength_get, _cl_c.IndexWriter_maxFieldLength_set)
    mergeFactor = property(_cl_c.IndexWriter_mergeFactor_get, _cl_c.IndexWriter_mergeFactor_set)
    maxMergeDocs = property(_cl_c.IndexWriter_maxMergeDocs_get, _cl_c.IndexWriter_maxMergeDocs_set)
    def __init__(self, *args):
        newobj = _cl_c.new_IndexWriter(*args)
        self.this = newobj.this
        self.thisown = 1
        del newobj.thisown
    def close(*args): return _cl_c.IndexWriter_close(*args)
    def docCount(*args): return _cl_c.IndexWriter_docCount(*args)
    def addDocument(*args): return _cl_c.IndexWriter_addDocument(*args)
    def optimize(*args): return _cl_c.IndexWriter_optimize(*args)

class IndexWriterPtr(IndexWriter):
    def __init__(self, this):
        self.this = this
        if not hasattr(self,"thisown"): self.thisown = 0
        self.__class__ = IndexWriter
_cl_c.IndexWriter_swigregister(IndexWriterPtr)

class HitCollector(object):
    def __repr__(self):
        return "<C lucene::search::HitCollector instance at %s>" % (self.this,)
    def collect(*args): return _cl_c.HitCollector_collect(*args)
    def __del__(self, destroy=_cl_c.delete_HitCollector):
        try:
            if self.thisown: destroy(self)
        except: pass
    def __init__(self, *args):
        if self.__class__ == HitCollector:
            args = (None,) + args
        else:
            args = (self,) + args
        newobj = _cl_c.new_HitCollector(*args)
        self.this = newobj.this
        self.thisown = 1
        del newobj.thisown
    def __disown__(self):
        self.thisown = 0
        _cl_c.disown_HitCollector(self)
        return weakref_proxy(self)

class HitCollectorPtr(HitCollector):
    def __init__(self, this):
        self.this = this
        if not hasattr(self,"thisown"): self.thisown = 0
        self.__class__ = HitCollector
_cl_c.HitCollector_swigregister(HitCollectorPtr)

class Hits(object):
    def __repr__(self):
        return "<C lucene::search::Hits instance at %s>" % (self.this,)
    def __init__(self, searcher=None, query=None, filter=None,
        _bypass=None
      ):
        if _bypass is not None:
            self.this = _bypass
            self.thisown = 1
            del _bypass.thisown
        else:
            newobj = _cl_c.new_Hits(
                (searcher is not None and searcher.this) or None,
                (query is not None and query.this) or None,
                (filter is not None and filter.this) or None
              )
            # Remember to execute the SWIG boilerplate:
            self.this = newobj.this
            self.thisown = 1
            del newobj.thisown

        self._searcher = searcher


    def __del__(self, destroy=_cl_c.delete_Hits):
        try:
            if self.thisown: destroy(self)
        except: pass
    def __len__(self):
        return _cl_c.Hits_Length(self.this)


    def doc(self, n):
        self._rangeCheck(n)
        return _cl_c.Hits_doc(self.this, n)


    def id(self, n):
        self._rangeCheck(n)
        return _cl_c.Hits_id(self.this, n)


    def score(self, n):
        self._rangeCheck(n)
        return _cl_c.Hits_score(self.this, n)



class HitsPtr(Hits):
    def __init__(self, this):
        self.this = this
        if not hasattr(self,"thisown"): self.thisown = 0
        self.__class__ = Hits
_cl_c.Hits_swigregister(HitsPtr)

class Searcher(object):
    def __init__(self): raise RuntimeError, "No constructor defined"
    def __repr__(self):
        return "<C lucene::search::Searcher instance at %s>" % (self.this,)
    def search(*args): return _cl_c.Searcher_search(*args)
    def __del__(self, destroy=_cl_c.delete_Searcher):
        try:
            if self.thisown: destroy(self)
        except: pass
    def docFreq(*args): return _cl_c.Searcher_docFreq(*args)
    def maxDoc(*args): return _cl_c.Searcher_maxDoc(*args)
    def doc(*args): return _cl_c.Searcher_doc(*args)
    def close(*args): return _cl_c.Searcher_close(*args)

class SearcherPtr(Searcher):
    def __init__(self, this):
        self.this = this
        if not hasattr(self,"thisown"): self.thisown = 0
        self.__class__ = Searcher
_cl_c.Searcher_swigregister(SearcherPtr)

class Query(object):
    def __init__(self): raise RuntimeError, "No constructor defined"
    def __repr__(self):
        return "<C lucene::search::Query instance at %s>" % (self.this,)
    name = property(_cl_c.Query_name_get, _cl_c.Query_name_set)
    def __del__(self, destroy=_cl_c.delete_Query):
        try:
            if self.thisown: destroy(self)
        except: pass
    boost = property(_cl_c.Query_boost_get, _cl_c.Query_boost_set)
    def sumOfSquaredWeights(*args): return _cl_c.Query_sumOfSquaredWeights(*args)
    def normalize(*args): return _cl_c.Query_normalize(*args)
    def prepare(*args): return _cl_c.Query_prepare(*args)
    def toString(*args): return _cl_c.Query_toString(*args)

class QueryPtr(Query):
    def __init__(self, this):
        self.this = this
        if not hasattr(self,"thisown"): self.thisown = 0
        self.__class__ = Query
_cl_c.Query_swigregister(QueryPtr)


new_MultiSearcher = _cl_c.new_MultiSearcher
class MultiSearcher(Searcher):
    def __init__(self): raise RuntimeError, "No constructor defined"
    def __repr__(self):
        return "<C lucene::search::MultiSearcher instance at %s>" % (self.this,)
    def __del__(self, destroy=_cl_c.delete_MultiSearcher):
        try:
            if self.thisown: destroy(self)
        except: pass
    def close(*args): return _cl_c.MultiSearcher_close(*args)
    def docFreq(*args): return _cl_c.MultiSearcher_docFreq(*args)
    def doc(*args): return _cl_c.MultiSearcher_doc(*args)
    def subSearcher(*args): return _cl_c.MultiSearcher_subSearcher(*args)
    def maxDoc(*args): return _cl_c.MultiSearcher_maxDoc(*args)

class MultiSearcherPtr(MultiSearcher):
    def __init__(self, this):
        self.this = this
        if not hasattr(self,"thisown"): self.thisown = 0
        self.__class__ = MultiSearcher
_cl_c.MultiSearcher_swigregister(MultiSearcherPtr)

class TermQuery(Query):
    def __repr__(self):
        return "<C lucene::search::TermQuery instance at %s>" % (self.this,)
    term = property(_cl_c.TermQuery_term_get, _cl_c.TermQuery_term_set)
    def __init__(self, *args):
        newobj = _cl_c.new_TermQuery(*args)
        self.this = newobj.this
        self.thisown = 1
        del newobj.thisown
    def __del__(self, destroy=_cl_c.delete_TermQuery):
        try:
            if self.thisown: destroy(self)
        except: pass
    def sumOfSquaredWeights(self, searcher):
        if not getattr(self, '_prepared', False):
            raise IOError('The prepare method must be called first.')
        return _cl_c.TermQuery_sumOfSquaredWeights(self, (searcher is not None and searcher.this) or None)


    def normalize(self, norm):
        if not getattr(self, '_prepared', False):
            raise IOError('The prepare method must be called first.')
        _cl_c.TermQuery_normalize(self, norm)


    def toString(*args): return _cl_c.TermQuery_toString(*args)
    def prepare(self, reader):
        _cl_c.TermQuery_prepare(self, reader)
        self._prepared = True



class TermQueryPtr(TermQuery):
    def __init__(self, this):
        self.this = this
        if not hasattr(self,"thisown"): self.thisown = 0
        self.__class__ = TermQuery
_cl_c.TermQuery_swigregister(TermQueryPtr)

class MultiTermQuery(Query):
    def __init__(self): raise RuntimeError, "No constructor defined"
    def __repr__(self):
        return "<C lucene::search::MultiTermQuery instance at %s>" % (self.this,)
    def __del__(self, destroy=_cl_c.delete_MultiTermQuery):
        try:
            if self.thisown: destroy(self)
        except: pass
    def sumOfSquaredWeights(self, searcher):
        if not getattr(self, '_prepared', False):
            raise IOError('The prepare method must be called first.')
        return _cl_c.MultiTermQuery_sumOfSquaredWeights(self, (searcher is not None and searcher.this) or None)


    def normalize(self, norm):
        if not getattr(self, '_prepared', False):
            raise IOError('The prepare method must be called first.')
        _cl_c.MultiTermQuery_normalize(self, norm)


    def toString(*args): return _cl_c.MultiTermQuery_toString(*args)

class MultiTermQueryPtr(MultiTermQuery):
    def __init__(self, this):
        self.this = this
        if not hasattr(self,"thisown"): self.thisown = 0
        self.__class__ = MultiTermQuery
_cl_c.MultiTermQuery_swigregister(MultiTermQueryPtr)

class PhraseQuery(Query):
    def __repr__(self):
        return "<C lucene::search::PhraseQuery instance at %s>" % (self.this,)
    def terms_get(*args): return _cl_c.PhraseQuery_terms_get(*args)
    terms = property(_cl_c.PhraseQuery_terms_get, _cl_c.PhraseQuery_terms_set)
    slop = property(_cl_c.PhraseQuery_slop_get, _cl_c.PhraseQuery_slop_set)
    def __init__(self, *args):
        newobj = _cl_c.new_PhraseQuery(*args)
        self.this = newobj.this
        self.thisown = 1
        del newobj.thisown
    def __del__(self, destroy=_cl_c.delete_PhraseQuery):
        try:
            if self.thisown: destroy(self)
        except: pass
    def add(*args): return _cl_c.PhraseQuery_add(*args)
    def sumOfSquaredWeights(self, searcher):
        if not getattr(self, '_prepared', False):
            raise IOError('The prepare method must be called first.')
        return _cl_c.PhraseQuery_sumOfSquaredWeights(self, (searcher is not None and searcher.this) or None)


    def normalize(self, norm):
        if not getattr(self, '_prepared', False):
            raise IOError('The prepare method must be called first.')
        _cl_c.PhraseQuery_normalize(self, norm)


    def toString(*args): return _cl_c.PhraseQuery_toString(*args)
    def prepare(self, reader):
        _cl_c.PhraseQuery_prepare(self, reader)
        self._prepared = True



class PhraseQueryPtr(PhraseQuery):
    def __init__(self, this):
        self.this = this
        if not hasattr(self,"thisown"): self.thisown = 0
        self.__class__ = PhraseQuery
_cl_c.PhraseQuery_swigregister(PhraseQueryPtr)

class BooleanClause(object):
    def __repr__(self):
        return "<C lucene::search::BooleanClause instance at %s>" % (self.this,)
    _deleteQuery = property(_cl_c.BooleanClause__deleteQuery_get, _cl_c.BooleanClause__deleteQuery_set)
    query = property(_cl_c.BooleanClause_query_get, _cl_c.BooleanClause_query_set)
    required = property(_cl_c.BooleanClause_required_get, _cl_c.BooleanClause_required_set)
    prohibited = property(_cl_c.BooleanClause_prohibited_get, _cl_c.BooleanClause_prohibited_set)
    def __init__(self, *args):
        newobj = _cl_c.new_BooleanClause(*args)
        self.this = newobj.this
        self.thisown = 1
        del newobj.thisown
    def __del__(self, destroy=_cl_c.delete_BooleanClause):
        try:
            if self.thisown: destroy(self)
        except: pass

class BooleanClausePtr(BooleanClause):
    def __init__(self, this):
        self.this = this
        if not hasattr(self,"thisown"): self.thisown = 0
        self.__class__ = BooleanClause
_cl_c.BooleanClause_swigregister(BooleanClausePtr)

class BooleanQuery(Query):
    def __repr__(self):
        return "<C lucene::search::BooleanQuery instance at %s>" % (self.this,)
    def clauses_get(*args): return _cl_c.BooleanQuery_clauses_get(*args)
    clauses = property(_cl_c.BooleanQuery_clauses_get, _cl_c.BooleanQuery_clauses_set)
    def __init__(self, *args):
        newobj = _cl_c.new_BooleanQuery(*args)
        self.this = newobj.this
        self.thisown = 1
        del newobj.thisown
    def __del__(self, destroy=_cl_c.delete_BooleanQuery):
        try:
            if self.thisown: destroy(self)
        except: pass
    def add(*args): return _cl_c.BooleanQuery_add(*args)
    def prepare(self, reader):
        _cl_c.BooleanQuery_prepare(self, reader)
        self._prepared = True


    def sumOfSquaredWeights(self, searcher):
        if not getattr(self, '_prepared', False):
            raise IOError('The prepare method must be called first.')
        return _cl_c.BooleanQuery_sumOfSquaredWeights(self, (searcher is not None and searcher.this) or None)


    def normalize(self, norm):
        if not getattr(self, '_prepared', False):
            raise IOError('The prepare method must be called first.')
        _cl_c.BooleanQuery_normalize(self, norm)


    def toString(*args): return _cl_c.BooleanQuery_toString(*args)

class BooleanQueryPtr(BooleanQuery):
    def __init__(self, this):
        self.this = this
        if not hasattr(self,"thisown"): self.thisown = 0
        self.__class__ = BooleanQuery
_cl_c.BooleanQuery_swigregister(BooleanQueryPtr)

class WildcardQuery(MultiTermQuery):
    def __repr__(self):
        return "<C lucene::search::WildcardQuery instance at %s>" % (self.this,)
    def __init__(self, *args):
        newobj = _cl_c.new_WildcardQuery(*args)
        self.this = newobj.this
        self.thisown = 1
        del newobj.thisown
    def __del__(self, destroy=_cl_c.delete_WildcardQuery):
        try:
            if self.thisown: destroy(self)
        except: pass
    def prepare(self, reader):
        _cl_c.WildcardQuery_prepare(self, reader)
        self._prepared = True


    def toString(*args): return _cl_c.WildcardQuery_toString(*args)

class WildcardQueryPtr(WildcardQuery):
    def __init__(self, this):
        self.this = this
        if not hasattr(self,"thisown"): self.thisown = 0
        self.__class__ = WildcardQuery
_cl_c.WildcardQuery_swigregister(WildcardQueryPtr)

class PrefixQuery(Query):
    def __repr__(self):
        return "<C lucene::search::PrefixQuery instance at %s>" % (self.this,)
    def __init__(self, *args):
        newobj = _cl_c.new_PrefixQuery(*args)
        self.this = newobj.this
        self.thisown = 1
        del newobj.thisown
    def __del__(self, destroy=_cl_c.delete_PrefixQuery):
        try:
            if self.thisown: destroy(self)
        except: pass
    def prepare(self, reader):
        _cl_c.PrefixQuery_prepare(self, reader)
        self._prepared = True


    def sumOfSquaredWeights(self, searcher):
        if not getattr(self, '_prepared', False):
            raise IOError('The prepare method must be called first.')
        return _cl_c.PrefixQuery_sumOfSquaredWeights(self, (searcher is not None and searcher.this) or None)


    def normalize(self, norm):
        if not getattr(self, '_prepared', False):
            raise IOError('The prepare method must be called first.')
        _cl_c.PrefixQuery_normalize(self, norm)


    def toString(*args): return _cl_c.PrefixQuery_toString(*args)

class PrefixQueryPtr(PrefixQuery):
    def __init__(self, this):
        self.this = this
        if not hasattr(self,"thisown"): self.thisown = 0
        self.__class__ = PrefixQuery
_cl_c.PrefixQuery_swigregister(PrefixQueryPtr)

class FilteredTermEnum(TermEnum):
    def __init__(self): raise RuntimeError, "No constructor defined"
    def __repr__(self):
        return "<C FilteredTermEnum instance at %s>" % (self.this,)
    def __del__(self, destroy=_cl_c.delete_FilteredTermEnum):
        try:
            if self.thisown: destroy(self)
        except: pass

class FilteredTermEnumPtr(FilteredTermEnum):
    def __init__(self, this):
        self.this = this
        if not hasattr(self,"thisown"): self.thisown = 0
        self.__class__ = FilteredTermEnum
_cl_c.FilteredTermEnum_swigregister(FilteredTermEnumPtr)

class FuzzyTermEnum(FilteredTermEnum):
    def __init__(self): raise RuntimeError, "No constructor defined"
    def __repr__(self):
        return "<C lucene::search::FuzzyTermEnum instance at %s>" % (self.this,)
    def __del__(self, destroy=_cl_c.delete_FuzzyTermEnum):
        try:
            if self.thisown: destroy(self)
        except: pass
    def EndEnum(*args): return _cl_c.FuzzyTermEnum_EndEnum(*args)
    def close(*args): return _cl_c.FuzzyTermEnum_close(*args)
    def setFuzzyThreshold(*args): return _cl_c.FuzzyTermEnum_setFuzzyThreshold(*args)

class FuzzyTermEnumPtr(FuzzyTermEnum):
    def __init__(self, this):
        self.this = this
        if not hasattr(self,"thisown"): self.thisown = 0
        self.__class__ = FuzzyTermEnum
_cl_c.FuzzyTermEnum_swigregister(FuzzyTermEnumPtr)

class FuzzyQuery(MultiTermQuery):
    def __repr__(self):
        return "<C lucene::search::FuzzyQuery instance at %s>" % (self.this,)
    def __init__(self, *args):
        newobj = _cl_c.new_FuzzyQuery(*args)
        self.this = newobj.this
        self.thisown = 1
        del newobj.thisown
    def __del__(self, destroy=_cl_c.delete_FuzzyQuery):
        try:
            if self.thisown: destroy(self)
        except: pass
    def prepare(self, reader):
        _cl_c.FuzzyQuery_prepare(self, reader)
        self._prepared = True


    def toString(*args): return _cl_c.FuzzyQuery_toString(*args)

class FuzzyQueryPtr(FuzzyQuery):
    def __init__(self, this):
        self.this = this
        if not hasattr(self,"thisown"): self.thisown = 0
        self.__class__ = FuzzyQuery
_cl_c.FuzzyQuery_swigregister(FuzzyQueryPtr)

class RangeQuery(Query):
    def __repr__(self):
        return "<C lucene::search::RangeQuery instance at %s>" % (self.this,)
    def __init__(self, *args):
        newobj = _cl_c.new_RangeQuery(*args)
        self.this = newobj.this
        self.thisown = 1
        del newobj.thisown
    def __del__(self, destroy=_cl_c.delete_RangeQuery):
        try:
            if self.thisown: destroy(self)
        except: pass
    def prepare(self, reader):
        _cl_c.RangeQuery_prepare(self, reader)
        self._prepared = True


    def sumOfSquaredWeights(self, searcher):
        if not getattr(self, '_prepared', False):
            raise IOError('The prepare method must be called first.')
        return _cl_c.RangeQuery_sumOfSquaredWeights(self, (searcher is not None and searcher.this) or None)


    def normalize(self, norm):
        if not getattr(self, '_prepared', False):
            raise IOError('The prepare method must be called first.')
        _cl_c.RangeQuery_normalize(self, norm)


    def toString(*args): return _cl_c.RangeQuery_toString(*args)

class RangeQueryPtr(RangeQuery):
    def __init__(self, this):
        self.this = this
        if not hasattr(self,"thisown"): self.thisown = 0
        self.__class__ = RangeQuery
_cl_c.RangeQuery_swigregister(RangeQueryPtr)

class Filter(object):
    def __repr__(self):
        return "<C lucene::search::Filter instance at %s>" % (self.this,)
    def __del__(self, destroy=_cl_c.delete_Filter):
        try:
            if self.thisown: destroy(self)
        except: pass
    def bits(*args): return _cl_c.Filter_bits(*args)
    def __init__(self, *args):
        if self.__class__ == Filter:
            args = (None,) + args
        else:
            args = (self,) + args
        newobj = _cl_c.new_Filter(*args)
        self.this = newobj.this
        self.thisown = 1
        del newobj.thisown
    def __disown__(self):
        self.thisown = 0
        _cl_c.disown_Filter(self)
        return weakref_proxy(self)

class FilterPtr(Filter):
    def __init__(self, this):
        self.this = this
        if not hasattr(self,"thisown"): self.thisown = 0
        self.__class__ = Filter
_cl_c.Filter_swigregister(FilterPtr)

class DateFilter(Filter):
    def __repr__(self):
        return "<C lucene::search::DateFilter instance at %s>" % (self.this,)
    def __del__(self, destroy=_cl_c.delete_DateFilter):
        try:
            if self.thisown: destroy(self)
        except: pass
    def __init__(self, field, fromTime, toTime):
        fromTime = _maybeDatetimeToInt(fromTime)
        toTime = _maybeDatetimeToInt(toTime)

        newobj = _cl_c.new_DateFilter(field, fromTime, toTime)
        # Remember to execute the SWIG boilerplate:
        self.this = newobj.this
        self.thisown = 1
        del newobj.thisown


    Before = staticmethod(_cl_c.DateFilter_Before)
    After = staticmethod(_cl_c.DateFilter_After)
    def bits(*args): return _cl_c.DateFilter_bits(*args)

class DateFilterPtr(DateFilter):
    def __init__(self, this):
        self.this = this
        if not hasattr(self,"thisown"): self.thisown = 0
        self.__class__ = DateFilter
_cl_c.DateFilter_swigregister(DateFilterPtr)

DateFilter_Before = _cl_c.DateFilter_Before

DateFilter_After = _cl_c.DateFilter_After

class TopDocs(object):
    def __init__(self): raise RuntimeError, "No constructor defined"
    def __repr__(self):
        return "<C lucene::search::TopDocs instance at %s>" % (self.this,)
    def __iter__(*args): return _cl_c.TopDocs___iter__(*args)
    totalHits = property(_cl_c.TopDocs_totalHits_get)
    scoreDocs = property(_cl_c.TopDocs_scoreDocs_get)
    def __del__(self, destroy=_cl_c.delete_TopDocs):
        try:
            if self.thisown: destroy(self)
        except: pass

class TopDocsPtr(TopDocs):
    def __init__(self, this):
        self.this = this
        if not hasattr(self,"thisown"): self.thisown = 0
        self.__class__ = TopDocs
_cl_c.TopDocs_swigregister(TopDocsPtr)

class ScoreDoc(object):
    def __repr__(*args): return _cl_c.ScoreDoc___repr__(*args)
    score = property(_cl_c.ScoreDoc_score_get, _cl_c.ScoreDoc_score_set)
    doc = property(_cl_c.ScoreDoc_doc_get, _cl_c.ScoreDoc_doc_set)
    def __init__(self, *args):
        newobj = _cl_c.new_ScoreDoc(*args)
        self.this = newobj.this
        self.thisown = 1
        del newobj.thisown
    def __del__(self, destroy=_cl_c.delete_ScoreDoc):
        try:
            if self.thisown: destroy(self)
        except: pass

class ScoreDocPtr(ScoreDoc):
    def __init__(self, this):
        self.this = this
        if not hasattr(self,"thisown"): self.thisown = 0
        self.__class__ = ScoreDoc
_cl_c.ScoreDoc_swigregister(ScoreDocPtr)

class IndexSearcher(Searcher):
    def __repr__(self):
        return "<C lucene::search::IndexSearcher instance at %s>" % (self.this,)
    def __init__(self, *args):
        newobj = _cl_c.new_IndexSearcher_FromString(*args)
        self.this = newobj.this
        self.thisown = 1
        del newobj.thisown
    def __del__(self, destroy=_cl_c.delete_IndexSearcher):
        try:
            if self.thisown: destroy(self)
        except: pass
    def close(*args): return _cl_c.IndexSearcher_close(*args)
    def docFreq(*args): return _cl_c.IndexSearcher_docFreq(*args)
    def doc(*args): return _cl_c.IndexSearcher_doc(*args)
    def maxDoc(*args): return _cl_c.IndexSearcher_maxDoc(*args)

class IndexSearcherPtr(IndexSearcher):
    def __init__(self, this):
        self.this = this
        if not hasattr(self,"thisown"): self.thisown = 0
        self.__class__ = IndexSearcher
_cl_c.IndexSearcher_swigregister(IndexSearcherPtr)

def IndexSearcher_FromIndexReader(*args):
    val = _cl_c.new_IndexSearcher_FromIndexReader(*args)
    val.thisown = 1
    return val

class Token(object):
    def __repr__(self):
        return "<C lucene::analysis::Token instance at %s>" % (self.this,)
    positionIncrement = property(_cl_c.Token_positionIncrement_get, _cl_c.Token_positionIncrement_set)
    text = property(_cl_c.Token_text_get, _cl_c.Token_text_set)
    start = property(_cl_c.Token_start_get, _cl_c.Token_start_set)
    end = property(_cl_c.Token_end_get, _cl_c.Token_end_set)
    type = property(_cl_c.Token_type_get, _cl_c.Token_type_set)
    sourceSlice = property(_cl_c.Token_sourceSlice_get, _cl_c.Token_sourceSlice_set)
    def __del__(self, destroy=_cl_c.delete_Token):
        try:
            if self.thisown: destroy(self)
        except: pass
    def __init__(self, *args):
        newobj = _cl_c.new_Token(*args)
        self.this = newobj.this
        self.thisown = 1
        del newobj.thisown

class TokenPtr(Token):
    def __init__(self, this):
        self.this = this
        if not hasattr(self,"thisown"): self.thisown = 0
        self.__class__ = Token
_cl_c.Token_swigregister(TokenPtr)

class TokenStream(object):
    def __init__(self): raise RuntimeError, "No constructor defined"
    def __repr__(self):
        return "<C lucene::analysis::TokenStream instance at %s>" % (self.this,)
    def next(*args): return _cl_c.TokenStream_next(*args)
    close = _makeSingleShotMethod(_cl_c.TokenStream_close, 'closed')


    def __del__(self, destroy=_cl_c.delete_TokenStream):
        try:
            if self.thisown: destroy(self)
        except: pass

class TokenStreamPtr(TokenStream):
    def __init__(self, this):
        self.this = this
        if not hasattr(self,"thisown"): self.thisown = 0
        self.__class__ = TokenStream
_cl_c.TokenStream_swigregister(TokenStreamPtr)

class Analyzer(object):
    def __repr__(self):
        return "<C lucene::analysis::Analyzer instance at %s>" % (self.this,)
    def tokenStream(*args): return _cl_c.Analyzer_tokenStream(*args)
    def __del__(self, destroy=_cl_c.delete_Analyzer):
        try:
            if self.thisown: destroy(self)
        except: pass
    def __init__(self, *args):
        if self.__class__ == Analyzer:
            args = (None,) + args
        else:
            args = (self,) + args
        newobj = _cl_c.new_Analyzer(*args)
        self.this = newobj.this
        self.thisown = 1
        del newobj.thisown
    def __disown__(self):
        self.thisown = 0
        _cl_c.disown_Analyzer(self)
        return weakref_proxy(self)

class AnalyzerPtr(Analyzer):
    def __init__(self, this):
        self.this = this
        if not hasattr(self,"thisown"): self.thisown = 0
        self.__class__ = Analyzer
_cl_c.Analyzer_swigregister(AnalyzerPtr)

class StandardTokenizer(object):
    def __repr__(self):
        return "<C lucene::analysis::standard::StandardTokenizer instance at %s>" % (self.this,)
    def __init__(self, reader):
        if reader is None:
            reader = ''
        if isinstance(reader, basestring):
            reader = StringReader(reader)

        newobj = _cl_c.new_StandardTokenizer(reader)
        # Remember to execute the SWIG boilerplate:
        self.this = newobj.this
        self.thisown = 1
        del newobj.thisown

        # Unless ref to reader is maintained, it can be released prematurely.
        self._reader = reader


    def __del__(self, destroy=_cl_c.delete_StandardTokenizer):
        try:
            if self.thisown: destroy(self)
        except: pass
    close = _makeSingleShotMethod(_cl_c.StandardTokenizer_close, 'closed')


    def next(*args): return _cl_c.StandardTokenizer_next(*args)

class StandardTokenizerPtr(StandardTokenizer):
    def __init__(self, this):
        self.this = this
        if not hasattr(self,"thisown"): self.thisown = 0
        self.__class__ = StandardTokenizer
_cl_c.StandardTokenizer_swigregister(StandardTokenizerPtr)

class StandardAnalyzer(Analyzer):
    def __repr__(self):
        return "<C lucene::analysis::standard::StandardAnalyzer instance at %s>" % (self.this,)
    def __init__(self, *args):
        newobj = _cl_c.new_StandardAnalyzer(*args)
        self.this = newobj.this
        self.thisown = 1
        del newobj.thisown
    def __del__(self, destroy=_cl_c.delete_StandardAnalyzer):
        try:
            if self.thisown: destroy(self)
        except: pass
    def tokenStream(self, fieldName, reader):
        ts = _cl_c.StandardAnalyzer_tokenStream(self, fieldName, reader)

        # Unless these refs are retained, necessary supporting objects might
        # be prematurely garbage collected:
        ts._ref_analyzer = self
        ts._ref_reader = reader

        return ts



class StandardAnalyzerPtr(StandardAnalyzer):
    def __init__(self, this):
        self.this = this
        if not hasattr(self,"thisown"): self.thisown = 0
        self.__class__ = StandardAnalyzer
_cl_c.StandardAnalyzer_swigregister(StandardAnalyzerPtr)

class QueryParser(object):
    def __repr__(self):
        return "<C lucene::queryParser::QueryParser instance at %s>" % (self.this,)
    def __init__(self, *args):
        newobj = _cl_c.new_QueryParser(*args)
        self.this = newobj.this
        self.thisown = 1
        del newobj.thisown
    def __del__(self, destroy=_cl_c.delete_QueryParser):
        try:
            if self.thisown: destroy(self)
        except: pass
    parseStatic = staticmethod(_cl_c.QueryParser_parseStatic)
    def parse(*args): return _cl_c.QueryParser_parse(*args)

class QueryParserPtr(QueryParser):
    def __init__(self, this):
        self.this = this
        if not hasattr(self,"thisown"): self.thisown = 0
        self.__class__ = QueryParser
_cl_c.QueryParser_swigregister(QueryParserPtr)

QueryParser_parseStatic = _cl_c.QueryParser_parseStatic

# Directory class:
Directory.__iter__ = lambda self: iter(self.list())

# TermEnum class:
TermEnum.__iter__ = lambda self: _cl_c.TermEnum__iter__(self)

# Token class:
Token.__repr__ = lambda self: '<Token [%d:%d] %s "%s"; pi %d>' % (
    self.start, self.end, self.type, self.text, self.positionIncrement
  )

# Add an optional argument to the Token constructor to facilite more concise
# setting of the positionIncrement.
_oldTokenConstructor = Token.__init__
def _newTokenConstructor(self, text, start, end, typ='word', positionIncrement=1):
  _oldTokenConstructor(self, text, start, end, typ)
  # It costs a method call to set positionIncrement; don't do so unless it has
  # a non-default value.
  if positionIncrement != 1:
    self.positionIncrement = positionIncrement
Token.__init__ = _newTokenConstructor

# TokenStream class:
TokenStream.__iter__ = lambda self: iter(self.next, None)

# StandardTokenizer class:
StandardTokenizer.__iter__ = lambda self: iter(self.next, None)


# Hits class:
def _Hits__rangeCheck(self, n):
    if n < 0 or n >= len(self):
        raise IndexError('Hit number %d not in range(0, %d).' % (n, len(self)))
Hits._rangeCheck = _Hits__rangeCheck
del _Hits__rangeCheck

Hits.__getitem__ = Hits.doc

def _Hits___iter__(self):
    for i in xrange(len(self)):
        yield self.doc(i)
Hits.__iter__ = _Hits___iter__


# DateFilter class:
def _new_DateFilter_Before(field, cutoffTime):
    cutoffTime = _maybeDatetimeToInt(cutoffTime)
    return _cl_c.DateFilter_Before(field, cutoffTime)
_new_DateFilter_Before = staticmethod(_new_DateFilter_Before)
DateFilter.Before = _new_DateFilter_Before

def _new_DateFilter_After(field, cutoffTime):
    cutoffTime = _maybeDatetimeToInt(cutoffTime)
    return _cl_c.DateFilter_After(field, cutoffTime)
_new_DateFilter_After = staticmethod(_new_DateFilter_After)
DateFilter.After = _new_DateFilter_After


# TopDocs class:
TopDocs.__len__ = lambda self: self.totalHits


# ScoreDoc class:
def _ScoreDoc___eq__(self, other):
    if not isinstance(other, ScoreDoc):
        return False
    return self.doc == other.doc and self.score == other.score
ScoreDoc.__eq__ = _ScoreDoc___eq__



