import datetime, os, os.path, sys, time

import _cl_py as _pc # SWIG-generated Python module
_c = _pc._cl_c       # SWIG-generated C++ extension module that underlies
                     #  Python module above


UNICODE_BUILD = bool(_c.cvar.UNICODE_BUILD)
if UNICODE_BUILD:
    import codecs
    PythonUnicodeStream = codecs.StreamReaderWriter
else:
    PythonUnicodeStream = type(None)

SUMO_BUILD = bool(_c.cvar.SUMO_BUILD)

_MAGIC = 42


# Locally reference some utility functions from the SWIG-generated module:
_makePythonIteratorCompliantNextMethod = _pc._makePythonIteratorCompliantNextMethod
_makeSingleShotMethod = _pc._makeSingleShotMethod
_maybeDatetimeToInt = _pc._maybeDatetimeToInt


class CLBugGuard(Exception):
    # This type of exception is raised to prevent the Python programmer from
    # activating a known CLucene bug.
    pass


class NotExposedToPython(NotImplementedError):
    pass


class _MustBeOpen(object):
    # For many I/O classes, CLucene segfaults unless an openness check is
    # enforced before attempting an I/O operation.
    def __init__(self, classDesc, closeMethod, openPredicate=None):
        # $openPredicate: A single-argument callable that returns True/False
        #   to indicate whether this resource is still considered open.
        #   The callable receives $self as its first argument.
        #   Defaults to _MustBeOpen.__flagBasedOpenPredicate.
        self.__classDesc = classDesc
        if closeMethod is None:
            self.__closeMethod = _NOOP
        else:
            self.__closeMethod = closeMethod

        self.__isOpen = True
        if openPredicate is not None:
            self.__openPredicate = openPredicate
        else:
            self.__openPredicate = _MustBeOpen.__flagBasedOpenPredicate

        try:
            self._ensureOpen()
        except IOError:
            raise IOError('Underlying resource was *already* closed before'
                ' %s.__init__.' % classDesc
              )

    def __flagBasedOpenPredicate(self):
        return self.__isOpen

    def _isOpen(self):
        openPred = getattr(self, '_MustBeOpen__openPredicate')
        return openPred(self)

    def _ensureOpen(self):
        if not self._isOpen():
            raise IOError('This %s has already been closed.' % self.__classDesc)

    def close(self, *args, **kwargs):
        self._ensureOpen()
        retVal = self.__closeMethod(self, *args, **kwargs)
        self.__isOpen = False
        # Reset the open predicate to our generic flag-based implementation so
        # we can be sure that it will return False in the future.
        self.__openPredicate = _MustBeOpen.__flagBasedOpenPredicate

        assert not self._isOpen()
        return retVal


def _mustbeopen(func):
    # Decorator that prevents a method from being called unless its associated
    # _MustBeOpen instance is open.
    def opennessEnforcer(self, *args, **kwargs):
        self._ensureOpen()
        return func(self, *args, **kwargs)
    return opennessEnforcer


def _NOOP(*args):
    pass


def _explainNotExposed(*args):
    # No matter what the arguments, this function informs the client programmer
    # that the corresponding method in CLucene has not been exposed to Python.
    raise NotExposedToPython


