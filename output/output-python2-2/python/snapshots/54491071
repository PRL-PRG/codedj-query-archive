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

import os, time, string, sys, Queue
import plugins

try:
	import alsaaudio
	HAVE_ALSA = True
except:
	print 'No ALSA support'
	HAVE_ALSA = False

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

USING_DEV = None

class Player:
	"""A class to playback sound files to an audio device."""
	state = 'stop'
	remain = 0
	elapse = 0
	last_elapse = 0
	total_time = 0
	seek_val = 0

	def __init__(self, id='esd', buffersize=4096, device='/dev/dsp',):
		"""Initialize the Player instance"""
		self.id = id
		self.buffersize = buffersize
		self.dev = None
		self.device = device
		self.queue = Queue.Queue(5)

	def open(self, rate=44100, channels=2):
		"""Open and configure the audio device driver"""
		global USING_DEV
		bits=16
		#try 3 times to open the device, then give up.
		for x in range(3):
			try:
				if HAVE_AO:
					self.dev = ao.AudioDevice(self.id, bits, rate, channels)
					USING_DEV = 'ao'
				elif HAVE_ALSA and self.id in ('alsa09',):
					self.dev = alsaaudio.PCM(cardname=self.device)
					self.dev.setchannels(channels)
					self.dev.setrate(rate)
					if sys.byteorder == 'big':
						format = alsaaudio.PCM_FORMAT_S16_BE
					else:
						format = alsaaudio.PCM_FORMAT_S16_LE
					self.dev.setformat(format)
					USING_DEV = 'alsa'
				elif HAVE_OSS:
					self.dev = ossaudiodev.open(self.device, 'w')
					if sys.byteorder == 'big':
						format = ossaudiodev.AFMT_S16_BE
					else:
						format = ossaudiodev.AFMT_S16_LE
					self.dev.setparameters(format, channels, rate)
					USING_DEV = 'oss'
				else:
					self.dev = linuxaudiodev.open('w')
					if sys.byteorder == 'big':
						format = linuxaudiodev.AFMT_S16_BE
					else:
						format = linuxaudiodev.AFMT_S16_LE
					self.dev.setparameters(rate, bits, channels, format)
					USING_DEV = 'other'
				break
			except:
				time.sleep(1)
				

	def close(self):
		"""Close the current device if open and delete it"""
		if self.dev:
			if USING_DEV in ('ao', 'alsa', 'other'):
				self.dev = None
			elif USING_DEV in ('oss',):
				self.dev.close()


	def write(self, buff, bytes):
		"""Write data to the audio device"""
		if USING_DEV in ('ao',):
			self.dev.play(buff, bytes)
		elif USING_DEV in ('oss', 'alsa'):
			self.dev.write(buff)
		else:
			while self.dev.obuffree() < bytes:
				time.sleep(0.2)
			self.dev.write(buff[:bytes])

	def play(self, name, type):
		"""Push the song info on the queue"""
		if not os.path.isfile(name):
			raise SyntaxError, "File not found or not accessible (%s)." % name
		self.queue.put((name, type))

	def run(self):
		"""Check the filename and type, create a decoder and start playing"""
		while True:
			(name, type) = self.queue.get()
			if os.path.isfile(name):
				self.decoder = plugins.get_decoder(name, type, self.buffersize)
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
		if HAVE_ALSA:
			try:
				mixer = alsaaudio.Mixer('PCM', 0, device)
				mixer.setvolume(vol)				
			except:
				print >>sys.stderr, "Failed to open mixer device %s" % device
				
		elif HAVE_OSS:
			try:
				mixer = ossaudiodev.openmixer(device)
				mixer.set(ossaudiodev.SOUND_MIXER_PCM, (vol, vol))
			except:
				print >>sys.stderr, "Failed to open mixer device %s" % device
		else:
			pass

	def get_volume(self, device=None):
		"""Return the current volume setting"""
		if HAVE_ALSA:
			try:
				mixer = alsaaudio.Mixer('PCM', 0, device)
				vol = mixer.getvolume()
				return float(max(vol[0], vol[1]))
			except:
				print >>sys.stderr, "Failed to open mixer device %s" % device
				return 0
				
		elif HAVE_OSS:
			try:
				mixer = ossaudiodev.openmixer(device)
				vol = mixer.get(ossaudiodev.SOUND_MIXER_PCM)
				return float(max(vol[0], vol[1]))
			except:
				print >>sys.stderr, "Failed to open mixer device %s" % device
				return 0
		else:
			return 0

