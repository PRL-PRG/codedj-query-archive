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


class Song:

	BY_TRACK = 0
	BY_TITLE = 1
	BY_ARTIST = 2
	BY_ALBUM = 3
	BY_GENRE = 4
	sort_order = BY_ALBUM

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

	def my_cmp(self, other):
		#print self, other
		if other == None: return 0
		if Song.sort_order == Song.BY_TRACK:
			item1 = self.album+self.track
			item2 = other.album+other.track
		elif Song.sort_order == Song.BY_ARTIST:
			item1 = self.artist
			item2 = other.artist
		elif Song.sort_order == Song.BY_ALBUM:
			item1 = self.album
			item2 = other.album
		elif Song.sort_order == Song.BY_GENRE:
			item1 = self.genre
			item2 = other.genre
		else:
			pass

		if item1 < item2: return -1
		elif item1 > item2: return 1
		else: return 0



class Playlist(saving.Saveable):
	"""A class to find and process mp3 and ogg files for a music player"""

	####################################################################
	def __init__(self, callback=None):
		"""Constructor for the song list"""
		self.the_filter = {}	#artist|album|genre: [list, of, filter, info]
		self.rndm = Random(time.time()) # for shuffle

		self.callback = callback

		self.iter_curr = 0
		self.shuffle_cache_size = 10
		self.shuffle_cache = []

	def __len__(self):
		return len(self.song_list)


	####################################################################
	def shuffle(self, CacheSize=None):
		"""Randomize the iterator index (so the next song is random)"""
		if CacheSize != None:
			self.shuffle_cache_size = CacheSize

		#shuffle the list?
		num_songs = len(self.song_list)
		while True:
			n = self.rndm.randrange(0, num_songs)
			if self.shuffle_cache_size >= num_songs:
				break
			if n not in self.shuffle_cache:
				self.shuffle_cache.append(n)
				if len(self.shuffle_cache) > self.shuffle_cache_size:
					self.shuffle_cache.pop(0)
				break
		self.iter_curr = n


	####################################################################
	def set(self, index):
		self.iter_curr = index
		return self.song_list[index]


	####################################################################
	def get(self, index=None):
		if index == None:
			return self.song_list[self.iter_curr]
		else:
			return self.song_list[index]


	####################################################################
	def get_index(self):
		return self.iter_curr


	####################################################################
	def first(self):
		self.iter_curr = 0
		return self.song_list[0]


	####################################################################
	def last(self):
		self.iter_curr = len(self.song_list)-1
		return self.song_list[-1]


	####################################################################
	def __iter__(self):
		self.iter_curr = -1  #because next() bumps by one first
		return self


	####################################################################
	def next(self):
		while 1:
			try:
				self.iter_curr += 1
				the_song = self.song_list[self.iter_curr]
			except:
				self.iter_curr = len(self.song_list)-1
				raise StopIteration

			#filter the list if filter is set
			found = 0
			for n in self.the_filter.keys():
				if getattr(the_song, n) in self.the_filter[n]:
					found += 1
			if found == len(self.the_filter):
				return the_song


	####################################################################
	def prev(self):
		while 1:
			if len(self.shuffle_cache):
				self.iter_curr = self.shuffle_cache[-1]
				#print self.iter_curr, self.shuffle_cache
				self.shuffle_cache.pop()
				the_song = self.song_list[self.iter_curr]
			else:
				try:
					self.iter_curr -= 1
					if self.iter_curr < 0:
						self.iter_curr = 0
						raise StopIteration
					the_song = self.song_list[self.iter_curr]
				except:
					raise StopIteration

			#filter the list if filter is set
			found = 0
			for n in self.the_filter.keys():
				if getattr(the_song, n) in self.the_filter[n]:
					found -= 1
			if found == len(self.the_filter):
				return the_song


	####################################################################
	def save(self, f):
		"""Save the current (filtered) playlist in xml format"""
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


	####################################################################
	def load(self, filename):
		"""Read an xml file of Songs and tag info"""
		try:
			self.song_list = []
			dom1 = parse(filename)
			songs = dom1.getElementsByTagName("Song")
			index = 0
			for song in songs:
				#don't let any exceptions stop us from getting other good data
				try:	title = unquote(song.getElementsByTagName("Title")[0].childNodes[0].data)
				except: title = ''
				try:	track = unquote(song.getElementsByTagName("Track")[0].childNodes[0].data)
				except: track = ''
				try:	artist = unquote(song.getElementsByTagName("Artist")[0].childNodes[0].data)
				except: artist = ''
				try:	album = unquote(song.getElementsByTagName("Album")[0].childNodes[0].data)
				except: album = ''
				try:	genre = unquote(song.getElementsByTagName("Genre")[0].childNodes[0].data)
				except: genre = ''
				try:	filename = unquote(song.getElementsByTagName("Location")[0].childNodes[0].data)
				except: filename = ''
				try:	type = unquote(song.getElementsByTagName("Type")[0].childNodes[0].data)
				except: type = ''

				self.song_list.append(Song(filename, title, track, album, artist, genre, 0, type))
			return True
		except:
			return False


	####################################################################
	def get_tag_info(self):
		"""Update the entire song_list with the tag info from each file"""
		for song in self.song_list:
			self.get_tag_info_from_file(song)


	####################################################################
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

		return song


	def get_id3_tag_info(self, song):
		if (HAVE_ID3V2):
			try: tag_info = tag(song.filename)
			except: pass

			try: song.title = tag_info.title
			except: pass
			try: song.track = string.zfill(str(tag_info.track[0]), 2) #it is a tuple (x of y)
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
			try:
				if tag_info.has_key('TITLE'): song.title = tag_info['TITLE']
			except: pass
			try:
				if tag_info.has_key('TRACKNUMBER'): song.track = tag_info['TRACKNUMBER']
			except: pass
			try:
				if tag_info.has_key('ALBUM'): song.album = tag_info['ALBUM']
			except: pass
			try:
				if tag_info.has_key('ARTIST'): song.artist = tag_info['ARTIST']
			except: pass
			try:
				if tag_info.has_key('GENRE'): song.genre = tag_info['GENRE']
			except: pass
			try:
				song.length = 0
			except: pass


	def get_ogg_info(self, song):
		try:
			tag_info = ogg.vorbis.VorbisFile(song.filename).comment().as_dict()
		except: pass
		try:
			if tag_info.has_key('TITLE'): song.title = tag_info['TITLE'][0]
		except: pass
		try:
			if tag_info.has_key('TRACKNUMBER'): song.track = tag_info['TRACKNUMBER'][0]
		except: pass
		try:
			if tag_info.has_key('ALBUM'): song.album = tag_info['ALBUM'][0]
		except: pass
		try:
			if tag_info.has_key('ARTIST'): song.artist = tag_info['ARTIST'][0]
		except: pass
		try:
			if tag_info.has_key('GENRE'): song.genre = tag_info['GENRE'][0]
		except: pass
		song.length = None


	def get_xattr_info(self, song):
		if (HAVE_XATTR):
			try:
				song.title = xattr.getxattr(song.filename, 'user.Title')
				song.track = xattr.getxattr(song.filename, 'user.Track')
				song.album = xattr.getxattr(song.filename, 'user.Album')
				song.artist = xattr.getxattr(song.filename, 'user.Artist')
				song.genre = xattr.getxattr(song.filename, 'user.Genre')