if SUMO_BUILD:
    InputStream = _pc.InputStream # Abstract class

    class FSInputStream(_MustBeOpen, _pc.FSInputStream):
        def __init__(self, *args, **kwargs):
            _MustBeOpen.__init__(self, 'FSInputStream', _pc.FSInputStream.close)
            _pc.FSInputStream.__init__(self, *args, **kwargs)

        __len__ = _mustbeopen(_pc.FSInputStream.__len__)
        clone = _mustbeopen(_pc.FSInputStream.clone)

        readByte = _mustbeopen(_pc.FSInputStream.readByte)
        readBytes = _mustbeopen(_pc.FSInputStream.readBytes)
        read = _mustbeopen(_pc.FSInputStream.read)
        readChars = _mustbeopen(_pc.FSInputStream.readChars)
        readInt = _mustbeopen(_pc.FSInputStream.readInt)
        readVInt = _mustbeopen(_pc.FSInputStream.readVInt)
        readLong = _mustbeopen(_pc.FSInputStream.readLong)
        readVLong = _mustbeopen(_pc.FSInputStream.readVLong)
        readString = _mustbeopen(_pc.FSInputStream.readString)
        seek = _mustbeopen(_pc.FSInputStream.seek)


    OutputStream = _pc.OutputStream # Abstract class
    def _OutputStream__del__(self, destroy=_c.delete_OutputStream):
        # Redefine __del__ so that it close()s the stream if that hasn't been
        # done.
        if getattr(self, 'thisown', False):
            try:
                if not self.isClosed():
                    self.close()
            except:
                pass
            destroy(self)
    OutputStream.__del__ = _OutputStream__del__


    class FSOutputStream(_MustBeOpen, _pc.FSOutputStream):
        def __init__(self, *args, **kwargs):
            _MustBeOpen.__init__(self, 'FSOutputStream', _pc.FSOutputStream.close)
            _pc.FSOutputStream.__init__(self, *args, **kwargs)

        __len__ = _mustbeopen(_pc.FSOutputStream.__len__)
        writeByte = _mustbeopen(_pc.FSOutputStream.writeByte)
        writeBytes = _mustbeopen(_pc.FSOutputStream.writeBytes)
        writeInt = _mustbeopen(_pc.FSOutputStream.writeInt)
        writeVInt = _mustbeopen(_pc.FSOutputStream.writeVInt)
        writeLong = _mustbeopen(_pc.FSOutputStream.writeLong)
        writeVLong = _mustbeopen(_pc.FSOutputStream.writeVLong)
        writeString = _mustbeopen(_pc.FSOutputStream.writeString)

        def writeChars(self, string):
            # YYY: Encoding issues?
            return _pc.FSOutputStream.writeChars(self, string, len(string))
        writeChars = _mustbeopen(writeChars)

        def seek(self, *args):
            raise NotExposedToPython
        seek = _mustbeopen(seek)

    Lock = _pc.Lock
    LockWith = _pc.LockWith


    # lucene::store

    class FSLock(_pc.FSLock):
        def __init__(self, filename):
            if '\0' in filename:
                raise ValueError('filename containing null byte not allowed.')
            _pc.FSLock.__init__(self, filename)


Directory = _pc.Directory # Abstract class

class FSDirectory(_MustBeOpen, _pc.FSDirectory):
    def __init__(self, directory, createIfDoesNotExist=False):
        _MustBeOpen.__init__(self, 'FSDirectory', _pc.FSDirectory.close)

        directory = os.path.abspath(directory)
        if os.path.isfile(directory):
            raise IOError('A file named "%s" already exists.' % directory)
        dirExists = os.path.isdir(directory)
        if createIfDoesNotExist:
            if dirExists:
                raise IOError('Directory "%s" already exists; you must explicitly'
                    ' delete it before calling this constructor with'
                    ' createIfDoesNotExist=True.'
                    % directory
                  )
            else:
                if not os.path.isdir(os.path.dirname(directory)):
                    raise IOError(
                        'Parent directory "%s" does not exist; cannot create "%s".'
                        % (os.path.dirname(directory), directory)
                      )

            # Create the directory here, not in CLucene, whose dir-creating
            # facility was observed to crash after extended use.
            # YYY:CL_BUG: CL segfaults in FSDirectory::create after extended
            # use, so for the moment I chose to circumvent the problem by
            # creating the dir with Python's os module, then unconditionally
            # passing False as the second arg to the FSDirectory constructor.
            os.makedirs(directory)
            assert os.path.isdir(directory)
        elif not createIfDoesNotExist and not dirExists:
            raise IOError('Directory "%s" does not exist.' % directory)
        _pc.FSDirectory.__init__(self, directory, False)


    def _shaveOrDie(self, filename, desc='The file'):
        # If the $filename doesn't fall below this directory, raise an IOError.
        # Return the filename *relative* to this directory.
        thisDir = self.name
        if os.path.commonprefix((thisDir, os.path.abspath(filename))) == thisDir:
            filename = filename[len(thisDir):] # Remove prefix.
        elif len([x for x in os.path.split(filename) if x]) > 1:
            raise IOError('%s must reside below "%s".  "%s" does not.'
                % (desc, thisDir, filename)
              )
        return filename

    def list(self):
        # Instead of wrapping the C++ impl, we use the Python stdlib impl.
        return os.listdir(self.name)
    list = _mustbeopen(list)

    # Proper guarding of close method is taken care of by _MustBeOpen.close.

    if SUMO_BUILD:
        # The static method getDirectory is not wrapped; I didn't see the
        # point, since the constructor accomplishes the same thing.

        # The static version of fileModified is not wrapped; I didn't see the point.

        fileExists = _mustbeopen(_pc.FSDirectory.fileExists)
        fileModified = _mustbeopen(_pc.FSDirectory.fileModified)
        fileLength = _mustbeopen(_pc.FSDirectory.fileLength)

        def deleteFile(self, filename):
            filename = self._shaveOrDie(filename, desc='The file to be deleted')
            return _pc.FSDirectory.deleteFile(self, filename, True)
        deleteFile = _mustbeopen(deleteFile)

        def renameFile(self, fromName, toName):
            fromName = self._shaveOrDie(fromName, desc='The source filename')
            toName = self._shaveOrDie(toName, desc='The destination filename')
            return _pc.FSDirectory.renameFile(self, fromName, toName)
        renameFile = _mustbeopen(renameFile)

        def createFile(self, filename):
            filename = self._shaveOrDie(filename, desc='The file to be created')
            return _pc.FSDirectory.createFile(self, filename)
        createFile = _mustbeopen(createFile)

        def openFile(self, filename):
            filename = self._shaveOrDie(filename, desc='The file to be opened')
            return _pc.FSDirectory.openFile(self, filename)
        openFile = _mustbeopen(openFile)

        def makeLock(self, filename):
            filename = self._shaveOrDie(filename, desc='The lock file')
            return _pc.FSDirectory.makeLock(self, filename)
        makeLock = _mustbeopen(makeLock)

        # refInc method not wrapped.


