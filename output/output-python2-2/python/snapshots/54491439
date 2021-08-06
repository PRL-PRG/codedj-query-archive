'''
player.py (play either ogg or mp3 files)

based on ogg123.py By Andrew Chatham Based on ogg123.c by Keneth Arnold.
also based on mpg123.py from the pymad module (no attribution in those sources)
'''

import os, time, string, sys

try:
	import ogg.vorbis
except:
	print 'No OGG support!'

try:
	import mad
except:
	print 'No MP3 support!'


class Player:
	state = 'stop'
	seek_val = 0

	def play(self):
		#TODO: use mime_type? to determine file type?
		#print self.name
		if os.path.isfile(self.name):
			(root, ext) = os.path.splitext(self.name)
			if ext == '.ogg':
				vf = ogg.vorbis.VorbisFile(self.name)
				#self.info_ogg(vf)
				self.start_ogg(vf)
			elif ext == '.mp3':
				mf = mad.MadFile(self.name)
				#self.info_mad(mf)
				self.start_mad(mf)
			else:
				raise ValueError, "Unrecognized file ext."
		else:
			raise ValueError, "Play takes a filename."

	def start_ogg(self, vf):
		self.state = 'play'

		total_time = int(vf.time_total(0))
		remain = total_time
		elapse = 0
		last_elapse = 0

		while self.state == 'play' or self.state == 'pause':
			if self.state == 'pause':
				time.sleep(1)
			elif self.seek_val:
				vf.time_seek(float(total_time * self.seek_val))
				self.seek_val = 0
			else:
				(buff, bytes, bit) = vf.read(4096)
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

	def start_mad(self, mf):
		self.state = 'play'

		total_time = mf.total_time()/1000
		remain = total_time
		elapse = 0
		last_elapse = 0

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

	def stop(self):
		self.state = 'stop'

	def pause(self):
		if self.state == 'play':
			self.state = 'pause'
		elif self.state == 'pause':
			self.state = 'play'

	def seek(self, percent):
		self.seek_val = percent

	def info_ogg(self, vf):
		print vf.comment().as_dict()

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


class AOPlayer(Player):
	'''A player which uses the ao module.'''
	def __init__(self, name, callback, id=None):
		import ao
		if id is None:
			id = ao.driver_id('esd') #also can be 'oss', 'alsa', 'alsa09', etc.
		self.dev = ao.AudioDevice(id)
		self.name = name
		self.callback = callback

	def write(self, buff, bytes):
		self.dev.play(buff, bytes)

class LADPlayer(Player):
	'''A player which uses the linuxaudiodev module.'''
	def __init__(self, name):
		import linuxaudiodev
		self.lad = linuxaudiodev
		self.dev = linuxaudiodev.open('w')
		self.dev.setparameters(44100, 16, 2, linuxaudiodev.AFMT_S16_NE)
		self.name = name

	def write(self, buff, bytes):
		while self.dev.obuffree() < bytes:
			time.sleep(0.2)
		self.dev.write(buff[:bytes])



