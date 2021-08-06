"""
	Copyright 2005 Kenneth Hayber <ken@hayber.us>, All rights reserved.

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

try:
	import mad
	HAVE_MP3 = True
except:
	HAVE_MP3 = False
	print 'No MP3 support!'
	
try:
	from pyid3lib import *
	HAVE_ID3V2 = True
except:
	from ID3 import *
	HAVE_ID3V2 = False
	print 'No id3v2 support'
import re
import genres
ID3V2_GENRE_RE = re.compile('\((?P<genre>\d+)\)')

def to_unicode(s, fallback_encoding='iso-8859-1'):
    try:
	return s.decode('utf-8')
    except UnicodeDecodeError:
	return s.decode(fallback_encoding, 'ignore')

def get_info(song):
	if HAVE_ID3V2:
		tag_info = tag(song.filename)
	else: #ID3V1
		try:
			tag_info = ID3(song.filename)
		except:
			return
	val = None
	for key in ('title', 'track', 'album', 'artist', 'contenttype', 'genre'):
		try:
			if key == 'track':
				# it is a tuple (x of y)
				if HAVE_ID3V2:
					val = int(tag_info.track[0])
				else:
					val = tag_info.track
			elif key == 'genre':
				val = tag_info.genre
			else:
				val = to_unicode(getattr(tag_info, key))
			if key == 'contenttype':
				key = 'genre'
				# ID3v2 genres are either a string/tuple index
				# e.g. '(17)' or the actual genre string.
				x = ID3V2_GENRE_RE.match(val)
				if x:
					val = int(x.group('genre'))
			if key == 'genre' and isinstance(val, int):
				val = genres.genre_list[val]
			if val != '':
				setattr(song, key, val)
		except (AttributeError, IndexError):
			pass
	# don't trust any length specs from a tag
	song.length = 0


class MP3Decoder:
	def __init__(self, filename, buffersize):
		"""Initialize the decoder"""
		self.buffersize = buffersize
		self.filename = filename

	def open(self):
		"""Open the file and prepare for decoding"""
		self.mf = mad.MadFile(self.filename) #, self.buffersize)

	def close(self):
		"""Close the file and do any needed cleanup"""
		pass

	def length(self):
		"""Return the length of the file in seconds as an integer"""
		return self.mf.total_time()/1000

	def samplerate(self):
		"""Return the sample rate of the file in samples per second"""
		return self.mf.samplerate()

	def channels(self):
		"""Return the number of channels in the file"""
		if self.mf.mode() == mad.MODE_SINGLE_CHANNEL:
			return 1
		else:
			return 2

	def read(self):
		"""
		Read data from the file and decode to PCM data. Return a buffer
		of data and length, or (None, 0) at EOF
		"""
		buff = self.mf.read()
		if buff:
			return (buff, len(buff))
		else:
			return (None, 0)

	def tell(self):
		"""Return the current playback position in seconds"""
		return self.mf.current_time() / 1000

	def seek(self, pos):
		"""Jump to pos as a percentage of the total length of the file"""
		self.mf.seek_time(long(self.length() * pos * 1000))

	def info(self):
		"""Display some MP3 information"""
		if self.mf.layer() == mad.LAYER_I:
			print "MPEG Layer I"
		elif self.mf.layer() == mad.LAYER_II:
			print "MPEG Layer II"
		elif self.mf.layer() == mad.LAYER_III:
			print "MPEG Layer III"
		else:
			print "unexpected layer value"

		if self.mf.mode() == mad.MODE_SINGLE_CHANNEL:
			print "single channel"
		elif self.mf.mode() == mad.MODE_DUAL_CHANNEL:
			print "dual channel"
		elif self.mf.mode() == mad.MODE_JOINT_STEREO:
			print "joint (MS/intensity) stereo"
		elif self.mf.mode() == mad.MODE_STEREO:
			print "normal L/R stereo"
		else:
			print "unexpected mode value"

		if self.mf.emphasis() == mad.EMPHASIS_NONE:
			print "no emphasis"
		elif self.mf.emphasis() == mad.EMPHASIS_50_15_US:
			print "50/15us emphasis"
		elif self.mf.emphasis() == mad.EMPHASIS_CCITT_J_17:
			print "CCITT J.17 emphasis"
		else:
			print "unexpected emphasis value"

		print "bitrate %lu bps" % self.mf.bitrate()
		print "samplerate %d Hz" % self.mf.samplerate()
		millis = self.mf.total_time()
		secs = millis / 1000
		print "total time %d ms (%dm%2ds)" % (millis, secs / 60, secs % 60)
	