class RAMDirectory(_MustBeOpen, _pc.RAMDirectory):
    def __init__(self):
        _MustBeOpen.__init__(self, 'RAMDirectory', _pc.RAMDirectory.close)
        _pc.RAMDirectory.__init__(self)

    list = _mustbeopen(_pc.RAMDirectory.list)

    # Notice that we call _MustBeOpen.close rather than _pc.RAMDirectory.close.
    # We passed a reference to _pc.RAMDirectory.close to the _MustBeOpen
    # constructor, so _MustBeOpen.close will call _pc.RAMDirectory.close.
    close = _mustbeopen(_MustBeOpen.close)

    # Without these guards, calling the method results in a segfault:
    fileExists = _explainNotExposed
    fileModified = _explainNotExposed
    fileLength = _explainNotExposed
    deleteFile = _explainNotExposed
    renameFile = _explainNotExposed
    createFile = _explainNotExposed
    openFile = _explainNotExposed
    makeLock = _explainNotExposed


# lucene::document

class Field(object):
    # Notice that the Field class is pure Python--not associated with a SWIG
    # shadow class.
    def __init__(self, name, value,
        isStored=False, isIndexed=True, isTokenized=True
      ):
        self._name = name
        self._value = value
        self._isStored = isStored
        self._isIndexed = isIndexed
        self._isTokenized = isTokenized

    name           = property(lambda self: self._name)
    value          = property(lambda self: self._value)
    isStored       = property(lambda self: self._isStored)
    isIndexed      = property(lambda self: self._isIndexed)
    isTokenized    = property(lambda self: self._isTokenized)

    # Static shortcut constructors:
    def Keyword(name, value):
        return Field(name, value, True, True, False)
    Keyword = staticmethod(Keyword)

    def Unindexed(name, value):
        return Field(name, value, True, False, False)
    Unindexed = staticmethod(Unindexed)

    def Text(name, value):
        return Field(name, value, True, True, True)
    Text = staticmethod(Text)

    def Unstored(name, value):
        return Field(name, value, False, True, True)
    Unstored = staticmethod(Unstored)


class DateField(object):
    MAX_DATE_STRING = 'zzzzzzzzz'
    MIN_DATE_STRING = '000000000'

    def __init__(self):
        raise TypeError('DateField is a purely static class.')

    def timeToString(t=None):
        if t is None:
            t = int(time.time())
        elif isinstance(t, datetime.datetime):
            t = int(time.mktime(t.timetuple()))
        return _c.DateField_timeToString(t)
    timeToString = staticmethod(timeToString)

    def stringToTicks(s):
        return _c.DateField_stringToTime(s)
    stringToTicks = staticmethod(stringToTicks)

    def stringToTime(s):
        t = DateField.stringToTicks(s)
        return datetime.datetime.fromtimestamp(t)
    stringToTime = staticmethod(stringToTime)


### READERS ###
 #
 # The exposed subclasses of lucene::util::Reader are meant to be treated as
 # "opaque handles" passed to the internals of CLucene, not used by ordinary
 # Python client code to read files or strings (why duplicate functionality
 # that's already provided by Python?).
 #   Because of these classes' intended opacity, only the constructor and the
 # close method are exposed.
 #
 # Also, the lucene::util::Reader class itself is not exposed.  There is little
 # point in exposing this abstract class, since it would not be [usefully]
 # subclassable by ordinary Python client code.  Enabling SWIG directors for
 # lucene::util::Reader would *KILL* performance, because typical CLucene
 # internal client code operates Readers very granularly.
 #   For example, CLucene's internals often extract characters from a Reader on
 # a character-by-character basis.  Since Python represents characters as
 # strings of length 1, character-by-character extraction is already fairly
 # inefficient.  Add to that the overhead of SWIG's directors, and the
 # inefficiency becomes prohibitive (about 90x in exploratory tests).
 #
 # So, the only remaining reason for exposing Reader itself would be to allow
 #   isinstance(objThatsEitherStringReaderOrFileReader, lucene.Reader)
 # to work as expected.  Since FileReader and StringReader can't be usefully
 # subclassed, I decided not to add more wrapping overhead just to make
 # isinstance work as expected.

