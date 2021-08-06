# -*- coding: utf-8 -*-
# -*- tab-width: 4; use-tabs: 1 -*-
# vim:tabstop=4:noexpandtab:
"""
The Nexuiz virtual file system.

Implements the standard 
"""
from __future__ import absolute_import, with_statement
import os, zipfile, itertools, warnings, posixpath
from zipfile import ZipFile
__all__ = 'NexFS',

class _StatResult(object):
	__slots__ = ('__weakref__', 'st_mode', 'st_ino', 'st_dev', 'st_nlink', 
		'st_uid', 'st_gid', 'st_size', 'st_atime', 'st_mtime', 'st_ctime', 
		'st_blksize', 'st_rdev', 'st_flags', 'st_gen', 'st_birthtime', 
		'st_rsize', 'st_creator', 'st_type', 'st_ftype', 'st_attrs', 
		'st_obtype')
	
	__indexedattrs = ('st_mode', 'st_ino', 'st_dev', 'st_nlink', 'st_uid', 
		'st_gid', 'st_size', 'st_atime', 'st_mtime', 'st_ctime')
	
	def __init__(self, stat=None):
		for a in self.__indexedattrs:
			setattr(self, a, 0)
		if stat is None: return
		for p in dir(stat):
			if p.startswith('st_'):
				try:
					setattr(self, p, getattr(stat, p))
				except AttributeError:
					print >> sys.stderr, "_StatResult: Oops, don't actually have attribute %s" % p
	
	# Tupple stuffs
	def __len__(self):
		return len(self.__indexedattrs)
	def __getitem__(self, index):
		attr = self.__indexedattrs[index]
		if isinstance(attr, basestring):
			return getattr(self, attr)
		else:
			return tuple(getattr(self, a) for a in attr)

