"""
	playlist.py
		Implement a playlist for music player apps.

	Copyright 2004 Kenneth Hayber <ken@hayber.us>
		All rights reserved.

	This program is free software; you can redistribute it and/or modify
	it under the terms of the GNU General Public License as published by
	the Free Software Foundation; either version 2 of the License.

	This program is distributed in the hope that it will be useful
	but WITHOUT ANY WARRANTY; without even the implied warranty of
	MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
	GNU General Public License for more details.

	You should have received a copy of the GNU General Public License
	along with this program; if not, write to the Free Software
	Foundation, Inc., 59 Temple Place, Suite 330, Boston, MA  02111-1307  USA
"""
from __future__ import generators

import rox, os, sys, re, time, string, gtk, gobject
from rox import saving
from urllib import quote, unquote

from random import Random
from xml.dom.minidom import parse, parseString, Document

import plugins


try:
	import xattr
	HAVE_XATTR = True
except:
	HAVE_XATTR = False
	print 'No xattr support'

def strip_padding(s):
	while len(s) > 0 and s[-1] in string.whitespace + "\0":
		s = s[:-1]
	return s


#Column indicies
COL_FILE = 0
COL_TITLE = 1
COL_TRACK = 2
COL_ALBUM = 3
COL_ARTIST = 4
COL_GENRE = 5
COL_LENGTH = 6
COL_TYPE = 7
COL_ICON = 8

FILENAME_RE = re.compile('^.*/(?P<artist>.*)/(?P<album>.*)/(?P<title>.*)')

class Song:
	def __init__(self, filename=None, title=None, track=None, album=None, artist=None,
						genre=None, length=None, type=None):
		"""Constructor for one song"""
		self.filename = filename
		self.title = title
		self.track = track
		self.album = album
		self.artist = artist
		self.genre = genre
		self.length = length
		self.type = type


