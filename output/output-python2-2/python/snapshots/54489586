##############################################################################
##
## $Id: apm.py,v 1.2 2003/04/10 22:13:55 riemer Exp $
##
## Copyright (C) 2002-2003 Tilo Riemer <riemer@lincvs.org>
## All rights reserved. 
##
## Redistribution and use in source and binary forms, with or without
## modification, are permitted provided that the following conditions
## are met:
##
## 1. Redistributions of source code must retain the above copyright
##    notice, this list of conditions and the following disclaimer.
## 2. Redistributions in binary form must reproduce the above copyright
##    notice, this list of conditions and the following disclaimer in the
##    documentation and/or other materials provided with the distribution.
## 3. The name of the author may not be used to endorse or promote products
##    derived from this software without specific prior written permission. 
##
## THIS SOFTWARE IS PROVIDED BY THE AUTHOR ``AS IS'' AND ANY EXPRESS OR
## IMPLIED WARRANTIES, INCLUDING, BUT NOT LIMITED TO, THE IMPLIED WARRANTIES
## OF MERCHANTABILITY AND FITNESS FOR A PARTICULAR PURPOSE ARE DISCLAIMED.
## IN NO EVENT SHALL THE AUTHOR BE LIABLE FOR ANY DIRECT, INDIRECT,
## INCIDENTAL, SPECIAL, EXEMPLARY, OR CONSEQUENTIAL DAMAGES (INCLUDING, BUT
## NOT LIMITED TO, PROCUREMENT OF SUBSTITUTE GOODS OR SERVICES; LOSS OF USE,
## DATA, OR PROFITS; OR BUSINESS INTERRUPTION) HOWEVER CAUSED AND ON ANY
## THEORY OF LIABILITY, WHETHER IN CONTRACT, STRICT LIABILITY, OR TORT
## (INCLUDING NEGLIGENCE OR OTHERWISE) ARISING IN ANY WAY OUT OF THE USE OF
## THIS SOFTWARE, EVEN IF ADVISED OF THE POSSIBILITY OF SUCH DAMAGE.
##
###############################################################################



import sys


#enums
OFFLINE     =  0
ONLINE      =  1
CHARGING    =  2   #implies ONLINE
NOBATTERY   =  3

UNKNOWN     = -1


#exceptions

class ApmError(Exception):
	"""Base class for APM exceptions"""
	pass


class ApmNoDevice(ApmError):
	"""Apm is not configured on this host"""

	def __init__(self):
		pass

	def __str__(self):
		return "Apm is not configured on this host"


class ApmNotImplemented(ApmError):
	"""No implementation for this operating system"""

	def __init__(self):
		pass

	def __str__(self):
		return "No implementation for this operating system"


class ApmNoApmLowLevel(ApmError):
	"""Apm_lowlevel module not found"""
	
	def __init__(self):
		pass

	def __str__(self):
		return "Apm_lowlevel module not found"



#interface

class Apm:
	"""Interface class for APM"""
	
	def __init__(self):
		res = sys.platform
		if res.find("freebsd4") > -1:
			self.apm = ApmGeneric()  #we use apm_lowlevel

		elif res.find("netbsd1") > -1:
			self.apm = ApmGeneric()  #we use apm_lowlevel
		
		elif res.find("linux2") > -1:
			self.apm = ApmLinux()

		elif res.find("darwin") > -1:
			self.apm = ApmGeneric()  #defined in apm_lowlevel
			
		else:
			self.apm = None #throw exception (os unknown)
			raise ApmNotImplemented
		

	def update(self):
		"""Updates the APM state"""
		self.apm.update()
		
	
	def percent(self):
		"""Returns percentage capacity of all batteries"""
		return self.apm.percent()
		
		
	def time(self):
		"""Returns time of all batteries (in minutes)"""
		return self.apm.time()
	
	
	def charging_state(self):
		"""Returns ac state (off-/online/charging)"""
		return self.apm.charging_state()
	




#base class for os dependent apm classes

class ApmBase:
	def __init__(self):
		self.ac_line_state = OFFLINE
		
		self.life_percent = 0
		self.life_time    = 0   #0 seconds

		#initial reading of apm info
		self.update()
	
	def percent(self):
		#returns percentage capacity of all batteries
		return self.life_percent
		
		
	def time(self):
		#returns time of all batteries (in minutes)
		return self.life_time
	
	
	def charging_state(self):
		return self.ac_line_state
	
	



#implementation using apm_lowlevel.py
#there are implementations of apmLowlevel.py for NetBSD 1.6 and
#soon for FreeBSD 4 at the moment

class ApmGeneric(ApmBase):
	def __init__(self):
		try:
			import apm_lowlevel
			self.apm_lowlevel = apm_lowlevel
		except ImportError:
			raise ApmNoApmLowLevel


		self.ac_line_state = OFFLINE
		
		self.life_percent = 0
		self.life_time    = 0   #0 seconds

		#initial reading of apm info
		self.update()

	
	def update(self):
		apm_info = self.apm_lowlevel.state()

		if (apm_info[0] < 0):
			raise ApmNoDevice

		self.life_percent = apm_info[1]
		self.life_time = apm_info[2]
		self.ac_line_state = apm_info[3]




	
#implementation for Linux
#if we want support more than battery info we need a low level module implemented using C

class ApmLinux(ApmBase):
	def update(self):
		#read /proc/apm and extract needed infos

		try:
			apm_proc = open("/proc/apm")
		except IOError:
			raise ApmNoDevice
			

		line = apm_proc.readline()
		token = line.split()
		if token[3] == "0x00":
			self.ac_line_state = OFFLINE
		elif token[4] == "0x03":
			self.ac_line_state = CHARGING
		else:
			self.ac_line_state = ONLINE

		self.life_percent = int(token[6].split("%")[0])

		self.life_time = int(token[7])


		apm_proc.close()
