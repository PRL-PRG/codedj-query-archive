"""
	player.py (play either ogg or mp3 files)

	based on ogg123.py By Andrew Chatham Based on ogg123.c by Keneth Arnold.
	also based on mpg123.py from the pymad module (no attribution in those sources)

	Portions, Copyright 2004 Kenneth Hayber <khayber@socal.rr.com>
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

import os, time, string, sys, Queue

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


TYPE_OGG = 'application/ogg'
TYPE_MP3 = 'audio/x-mp3'
TYPE_LIST = [TYPE_OGG, TYPE_MP3]


class Player:
	"""
		A class to playback MP3 and OGG sound files to an audio device.
		Supports/depends on libmad, libvorbis, libogg, libao, pyao,
		linuxaudiodev and/or ossaudiodev
	"""
	state = 'stop'
	remain = 0
	elapse = 0
	last_elapse = 0
	total_time = 0
	seek_val = 0

	def __init__(self, id=None, buffersize=4096):
		"""Initialize the Player instance"""
		if HAVE_AO:
			if id is None:
				self.id = ao.driver_id('esd') #also can be 'oss', 'alsa', 'alsa09', etc.
			else:
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
		if (type == TYPE_OGG and not HAVE_OGG):
			raise TypeError, _('You must have OGG support to play ogg files (%s).') % name
		if (type == TYPE_MP3 and not HAVE_MAD):
			raise TypeError, _('You must have MAD support to play mp3 files (%s).') % name
		self.queue.put((name, type))

	def run(self):
		"""Check the filename and type and start the appropriate play-loop"""
		while True:
			(name, type) = self.queue.get()
			if os.path.isfile(name):
				if (type == TYPE_OGG and HAVE_OGG):
					vf = ogg.vorbis.VorbisFile(name)
					#self.info_ogg(vf)
					self.start_ogg(vf)

				elif (type == TYPE_MP3 and HAVE_MAD):
					mf = mad.MadFile(name, self.buffersize)
					#self.info_mad(mf)
					self.start_mad(mf)

				else:
					raise ValueError, 'Unsupported file (%s).' % name
			else:
				raise SyntaxError, 'play takes a filename.'

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

	def set_volume(self, volume):
		"""Set the PCM volume to a new value"""
		vol = int(volume)
		if HAVE_OSS:
			mixer = ossaudiodev.openmixer()
			if mixer != None:
				mixer.set(ossaudiodev.SOUND_MIXER_PCM, (vol, vol))
		else:
			pass

	def get_volume(self):
		"""Return the current volume setting"""
		if HAVE_OSS:
			mixer = ossaudiodev.openmixer()
			if mixer != None:
				vol = mixer.get(ossaudiodev.SOUND_MIXER_PCM)
				return float(max(vol[0], vol[1]))
		else:
			return 0


	#############################
	# OGG-specific stuff
	#############################
	def info_ogg(self, vf):
		"""Display some OGG information"""
		print vf.comment().as_dict()

	def start_ogg(self, vf):
		"""Open the audio device and start playing an OGG file"""
		self.state = 'play'

		self.total_time = int(vf.time_total(0))
		self.remain = self.total_time
		self.elapse = 0
		last_elapse = 0

		try:
			vi = vf.info()
			self.open(vi.rate, vi.channels)

			while self.state == 'play' or self.state == 'pause':
				if self.state == 'pause':
					time.sleep(1)
				elif self.seek_val:
					vf.time_seek(float(self.total_time * self.seek_val))
					self.seek_val = 0
				else:
					#for some reason with ossaudiodev a buffer greater
					#than 512 bytes causes problems with ogg, smaller seems better
					# but I'm afraid performance will suck.
					if not HAVE_AO and HAVE_OSS:
						self.buffersize = 256
					(buff, bytes, bit) = vf.read(self.buffersize)
					if bytes == 0:
						self.state = 'eof'
						self.elapse = self.total_time
						last_elapse = 0
						self.remain = 0
					else:
						self.elapse = int(vf.time_tell())
						self.remain = max(0, self.total_time - self.elapse)
						self.write(buff, bytes)
				if self.elapse != last_elapse or self.state == 'pause':
					last_elapse = self.elapse
		except:
			self.state = 'stop'

		self.close()


	#############################
	# MAD/MP3-specific stuff
	#############################
	def info_mad(self, mf):
		"""Display some MP3 information"""
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

	def start_mad(self, mf):
		"""Open the audio device and start playing an MP3 file"""
		self.state = 'play'

		self.total_time = mf.total_time()/1000
		self.remain = self.total_time
		self.elapse = 0
		last_elapse = 0

		if mf.mode() == mad.MODE_SINGLE_CHANNEL:
			channels = 1
		else:
			channels = 2

		try:
			self.open(mf.samplerate(), channels)

			while self.state == 'play' or self.state == 'pause':
				if self.state == 'pause':
					time.sleep(1)
				elif self.seek_val:
					mf.seek_time(long(self.total_time * self.seek_val * 1000))
					self.seek_val = 0
				else:
					buff = mf.read()
					if buff is None:
						self.state = 'eof'
						self.elapse = self.total_time
						last_elapse = 0
						remain = 0
					else:
						self.elapse = mf.current_time() / 1000
						self.remain = max(0, self.total_time - self.elapse)
						self.write(buff, len(buff))
				if self.elapse != last_elapse or self.state == 'pause':
					last_elapse = self.elapse
		except:
			self.state = 'stop'

		self.close()

