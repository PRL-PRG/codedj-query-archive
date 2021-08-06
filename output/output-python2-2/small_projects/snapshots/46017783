"""
	MSN Mago
		Copyright (c) 2007-2008 Krzysztof Olczyk (olczyk.krzysztof at gmail dot com)

	This file is part of MSN Mago.

	MSN Mago is free software: you can redistribute it and/or modify
	it under the terms of the GNU General Public License as published by
	the Free Software Foundation, either version 3 of the License, or
	(at your option) any later version.

	MSN Mago is distributed in the hope that it will be useful,
	but WITHOUT ANY WARRANTY; without even the implied warranty of
	MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
	GNU General Public License for more details.

	You should have received a copy of the GNU General Public License
	along with MSN Mago.  If not, see <http://www.gnu.org/licenses/>.


	users.py - classes representing users communicating with a bot
				and activities they are performing
"""

import os
import random

from msnlib.msnlib import *
from msnlib.ftp import MsnFTP
from utils import *
import cmds;

class BaseUser(object):
	"""
	Base class for any user present in the functionality
	of a bot
	"""
	def __init__(self, md, conf):
		self.email = ""
		self.cmd = cmds.cmd
		self.infiles = []
		self.outfiles = []
		self.allowed = 0
		self.updatemd(md)
		self.conf = conf
		self.disabled = ['enable', 'disable']
		self.enabled = []

	def updatemd(self, md):
		self.md = md

	def execcmd(self, routine):
		"Executes command sent by user"
		if self.allowed:
			if routine == "go back":
					self.sendmsg("You are again in main mode :)")
					self.cmd = cmds.cmd
			else:
				self.cmd = self.cmd(self, routine)
		else:
			self.sendmsg("Sorry, you are not authorized to control me")

	def execmacro(self, routines):
		"Executes macro"
		for routine in isplit(routines, ";", '@"'):
			BaseUser.execcmd(self, routine.strip())

	def sendmsg(self, msg):
		"Sends message to the user"
		pass

class RealUser(BaseUser):
	"""
	Represents real user - msn contact interacting with bot
	"""
	def __init__(self, md, email, conf):
		BaseUser.__init__(self, md, conf)
		self.email = email
		if email in conf.allowed_users:
			self.allowed = 1

	def sendmsg(self, msg):
		self.md.sendmsg(self.email, msg)

class ConfigUser(BaseUser):
	"""
	Represents a pseudo-user - used for execution
	of script present in config file
	"""
	def __init__(self, md, conf):
		BaseUser.__init__(self, md, conf)
		self.allowed = 1
		self.disabled = []
		self.enabled = []

	def sendmsg(self, msg):
		print msg

	def load_config_file(self, fn):
		"Loads the context of config file"
		conff = open(fn, "r")
		conf = ""
		for confline in conff:
			conf += confline
		conff.close()
		return conf

	def applyconfig(self):
		"Applies configuration from config files from different locations"
		confs = ("/etc/msnmago.conf", "/etc/msnmago/conf", "c:\msnmago.conf", "c:\windows\msnmago.conf", buildpath((os.getcwd(), "msnmago.conf")), "~/.msnmago.conf")
		for cf in confs:
			if os.path.exists(cf):
				self.sendmsg("Loading configuration from file %s..." % cf)
				self.execmacro(self.load_config_file(cf))


class FileTransfer(object):
	"""
	Represents file transfer with user
	"""

	CookieCounter = random.randint(1000, 2000)  # cookie counter
	AuthCookieCounter = random.randint(1000, 2000) # authorisation cookie counter
	Port = 6891 # standard port for file transfers
	transfers = {} # dictionary of active file transfers

	def __init__(self, md, user, filename, realname = None, filesize = None, cookie = 0, op = "send"):
		if realname == None: realname = filename
		self.md = md
		self.usr = user
		self.filename = filename
		self.realname = realname
		if filesize != None:
			self.filesz = filesize
		else:
			try:
				self.filesz = os.path.getsize(realname)
			except:
				self.filesz = 0
		self.cookie = cookie
		self.authcookie = None
		self.op = op

		if not cookie:
			self.cookie = str(FileTransfer.CookieCounter)
			FileTransfer.CookieCounter += random.randint(1, 10)

		FileTransfer.transfers[self.cookie] = self

	def invite(self):
		""" invites the user for a file transfer """
		self.md.invitefile(self.usr.email, self.filename, self.filesz, int(self.cookie), 1)

	def accept(self, offer_server):
		""" accepts received invitation """
		print "Invitated to transfer file %s " % self.filename
		if offer_server:
			if self.authcookie == None:
				self.authcookie = str(FileTransfer.AuthCookieCounter)
				FileTransfer.AuthCookieCounter += random.randint(1, 10)

			self.md.acceptfile(self.usr.email, self.cookie, self.usr.conf.myip, FileTransfer.Port, int(self.authcookie), sender_connect = 1)
			MsnFTP(self.authcookie, self.realname, self.filesz, self.usr.conf.myip, 'local', FileTransfer.Port, self.md.email, self.op, 1)
		else:
			self.md.acceptfile(self.usr.email, self.cookie)

	@staticmethod
	def dispatchMsg(msg):
		""" dispatches ACCEPT invitation message to the adequate FileTransfer object """
		cookie = msg["Invitation-Cookie"]
		if cookie in FileTransfer.transfers:
			FileTransfer.transfers[cookie].processResp(msg)

	def processResp(self, msg):
		""" process ACCEPT message """
		print "Sending file %s to user %s" % (self.filename, self.usr.email)
		if "AuthCookie" in msg:
			self.authcookie = msg["AuthCookie"]

		if self.authcookie == None:
			self.authcookie = str(FileTransfer.AuthCookieCounter)
			FileTransfer.AuthCookieCounter += random.randint(1, 10)

		if "IP-Address" in msg:
			MsnFTP(self.authcookie, self.realname, self.filesz, msg["IP-Address"], msg["IP-Address-Internal"], msg["Port"], self.md.email, self.op, 0)
		else:
			print "IP: ",self.usr.conf.myip
			self.md.acceptfile(self.usr.email, self.cookie, self.usr.conf.myip, FileTransfer.Port, int(self.authcookie))
			MsnFTP(self.authcookie, self.realname, self.filesz, self.usr.conf.myip, 'local', FileTransfer.Port, self.md.email, self.op, 1)
