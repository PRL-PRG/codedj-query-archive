# -*- coding: utf-8 -*-
# -*- tab-width: 4; use-tabs: 1 -*-
# vim:tabstop=4:noexpandtab:
"""
Wraps GnomeVFS in Deferreds
"""
from __future__ import division, absolute_import, with_statement
import gnomevfs.async
from defer import Deferred
__all__ = 'Handle',

class Handle(object):
	__slots__ = '__weakref__', '_real_handle'
	
	def __init__(self, handle):
		self._real_handle = handle
	
	def __getattr__(self, name):
		return getattr(self._realhandle, name)
	
	def cancel(self):
		"""
		Cancel an asynchronous operation and close all its callbacks.
		
		In a single-threaded application, its guaranteed that if you call this 
		before the operation finished callback has been called the callback 
		will never be called.
		
		However, in a multithreaded application, or to be more specific, if you 
		call cancel() from another thread than the thread handling the glib 
		mainloop, there is a race condition where if the operation finished 
		callback was just dispatched, you might still cancel the operation. So, 
		in this case you need to handle the fact that the operation callback 
		might still run even though another thread has cancelled the operation.
		
		One way to avoid problems from this is to mark the data structure 
		you're using as callback_data as destroyed, and then queue an idle and 
		do the actual freeing in an idle handler. The idle handler is 
		guaranteed to run after the callback has been exectuted, so by then it 
		is safe to destroy the callback_data. The callback handler must handle 
		the case where the callback_data is marked destroyed by doing nothing.
		
		This is clearly not ideal for multithreaded applications, but as good 
		as we can with the current API. Eventually we'll have to change the API 
		to make this work better.
		"""
		return self._realhandle.cancel()
	
	def close(self):
		"""
		Close handle opened with open(). When the close has completed, callback 
		will be called with the result of the operation.
		
		Callback arguments: FIXME
		"""
		rv = Deferred()
		h = self._real_handle.close(lambda handle, *pargs: rv.callback(Handle(handle), *pargs))
		rv._set_cancel_call(h.cancel)
		return rv
	
	def control(self, operation, data):
		"""
		Execute a backend dependent operation specified by the string 
		operation. This is typically used for specialized vfs backends that 
		need additional operations that gnome-vfs doesn't have. Compare it to 
		the unix call ioctl(). The format of operation_data depends on the 
		operation. Operation that are backend specific are normally namespaced 
		by their module name.
		
		When the operation is complete, callback will be called with the result 
		of the operation, operation_data.
		
		* operation : operation to execute.
		* data : data needed to execute the operation.

		Since 2.2 
		
		Callback arguments: FIXME
		"""
		rv = Deferred()
		h = self._real_handle.control(operation, data, 
			lambda handle, *pargs: rv.callback(Handle(handle), *pargs)
			)
		rv._set_cancel_call(h.cancel)
		return rv
	
	def read(self, bytes):
		"""
		Read bytes bytes from the file pointed to be handle into buffer. When 
		the operation is complete, callback will be called with the result of 
		the operation.
		
		* bytes : number of bytes to read.
	
		Callback arguments: handle, buffer, exc_type, bytes_requested
		"""
		rv = Deferred()
		h = self._real_handle.read(bytes, 
			lambda handle, *pargs: rv.callback(Handle(handle), *pargs)
			)
		rv._set_cancel_call(h.cancel)
		return rv
	
	def write(self, buffer):
		"""
		Write bytes bytes from buffer into the file pointed to be handle. When 
		the operation is complete, callback will be called with the result of 
		the operation.

		* buffer : block of memory containing data to be written.
		
		Callback arguments: handle, bytes, exc_type, bytes_requested
		"""
		rv = Deferred()
		h = self._real_handle.write(bytes, 
			lambda handle, *pargs: rv.callback(Handle(handle), *pargs)
			)
		rv._set_cancel_call(h.cancel)
		return rv

def open(uri, open_mode=None, priority=None):
	"""O|ii
	Open uri according to mode open_mode. Once the file has been successfully 
	opened, callback will be called with the GnomeVFSResult.
	
	* uri : uri to open.
	* open_mode : open mode.
	* priority : a value from PRIORITY_MIN to PRIORITY_MAX (normally should be 
		PRIORITY_DEFAULT) indicating the priority to assign to this job in 
		allocating threads from the thread pool.
	
	Callback arguments: handle, exc_info
	"""
	rv = Deferred()
	args = {}
	if open_mode is not None: args['open_mode'] = open_mode
	if priority is not None: args['priority'] = priority
	h = gnomevfs.async.open(uri, 
		lambda handle, exc: rv.callback(Handle(handle), exc), 
		**args)
	rv._set_cancel_call(h.cancel)
	return rv

def create(uri, open_mode=None, exclusive=None, perm=None, priority=None):
	"""O|iiii
	Create a file at uri according to mode open_mode, with permissions perm (in 
	the standard UNIX packed bit permissions format). When the create has been 
	completed callback will be called with the result code and data.
	
	* uri : uri to create a file at.
	* open_mode : mode to leave the file opened in after creation (or 
		OPEN_MODE_NONE to leave the file closed after creation).
	* exclusive : Whether the file should be created in "exclusive" mode: i.e. 
		if this flag is nonzero, operation will fail if a file with the same 
		name already exists.
	* perm : bitmap representing the permissions for the newly created file 
		(Unix style).
	* priority : a value from PRIORITY_MIN to PRIORITY_MAX (normally should be 
		PRIORITY_DEFAULT) indicating the priority to assign this job in 
		allocating threads from the thread pool.
	
	Callback arguments: handle, exc_info
	"""
	rv = Deferred()
	args = {}
	if open_mode is not None: args['open_mode'] = open_mode
	if exclusive is not None: args['exclusive'] = exclusive
	if perm is not None: args['perm'] = perm
	if priority is not None: args['priority'] = priority
	h = gnomevfs.async.create(uri, 
		lambda handle, *pargs: rv.callback(Handle(handle), *pargs), 
		**args)
	rv._set_cancel_call(h.cancel)
	return rv