class NexFS(object):
	"""
	Implements the standard filesystem functions in os and os.path. If a 
	function isn't defined here, use posixpath.
	
	Parameters:
	* The root is represented by None
	* File and directory names are case-insensitive (but case storing)
	* No metadata is stored about directories
	* There is no current directory
	
	Notes:
	* stat() returns a stat structure which reflects the underlying filesystem 
	  and the actual data as best as we can determine (directories are 
	  completely fabricated
	* There are no symlinks. (If symlinks exist in the underlying fs, they're 
	  resolved. If the link is broken, the file doesn't exist.)
	"""
	# Only load .pk3's and use os functions otherwise? No, case-handling then becomes really weird.
	#TODO: Add functionality for tracking multiple sources.
	
	_root = {} # None until init'd
	# Each dir is (basename, contents)
	# contents == {basename.lower() : file|dir}
	# Each file is (basename, source)
	# source == string (real file) or (file.pk3, fileinpk3) (file inside pack)
	_nex_paths = []
	
	def __init__(self, *pargs):
		self._nex_paths = [os.path.expanduser('~/.nexuiz/data/data')]+[os.path.abspath(p) for p in pargs]
		self._root = None
	
	def has_loaded(self):
		"""nfs.has_loaded() -> bool
		Returns True if we have traversed the directories and everything.
		"""
		return self._root is not None
	
	def addpath(self, path):
		"""nfs.addpath(string) -> None
		Adds a path to the internal list of paths to search. Loads it if 
		data has already been loaded.
		"""
		self._nex_paths.append(path)
		if self.has_loaded():
			self._loaddirs(path)
	
	def _walktree(self, fname):
		"""
		Internal.
		Walks the tree, yielding each step. This is for _getnode(), _mkdirs(), 
		normcase(), and other functions that have to visit each directory. 
		
		Note: The root is yielded with a basename of None.
		"""
		# This is the only function that actually reads the entire structure.
		# Most only do the dirfile item and/or maybe one level farther down.
		if not self.has_loaded():
			self._loaddirs()
		fname = self.normpath(fname)
		if fname is None:
			dirs = ()
		else:
			dirs = fname.split('/')
		rv = (None, self._root)
		traversed = []
		for d in dirs:
			yield rv
			if not isinstance(rv[1], dict):
				e = OSError(20, "Not a directory")
				e.filename = '/'.join(traversed)
				raise e
			dl = d.lower()
			traversed.append(d)
			try:
				rv = rv[1][dl]
			except KeyError:
				e = OSError(2, 'No such file or directory')
				e.filename = '/'.join(traversed)
				raise e
		yield rv
	
	def _getnode(self, fname):
		for f in self._walktree(fname):
			rv = f
		return rv
	
	def _addfile(self, fname, source):
		fname = self.normpath(fname)
		if fname is None: # root
			raise IOError(21, 'Is a directory')
		dn,bn = self.split(fname)
		
		d = self._getnode(dn)
		if not isinstance(d[1], dict):
			raise IOError(21, 'Is a directory')
		d[1][bn.lower()] = (bn, source)
	
	def _adddirs(self, fname, _=None):
		"""
		Internal.
		Adds each dir in the path.
		"""
		fname = self.normpath(fname)
		if fname is None: return # It's the root
		for d, bn in itertools.izip(self._walktree(fname), fname.split('/')):
			if not isinstance(d[1], dict):
				e = OSError(20, "Not a directory")
				e.filename = fname
				raise e
			if bn.lower() not in d[1]:
				d[1][bn.lower()] = (bn, {})
	
	def _adddir(self, fname, _=None):
		fname = self.abspath(fname)
		if fname is None: return # It's the root
		dn, bn = self.split(fname)
		d = self._getnode(dn)[1]
		if not isinstance(d, dict):
			e = OSError(20, "Not a directory")
			e.filename = fname
			raise e
		if bn.lower() not in d:
			d[bn.lower()] = (bn, {})	
	
	def _procfile(self, fn, src):
		fullname = os.path.join(src, fn)
		if fn.endswith('.pk3'):
			# NOTE: .pk3's aren't allowed except in the root, but it's easier to implement this recursively.
			try:
				print "Adding packfile %r" % fn
				zf = ZipFile(fullname, 'r')
			except (zipfile.BadZipfile, IOError):
				warnings.warn("Not a package: %r" % fullname)
			else:
				try:
					pdir = self.abspath(os.path.dirname(fn))
					self._adddirs(pdir)
					for f in zf.namelist():
						if f[-1] == '/':
							self._adddirs(self.join(pdir, f[:-1]))
						else:
							self._adddirs(self.join(pdir, self.dirname(f)))
							self._addfile(self.join(pdir, f), (fullname,f))
				finally:
					zf.close()
		else:
			if os.path.isdir(fullname):
				self._adddir(fn, src)
				for f in sorted(os.listdir(fullname)):
					self._procfile(os.path.join(fn,f), src)
			else:
				if os.path.exists(fullname): # Needed for broken symlinks
					self._addfile(fn, fullname)
	
	def _loaddirs(self, dirs=[]):
		"""
		Internal.
		Builds the data structures.
		"""
		if not self.has_loaded(): 
			self._root = {}
			dirs = list(dirs)+self._nex_paths # Only do this the first time
		print '_loaddirs: %r' % dirs
		for d in dirs:
			for fn in sorted(os.listdir(d)):
				self._procfile(fn, d)
	
	def reload(self):
		"""nfs.reload() -> None
		Rescans all of the currently set data dirs.
		"""
		self._root = None # Unload all our data
		self._loaddirs()
	
	def getsource(self, file):
		n = self._getnode(file)
		if isinstance(n[1], dict):
			raise IOError(21, 'Is a directory')
		return n[1]
	
	# Actual functions
	# From __builtin__
	def open(self, fn, mode='r'):
		"""nd.open(string, [string]) -> file
		Opens the given file with the given mode, returning a file-like object. 
		
		NOTE: The file returned is read-only. Don't try using write modes.
		"""
		from cStringIO import StringIO # I like the ro behavior of StringIO
		if 'w' in mode or 'a' in mode or 'r+' in mode:
			raise ValueError("open() is read-only, got mode %r" % mode)
		f = self._getnode(fn)
		src = f[1]
		if isinstance(src, dict):
			raise IOError(21, 'Is a directory')
		
		if isinstance(src, tuple):
			return StringIO(ZipFile(src[0], 'r').read(src[1]))
		else:
			return open(src, mode)
	
	# From os
	def listdir(self, path):
		"""listdir(path) -> list_of_strings
		
		Return a list containing the names of the entries in the directory.
		
		path: path of directory to list
		
		The list is in arbitrary order.  It does not include the special
		entries '.' and '..' even if they are present in the directory.
		"""
		path = self.abspath(path)
		d = self._getnode(path)
		if not isinstance(d[1], dict):
			e = OSError(20, "Not a directory")
			e.filename = path
			raise e
		return [f[0] for f in d[1].itervalues()]
	
	def stat(self, path):
		fo = self._getnode(path)
		src = fo[1]
		if isinstance(src, dict):
			rv = _StatResult()
			# TODO: Populate a fake stat_result object
			return rv
		elif isinstance(src, tuple):
			import time
			zi = ZipFile(src[0]).getinfo(src[1])
			rv = _StatResult(os.stat(src[0])) # We use this as a template
			rv.st_mtime = time.mktime(zi.date_time)
			rv.st_size = zi.file_size
			return rv
		else:
			return os.stat(src)
	
	def walk(self, top, topdown=True, onerror=None):
		"""Directory tree generator.

		For each directory in the directory tree rooted at top (including top
		itself, but excluding '.' and '..'), yields a 3-tuple

			dirpath, dirnames, filenames

		dirpath is a string, the path to the directory.  dirnames is a list of
		the names of the subdirectories in dirpath (excluding '.' and '..').
		filenames is a list of the names of the non-directory files in dirpath.
		Note that the names in the lists are just names, with no path components.
		To get a full path (which begins with top) to a file or directory in
		dirpath, do os.path.join(dirpath, name).

		If optional arg 'topdown' is true or not specified, the triple for a
		directory is generated before the triples for any of its subdirectories
		(directories are generated top down).  If topdown is false, the triple
		for a directory is generated after the triples for all of its
		subdirectories (directories are generated bottom up).

		When topdown is true, the caller can modify the dirnames list in-place
		(e.g., via del or slice assignment), and walk will only recurse into the
		subdirectories whose names remain in dirnames; this can be used to prune
		the search, or to impose a specific order of visiting.  Modifying
		dirnames when topdown is false is ineffective, since the directories in
		dirnames have already been generated by the time dirnames itself is
		generated.

		By default errors from the os.listdir() call are ignored.  If
		optional arg 'onerror' is specified, it should be a function; it
		will be called with one argument, an os.error instance.  It can
		report the error to continue with the walk, or raise the exception
		to abort the walk.  Note that the filename is available as the
		filename attribute of the exception object.

		Caution:  if you pass a relative pathname for top, don't change the
		current working directory between resumptions of walk.  walk never
		changes the current directory, and assumes that the client doesn't
		either.

		Example:

		from os.path import join, getsize
		for root, dirs, files in NexFS().walk('python/Lib/email'):
			print root, "consumes",
			print sum([getsize(join(root, name)) for name in files]),
			print "bytes in", len(files), "non-directory files"
			if 'CVS' in dirs:
				dirs.remove('CVS')  # don't visit CVS directories
		"""
		# Complete and utter rip from the os module
	
		# We may not have read permission for top, in which case we can't
		# get a list of the files the directory contains.  os.path.walk
		# always suppressed the exception then, rather than blow up for a
		# minor reason when (say) a thousand readable directories are still
		# left to visit.  That logic is copied here.
		try:
			names = self.listdir(top)
		except error, err:
			if onerror is not None:
				onerror(err)
			return

		dirs, nondirs = [], []
		for name in names:
			if self.isdir(self.join(top, name)):
				dirs.append(name)
			else:
				nondirs.append(name)

		if topdown:
			yield top, dirs, nondirs
		for name in dirs:
			path = self.join(top, name)
			for x in self.walk(path, topdown, onerror):
				yield x
		if not topdown:
			yield top, dirs, nondirs
	
	# From os.path
	def abspath(self, path):
		"""
		Return an absolute path. Mostly for the normalization.
		"""
		# Remember, the path doesn't actually have to exist,
		return self.normpath(path)
	
	def basename(self, p):
		"""
		Returns the final component of a pathname
		"""
		return self.split(p)[1]
	
	def dirname(self, p):
		"""
		Returns the directory component of a pathname
		"""
		return self.split(p)[0]
	
	def exists(self, path):
		"""
		Test whether a path exists.  Returns False for broken symbolic links
		"""
		try:
			d = self._getnode(path)
		except OSError:
			return False
		else:
			return True
	
	def getatime(self, path):
		"""
		Return the last access time of a file, reported by NexFS.stat().
		"""
		return self.stat(filename).st_atime
	
	def getctime(self, path):
		"""
		Return the metadata change time of a file, reported by os.stat().
		"""
		return self.stat(filename).st_ctime
	
	def getmtime(self, path):
		"""
		Return the modification time of a file, reported by NexFS.stat().
		"""
		return self.stat(filename).st_mtime
	
	def getsize(self, path):
		"""
		Return the size of a file, reported by NexFS.stat().
		"""
		return self.stat(filename).st_size
	
	def isfile(self, path):
		"""
		Test whether a path is a regular file
		"""
		return not self.isdir(path)
	
	def isdir(self, path):
		"""
		Test whether a path is a directory
		"""
		path = self.normpath(path)
		d = self._getnode(path)
		return isinstance(d[1], dict)
	
	def join(self, a, *p):
		"""
		Join two or more pathname components, inserting '/' as needed
		"""
		if a is None or len(a) == 0:
			return posixpath.join(*p)
		else:
			return posixpath.join(a, *p)
	
	def normcase(self, s):
		"""
		Normalize case of pathname.
		"""
		rv = []
		for cd in self._walktree(s):
			if cd[0] is not None:
				rv.append(cd[0])
		if len(rv):
			return '/'.join(rv)
		else:
			return None
	
	def normpath(self, path):
		"""
		Normalize path, eliminating double slashes, etc.
		"""
		if path is None or path in ('', '.'):
			return None
		rv = posixpath.normpath(path)
		if rv[0] == '/':
			rv = rv[1:]
		bits = rv.split('/')
		while len(bits) and bits[0] in ('.', '..'):
			del bits[0]
		return '/'.join(bits)
		
	def realpath(self, filename):
		"""
		Return the canonical path of the specified filename, eliminating any 
		symbolic links encountered in the path.
		"""
		return self.normcase(filename)
		
	def samefile(self, f1, f2):
		"""
		Test whether two pathnames reference the same actual file
		"""
		return self.abspath(f1) == self.abspath(f2)
		
	def sameopenfile(self, path):
		"""
		Test whether two open file objects reference the same file
		"""
		raise NotImplementedError
		
	def samestat(self, path):
		"""
		Test whether two stat buffers reference the same file
		"""
		raise NotImplementedError
	
	def split(self, path):
		"""
		Split a pathname.  Returns tuple "(head, tail)" where "tail" is
		everything after the final slash.  Either part may be empty.
		"""
		if path is None:
			return None, None # the dir is None, the basename is None
		else:
			return posixpath.split(path)
	
	splitdrive = staticmethod(posixpath.splitdrive)
	splitext = staticmethod(posixpath.splitext)
	commonprefix = staticmethod(posixpath.commonprefix)

if __name__ == '__main__':
	nfs = NexFS()
	nfs.addpath(os.path.join('.', 'data'))
	nfs.reload()
	raise Exception("Dumping you into the terminal")