class Playlist(saving.Saveable, gobject.GObject):
	"""A class to find and process mp3 and ogg files for a music player"""

	def __init__(self, CacheSize, guess_re=None):
		"""Constructor for the song list"""
		self.rndm = Random(time.time()) # for shuffle
		self.curr_index = -1
		self.shuffle_cache = []
		self.shuffle_cache_size = CacheSize
		self.library = []
		self.guess_re = guess_re

		self.filter_col = None
		self.filter_data = None

		#filename, title, track, album, artist, genre, length, type
		self.model = gtk.ListStore(str, str, int, str, str, str, int, str, str)
		self.song_list_filter = self.model.filter_new()
		self.song_list_filter.set_visible_func(self.the_filter)
		self.song_list = gtk.TreeModelSort(self.song_list_filter)
		self.song_list.set_sort_func(COL_TRACK, self.comparemethod, COL_TRACK)


	def __len__(self):
		return len(self.song_list)

	def shuffle(self):
		"""Randomize the iterator index (so the next song is random)"""
		try:
			self.shuffle_cache.append(self.get_index())
			if len(self.shuffle_cache) > self.shuffle_cache_size:
				self.shuffle_cache.pop(0)
		except:
			pass

		num_songs = len(self)
		if len(self.shuffle_cache) >= num_songs:
			self.shuffle_cache = [] #we used them all up, so reset the cache

		while True:
			n = self.rndm.randrange(0, num_songs)
			if n not in self.shuffle_cache:
				break
		self.set_index(n)

	def get_model(self):
		return self.song_list

	def get_song(self, index):
		"""Create a Song object from the data at index"""
		iter = self.song_list.get_iter((index,))
		filename = self.song_list.get_value(iter, COL_FILE)
		title = self.song_list.get_value(iter, COL_TITLE)
		track = self.song_list.get_value(iter, COL_TRACK)
		album = self.song_list.get_value(iter, COL_ALBUM)
		artist = self.song_list.get_value(iter, COL_ARTIST)
		genre = self.song_list.get_value(iter, COL_GENRE)
		length = self.song_list.get_value(iter, COL_LENGTH)
		type = self.song_list.get_value(iter, COL_TYPE)
		return Song(filename, title, track, album, artist, genre, length, type)

	def set(self, index):
		try:
			self.shuffle_cache.append(self.get_index())
			if len(self.shuffle_cache) > self.shuffle_cache_size:
				self.shuffle_cache.pop(0)
		except:
			pass
		self.set_index(index)

	def get(self, index=None):
		if index == None:
			try:
				index = self.get_index()
			except:
				index = 0
				self.set_index(index)
		return self.get_song(index)

	def delete(self, index):
		try:
			del self.song_list[index]
		except:
			rox.report_exception()

	def set_index(self, index):
		self.curr_index = index
		#iter = self.song_list.get_iter((self.curr_index,))
		#self.model.set(iter, COL_ICON, 'media-track')

	def get_index(self):
		if self.curr_index == -1:
			self.curr_index = 0
		return self.curr_index

	def first(self):
		self.set_index(0)
		return self.get_song(self.get_index())

	def last(self):
		self.set_index(len(self)-1)
		return self.get_song(self.get_index())

	def next(self):
		try:
			self.shuffle_cache.append(self.get_index())
			if len(self.shuffle_cache) > self.shuffle_cache_size:
				self.shuffle_cache.pop(0)
		except:
			pass

		try:
			self.set_index(self.get_index()+1)
			return self.get_song(self.get_index())
		except:
			self.set_index(len(self)-1)
			raise StopIteration

	def prev(self):
		try:
			self.set_index(self.shuffle_cache.pop())
			return self.get_song(self.get_index())
		except:
			raise StopIteration

	def get_previous(self):
		return len(self.shuffle_cache)

	def the_filter(self, model, iter):
		"""Implement a simple filter for the playlist"""
		if self.filter_col:
			if model.get_value(iter, self.filter_col) == self.filter_data:
				return True
			else:
				return False
		else:
			return True

	def set_filter(self, column, data):
		"""The filter function above is a callback.  This is the control interface"""
		self.filter_col = column
		self.filter_data = data
		self.song_list_filter.refilter()

	def save(self, f):
		"""Save the current (filtered?) playlist in xml format"""
		f.write("<?xml version='1.0'?>\n<SongList>\n")

		for index in range(len(self)):
			song = self.get_song(index)
			f.write("\t<Song>\n")
			f.write("\t\t<Title>%s</Title>\n" % quote(song.title))
			f.write("\t\t<Track>%s</Track>\n" % str(song.track))
			f.write("\t\t<Album>%s</Album>\n" % quote(song.album))
			f.write("\t\t<Artist>%s</Artist>\n" % quote(song.artist))
			f.write("\t\t<Genre>%s</Genre>\n" % quote(song.genre))
			f.write("\t\t<Type>%s</Type>\n" % quote(song.type))
			f.write("\t\t<Location>%s</Location>\n" % quote(song.filename))
			f.write("\t</Song>\n")
		f.write("</SongList>")
		f.close()

	def load(self, filename):
		"""Read an xml file of Songs and tag info"""
		dom1 = parse(filename)
		songs = dom1.getElementsByTagName("Song")

		for song in songs:
			while gtk.events_pending():
				gtk.main_iteration()

			try: title = unquote(song.getElementsByTagName("Title")[0].childNodes[0].data)
			except: pass
			try: track = int(unquote(song.getElementsByTagName("Track")[0].childNodes[0].data))
			except: pass
			try: artist = unquote(song.getElementsByTagName("Artist")[0].childNodes[0].data)
			except: pass
			try: album = unquote(song.getElementsByTagName("Album")[0].childNodes[0].data)
			except: pass
			try: genre = unquote(song.getElementsByTagName("Genre")[0].childNodes[0].data)
			except: pass
			try: filename = unquote(song.getElementsByTagName("Location")[0].childNodes[0].data)
			except: pass
			try: type = unquote(song.getElementsByTagName("Type")[0].childNodes[0].data)
			except: pass
			length = 0

			iter_new = self.model.append()
			self.model.set(iter_new,
						COL_FILE, filename,
						COL_TITLE, title,
						COL_TRACK, track,
						COL_ALBUM, album,
						COL_ARTIST, artist,
						COL_GENRE, genre,
						COL_LENGTH, length,
						COL_TYPE, type)
			self.callback()

	def get_tag_info(self):
		"""Update the entire song_list with the tag info from each file"""
		for index in len(self):
			song = self.get_song(index)
			self.get_tag_info_from_file(song)

	def get_tag_info_from_file(self, song):
		"""Get the tag info from specified filename"""
		song.type = str(rox.mime.get_type(song.filename))
		
		try:
			if not self.get_xattr_info(song):
				plugins.get_info(song)
		except:
			rox.info('Unsupported format: %s' % song.filename)

		try:
			song.title = song.title.encode('utf8')
		except: rox.report_exception()
		try:
			song.artist = song.artist.encode('utf8')
		except: rox.report_exception()
		try:
			song.album = song.album.encode('utf8')
		except: rox.report_exception()
		try:
			song.genre = song.genre.encode('utf8')
		except: rox.report_exception()

		song.title = strip_padding(song.title)
		song.artist = strip_padding(song.artist)
		song.album = strip_padding(song.album)
		song.genre = strip_padding(song.genre)
		song.length = 0

		return song


	def get_xattr_info(self, song):
		if (HAVE_XATTR):
			try:
				song.title = xattr.getxattr(song.filename, 'user.title')
				song.track = int(xattr.getxattr(song.filename, 'user.track'))
				song.album = xattr.getxattr(song.filename, 'user.album')
				song.artist = xattr.getxattr(song.filename, 'user.artist')
				song.genre = xattr.getxattr(song.filename, 'user.genre')
