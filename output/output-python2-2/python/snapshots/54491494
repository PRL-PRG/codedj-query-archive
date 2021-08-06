#!/usr/bin/env python
#
# Name: pympg123
# Desc: library for controlling mpg123 (http://www.mpg123.de)
# Date: 02/21/2002
# Vers: 0.1.0
#
# Copyright (C) 2002 Ben Wilson
#  
#
# This library is free software; you #can redistribute it and/or
# modify it under the terms of the GNU Lesser General Public
# License as published by the Free Software Foundation; either
# version 2.1 of the License, or (at your option) any later version.
#
# This library is distributed in the hope that it will be useful,
# but WITHOUT ANY WARRANTY; without even the implied warranty of
# MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the GNU
# Lesser General Public License for more details.
#
# You should have received a copy of the GNU Lesser General Public
# License along with this library; if not, write to the Free Software
# Foundation, Inc., 59 Temple Place, Suite 330, Boston, MA  02111-1307  USA
#
#
# 
#	
#	http://mpy3.thelocust.org
#	http://mpy3.sourceforge.net
#	Contact: ben@thelocust.org / thelocust@users.sourceforge.net
#
#
# init the player like so:
# 
#	foo = pympg123.player()
# 	foo.play("filename")
#	while 1:
#		if foo.state == "play":
#			foo.handle_audio()
#
#	You MUST handle the output of this thing.  Otherwise, it will eventually
#	stop taking commands.
#
#	The state that foo.state can be in are as follows:
#	play - it's playing a file
#	stop - it is stopped
#	pause - these are pretty self-explanatory, eh?
#	eof - when it finds the end of the file, it will stop automatically.
#		so, if you wanted that loop up above to play the same song
#		over and over again, you could add
#
#		elif foo.state == "eof":
#			foo.play("filename") 
#
#	
#	pympg123 also keeps track of the following timestamps/etc.
#
#	tothour, totmin, and totsec - total time in file, broken down into hours,
#			minutes and seconds
#
#	duration - 	number of seconds TOTAL in file.  this is what tothour,totmin
#			and totsec are parsed from.  so, for a 6 minute file, this would be
#			360.
#	
#			therefore, tothour:totmin:totsec in a file that is 1 hour, 32 minutes
#			and 59 seconds long would be as such: 01:32:59
#
#	hour, min, and sec - current time stamps of playing file.  these are 
#			where rxaudio is in the current file.  
#	
#	progess - number of seconds into the file that rxaudio is currently playing
#
#	percent -  	silly text string that states progress/duration + "%", so if
#			so if duration was 150, and progress was 50, percent would look like
#			"33%"	
#
#	remain - remaining time in seconds
#	hourremain - remaining number of hours
#	minremain - remaining number of minutes
#	secremain - remaining number of seconds
#
#	level =
#	layer = MPEG layer type (MP3 is mpeg layer 3)
#	bitrate = most MP3s are at 128kbps, making this 128.  Usually, it's 128, 160,192, etc
#	frequency = sampling frequency.  usually 44100
#	mode = MONO, STEREO, JOINT-STEREO, etc.



import os,sys,select,re
import string
from fcntl import fcntl, F_SETFL, F_SETFD
import time
import termios
import popen2