def create_symbolic_link(uri, reference, priority=0)
	"""OO|i
	Create a symbolic link at uri pointing to uri_reference. When the operation 
	has completed callback will be called with the result of the operation and 
	data.

	* uri : location to create the link at.
	* reference : location to point uri to (can be a uri fragment, i.e. 
		relative).
	* callback : function to be called when the operation is complete.
	* priority : a value from PRIORITY_MIN to PRIORITY_MAX (normally should be 
		PRIORITY_DEFAULT) indicating the priority to assign to this job in 
		allocating threads from the thread pool.
	* data : data to pass to callback. 
	
	Callback arguments: handle, exc_info
	"""
	rv = Deferred()
	args = {}
	if open_mode is not None: args['open_mode'] = open_mode
	if exclusive is not None: args['exclusive'] = exclusive
	if perm is not None: args['perm'] = perm
	if priority is not None: args['priority'] = priority
	h = gnomevfs.async.create_symbolic_link(uri, 
		lambda handle, *pargs: rv.callback(Handle(handle), *pargs), 
		**args)
	rv._set_cancel_call(h.cancel)
	return rv

def find_directory(near_uri_list, kind, create_if_needed, find_if_needed, permissions, priority=None):
	"""Oiiii|i
	Used to return special directories such as Trash and Desktop from different 
	file systems.
	
	There is quite a complicated logic behind finding/creating a Trash 
	directory and you need to be aware of some implications: Finding the Trash 
	the first time when using the file method may be pretty expensive. A cache 
	file is used to store the location of that Trash file for next time. If 
	create_if_needed is specified without find_if_needed, you may end up 
	creating a Trash file when there already is one. Your app should start out 
	by doing a gnomevfs.find_directory with the find_if_needed to avoid this 
	and then use the create_if_needed flag to create Trash lazily when it is 
	needed for throwing away an item on a given disk.

	When the operation has completed, callback will be called with the result 
	of the operation and user_data.

	* near_uri_list : a GList of GnomeVFSURIs, find a special directory on the 
		same volume as near_uri_list.
	* kind : kind of special directory.
	* create_if_needed : if directory we are looking for does not exist, try to 
		create it.
	* find_if_needed : if we don't know where the directory is yet, look for 
		it.
	* permissions : if creating, use these permissions.
	* priority : a value from PRIORITY_MIN to PRIORITY_MAX (normally should be 
		PRIORITY_DEFAULT) indicating the priority to assign to this job in 
		allocating threads from the thread pool.
	
	Callback arguments: handle, results
	"""
	rv = Deferred()
	args = {}
	if priority is not None: args['priority'] = priority
	h = gnomevfs.async.find_directory(
		near_uri_list, kind, create_if_needed, find_if_needed, permissions, 
		lambda handle, *pargs: rv.callback(Handle(handle), *pargs), 
		**args)
	rv._set_cancel_call(h.cancel)
	return rv

def get_file_info(urilist, options=None, priority=None):
	"""O|ii
	Fetch information about the files indicated in uri_list and return the 
	information progressively to callback.

	* urilist : a GList of GnomeVFSURIs to fetch information about.
	* options : packed boolean type providing control over various details of 
		the get_file_info operation.
	* priority : a value from PRIORITY_MIN to PRIORITY_MAX (normally should be 
		PRIORITY_DEFAULT) indicating the priority to assign to this job in 
		allocating threads from the thread pool.
	
	Callback arguments: handle, results
	"""
	rv = Deferred()
	args = {}
	if options is not None: args['exclusive'] = exclusive
	if priority is not None: args['priority'] = priority
	h = gnomevfs.async.get_file_info(urilist, 
		lambda handle, *pargs: rv.callback(Handle(handle), *pargs), 
		**args)
	rv._set_cancel_call(h.cancel)
	return rv

def load_directory(uri, options=None, items_per_notification=None, priority=None):
	"""O|iIi
	Read the contents of the directory at uri, passing back GnomeVFSFileInfo 
	structs about each file in the directory to callback. 
	items_per_notification files will be processed between each call to 
	callback.

	* uri : uri of the directory to be loaded. 
	* options : packed boolean type providing control over various details of the 
		get_file_info operation.
	* items_per_notification : number of files to process in a row before calling 
		callback
	* priority : a value from PRIORITY_MIN to PRIORITY_MAX (normally should be 
		PRIORITY_DEFAULT) indicating the priority to assign to this job in 
		allocating threads from the thread pool.
	
	Callback arguments: handle, results, exc_info
	"""
	rv = Deferred()
	args = {}
	if options is not None: args['options'] = options
	if items_per_notification is not None: 
		args['items_per_notification'] = items_per_notification
	if priority is not None: args['priority'] = priority
	h = gnomevfs.async.load_directory(uri, 
		lambda handle, *pargs: rv.callback(Handle(handle), *pargs), 
		**args)
	rv._set_cancel_call(h.cancel)
	return rv