FileReader = _pc.FileReader
StringReader = _pc.StringReader


class Document(object):
    # Notice that the Document class is pure Python--not associated with a SWIG
    # shadow class.
    def __init__(self, _notInternalClient=True):
        # Internal client code will separately create the _fields dict.
        if _notInternalClient:
            self._fields = {}

    def __iter__(self):
        return self._fields.itervalues()

    def add(self, field):
        if not isinstance(field, Field):
            raise TypeError('pyclene.lucene.Field instance required.')
        self[field.name] = field

    def __setitem__(self, fieldName, field):
        name = field.name
        if name in self._fields:
            raise KeyError('A field named "%s" is already present.' % name)
        self._fields[name] = field

    def __delitem__(self, _):
        raise TypeError('Cannot delete Field from Document.')

    def __getitem__(self, fieldName):
        # Returns the Field's value:
        return self._fields[fieldName].value

    def field(self, fieldName):
        # Returns the Field object itself:
        return self._fields[fieldName]

    def fields(self):
        return self._fields.itervalues()


# lucene::index

class TermDocs(_MustBeOpen, _pc.TermDocs):
    def __init__(self):
        _MustBeOpen.__init__(self, 'TermDocs', _pc.TermDocs.close)
        # _pc.TermDocs considers itself abstract.

    def __del__(self):
        if getattr(self, 'thisown', True):
            _c.delete_TermDocs(self.this)

    def __iter__(self):
        # The return values of the various informational methods are invalid
        # until next() method has been called at least once.
        while self.next():
            yield (self.doc(), self.freq())

    seek = _mustbeopen(_pc.TermDocs.seek)
    doc = _mustbeopen(_pc.TermDocs.doc)
    freq = _mustbeopen(_pc.TermDocs.freq)
    next = _mustbeopen(_pc.TermDocs.next)
    # The read method is not wrapped.
    skipTo = _mustbeopen(_pc.TermDocs.skipTo)


class TermPositions(TermDocs, _pc.TermPositions):
    def next(self):
        ret = TermDocs.next(self)
        if ret:
            # YYY:CL_BUG:
            # pyclene enforces the constraint declared in the JavaDocs for
            # TermPositions.nextPosition because CLucene does not enforce it.
            # Should this check be added to CLucene?
            self._nNextPosCallsAllowed = self.freq()
        else:
            self._nNextPosCallsAllowed = 0
        return ret

    def nextPosition(self):
        if self._nNextPosCallsAllowed == 0:
            raise IOError('There are no more occurrences of the term in the'
                ' current document.  This nextPosition() call is considered an'
                ' error.  nextPosition() should not be called no more than'
                ' freq() times without calling next().'
              )
        try:
            ret = _pc.TermPositions.nextPosition(self)
        except Exception, e:
            raise IOError(str(e))
        else:
            self._nNextPosCallsAllowed -= 1
            return ret
    nextPosition = _mustbeopen(nextPosition)


TermEnum = _pc.TermEnum