class Player:
	
	# states include stop, play, pause, eof
	state = ""
	
	debug_on = 0
	running = 0
	newinfo = 0

	def __init__(self, mp3_player, callback):
		self.state = "stop"

		self.initvars()
		
		self.callback = callback
		self.mp3_player = mp3_player
		
		self.sec_last = 0
		
		self.proc_start()

	def debug(self, text):
		if (self.debug_on == 1):
			print "pyMPG123: " + text + "\r"
		

	def proc_start(self):
		# shocking as it may seem, this starts the mpg123 process.  mpg123 must be in yer path,
		# but you knew that, right?
		self.debug("Starting Process")

		# opens the process -- sets the "process in" pipe as self.pin, and out as self.pout
		# note that 'boo' is specified as the filename, as of mpg123 59r, you have to specify a filename, or
		# just SOMETHING after the -R (remote) switch for it to work!
		self.process = popen2.Popen3(self.mp3_player+" -R boo",True,1)
		self.pin = self.process.tochild #self.process[0]
		self.pout = self.process.fromchild #self.process[1]
		
	def shutdown(self):
		self.debug("Shutdown")
		self.pin.write("QUIT\012")
		os.kill(self.process.pid, 9)
		self.running = 0

	def initvars(self):
		# clear all the vars
		self.duration = 0
		self.progress = 0
		self.remain = 0
		self.newinfo = 1
		
		self.percent = '0%'
		self.hour, self.min, self.sec = "00","00","00"
		self.tothour, self.totmin, self.totsec = "00","00","00"	
		self.hourremain, self.minremain, self.secremain = "00", "00", "00"
		self.frequency, self.bitrate = "",""

	def play(self, filename):
		# play a file
		self.initvars()
		self.debug("LOAD " + filename)
		self.state = "play"

		# as long as the file is present, then open the file, and start to play it
		if (len(filename) > 0):
			self.pin.write("LOAD " + filename + "\012")
			self.pin.flush()

		
	def pause(self):
		# a pause toggle
		self.debug("PAUSE")

		if (self.state != "pause"):
			self.pin.write("PAUSE\012")
			self.pin.flush()
			self.state = "pause"
		elif (self.state == "pause"):
			self.pin.write("PAUSE\012")
			self.pin.flush()
			self.state = "play"
			
	def stop(self):

                self.debug("STOP\012")

		if self.state != "stop":
			self.pin.write("STOP\012")
			self.pin.flush()
		self.state = "stop"
		self.initvars()
				
	

	def handle_audio(self):
		#originally, i had a timeout of "1" in the fourth parameter of select.select there
		#that is the timeout value for getting whatever is on the line
		#1 was a little too much, and caused a general slowdown.  0.1 is much better -- much more
		# accurate, too.  0 is a non-blocking value.  we want it to block, though -- otherwise it may not catch
		# the output immediately.

		self.debug("Starting Loop!")
		self.running = 1
			
		# while we are running
		while self.running == 1:

			if self.newinfo == 1:
				self.callback(self.state, self.remain, self.progress)
				self.newinfo = 0

			# hit the "process out" to see if there is something we need to pick up (every 0.5 seconds?)
			r,w,e=select.select([self.pout],[],[],0.1)
			# if there is
			if r:
				# get the line off the process out pipe
				x = self.pout.readline()
				self.debug(x)

				# use a little regular expression magic to get the information from the line
				
				# stream info information
				#@S 1.0 3 44100 Joint-Stereo 2 626 2 1 0 0 192 0
				#@S <a> <b> <c> <d> <e> <f> <g> <h> <i> <j> <k> <l>
				# Status message after loading a song (stream info)
				# a = mpeg type (string)
				# b = layer (int)
				# c = sampling frequency (int)
				# d = mode (string)
				# e = mode extension (int)
				# f = framesize (int)
				# g = stereo (int)
				# h = copyright (int)
				# i = error protection (int)
				# j = emphasis (int)
				# k = bitrate (int)
				# l = extension (int)
				y=re.match('@S\s(\d.\d)\s(\d)\s(\d+)\s(\S+)\s(\d+)\s(\d+)\s(\d+)\s(\d+)\s(\d+)\s(\d+)\s(\d+)\s(\d+)',x)
				if y:
					self.debug("GETTING MP3 INFO")
					self.level = str(y.group(1))
					self.layer = str(y.group(2))
					self.bitrate = str(y.group(11))
					self.frequency = str(y.group(3))
					self.mode = str(y.group(4))
					self.channels = str(y.group(5))

					self.newinfo = 1
					
				# current time information
				#@F 0 5270 0.00 137.67
				#@F <a> <b> <c> <d>
				#Status message during playing (frame info)
				#a = framecount (int)
				#b = frames left this song (int)
				#c = seconds (float)
				#d = seconds left (float)

				y=re.match('@F\s(\d+)\s(\d+)\s(\d+.\d+)\s(\d+.\d+)',x)
				if y:
					#self.debug("FRAME INFO")
					self.state = "play"
					
					self.remain = int(float(y.group(4)))
					self.progress = int(float(y.group(3)))

					if self.sec_last != self.progress:
						self.newinfo = 1
						self.sec_last = self.progress

					self.remain_for_eof = float(y.group(4))
					if self.remain_for_eof <= 0.05:
						self.state = "eof"
						self.newinfo = 1

				# notify us of the end of a song.
				y=re.match('@P\s0',x)
				if y:
					if self.state != "stop":
						self.state = "eof"
						self.newinfo = 1

				# notify us of the end of a song.
				y=re.match('@E',x)
				if y:
					self.debug("error" + x)
					self.shutdown()