#				song.length = xattr.getxattr(song.filename, 'user.time')
#				print song.title, song.album, song.artist, song.genre
				return True
			except:
				return False
		return False

	def get_songs(self, library, callback, replace=True):
		"""load all songs found by iterating over library into song_list..."""
		if replace:
			self.curr_index = -1 #reset cuz we don't know how many songs we're gonna load

		self.callback = callback

		if replace:
			self.library = library
		else:
			self.library.extend(library)

		self.model.clear()
		for library_element in self.library:
			library_element = os.path.expanduser(library_element)
			if os.access(library_element, os.R_OK):
				#check if the element is a folder
				if os.path.isdir(library_element):
					self.process_dir(library_element)
				else:
					#check for playlist files...
					(root, ext) = os.path.splitext(library_element)
					if ext == '.pls':
						self.process_pls(library_element)
					elif ext == '.m3u':
						self.process_m3u(library_element)
					elif ext == '.xml' or ext == '.music':
						self.load(library_element)
					else:
						#assume the element is just a song...
						self.add_song(library_element)

	def add_song(self, filename):
		"""Add a file to the song_list if the mime_type is acceptable"""

		while gtk.events_pending():
			gtk.main_iteration()
			
		type = str(rox.mime.get_type(filename))
		if type in plugins.TYPE_LIST and os.access(filename, os.R_OK):
			song = self.guess(filename, type)
			if song != None:
				self.get_tag_info_from_file(song)
				if song.track == None:
					song.track = 0
				if song.length == None:
					song.length = 0

				iter_new = self.model.append(None)
				self.model.set(iter_new,
							COL_FILE, song.filename,
							COL_TITLE, song.title,
							COL_TRACK, song.track,
							COL_ALBUM, song.album,
							COL_ARTIST, song.artist,
							COL_GENRE, song.genre,
							COL_LENGTH, song.length,
							COL_TYPE, song.type)
				self.callback()

	def comparemethod(self, model, iter1, iter2, user_data):
		"""Method to sort by Track and others"""
		try:
			if user_data == COL_TRACK:
				artist1 = model.get_value(iter1, COL_ARTIST)
				artist2 = model.get_value(iter2, COL_ARTIST)
				if artist1 == artist2:
					album1 = model.get_value(iter1, COL_ALBUM)
					album2 = model.get_value(iter2, COL_ALBUM)
					if album1 == album2:
						item1 = model.get_value(iter1, COL_TRACK)
						item2 = model.get_value(iter2, COL_TRACK)
					else:
						item1 = album1
						item2 = album2
				else:
					item1 = artist1
					item2 = artist2

			if item1 < item2:
				return -1
			elif item1 > item2:
				return 1
			else:
				return 0
		except:
			return 0

	def guess(self, filename, type):
		"""Guess some info about the file based on path/filename"""
		try:	m = re.match(self.guess_re, os.path.abspath(filename))
		except:	m = DEFAULT_FILENAME_RE.match(filename)
		try:	title = m.group('title')
		except:	title = filename
		try:	album = m.group('album')
		except:	album = 'unknown'
		try:	artist = m.group('artist')
		except:	artist = 'unknown'
		try:	track = int(m.group('track'))
		except:	track = 0

		(title, ext) = os.path.splitext(title)
		genre = 'unknown'
		length = 0

		#Ignore hidden files
		if title[0] == '.':
			return None
		return Song(filename, title, track, album, artist, genre, length, type)

	def process_pls(self, pls_file):
		"""Open and read a playlist (.pls) file."""
		pls = open(pls_file, 'r')
		if pls:
			for line in pls.xreadlines():
				filename = re.match('^File[0-9]+=(.*)', line)
				if filename:
					self.add_song(filename.group(1))

	def process_m3u(self, m3u_file):
		"""Open and read a playlist (.m3u) file."""

		dir = os.path.dirname(m3u_file)
		m3u = open(m3u_file, 'r')
		if m3u:
			for line in m3u.xreadlines():
				filename = line.strip()
				if filename and not filename.startswith('#'):
					if filename[0] != '/':
						filename = os.path.join(dir,
						    filename)
					self.add_song(filename)

	def process_dir(self, directory):
		"""Walk a directory adding all files found"""
		# (Note: add_song filters the list by mime_type)
		def visit(self, dirname, names):
			names.sort()
			for filename in names:
				self.add_song(dirname+'/'+filename)

		os.path.walk(directory, visit, self)

	def save_to_stream(self, stream):
		self.save(stream)

	def set_uri(self, uri):
		#print uri
		pass
