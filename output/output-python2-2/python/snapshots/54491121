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
		
def get_info(song):
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
	
