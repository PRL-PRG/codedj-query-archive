'''
player.py (play either ogg or mp3 files)

based on ogg123.py By Andrew Chatham Based on ogg123.c by Keneth Arnold.
also based on mpg123.py from the pymad module (no attribution in those sources)
'''

import os, time, string, sys

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
	import ao
	HAVE_AO = True
except:
	HAVE_AO=False
	print 'No AO support!!'
	import linuxaudiodev


TYPE_OGG = 'application/ogg'
TYPE_MP3 = 'audio/x-mp3'
TYPE_LIST = [TYPE_OGG, TYPE_MP3]


class Player:
	state = 'stop'
	seek_val = 0

	def __init__(self, name, m_type, callback, id=None, buffersize=4096):
		if HAVE_AO:
			if id is None:
				self.id = ao.driver_id('esd') #also can be 'oss', 'alsa', 'alsa09', etc.
			else:
				self.id = id

		self.name = name
		self.m_type = m_type
		self.callback = callback
		self.buffersize = buffersize

		if (self.m_type == TYPE_OGG and not HAVE_OGG):
			raise TypeError, _('You must have OGG support to play ogg files (%s).') % self.name

		if (self.m_type == TYPE_MP3 and not HAVE_MAD):
			raise TypeError, _('You must have MAD support to play mp3 files (%s).') % self.name


	# Open and configure the audio device driver
	def open(self, rate=44100, channels=2):
		bits=16
		if HAVE_AO:
			self.dev = ao.AudioDevice(self.id, bits, rate, channels)
		else:
			self.dev = linuxaudiodev.open('w')
			self.dev.setparameters(rate, bits, channels, linuxaudiodev.AFMT_S16_NE)


	# Write data to the audio device
	def write(self, buff, bytes):
		if HAVE_AO:
			self.dev.play(buff, bytes)
		else:
			while self.dev.obuffree() < bytes:
				time.sleep(0.2)
			self.dev.write(buff[:bytes])

	# Figure out what type the file is and start playing it.
	def play(self):
		if os.path.isfile(self.name):
			if (self.m_type == TYPE_OGG and HAVE_OGG):
				vf = ogg.vorbis.VorbisFile(self.name)
				#self.info_ogg(vf)
				self.start_ogg(vf)

			elif (self.m_type == TYPE_MP3 and HAVE_MAD):
				mf = mad.MadFile(self.name, self.buffersize)
				#self.info_mad(mf)
				self.start_mad(mf)

			else:
				raise ValueError, 'Unsupported file (%s).' % self.name
		else:
			raise SyntaxError, 'Play takes a filename.'

	def stop(self):
		self.state = 'stop'
		del self.dev
		time.sleep(0.2) # just to be sure that the device has time to shutdown

	def pause(self):
		if self.state == 'play':
			self.state = 'pause'
		elif self.state == 'pause':
			self.state = 'play'

	def seek(self, percent):
		self.seek_val = percent


	#############################
	# OGG-specific stuff
	#############################
	def info_ogg(self, vf):
		print vf.comment().as_dict()

	# OGG playback loop
	def start_ogg(self, vf):
		self.state = 'play'

		total_time = int(vf.time_total(0))
		remain = total_time
		elapse = 0
		last_elapse = 0

		vi = vf.info()
		self.open(vi.rate, vi.channels)

		while self.state == 'play' or self.state == 'pause':
			if self.state == 'pause':
				time.sleep(1)
			elif self.seek_val:
				vf.time_seek(float(total_time * self.seek_val))
				self.seek_val = 0
			else:
				(buff, bytes, bit) = vf.read(self.buffersize)
				if bytes == 0:
					self.state = 'eof'
					elapse = total_time
					last_elapse = 0
					remain = 0
				else:
					elapse = int(vf.time_tell())
					remain = total_time - elapse
					self.write(buff, bytes)
			if elapse != last_elapse:
				last_elapse = elapse
				self.callback(self.state, remain, elapse)


	#############################
	# MAD/MP3-specific stuff
	#############################
	def info_mad(self, mf):
		if mf.layer() == mad.LAYER_I:
			print "MPEG Layer I"
		elif mf.layer() == mad.LAYER_II:
			print "MPEG Layer II"
		elif mf.layer() == mad.LAYER_III:
			print "MPEG Layer III"
		else:
			print "unexpected layer value"

		if mf.mode() == mad.MODE_SINGLE_CHANNEL:
			print "single channel"
		elif mf.mode() == mad.MODE_DUAL_CHANNEL:
			print "dual channel"
		elif mf.mode() == mad.MODE_JOINT_STEREO:
			print "joint (MS/intensity) stereo"
		elif mf.mode() == mad.MODE_STEREO:
			print "normal L/R stereo"
		else:
			print "unexpected mode value"

		if mf.emphasis() == mad.EMPHASIS_NONE:
			print "no emphasis"
		elif mf.emphasis() == mad.EMPHASIS_50_15_US:
			print "50/15us emphasis"
		elif mf.emphasis() == mad.EMPHASIS_CCITT_J_17:
			print "CCITT J.17 emphasis"
		else:
			print "unexpected emphasis value"

		print "bitrate %lu bps" % mf.bitrate()
		print "samplerate %d Hz" % mf.samplerate()
		millis = mf.total_time()
		secs = millis / 1000
		print "total time %d ms (%dm%2ds)" % (millis, secs / 60, secs % 60)


	# MP3 playback loop
	def start_mad(self, mf):
		self.state = 'play'

		total_time = mf.total_time()/1000
		remain = total_time
		elapse = 0
		last_elapse = 0

		if mf.mode() == mad.MODE_SINGLE_CHANNEL:
			channels = 1
		else:
			channels = 2
		self.open(mf.samplerate(), channels)

		while self.state == 'play' or self.state == 'pause':
			if self.state == 'pause':
				time.sleep(1)
			elif self.seek_val:
				mf.seek_time(long(total_time * self.seek_val * 1000))
				self.seek_val = 0
			else:
				buff = mf.read()
				if buff is None:
					self.state = 'eof'
					elapse = total_time
					last_elapse = 0
					remain = 0
				else:
					elapse = mf.current_time() / 1000
					remain = total_time - elapse
					self.write(buff, len(buff))
			if elapse != last_elapse:
				last_elapse = elapse
				self.callback(self.state, remain, elapse)
