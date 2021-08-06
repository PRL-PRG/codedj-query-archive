"""
	player.py (play various sound/music files)

	based on ogg123.py By Andrew Chatham Based on ogg123.c by Keneth Arnold.
	also based on mpg123.py from the pymad module (no attribution in those sources)

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

import os, time, string, sys, Queue, mbtypes

#attempt to import various modules. remember which were successful
#so that we can dynamically adapt to the current environment
try:
	import wave
	HAVE_WAV = True
except:
	HAVE_WAV = False
	print 'No WAV support'

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

try:
	import flac
	import flac.metadata as metadata
	import flac.decoder as fldecoder
	HAVE_FLAC = True

except:
	HAVE_FLAC = False
	print "No FLAC support!"

try:
	import ossaudiodev
	HAVE_OSS = True
except:
	print 'No OSS support!!'
	HAVE_OSS = False

try:
	import ao
	HAVE_AO = True
except:
	HAVE_AO = False

if not HAVE_AO and not HAVE_OSS:
	print 'No OSS and no AO support Falling back to linuxaudiodev which sucks!!'
	import linuxaudiodev


class Player:
	"""A class to playback sound files to an audio device."""
	state = 'stop'
	remain = 0
	elapse = 0
	last_elapse = 0
	total_time = 0
	seek_val = 0

	def __init__(self, id='esd', buffersize=4096):
		"""Initialize the Player instance"""
		self.id = id
		self.buffersize = buffersize
		self.dev = None
		self.queue = Queue.Queue(5)

	def open(self, rate=44100, channels=2):
		"""Open and configure the audio device driver"""
		bits=16
		#try 3 times to open the device, then give up.
		for x in range(3):
			try:
				if HAVE_AO:
					self.dev = ao.AudioDevice(self.id, bits, rate, channels)
				elif HAVE_OSS:
					self.dev = ossaudiodev.open('w')
					self.dev.setparameters(ossaudiodev.AFMT_S16_LE, channels, rate)
				else:
					self.dev = linuxaudiodev.open('w')
					self.dev.setparameters(rate, bits, channels, linuxaudiodev.AFMT_S16_NE)
				break
			except:
				time.sleep(1)

	def close(self):
		"""Close the current device if open and delete it"""
		if self.dev:
			if HAVE_AO:
				self.dev = None
			elif HAVE_OSS:
				self.dev.close()
			else:
				self.dev = None


	def write(self, buff, bytes):
		"""Write data to the audio device"""
		if HAVE_AO:
			self.dev.play(buff, bytes)
		elif HAVE_OSS:
			self.dev.write(buff)
		else:
			while self.dev.obuffree() < bytes:
				time.sleep(0.2)
			self.dev.write(buff[:bytes])

	def play(self, name, type):
		"""Push the song info on the queue"""
		if not os.path.isfile(name):
			raise SyntaxError, _("File not found or not accessible (%s).") % name
		if (type == mbtypes.TYPE_OGG and not HAVE_OGG):
			raise TypeError, _('You must have OGG support to play ogg files (%s).') % name
		if (type == mbtypes.TYPE_MP3 and not HAVE_MAD):
			raise TypeError, _('You must have MAD support to play mp3 files (%s).') % name
		if (type == mbtypes.TYPE_FLAC and not HAVE_FLAC):
			raise TypeError, _('You must have FLAC support to play flac files (%s).') % name
		self.queue.put((name, type))

	def run(self):
		"""Check the filename and type, create a decoder and start playing"""
		while True:
			(name, type) = self.queue.get()
			if os.path.isfile(name):
				try:
					if (type == mbtypes.TYPE_OGG and HAVE_OGG):
						#for some reason with ossaudiodev a buffer greater
						#than 512 bytes causes problems with ogg, smaller seems better
						# but I'm afraid performance will suck.
						if not HAVE_AO and HAVE_OSS:
							self.buffersize = 256
						self.decoder = OGGDecoder(name, self.buffersize)
	
					elif (type == mbtypes.TYPE_MP3 and HAVE_MAD):
						self.decoder = MP3Decoder(name, self.buffersize)
	
					elif (type == mbtypes.TYPE_FLAC and HAVE_FLAC):
						self.decoder = FLACDecoder(name, self.buffersize)
	
					elif (type == mbtypes.TYPE_WAV):
						self.decoder = WAVDecoder(name, self.buffersize)
	
					else:
						raise ValueError, 'Unsupported file (%s).' % name
				except:
					rox.report_exception()
			else:
				raise SyntaxError, 'play takes a filename.'

			self.start()

	def start(self):
		"""Start playing a file calling the current decoder to get file info and data"""
		self.state = 'play'

		self.decoder.open()
		self.total_time = self.decoder.length()
		self.remain = self.total_time
		self.elapse = 0
		last_elapse = 0

		try:
			self.open(self.decoder.samplerate(), self.decoder.channels())

			while self.state == 'play' or self.state == 'pause':
				if self.state == 'pause':
					time.sleep(1)
				elif self.seek_val:
					self.decoder.seek(self.seek_val)
					self.seek_val = 0
				else:
					(buff, bytes) = self.decoder.read()
					if buff is None:
						self.state = 'eof'
						self.elapse = self.total_time
						last_elapse = 0
						remain = 0
					else:
						self.elapse = self.decoder.tell()
						self.remain = max(0, self.total_time - self.elapse)
						self.write(buff, bytes)
				if self.elapse != last_elapse or self.state == 'pause':
					last_elapse = self.elapse
		except:
			self.state = 'stop'
		self.decoder.close()
		self.close()

	def stop(self):
		"""Set a flag telling the current play-loop to exit and close the device"""
		self.state = 'stop'
		while True:
			try: self.queue.get_nowait()
			except Queue.Empty: break

	def pause(self):
		"""Pause playback (works as a toggle between play and pause)"""
		if self.state == 'play':
			self.state = 'pause'
		elif self.state == 'pause':
			self.state = 'play'

	def seek(self, percent):
		"""Jump to a specific point in the song by percent"""
		self.seek_val = percent

	def set_volume(self, volume, device=None):
		"""Set the PCM volume to a new value"""
		vol = int(volume)
		if HAVE_OSS:
			try:
				mixer = ossaudiodev.openmixer(device)
				if mixer != None:
					mixer.set(ossaudiodev.SOUND_MIXER_PCM, (vol, vol))
			except:
				print >>sys.stderr, "Failed to open mixer device %s" % device
		else:
			pass

	def get_volume(self, device=None):
		"""Return the current volume setting"""
		if HAVE_OSS:
			try:
				mixer = ossaudiodev.openmixer(device)
				if mixer != None:
					vol = mixer.get(ossaudiodev.SOUND_MIXER_PCM)
					return float(max(vol[0], vol[1]))
			except:
				print >>sys.stderr, "Failed to open mixer device %s" % device
				return 0
		else:
			return 0


#############################
# MAD/MP3-specific stuff
#############################
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


#############################
# OGG-specific stuff
#############################
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


#############################
# FLAC-specific stuff
#############################
class FLACDecoder:
	"""
	A decoder to process FLAC sound files.  Utilizes the standard pyflac module
	Called from the Player class to read and decode FLAC files.
	"""
	def __init__(self, filename, buffersize):
		"""Initialize the decoder"""
		self.buffersize = buffersize
		self.filename = filename
		self.last_decode = 0

		chain = metadata.Chain()
		chain.read(filename)
		it = metadata.Iterator()
		it.init(chain)
		while 1:
			block = it.get_block()
			if block.type == metadata.STREAMINFO:
				streaminfo = block.data.stream_info
				self.total_samples = streaminfo.total_samples
				self.sample_rate = streaminfo.sample_rate
				self.nchannels = streaminfo.channels

			if not it.next():
				break

	def open(self):
		"""Open the file and prepare for decoding"""

		def metadata_callback(dec, block):	pass
		def error_callback(dec, status):	pass
		def write_callback(dec, buff, size):
			if self.buff == '':
				self.buff = buff[:-1]
			else:
				self.buff += buff[:-1]
			return fldecoder.FLAC__FILE_DECODER_OK

		# create a new file decoder
		mydec = fldecoder.FileDecoder()

		# set some properties
		mydec.set_md5_checking(False);
		mydec.set_filename(self.filename)
		mydec.set_metadata_respond_all()

		# set the callbacks
		mydec.set_write_callback(write_callback)
		mydec.set_error_callback(error_callback)
		mydec.set_metadata_callback(metadata_callback)

		# initialise, process metadata
		mydec.init()
		mydec.process_until_end_of_metadata()

		self.mydec = mydec

	def close(self):
		"""Close the file and do any needed cleanup"""
		self.mydec.finish()

	def length(self):
		"""Return the length of the file in seconds as an integer"""
		return self.total_samples / self.sample_rate

	def samplerate(self):
		"""Return the sample rate of the file in samples per second"""
		return self.sample_rate

	def channels(self):
		"""Return the number of channels in the file"""
		return self.nchannels

	def read(self):
		"""
		Read data from the file and decode to PCM data. Return a buffer
		of data and length, or (None, 0) at EOF
		"""
		self.buff = ''
		self.mydec.process_single()
		bytes = len(self.buff)
		if bytes:
			return (self.buff, bytes)
		else:
			return (None, 0)

	def tell(self):
		"""Return the current playback position in seconds"""
		curr_decode = self.mydec.get_decode_position()
		return int(curr_decode / (self.sample_rate * self.channels()))

	def seek(self, pos):
		"""Jump to pos as a percentage of the total length of the file"""
		pass #TODO

	def info(self):
		"""Display some FLAC information"""
		# create a chain
		chain = metadata.Chain()
		chain.read(self.filename)

		# get iterator, initialise
		it = metadata.Iterator()
		it.init(chain)

		cur_block = 0;
		while 1:
			block = it.get_block()
			# print some common fields
			print 'METADATA BLOCK #%d' % cur_block
			print '  type: %d (%s)' % (block.type, metadata.TypeString(block.type))
			print '  is_last: %d' % block.is_last
			print '  length: %d' % block.length
			cur_block += 1

			if block.type == metadata.STREAMINFO:
				# print STREAMINFO fields
				streaminfo = block.data.stream_info
				print '  minimum blocksize: %d' % streaminfo.min_blocksize
				print '  maximum blocksize: %d' % streaminfo.max_blocksize
				print '  minimum framesize: %d' % streaminfo.min_framesize
				print '  maximum framesize: %d' % streaminfo.max_framesize
				print '  sample_rate: %d' % streaminfo.sample_rate
				print '  channels: %d' % streaminfo.channels
				print '  bits-per-sample: %d' % streaminfo.bits_per_sample
				print '  total samples: %d' % streaminfo.total_samples
				#print '  md5sum: %s' % streaminfo.md5sum

			elif block.type == metadata.SEEKTABLE:
				# print SEEKTABLE fields
				seektable = block.data.seek_table
				print '  seek points: %d' % seektable.num_points
				for i in range(seektable.num_points):
					pt = seektable.points[i]
					print '    point %d: sample_number=%d, stream_offset=%d, frame_samples=%d' % (i, pt.sample_number, pt.stream_offset, pt.frame_samples)

			elif block.type == metadata.CUESHEET:
				# print CUESHEET
				cuesheet = block.data.cue_sheet
				print '  media catalog number: %s' % cuesheet.media_catalog_number
				print '  lead-in: %d' % cuesheet.lead_in
				print '  is CD: %d' % cuesheet.is_cd
				print '  number of tracks: %d' % cuesheet.num_tracks
				for i in range(cuesheet.num_tracks):
					tr = cuesheet.tracks[i]
					print '    track[%d]' % i
					print '      offset: %d' % tr.offset
					print '      number: %d' % ord(tr.number)
					print '      ISRC: %s' % tr.isrc
					if tr.type == 0:
						print '      type: AUDIO'
					else:
						print '      type: NON-AUDIO'
					if tr.pre_emphasis == 1:
						print '      pre-emphasis: true'
					else:
						print '      pre-emphasis: false'
					print '      number of index points: %d' % ord(tr.num_indices)
					for j in range(ord(tr.num_indices)):
						print '        index[%d]' % j
						print '          offset: %d' % tr.indices[j].offset
						print '          number: %d' % ord(tr.indices[j].number)

			elif block.type == metadata.VORBIS_COMMENT:
				# print vorbis tags
				comment = block.data.vorbis_comment
				print '  vendor string: %s' % comment.vendor_string
				print '  comments: %d' % comment.num_comments
				for i in range(comment.num_comments):
					print '    comment[%d]: %s' % (i, comment.comments[i])

			if not it.next():
				break


#############################
# WAV-specific stuff
#############################
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