#				song.length = xattr.getxattr(song.filename, 'user.Time')
#				print song.title, song.album, song.artist, song.genre
				return True
			except:
				return False
		return False


	####################################################################
	def get_songs(self, library, guess_re):
		"""load all songs found by iterating over library into song_list..."""
		self.iter_curr = 0 #reset cuz we don't know how many songs we're gonna load

		self.guess_re = guess_re

		self.song_list = []		#index: Song
#		self.album_list = {}	#album: True
#		self.artist_list = {}	#artist: True
#		self.genre_list = {}	#genre: True

		for library_element in library:
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

#		Song.sort_order = Song.BY_TRACK
#		self.song_list.sort(Song.my_cmp)


	####################################################################
	def add_song(self, filename):
		"""Add a file to the song_list if the mime_type is acceptable"""
		type = str(rox.mime.get_type(filename))
		if type in TYPE_LIST and os.access(filename, os.R_OK):
			song = self.guess(filename, type)
			if song != None:
				self.song_list.append(song)
#				self.album_list[song.album] = True
#				self.artist_list[song.artist] = True
#				self.genre_list[song.genre] = True
#				self.emit('pls_changed')
				self.get_tag_info_from_file(song)
				self.callback('loading', 0, 0)


	####################################################################
	def guess(self, filename, type):
		"""Guess some info about the file based on path/filename"""

		try:
			m = re.match(self.guess_re, filename)
		except:
			m = re.match('^.*/(?P<artist>.*)/(?P<album>.*)/(?P<title>.*)', filename)

		try:
			title = m.group('title')
		except:
			title = filename
		try:
			album = m.group('album')
		except:
			album = 'unknown'
		try:
			artist = m.group('artist')
		except:
			artist = 'unknown'
		try:
			track = m.group('track')
		except:
			track = ''

		(title, ext) = os.path.splitext(title)
		genre = 'unknown'
		length = None

		#Ignore hidden files
		if title[0] == '.':
			return None

		return Song(filename, title, track, album, artist, genre, length, type)


	####################################################################
	def process_pls(self, pls_file):
		"""Open and read a playlist (.pls) file."""
		pls = open(pls_file, 'r')
		if pls:
			for line in pls.xreadlines():
				filename = re.match('^File[0-9]+=(.*)', line)
				if filename:
					self.add_song(filename.group(1))


	####################################################################
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

	####################################################################
	def process_dir(self, directory):
		"""Walk a directory adding all files found"""
		# (Note: add_song filters the list by mime_type)
		def visit(self, dirname, names):
			for filename in names:
				self.add_song(dirname+'/'+filename)

		os.path.walk(directory, visit, self)


	####################################################################
	def save_to_stream(self, stream):
		self.save(stream)

	def set_uri(self, uri):
		#print uri
		pass
