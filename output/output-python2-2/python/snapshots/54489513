##############################################################################
##
## $Id: pmu.py,v 1.2 2003/04/10 22:13:55 riemer Exp $
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

class Pmu:
	"""Interface class for PMU"""
	
	def __init__(self):
		res = sys.platform
		if res.find("linux2") > -1:
			self.pmu = PmuLinux()

		else:
			self.pmu = None #throw exception (os unknown)
			raise NotImplemented
		

	def update(self):
		"""Updates the PMU state"""
		self.pmu.update()
		
	
	def percent(self):
		"""Returns percentage capacity of all batteries"""
		return self.pmu.percent()
		
		
	def time(self):
		"""Returns time of all batteries (in minutes)"""
		return self.pmu.time()
	
	
	def charging_state(self):
		"""Returns ac state (off-/online/charging)"""
		return self.pmu.charging_state()
	




#base class for os dependent pmu classes

class PmuBase:
	def __init__(self):
		self.ac_line_state = OFFLINE
		
		self.life_percent = 0
		self.life_time    = 0   #0 seconds

		#initial reading of pmu info
		self.update()
	
	def percent(self):
		#returns percentage capacity of all batteries
		return self.life_percent
		
		
	def time(self):
		#returns time of all batteries (in minutes)
		return self.life_time
	
	
	def charging_state(self):
		return self.ac_line_state
	


	
#implementation for Linux
#if we want support more than battery info we need a low level module implemented using C

class PmuLinux(PmuBase):
	def update(self):
		#read /proc/pmu and extract needed infos

		try:
			pmu_proc = open("/proc/pmu/info")
		except IOError:
			raise EnvironmentError
		
		data = {}
		lines = pmu_proc.readlines()
		for line in lines:
			token = line.split(':')
			data[token[0].strip()] = token[1].strip()
		
		try:
			pmu_battery = open("/proc/pmu/battery_0")
		except:
			self.ac_line_state = ONLINE
			return
				 
		lines = pmu_battery.readlines()
		for line in lines:
			try:
				token = line.split(':')
				data[token[0].strip()] = token[1].strip()
			except:
				pass
					
		if data['Battery count'] != "0" and data['AC Power'] == "1" and \
			(int(data['charge']) < int(data['max_charge'])):
			self.ac_line_state = CHARGING
		elif data['Battery count'] == "0" or data['AC Power'] == "1":
			self.ac_line_state = ONLINE
		else:
			self.ac_line_state = OFFLINE

		self.life_percent = int(100 * int(data['charge']) / int(data['max_charge']))
	
		self.life_time = int(int(data['time rem.']) / 60)


		pmu_proc.close()
		pmu_battery.close()
		