class IndexReader(_MustBeOpen, _pc.IndexReader):
    def __init__(self, USE_STATICMETHOD_INDEXREADER_OPEN_INSTEAD=None):
        if USE_STATICMETHOD_INDEXREADER_OPEN_INSTEAD != _MAGIC:
            raise NotImplementedError('To create an IndexReader, client code'
                ' should use static method IndexReader.open instead of this'
                ' constructor.'
              )

        _MustBeOpen.__init__(self, 'IndexReader', None)
        # Notice that we do *not* call _pc.IndexReader.__init__.
        # (This constructor is only used by the static open method to create
        # an object onto which it can attach the trappings of a _pc.IndexReader
        # instance for SWIG's benefit.)

    def __del__(self):
        if self._isOpen():
            self.close()
        _pc.IndexReader.__del__(self)

    def close(self):
        if getattr(self, 'thisown', False):
            retVal = _pc.IndexReader.close(self)
        _MustBeOpen.close(self)
        return retVal
    close = _mustbeopen(close)

    def open(directory):
        # I chose not to implement the closeDir parameter of CL's
        # IndexReader.open because of Python-proxy/C++-object issues.
        # Specifically, if closeDir was true, C++ code in CL would eventually
        # close() the C++ RAMDirectory object, but since I've purposely not
        # enabled SWIG directors for RAMDirectory, the Python proxy was not
        # notified of its C++ companion's closure.  This created the potential
        # for I/O ops on a closed RAMDirectory object.  The problem could be
        # solved by enabling SWIG directors for RAMDirectory, but I deemed the
        # performance cost too great.

        isDirectory = not isinstance(directory, basestring)
        if not isDirectory:
            if not os.path.isdir(directory):
                raise IOError('"%s" is not a directory.' % directory)
            if len(os.listdir(directory)) == 0:
                raise IOError('The "%s" directory is empty.' % directory)
            r = _pc.IndexReader_openFromDirName(directory, True)
        else:
            if len(directory.list()) == 0:
                # Otherwise CL raises Exception; we want IOError.
                raise IOError('The "%s" directory is empty.' %
                    (hasattr(directory, 'name') and directory.name)
                    or 'supplied'
                  )
            r = _pc.IndexReader_openFromDir(directory, False)

        # Transform the lucene::index::IndexReader pointer returned from the
        # C++ layer into an instance of pyclene.lucene.IndexReader.
        # Pass the magic internal constructor parameter so the constructor
        # won't protest with an exception.
        self = IndexReader(USE_STATICMETHOD_INDEXREADER_OPEN_INSTEAD=_MAGIC)
        self.this = r.this
        # Transfer ownership of the underlying C++ object from r to self.
        self.thisown = True
        r.thisown = False

        if isDirectory:
            # Maintain a reference to the directory so it won't be prematurely
            # garbage collected while self is alive.
            self._ref_directory = directory

        return self
    open = staticmethod(open)

    def lastModified(directory):
        # Returns the time the index in this directory was last modified.
        t = _pc.IndexReader.lastModified(directory)
        return datetime.datetime.fromtimestamp(t)
    lastModified = staticmethod(lastModified)

    # The static method _pc.IndexReader.indexExists doesn't need to be modified
    # here.  There's no conversion required and, since it's static, no openness
    # enforcement.

    def isLocked(directory):
        if not isinstance(directory, Directory) and not os.path.isdir(directory):
            raise IOError('"%s" is not a directory.' % directory)
        if not IndexReader.indexExists(directory):
            raise IOError('No index exists at the specified location.')
        return _pc.IndexReader.isLocked(directory)
    isLocked = staticmethod(isLocked)

    def unlock(directory):
        # The isLocked method will perform sanity checks for us.
        if IndexReader.isLocked(directory):
            _pc.IndexReader.unlock(directory)
    unlock = staticmethod(unlock)

    numDocs = _mustbeopen(_pc.IndexReader.numDocs)
    maxDoc = _mustbeopen(_pc.IndexReader.maxDoc)

    def document(self, pos):
        # YYY:CL_BUG:
        # Special-case check against upper and lower bounds (CLucene goes into
        # infinite loop if pos >= maxDoc; performs invalid memory access if
        # pos < 0).
        # Note that maxDoc is actually *one beyond* the max doc identifier.
        if pos < 0 or pos >= _pc.IndexReader.maxDoc(self):
            raise IOError('No document exists at position %s' % pos)
        else:
            try:
                doc = _pc.IndexReader.document(self, pos)
            except Exception, e:
                raise IOError(str(e))

        return doc
    document = _mustbeopen(document)

    def isDeleted(self, pos):
        if pos < 0 or pos >= self.maxDoc():
            raise IOError('%s is beyond the end of the index.')
        return _pc.IndexReader.isDeleted(self, pos)
    isDeleted = _mustbeopen(isDeleted)

    # getNorms method not wrapped.

    def terms(self, after=None):
        if after is None:
            return _pc.IndexReader.termsAll(self)
        else:
            return _pc.IndexReader.termsSpecific(self, after)
    terms = _mustbeopen(terms)

    docFreq = _mustbeopen(_pc.IndexReader.docFreq)

    def termDocs(self, containing=None):
        if containing is None:
            tdPtr = _pc.IndexReader.termDocsAll(self)
        else:
            tdPtr = _pc.IndexReader.termDocsContaining(self, containing)

        # YYY:2004.08.20: More efficient (but uglier) impl changes class on the fly:

        # tdPtr is an instance of _cl_py.TermDocs, but we've defined numerous
        # safeguards and conveniences in lucene.TermDocs, which we want to
        # apply to any Python code that accesses tdPtr.  So we change the
        # existing object's class, and initiate it as a _MustBeOpen instance.
        tdPtr.__class__ = TermDocs
        _MustBeOpen.__init__(tdPtr, 'TermDocs', _pc.TermDocs.close)
        return tdPtr
        # OLD_IMPL:
        # Convert from pyclene._cl_py.TermDocs to pyclene.lucene.TermDocs
        # instance (which will have all of the conveniences defined in
        # pyclene.lucene.TermDocs).
        # td = TermDocs()
        # td.this = tdPtr.this
        # tdPtr.thisown = False
        # td.thisown = True
        # return td
    termDocs = _mustbeopen(termDocs)

    def termPositions(self, containing=None):
        if containing is None:
            tpPtr = _pc.IndexReader.termPositionsAll(self)
        else:
            tpPtr = _pc.IndexReader.termPositionsContaining(self, containing)

        # YYY:2004.08.20: More efficient (but uglier) impl changes class on the fly:

        # tpPtr is an instance of _cl_py.TermPositions, but we've defined
        # numerous safeguards and conveniences in lucene.TermPositions, which
        # we want to apply to any Python code that accesses tpPtr.  So we
        # change the existing object's class, and initiate it as a _MustBeOpen
        # instance.
        tpPtr.__class__ = TermPositions
        _MustBeOpen.__init__(tpPtr, 'TermPositions', _pc.TermPositions.close)
        return tpPtr
        # OLD_IMPL:
        # tp = TermPositions()
        # tp.this = tpPtr.this
        # tp.thisown = True
        # return tp
    termPositions = _mustbeopen(termPositions)

    def delete(self, posOrTerm):
        if isinstance(posOrTerm, int):
            # The document method performs sanity checks that go beyond mere
            # upper/lower bounds checking--documents may have been deleted from
            # anywhere, not just the beginning or the end.
            self.document(posOrTerm)
            return _pc.IndexReader.deleteAt(self, posOrTerm)
        else:
            return _pc.IndexReader.deleteWithTerm(self, posOrTerm)
    delete = _mustbeopen(delete)


