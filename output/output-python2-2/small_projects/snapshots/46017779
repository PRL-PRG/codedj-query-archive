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


	cmds.py - callables realizing available commands
"""

import sys
import os
import shlex
import functools
import thread


import utils
from utils import fixnls
from config import config

import users

def getparser(str):
	"""instantiates standard command parser """
	parser = shlex.shlex(str, "", 1)

	parser.wordchars += "*"
	parser.commenters = "#"
	parser.quotes = '''"'@'''
	parser.escapedquotes = '''"@'''
	parser.whitespace_split = 1
	parser.source = "include"
	return parser

class StatedCommand(object):
	""" base class for callables for commands that can persist state """

	def __new__(cls, usr, data, *args, **eargs):
		obj = object.__new__(cls)
		obj.__init__(usr, data, *args, **eargs)
		obj.usr = usr
		if data.strip() != "":
			obj(usr, data)
			return cmd
		else:
			usr.sendmsg(obj.pm_msg)
			return obj

class bash(StatedCommand):
	""" bash command - executes shell command """
	def __init__(self, usr, data):
		self.pm_msg = "OK, I will execute everyting you write"
		self.popen = None
		self.term = ""

	@staticmethod
	def processoutput(usr, p):
		while p and p.stdout and not p.stdout.closed:
			data = p.recv()
			if data:
				usr.sendmsg(utils.towin32NL(data))

	def __call__(self, usr, data):
		print "Executing " + data
		try:
			if self.popen and self.popen.poll() != None:
				self.popen = None

			if data.strip() == "ABANDON":
				self.popen.stdout = None
				self.popen = None
				usr.sendmsg("Process abandoned. You are in shell execution mode.")
			elif data.strip() == "APPEND NL":
				self.term = "\n"
				usr.sendmsg("OK, I will append new-line character to any string sent")
			elif data.strip() == "DO NOT APPEND NL":
				self.term = ""
				usr.sendmsg("OK, I will not append new-line character to any string sent")
			else:
				if not self.popen:
					self.popen = utils.Popen(data.strip(), shell=1, stdin=utils.PIPE, stdout=utils.PIPE, stderr=utils.STDOUT, universal_newlines=1)
					thread.start_new_thread(bash.processoutput, (usr, self.popen))
				else:
					self.popen.send(data.strip().replace("NL", "\n").replace("CR", "\r").replace("ESC", "\033") + self.term)
		except:
			usr.sendmsg("Error executing " + data)

		return self

class echo(StatedCommand):
	""" echo command - echoes strings """
	def __init__(self, usr, data):
		self.pm_msg = "OK, I will echo now everything you write"

	def __call__(self, usr, data):
		print "Echoing message " + data + " from " + usr.email
		usr.sendmsg(data)
		return self



def die(usr, data):
	""" die command - terminates msnmago """
	usr.sendmsg("Server terminated.")
	try:
		usr.md.disconnect()
	except:
		pass
	sys.exit(0)



def info(usr, data):
	""" info command - send information about the version """
	usr.sendmsg("MSN Mago v.0.1\r\n\r\nCopyright [c] 2007 Krzysztof Olczyk\r\n\r\nLicensed under GNU Public License v 3");
	return cmd



def set(usr, data):
	""" set command - sets parameter """
	parser = getparser(data)

	try:
		var = parser.get_token()
		if parser.get_token() != "to":
			raise exception()
		val = parser.get_token()
	except:
		usr.sendmsg("Invalid syntax. It should be\n\nset parameter to value")
		usr.sendmsg(str(sys.exc_info()[0]))
		return cmd

	if var.lower() == "name":
		usr.md.change_nick(val)
	elif var.lower() == "status":
		usr.md.change_status(val)

	return cmd

def allow(usr, data):
	""" allow command - allows given contact to invoke commands """
	parser = getparser(data)
	email = parser.get_token()
	usr.conf.assureuser(usr.md, email)
	usr.conf.usrs[email].allowed = 1
	usr.conf.allowed_users.append(email)
	usr.sendmsg("User %s allowed to communicate." % email)
	return cmd

#CUSTOM COMMANDS
custom_cmds = {}

class defcmd(object):
	""" base class for custom commands """
	def __init__(self, code):
		self.code = fixnls(code)

	def __call__(self, usr, data):
		usr.sendmsg("This command is corrupted. Check your script")
		return cmd

class defpythoncmd(defcmd):
	""" class for custom command defined in python """
	def __call__(self, usr, data):
		args = shlex.split(data)
		exec(self.code, {'sendmsg' : usr.sendmsg, 'args' : args, 'do' : usr.execcmd})
		return cmd

class defmacrocmd(defcmd):
	""" class for custom command being a macro of commands """
	def __call__(self, usr, data):
		mycode = self.code.replace("%*", data)
		parser = getparser(data)
		i = 0
		for token in parser:
			mycode = mycode.replace("%%%d" % i, token)
			i += 1
		usr.execmacro(mycode)
		return cmd

class defappcmd(defcmd):
	""" class for custom command realized by external application """
	def __call__(self, usr, data):
		if data.strip() != '':
			p = os.popen(self.code, 'rw')
			p.write(data)
		else:
			p = os.popen(self.code, 'r')
		res = ""
		for l in p:
			res += "\r\n" + l
		usr.sendmsg(res)
		return cmd

def def_(usr, data):
	""" def command - defines custom command """
	parser = getparser(data)

	try:
		funcname = parser.get_token()
		if parser.get_token() != "as":
			raise exception()
		functype = parser.get_token()
		code = parser.get_token()
		funccls = defcmd
		if functype == "pythonic":
			funccls = defpythoncmd
		elif functype == "macro":
			funccls = defmacrocmd
		elif functype == "application":
			funccls = defappcmd
		else:
			usr.sendmsg("Unknown def type: %s" % functype)
			return cmd

		custom_cmds[funcname] = funccls(code)
		usr.sendmsg("Command %s defined" % funcname)

	except:
		usr.sendmsg("Error in def")

	return cmd

def disable(usr, data, enable = 0):
	""" disable command - forbids users to execute certain commands """
	parser = getparser(data)
	try:
		c = parser.get_token()
		if parser.get_token() != "to":
			raise Exception()
		email = parser.get_token()
		usr.conf.assureuser(usr.md, email)
		if enable:
			usr.conf.usrs[email].enabled.append(c)
		else:
			usr.conf.usrs[email].disabled.append(c)
		usr.sendmsg("Command %s %s to %s." % (c, {0 : 'disabled', 1 : 'enabled'}[enable], email) )
	except:
		usr.sendmsg("Error in disable/enable command");

	return cmd

class broadcast(StatedCommand):
	""" broadcast command - broadcasting message to other online users of bot """
	def __init__(self, usr, data, secret = 0):
		self.pm_msg = "OK, I will broadcast everyting you write"
		self.secret = secret

	def __call__(self, usr, data):
		for (ue) in usr.conf.usrs:
			if ue == usr.email:
				continue
			if self.secret:
				usr.conf.usrs[ue].sendmsg(data)
			else:
				usr.conf.usrs[ue].sendmsg("From %s: %s" % (usr.email, data))
		return self

def screenshot(usr, data):
	""" screenshot - does and sends as png the screenshot """
	usr.sendmsg("Preparing screenshot...")
	sf = utils.doss()
	ft = users.FileTransfer(usr.md, usr, "screenshot.jpg", sf)
	ft.invite()
	return cmd


class downloads(StatedCommand):
	""" downloads command """
	def __init__(self, usr, data):
		self.pm_msg = "File manager mode."
		self.path = []

	def __call__(self, usr, data):
		try:
			self.call(usr, data)
		except:
			usr.sendmsg("Error executing command")
		return self

	def call(self, usr, data):
		parser = getparser(data)
		cmd = parser.get_token()

		top =  [os.getcwd(), 'downloads']

		if cmd == "ls":
			cmd = "list"
			data = "files"
		elif cmd == "cd ..":
			cmd = "up"

		if cmd == "list":
			dir = utils.buildpath( top + self.path)
			dirc = os.walk(dir)
			what_to_list = parser.get_token()
			dirs = dirc.next()[ { 'dirs' : 1, 'files' : 2, None : 2 }[what_to_list] ]
			res = []
			for dr in dirs:
				res.append(dr)
			usr.sendmsg(reduce(lambda x, y: x + "\r\n" + y, res))
		elif cmd == "get":
			fn = parser.get_token()
			ffn = utils.buildpath(top + self.path + [fn])
			if os.path.exists(ffn):
				ft = users.FileTransfer(usr.md, usr, fn.replace(os.sep, "_"), ffn)
				ft.invite()
			else:
				usr.sendmsg("File %s does not exist!" % utils.buildpath(self.path + [fn]))
		elif cmd == "cd":
			self.path.append(parser.get_token())
			dir = utils.buildpath(self.path)
			if not os.path.exists(utils.buildpath(top + self.path)):
				self.path = self.path[:-1]
				usr.sendmsg("Direcotry %s does not exist!" % dir)
			else:
				usr.sendmsg("Directory changed to %s" % dir)
		elif cmd == "up":
			self.path = self.path[:-1]
			if len(self.path):
				dir = utils.buildpath(self.path)
			else:
				dir = "root directory"
			usr.sendmsg("Directory changed to %s" % dir)


#list of recognized command
cmds = { "echo" : echo, "die" : die, "shell" : bash, "info" : info, "set" : set, "allow" : allow, "def" : def_, \
			"disable" : disable, "enable" : functools.partial(disable, enable = 1), "broadcast" : broadcast,
			"broadcast_secret" : functools.partial(broadcast, secret = 1), "screenshot" : screenshot,
			"downloads" : downloads }



def cmd(usr, data):
	""" command manager - it receives user's tokens where no other command is active """
	msgspl = data.split(" ")
	acmd, msg = (msgspl[0], " ".join(msgspl[1:]))

	if not len(acmd) or acmd[0] == "#":  # ignore comments
		return cmd

	if acmd in cmds: # look for command's implementation
		scmd = cmds[acmd]
	elif acmd in custom_cmds:
		scmd = custom_cmds[acmd]
	else:
		usr.sendmsg("Error: Wrong command " + acmd)
		return cmd

	if (acmd in usr.disabled or '*' in usr.disabled) and acmd not in usr.enabled: # check if command is not disabled for the user
		usr.sendmsg("Sorry. You are not authorized to execute this command.")
		return cmd

	return scmd(usr, msg)

