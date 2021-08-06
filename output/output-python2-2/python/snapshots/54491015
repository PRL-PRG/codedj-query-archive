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
	import wave
	HAVE_WAV = True
except:
	HAVE_WAV = False
	print 'No WAV support'


def get_info(song):
	#TODO or NOP?
	pass


class WAVDecoder:
	"""
	A decoder to process WAV sound files.  Utilizes the standard python wave module
	Called from the Player class to read and decode WAV files.
	"""
	def __init__(self, filename, buffersize):
		"""Initialize the decoder"""
		self.buffersize = buffersize
		self.filename = filename

	def open(self):
		"""Open the file and prepare for decoding"""
		self.wv = wave.open(self.filename, 'r')

	def close(self):
		"""Close the file and do any needed cleanup"""
		pass

	def length(self):
		"""Return the length of the file in seconds as an integer"""
		return self.wv.getnframes() / self.wv.getframerate()

	def samplerate(self):
		"""Return the sample rate of the file in samples per second"""
		return self.wv.getframerate()

	def channels(self):
		"""Return the number of channels in the file"""
		return self.wv.getnchannels()

	def read(self):
		"""
		Read data from the file and decode to PCM data. Return a buffer
		of data and length, or (None, 0) at EOF
		"""
		buff = self.wv.readframes(self.buffersize)
		bytes = len(buff)
		if not bytes:
			buff = None
		return (buff, bytes)

	def tell(self):
		"""Return the current playback position in seconds"""
		return self.wv.tell() / self.wv.getframerate()

	def seek(self, pos):
		"""Jump to pos as a percentage of the total length of the file"""
		self.wv.setpos(int(pos * self.wv.getnframes()))

	def info(self):
		"""Print some info about this WAV file"""
		print "no info for WAV files"