class IndexWriter(_MustBeOpen, _pc.IndexWriter):
    def __init__(self, stringOrDir, analyzer, create):
        # Notice that the closeMethod we pass to _MustBeOpen.__init__ does
        # nothing.  Unlike most subclasses of _MustBeOpen in this module, this
        # class implements its own close() method, which calls
        # _MustBeOpen.close, rather than the other way around.
        _MustBeOpen.__init__(self, 'IndexWriter', None)

        _pc.IndexWriter.__init__(self, stringOrDir, analyzer, create)

        if isinstance(stringOrDir, Directory):
            # Maintain a reference to the Directory to prevent it from being
            # deleted while self is still alive.
            self._ref_directory = stringOrDir

        # Maintain a reference to the Analyzer to prevent it from being deleted
        # while self is still alive.
        self._ref_analyzer = analyzer

    def __del__(self):
        # The close method must be called before this object is garbage
        # collected, or it can leak.
        if self._isOpen():
            self.close()
        _pc.IndexWriter.__del__(self)

    docCount = _mustbeopen(_pc.IndexWriter.docCount)
    addDocument = _mustbeopen(_pc.IndexWriter.addDocument)
    optimize = _mustbeopen(_pc.IndexWriter.optimize)
    addIndexes = _mustbeopen(_pc.IndexWriter.addIndexes)

    def close(self, alsoCloseUnderlyingDirectory=False):
        retVal = _pc.IndexWriter.close(self, alsoCloseUnderlyingDirectory)
        _MustBeOpen.close(self)
        return retVal
    close = _mustbeopen(close)


# lucene::analysis

Token = _pc.Token

TokenStream = _pc.TokenStream

# StandardTokenizer class:
class StandardTokenizer(_pc.StandardTokenizer, TokenStream):
    # We deliberately don't expose the entire inheritance hierarchy between
    # lucene::analysis::TokenStream and lucene::analysis::standard::StandardTokenizer.
    # As a result, SWIG doesn't generate code that recognizes StandardTokenizer
    # as a descendant of TokenStream.
    # Here, we paper over the problem by deriving StandardTokenizer from both
    # _pc.StandardTokenizer and TokenStream, even though this is logically
    # redundant.
    pass

    # If StandardTokenizer is changed to actually do something in its close
    # method, this Python class will need a __del__ method that calls close
    # if it hasn't been called already.


Analyzer = _pc.Analyzer
StandardAnalyzer = _pc.StandardAnalyzer

# lucene::queryParser

# Use a metaclass to paper over the unpythonicity of giving the same name to
# both an instance method and a static method.
def _parseFunc(selfOrStringQuery, fieldOrStringQuery=None, analyzer=None):
    if isinstance(selfOrStringQuery, _pc.QueryParser):
        # Act like an instance method (selfOrStringQuery is a QueryParser object).
        if analyzer is not None:
            raise TypeError('The non-static variant of the parse method'
                ' does not accept an analyzer parameter.'
              )
        return _pc.QueryParser.parse(selfOrStringQuery, fieldOrStringQuery)
    else:
        # Act like a static method (selfOrStringQuery is a string).
        return _pc.QueryParser.parseStatic(selfOrStringQuery, fieldOrStringQuery, analyzer)

class _QueryParserMetaClass(type):
    def __getattribute__(self, name):
        if name == 'parse':
            return _parseFunc
        else:
            return type.__getattribute__(self, name)

