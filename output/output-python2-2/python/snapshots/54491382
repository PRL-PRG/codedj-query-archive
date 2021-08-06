from __future__ import generators

import rox, os, re, stat, time
from urllib import quote, unquote
from ID3 import *

from random import Random

from xml.dom.minidom import parse, parseString, Document

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


TYPE_OGG = 'application/ogg'
TYPE_MP3 = 'audio/x-mp3'
TYPE_LIST = [TYPE_OGG, TYPE_MP3]


class PlaylistEntry:
	def __init__(self, filename=None, title=None, track=None, album=None, artist=None,
						genre=None, length=None, type=None, comment=None):
		self.filename = filename
		self.title = title
		self.track = track
		self.album = album
		self.artist = artist
		self.genre = genre
		self.length = length
		self.type = type
		self.comment = comment


class Playlist:
	"A class to find and process mp3 and ogg files for a music player"

	####################################################################
	def __init__(self):
		self.the_filter = {}	#artist|album|genre: [list, of, filter, info]
		self.rndm = Random(time.time()) # for shuffle

		self.iter_curr = 0
		self.shuffle_cache_size = 10
		self.shuffle_cache = []


	####################################################################
	def shuffle(self, CacheSize=None):
		"randomize the iterator index"

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
	def __iter__(self):
		self.iter_curr = 0
		return self


	####################################################################
	def first(self):
		self.iter_curr = 0
		return self.song_list[0]


	####################################################################
	def last(self):
		self.iter_curr = len(self.song_list)-1
		return self.song_list[-1]


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
	def save(self, filename):
		f = file(filename, 'w+')
		f.write("<?xml version='1.0'?>\n<SongList>\n")

		for song in self.song_list:
			f.write("\t<Song>\n")
			f.write("\t\t<Title>%s</Title>\n" % quote(song.title))
			f.write("\t\t<Track>%s</Track>\n" % quote(song.track))
			f.write("\t\t<Album>%s</Album>\n" % quote(song.album))
			f.write("\t\t<Artist>%s</Artist>\n" % quote(song.artist))
			f.write("\t\t<Genre>%s</Genre>\n" % quote(song.genre))
			f.write("\t\t<Comment>%s</Comment>\n" % quote(song.comment))
			f.write("\t\t<Type>%s</Type>\n" % quote(song.type))
			f.write("\t\t<Location>%s</Location>\n" % quote(song.filename))
			f.write("\t</Song>\n")
		f.write("</SongList>")
		f.close()


	####################################################################
	def load(self, filename):
		try:
			self.song_list = []
			dom1 = parse(filename)
			songs = dom1.getElementsByTagName("Song")
			index = 0
			for song in songs:
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
				try:	comment = unquote(song.getElementsByTagName("Comment")[0].childNodes[0].data)
				except: comment = ''
				try:	filename = unquote(song.getElementsByTagName("Location")[0].childNodes[0].data)
				except: filename = ''
				try:	type = unquote(song.getElementsByTagName("Type")[0].childNodes[0].data)
				except: type = ''

				self.song_list.append(PlaylistEntry(filename, title, track, album, artist, genre, 0, type, comment))
			return True
		except:
			return False


	####################################################################
	def get_tag_info(self):
		"Update the entire song_list with the tag info from each file"
		for song in self.song_list:
			self.get_tag_info_from_file(song)


	####################################################################
	def get_tag_info_from_file(self, song):
		"Get the tag info from specified filename"
		song.type = str(rox.mime.get_type(song.filename))

		if song.type == TYPE_MP3 and HAVE_MAD:
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
				if tag_info.has_key('COMMENT'): song.comment = tag_info['COMMENT']
			except: pass
			song.length = None

		elif song.type == TYPE_OGG and HAVE_OGG:
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
			try:
				if tag_info.has_key('COMMENT'): song.comment = tag_info['COMMENT'][0]
			except: pass
			song.length = None

		else:
			print song.filename

		return song


	####################################################################
	def get_songs(self, library, guess_re):
		"load all songs found by iterating over library into song_list..."

		library_path = library.split(":")
		self.guess_re = guess_re

		self.song_list = []		#index: PlaylistEntry
		self.album_list = {}	#album: True
		self.artist_list = {}	#artist: True
		self.genre_list = {}	#genre: True

		for library_element in library_path:
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

					else:
						#assume the element is a song file...
						self.add_song(library_element)

		self.get_tag_info()


	####################################################################
	def add_song(self, filename):
		"Add a file to the song_list if the mime_type is acceptable"
		# (Note: do this quickly, do not process tag info here)

		type = str(rox.mime.get_type(filename))
		if type in TYPE_LIST and os.access(filename, os.R_OK):
			song = self.guess(filename, type)
			if song != None:
				self.song_list.append(song)
				self.album_list[song.album] = True
				self.artist_list[song.artist] = True
				self.genre_list[song.genre] = True


	####################################################################
	def guess(self, filename, type):
		"Guess some info about the file based on path/filename"

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
		comment = ''

		#Ignore hidden files
		if title[0] == '.':
			return None

		return PlaylistEntry(filename, title, track, album, artist, genre, length, type, comment)


	####################################################################
	def process_pls(self, pls_file):
		"Open and read a playlist (.pls) file."
		pls = open(pls_file, 'r')
		if pls:
			for line in pls.xreadlines():
				filename = re.match('^File[0-9]+=(.*)', line)
				if filename:
					self.add_song(filename.group(1))


	####################################################################
	def process_m3u(self, m3u_file):
		"Open and read a playlist (.m3u) file."

		m3u = open(m3u_file, 'r')
		if m3u:
			for line in m3u.xreadlines():
				filename = re.match('(^/.*)', line)
				if filename:
					self.add_song(filename.group(1))


	####################################################################
	def process_dir(self, directory):
		"Walk a directory adding all files found"
		# (Note: add_song filters the list by mime_type)

		def visit(self, dirname, names):
			for filename in names:
				self.add_song(dirname+'/'+filename)

		os.path.walk(directory, visit, self)


