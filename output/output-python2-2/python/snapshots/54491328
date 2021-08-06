"""
	playlist.py
		Implement a mp3/ogg playlist for music player apps.

	Copyright 2004 Kenneth Hayber <khayber@socal.rr.com>
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

import rox, os, re, stat, time, string, gobject
from rox import g, saving
from urllib import quote, unquote

import genres
from random import Random
from xml.dom.minidom import parse, parseString, Document

try:
	import xattr
	HAVE_XATTR = True
except:
	HAVE_XATTR = False
	print 'No xattr support'

try:
	from pyid3lib import *
	HAVE_ID3V2 = True
except:
	from ID3 import *
	HAVE_ID3V2 = False
	print 'No id3v2 support'

try:
	import ogg.vorbis
	HAVE_OGG = True
except:
	HAVE_OGG = False
	print 'No OGG support!'

try:
	import mad
	HAVE_MAD = True
except:
	HAVE_MAD = False
	print 'No MP3 support!'

if not HAVE_MAD and not HAVE_OGG:
	raise ImportError, 'You must have at least one of either Ogg Vorbis or\nMAD libraries installed with the python bindings.'


def strip_padding(s):
	while len(s) > 0 and s[-1] in string.whitespace + "\0":
		s = s[:-1]
	return s


TYPE_OGG = 'application/ogg'
TYPE_MP3 = 'audio/x-mp3'
TYPE_LIST = [TYPE_OGG, TYPE_MP3]

#Column indicies
COL_FILE = 0
COL_TITLE = 1
COL_TRACK = 2
COL_ALBUM = 3
COL_ARTIST = 4
COL_GENRE = 5
COL_LENGTH = 6
COL_TYPE = 7


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


class Playlist(saving.Saveable):
	"""A class to find and process mp3 and ogg files for a music player"""

	def __init__(self, CacheSize, guess_re=None):
		"""Constructor for the song list"""
		self.rndm = Random(time.time()) # for shuffle
		self.iter_curr = -1
		self.shuffle_cache = []
		self.shuffle_cache_size = CacheSize
		self.library = []
		self.guess_re = guess_re

		#filename, title, track, album, artist, genre, length, type
		self.song_list = g.ListStore(str, str, int, str, str, str, int, str)
		self.song_list.set_sort_func(COL_TRACK, self.comparemethod, COL_TRACK)

	def __len__(self):
		return len(self.song_list)

	def shuffle(self):
		"""Randomize the iterator index (so the next song is random)"""
		if self.iter_curr != -1:
			self.shuffle_cache.append(self.iter_curr)
			if len(self.shuffle_cache) > self.shuffle_cache_size:
				self.shuffle_cache.pop(0)

		#shuffle the list?
		num_songs = len(self.song_list)
		while True:
			n = self.rndm.randrange(0, num_songs)
			if self.shuffle_cache_size >= num_songs:
				break
			if n not in self.shuffle_cache:
				break
		self.iter_curr = n

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
		if self.iter_curr != -1:
			self.shuffle_cache.append(self.iter_curr)
			if len(self.shuffle_cache) > self.shuffle_cache_size:
				self.shuffle_cache.pop(0)
		self.iter_curr = index

	def get(self, index=None):
		if self.iter_curr == -1:
			self.iter_curr = 0
		if index == None:
			return self.get_song(self.iter_curr)
		else:
			return self.get_song(index)

	def get_index(self):
		if self.iter_curr == -1:
			raise 'No index set'
		return self.iter_curr

	def first(self):
		self.iter_curr = 0
		return self.get_song(0)

	def last(self):
		self.iter_curr = len(self.song_list)-1
		return self.get_song(self.iter_curr)

	def next(self):
		if self.iter_curr != -1:
			self.shuffle_cache.append(self.iter_curr)
			if len(self.shuffle_cache) > self.shuffle_cache_size:
				self.shuffle_cache.pop(0)
		try:
			self.iter_curr += 1
			return self.get_song(self.iter_curr)
		except:
			self.iter_curr = len(self.song_list)-1
			raise StopIteration

	def prev(self):
		try:
			self.iter_curr = self.shuffle_cache.pop()
			return self.get_song(self.iter_curr)
		except:
			raise StopIteration

	def get_previous(self):
		return len(self.shuffle_cache)

	def save(self, f):
		"""Save the current (filtered?) playlist in xml format"""
		f.write("<?xml version='1.0'?>\n<SongList>\n")

		for song in self:
			f.write("\t<Song>\n")
			f.write("\t\t<Title>%s</Title>\n" % quote(song.title))
			f.write("\t\t<Track>%s</Track>\n" % quote(song.track))
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
		index = 0
		for song in songs:
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

			self.album_list[album] = True
			self.artist_list[artist] = True
			self.genre_list[genre] = True

			iter_new = self.song_list.append()
			self.song_list.set(iter_new,
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
		for index in len(self.song_list):
			song = self.get_song(index)
			self.get_tag_info_from_file(song)

	def get_tag_info_from_file(self, song):
		"""Get the tag info from specified filename"""
		song.type = str(rox.mime.get_type(song.filename))

		if not self.get_xattr_info(song):
			if song.type == TYPE_MP3 and HAVE_MAD:
				#print 'using mp3 tags'
				self.get_id3_tag_info(song)
			elif song.type == TYPE_OGG and HAVE_OGG:
				#print 'using ogg info'
				self.get_ogg_info(song)
			else:
				print song.filename

		try:
			song.title = unicode(song.title,'latin-1')
			song.title = song.title.encode('utf8')
		except: pass
		try:
			song.artist = unicode(song.artist,'latin-1')
			song.artist = song.artist.encode('utf8')
		except: pass
		try:
			song.album = unicode(song.album,'latin-1')
			song.album = song.album.encode('utf8')
		except: pass
		try:
			song.genre = unicode(song.genre,'latin-1')
			song.genre = song.genre.encode('utf8')
		except: pass

		song.title = strip_padding(song.title)
		song.artist = strip_padding(song.artist)
		song.album = strip_padding(song.album)
		song.genre = strip_padding(song.genre)
		song.length = 0

		return song

	def get_id3_tag_info(self, song):
		if (HAVE_ID3V2):
			try: tag_info = tag(song.filename)
			except: pass
			try: song.title = tag_info.title
			except: pass
			try: song.track = int(tag_info.track[0]) #it is a tuple (x of y)
			except: pass
			try: song.album = tag_info.album
			except: pass
			try: song.artist = tag_info.artist
			except: pass
			try:
				#ID3v2 genres are either a string/tuple index e.g. '(17)'
				#or the actual genre string.
				x = re.match('\(([0-9]+)\)', tag_info.contenttype)
				if x:
					genre = genres.genre_list[int(x.group(1))]
				else:
					genre = tag_info.contenttype
				song.genre = genre
			except: pass
			try: song.length = tag_info.songlen
			except: pass
		else: #ID3V1
			try:
				tag_info = ID3(song.filename)
			except: pass
			if tag_info.has_key('TITLE'): song.title = tag_info['TITLE']
			if tag_info.has_key('TRACKNUMBER'): song.track = int(tag_info['TRACKNUMBER'])
			if tag_info.has_key('ALBUM'): song.album = tag_info['ALBUM']
			if tag_info.has_key('ARTIST'): song.artist = tag_info['ARTIST']
			if tag_info.has_key('GENRE'): song.genre = tag_info['GENRE']
			song.length = 0

	def get_ogg_info(self, song):
		try:
			tag_info = ogg.vorbis.VorbisFile(song.filename).comment().as_dict()
			if tag_info.has_key('TITLE'): song.title = tag_info['TITLE'][0]
			if tag_info.has_key('TRACKNUMBER'): song.track = int(tag_info['TRACKNUMBER'][0])
			if tag_info.has_key('ALBUM'): song.album = tag_info['ALBUM'][0]
			if tag_info.has_key('ARTIST'): song.artist = tag_info['ARTIST'][0]
			if tag_info.has_key('GENRE'): song.genre = tag_info['GENRE'][0]
			song.length = 0
		except:
			pass

	def get_xattr_info(self, song):
		if (HAVE_XATTR):
			try:
				song.title = xattr.getxattr(song.filename, 'user.Title')
				song.track = int(xattr.getxattr(song.filename, 'user.Track'))
				song.album = xattr.getxattr(song.filename, 'user.Album')
				song.artist = xattr.getxattr(song.filename, 'user.Artist')
				song.genre = xattr.getxattr(song.filename, 'user.Genre')
#				song.length = xattr.getxattr(song.filename, 'user.Time')
#				print song.title, song.album, song.artist, song.genre
				return True
			except:
				return False
		return False

	def get_songs(self, library, callback, replace=True):
		"""load all songs found by iterating over library into song_list..."""
		self.iter_curr = -1 #reset cuz we don't know how many songs we're gonna load

		self.callback = callback

		if replace:
			self.library = library
		else:
			self.library.extend(library)

		self.song_list.clear()
		self.album_list = {}	#album: True
		self.artist_list = {}	#artist: True
		self.genre_list = {}	#genre: True

		for library_element in self.library:
			library_element = os.path.expanduser(library_element)
			if os.access(library_element, os.R_OK):
				#check if the element is a folder
				if stat.S_ISDIR(os.stat(library_element)[stat.ST_MODE]):
					self.process_dir(library_element)
				else:
					#check for playlist files...
					(root, ext) = os.path.splitext(library_element)
					if ext == '.pls':
						self.process_pls(library_element)
					elif ext == '.m3u':
						self.process_m3u(library_element)
					elif ext == '.xml':
						self.load(library_element)
					else:
						#assume the element is just a song...
						self.add_song(library_element)
		#print self.album_list.keys()
		#print self.artist_list.keys()
		#print self.genre_list.keys()

	def add_song(self, filename):
		"""Add a file to the song_list if the mime_type is acceptable"""
		type = str(rox.mime.get_type(filename))
		if type in TYPE_LIST and os.access(filename, os.R_OK):
			song = self.guess(filename, type)
			if song != None:
				self.get_tag_info_from_file(song)
				if song.track == None:
					song.track = 0
				if song.length == None:
					song.length = 0

				self.album_list[song.album] = True
				self.artist_list[song.artist] = True
				self.genre_list[song.genre] = True

				iter_new = self.song_list.append()
				self.song_list.set(iter_new,
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
				#print item1, item2

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
		try:	m = re.match(self.guess_re, filename)
		except:	m = re.match('^.*/(?P<artist>.*)/(?P<album>.*)/(?P<title>.*)', filename)
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
				if filename:
					if filename[0] == '/':
						self.add_song(filename)
					else:
						self.add_song('/'.join((dir,
								       filename)))

	def process_dir(self, directory):
		"""Walk a directory adding all files found"""
		# (Note: add_song filters the list by mime_type)
		def visit(self, dirname, names):
			for filename in names:
				self.add_song(dirname+'/'+filename)

		os.path.walk(directory, visit, self)

	def save_to_stream(self, stream):
		self.save(stream)

	def set_uri(self, uri):
		#print uri
		pass
