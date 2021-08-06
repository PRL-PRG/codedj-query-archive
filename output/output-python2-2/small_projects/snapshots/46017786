"""
	MSN Mago
		Copyright (c) 2007-2008 Krzysztof Olczyk (olczyk.krzysztof at gmail dot com)

	This file is part of MSN Mago.

	MSN Mago is free software: you can redistribute it and/or modify
	it under the terms of the GNU General Public License as published by
	the Free Software Foundation, either version 3 of the License, or
	(at your option) any later version.

	MSN Mago is distributed in the hope that it will be useful,
	but WITHOUT ANY WARRANTY; without even the implied warranty of
	MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
	GNU General Public License for more details.

	You should have received a copy of the GNU General Public License
	along with MSN Mago.  If not, see <http://www.gnu.org/licenses/>.


	utils.py - utility functions used in MSN Mago
"""

import os
import subprocess
import errno
import time
import sys
import re


try:
	import gtk.gdk
	import pygtk
	
	pygtk.require20()

	def doss():
		""" captures screenshot and returns name of JPEG file containing it """
		w = gtk.gdk.get_default_root_window()
		sz = w.get_size()
		pb = gtk.gdk.Pixbuf(gtk.gdk.COLORSPACE_RGB,False,8,sz[0],sz[1])
		pb = pb.get_from_drawable(w,w.get_colormap(),0,0,0,0,sz[0],sz[1])
		if (pb != None):
			fn = os.tempnam()
			pb.save(fn, "jpeg")
			return fn
		else:
			return None
except:
	def doss():
		return None

def fixnls(s):
	""" Converts any LINE-END convention to unix-like single \n """
	return s.replace("\r\n", "\n").replace("\r", "\n")
	
def towin32NL(s):
	""" Converts any LINE-END convention to windows-like \r\n """
	return re.sub("\r\n?|\r?\n", "\r\n", s)

def isplit(s, spliton, notinside):
	""" splits string inteligently - splits on chars in spliton,
		but not in a block delimited by pairs of any chars in notinside"""
	inside = 0
	acc = ""
	for c in s:
		if c in notinside:
			if inside == c:
				inside = 0
			elif inside == 0:
				inside = c
		if c in spliton and not inside:
			yield acc
			acc = ""
		else:
			acc += c
	if acc.strip() != "":
		yield acc
	return

def buildpath(dirs):
	""" builds path of its elements in iterable dirs """
	sep = { 1 : "\\", 0 : "/" }[os.name == "nt"]
	return reduce(lambda s1, s2: s1 + sep + s2, dirs)

# --- Popen class comes from http://aspn.activestate.com/ASPN/Cookbook/Python/Recipe/440554
PIPE = subprocess.PIPE
STDOUT = subprocess.STDOUT

if subprocess.mswindows:
	from win32file import ReadFile, WriteFile
	from win32pipe import PeekNamedPipe
	import msvcrt
else:
	import select
	import fcntl

class Popen(subprocess.Popen):
	def recv(self, maxsize=None):
		return self._recv('stdout', maxsize)

	def recv_err(self, maxsize=None):
		return self._recv('stderr', maxsize)

	def send_recv(self, input='', maxsize=None):
		return self.send(input), self.recv(maxsize), self.recv_err(maxsize)

	def get_conn_maxsize(self, which, maxsize):
		if maxsize is None:
			maxsize = 1024
		elif maxsize < 1:
			maxsize = 1
		return getattr(self, which), maxsize

	def _close(self, which):
		getattr(self, which).close()
		setattr(self, which, None)

	if subprocess.mswindows:
		def send(self, input):
			if not self.stdin:
				return None

			try:
				x = msvcrt.get_osfhandle(self.stdin.fileno())
				(errCode, written) = WriteFile(x, input)
			except ValueError:
				return self._close('stdin')
			except (subprocess.pywintypes.error, Exception), why:
				if why[0] in (109, errno.ESHUTDOWN):
					return self._close('stdin')
				raise

			return written

		def _recv(self, which, maxsize):
			conn, maxsize = self.get_conn_maxsize(which, maxsize)
			if conn is None:
				return None

			try:
				x = msvcrt.get_osfhandle(conn.fileno())
				(read, nAvail, nMessage) = PeekNamedPipe(x, 0)
				if maxsize < nAvail:
					nAvail = maxsize
				if nAvail > 0:
					(errCode, read) = ReadFile(x, nAvail, None)
			except ValueError:
				return self._close(which)
			except (subprocess.pywintypes.error, Exception), why:
				if why[0] in (109, errno.ESHUTDOWN):
					return self._close(which)
				raise

			if self.universal_newlines:
				read = self._translate_newlines(read)
			return read

	else:
		def send(self, input):
			if not self.stdin:
				return None

			if not select.select([], [self.stdin], [], 0)[1]:
				return 0

			try:
				written = os.write(self.stdin.fileno(), input)
			except OSError, why:
				if why[0] == errno.EPIPE: #broken pipe
					return self._close('stdin')
				raise

			return written

		def _recv(self, which, maxsize):
			conn, maxsize = self.get_conn_maxsize(which, maxsize)
			if conn is None:
				return None

			flags = fcntl.fcntl(conn, fcntl.F_GETFL)
			if not conn.closed:
				fcntl.fcntl(conn, fcntl.F_SETFL, flags| os.O_NONBLOCK)

			try:
				if not select.select([conn], [], [], 0)[0]:
					return ''

				r = conn.read(maxsize)
				if not r:
					return self._close(which)

				if self.universal_newlines:
					r = self._translate_newlines(r)
				return r
			finally:
				if not conn.closed:
					fcntl.fcntl(conn, fcntl.F_SETFL, flags)

message = "Other end disconnected!"

def recv_some(p, t=.1, e=1, tr=5, stderr=0):
	if tr < 1:
		tr = 1
	x = time.time()+t
	y = []
	r = ''
	pr = p.recv
	if stderr:
		pr = p.recv_err
	while time.time() < x or r:
		r = pr()
		if r is None:
			if e:
				raise Exception(message)
			else:
				break
		elif r:
			y.append(r)
		else:
			time.sleep(max((x-time.time())/tr, 0))
	return ''.join(y)

def send_all(p, data):
	while len(data):
		sent = p.send(data)
		if sent is None:
			raise Exception(message)
		data = buffer(data, sent)