class QueryParser(_pc.QueryParser):
    __metaclass__ = _QueryParserMetaClass

    def __init__(self, fieldName, analyzer):
        if fieldName is None: # Mustn't let a NULL char* through.
            raise TypeError('fieldName must not be None.')

        _pc.QueryParser.__init__(self, fieldName, analyzer)

        # Make sure the Analyzer isn't deleted while our C++ companion still
        # holds a reference to it.
        self._ref_analyzer = analyzer


# lucene::search

Filter = _pc.Filter
DateFilter = _pc.DateFilter


HitCollector = _pc.HitCollector


# YYY: Write note explaing why we don't subclass _pc.Searcher and implement
# this functionality there (it's because the inheritance hierarchies would then
# not match between pyclene.lucene and pyclene._cl_py).

def _makeCloser_passesPointer(baseClass):
    def _closer(self):
        # Mustn't call _MustBeOpen.close first, but rather, the close method of
        # the other base class.
        retVal = baseClass.close(self.this)
        _MustBeOpen.close(self)
        return retVal
    return _closer

def _makeDocFreq_passesPointer(baseClass):
    def _docFreq(self, term):
        return baseClass.docFreq(self.this, term)
    return _docFreq

def _makeDoc_passesPointer(baseClass):
    def _doc(self, pos):
        # Special-case check against bounds (CLucene IndexSearcher goes into
        # infinite loop if we try to access document >= maxDoc; CLucene
        # MultiSearcher crashes if pos < 0).
        # Note that maxDoc is actually *one beyond* the max doc identifier.
        if pos < 0 or pos >= self.maxDoc():
            raise IndexError('No document exists at position %s' % str(pos))
        else:
            try:
                doc = baseClass.doc(self.this, pos)
            except Exception, e:
                raise IOError(str(e))

        return doc
    return _doc

def _makeMaxDoc_passesPointer(baseClass):
    def _maxDoc(self):
        return baseClass.maxDoc(self.this)
    return _maxDoc

def _Searcher_search(self, query, filter=None, limitNDocs=None, collector=None,
    # _smCache saves a couple of namespace lookups, which is actually important
    # because the search method is a hotspot.
    _smCache=_c.Searcher_search
  ):
    # Openness enforcement note:
    # This method is so performance-critical that we deliberately violate the
    # encapsulation of superclass _MustBeOpen in exchange for a performance
    # gain (we just look up a boolean flag, rather than calling multiple
    # methods).
    if not self._MustBeOpen__isOpen:
        raise IOError('IndexSearcher must be open to perform this operation.')

    hits = _smCache(self.this, query, filter, limitNDocs, collector)

    # Maintain a reference to the Query so it won't be deleted while this
    # Searcher is still alive.
    # YYY: If the Searcher happens to live a long time and service many
    # queries, how can we ensure that queries that have become inactive are
    # released?
    self._ref_queries.setdefault(query, 1)

    if hits is not None:
        # Don't let the Searcher be deleted while there are still live result
        # objects using it.
        hits._ref_indexSearcher = self
    return hits


class IndexSearcher(_MustBeOpen, _pc.IndexSearcher):
    def __init__(self, ind):
        _MustBeOpen.__init__(self, 'IndexSearcher', None)

        indIsReader = isinstance(ind, _pc.IndexReader)
        if indIsReader:
            initFunc = _c.new_IndexSearcher_FromIndexReader
        else:
            # In the absence of this check, CLucene crashes if passed a
            # nonexistent directory:
            if not os.path.isdir(ind):
                raise IOError('Directory "%s" does not exist.' % ind)
            initFunc = _c.new_IndexSearcher_FromString

        self.this = initFunc(ind)
        self.thisown = True

        if indIsReader:
            # Maintain a reference to the IndexReader so it won't be deleted
            # while this Searcher is still alive.
            self._ref_indexReader = ind

        # Used to ensure that the Query objects passed to the search method
        # survive at least as long as self.
        self._ref_queries = {}

    def __del__(self):
        # If the constructor got as far as creating the underlying C++ object,
        # try to close() that object.
        if hasattr(self, 'this') and self._isOpen():
            self.close()

        _pc.IndexSearcher.__del__(self)

    close = _mustbeopen(_makeCloser_passesPointer(_pc.IndexSearcher))
    docFreq = _mustbeopen(_makeDocFreq_passesPointer(_pc.IndexSearcher))
    doc = _mustbeopen(_makeDoc_passesPointer(_pc.IndexSearcher))
    maxDoc = _mustbeopen(_makeMaxDoc_passesPointer(_pc.IndexSearcher))
    search = _Searcher_search # Deliberately not guarded by _mustbeopen.


