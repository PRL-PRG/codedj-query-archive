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
	import ogg.vorbis
	HAVE_OGG = True
except:
	HAVE_OGG = False
	print 'No OGG support!'

		
def get_info(song):
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


class OGGDecoder:
	"""
	A decoder to process OGG sound files.  Utilizes pyogg and pyvorbis modules
	Called from the Player class to read and decode OGG files.
	"""
	def __init__(self, filename, buffersize):
		"""Initialize the decoder"""
		self.buffersize = buffersize
		self.filename = filename

	def open(self):
		"""Open the file and prepare for decoding"""
		self.vf = ogg.vorbis.VorbisFile(self.filename)
		self.vi = self.vf.info()

	def close(self):
		"""Close the file and do any needed cleanup"""
		pass

	def length(self):
		"""Return the length of the file in seconds as an integer"""
		return int(self.vf.time_total(0))

	def samplerate(self):
		"""Return the sample rate of the file in samples per second"""
		return self.vi.rate

	def channels(self):
		"""Return the number of channels in the file"""
		return self.vi.channels

	def read(self):
		"""
		Read data from the file and decode to PCM data. Return a buffer
		of data and length, or (None, 0) at EOF
		"""
		#recent versions don't want buffersize? (buff, bytes, bit) = self.vf.read(self.buffersize)
		(buff, bytes, bit) = self.vf.read()
		if bytes == 0:
			return (None, 0)
		return (buff, bytes)

	def tell(self):
		"""Return the current playback position in seconds"""
		return int(self.vf.time_tell())

	def seek(self, pos):
		"""Jump to pos as a percentage of the total length of the file"""
		self.vf.time_seek(float(pos * self.length()))

	def info(self):
		"""Display some information"""
		print self.vf.comment().as_dict()