class MultiSearcher(_MustBeOpen, _pc.MultiSearcher):
    def __init__(self, searchers):
        _MustBeOpen.__init__(self, 'MultiSearcher', None)

        searcherPointers = [(s is not None and s.this) or None for s in searchers]

        self.this = _pc.new_MultiSearcher(searcherPointers)
        self.thisown = True

        # Maintain references to the underlying Searchers so they're not
        # deleted before self is.
        self._searchers = searchers

        # Used to ensure that the Query objects passed to the search method
        # survive at least as long as self.
        self._ref_queries = {}

    def close(self):
        # DO NOT call our superclass _pc.MultiSearcher's close method; it
        # closes all of the searchers underlying this MultiSearcher, which we
        # don't want (the Python garbage collector will prevent leaks).
        _MustBeOpen.close(self)
    close = _mustbeopen(close)

    docFreq = _mustbeopen(_makeDocFreq_passesPointer(_pc.MultiSearcher))
    doc = _mustbeopen(_makeDoc_passesPointer(_pc.MultiSearcher))
    maxDoc = _mustbeopen(_makeMaxDoc_passesPointer(_pc.MultiSearcher))
    search = _Searcher_search # Deliberately not guarded by _mustbeopen.

    def subSearcher(self, docNo):
        # "docNo >= self.maxDoc()" because maxDoc() is one *greater* than the
        # maximum existing document identifier.
        if docNo < 0 or docNo >= self.maxDoc():
            raise IOError('Document identifier %s is out of bounds.' % docNo)
        return _pc.MultiSearcher.subSearcher(self.this, docNo)
    subSearcher = _mustbeopen(subSearcher)


Hits = _pc.Hits


Query = _pc.Query
TermQuery = _pc.TermQuery
MultiTermQuery = _pc.MultiTermQuery
PhraseQuery = _pc.PhraseQuery


class BooleanClause(_pc.BooleanClause):
    def __init__(self, query, required, prohibited):
        # In order to make BooleanClause's memory management scheme compatible
        # with Python, we order the underlying C++ BooleanClause instance *not*
        # to delete its Query, then we maintain a reference to the Query here
        # at the Python level and rely the Python garbage collector to
        # eventually delete the underlying C++ Query instance.
        _pc.BooleanClause.__init__(self, query, False, required, prohibited)

        self._ref_query = query

class BooleanQuery(_pc.BooleanQuery):
    def __del__(self):
        _pc.BooleanQuery.__del__(self)
        # The C++ BooleanClause object that underlay this BooleanQuery is
        # now deleted.  They each held a reference to a Query object, and in
        # the BooleanQuery.add method, we forced the Python proxies of those
        # Query objects to disown their C++ companions.  Now, we restore
        # ownership of the C++ Query objects to their Python proxies, allowing
        # the proxies to delete them whenever appropriate.
        if hasattr(self, '_ref_queriesHeldByBooleanClauses'):
            for q in self._ref_queriesHeldByBooleanClauses:
                q.thisown = True

    def add(self, booleanClause):
        # The add(query, deleteQuery, required, prohibited) variant of the
        # add method is deliberately *not* exposed to Python, in order to
        # simplify reference management of C++ Query objects held within
        # BooleanClauses.
        _pc.BooleanQuery.add(self, booleanClause)

        if not hasattr(booleanClause, '_ref_booleanQueries'):
            booleanClause._ref_booleanQueries = []
        if self not in booleanClause._ref_booleanQueries:
            booleanClause._ref_booleanQueries.append(self)

        if not hasattr(self, '_ref_queriesHeldByBooleanClauses'):
            self._ref_queriesHeldByBooleanClauses = []

        # The underlying C++ BooleanQuery (self.this) will delete its C++
        # clauses when it is deleted, so the Python BooleanClause proxy must
        # not attempt to do so.
        # However, the C++ BooleanClause must stay alive as long as the Python
        # BooleanClause proxy is alive, so by extension, this BooleanQuery,
        # which owns the C++ BooleanClause, must stay alive as long as the
        # Python BooleanClause proxy is referenced anywhere other than here.
        # Furthermore, the BooleanClause holds a reference to a Query, which
        # must not be deleted until this BooleanQuery and the BooleanClauses
        # it contains are deleted.
        booleanClause.thisown = False
        booleanClause._ref_query.thisown = False
        self._ref_queriesHeldByBooleanClauses.append(booleanClause._ref_query)


WildcardQuery = _pc.WildcardQuery
PrefixQuery = _pc.PrefixQuery
FuzzyQuery = _pc.FuzzyQuery
RangeQuery = _pc.RangeQuery


TopDocs = _pc.TopDocs

ScoreDoc = _pc.ScoreDoc


# Give the C module references to various contents of this Python module.
_c.setGlobals(sys.modules['pyclene.lucene'], Document, Field)
