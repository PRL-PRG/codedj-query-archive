#!/usr/bin/env python

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


	REQUIREMENTS:
		- Python 2.5
		- pyWin32 (only if running on Windows)
		- pyGTK (optional; for screenshot to work)
"""

import sys
import os
import time
import select
import socket
import thread
import re
import email as email_lib

from msnlib.msnlib import *
import msnlib.msncb as msncb
from msnlib.ftp import MsnFTP

from utils import *
import users;
import cmds;
from config import *

m = msnd()  # msn descriptor
m.cb = msncb.cb() # callbacks

conf = config() # configuration
usrs = conf.usrs
usrs['config'] = users.ConfigUser(m, conf)

def cb_msg(md, type, tid, params, sbd):
	"Message received callback"

	msncb.cb_msg(md, type, tid, params, sbd)

	email = tid.split(" ")[0]
	if email not in usrs:
		usrs[email] = users.RealUser(md, email, conf)
	else:
		usrs[email].updatemd(md)

	msg = email_lib.message_from_string(params)
	routine = msg.get_payload()

	if re.search("text/plain", msg["Content-Type"]):
		usrs[email].execcmd(routine)
	elif re.search("text/x-msmsgsinvite", msg["Content-Type"]):
		msg = email_lib.message_from_string(routine)

		if msg["Invitation-Command"] == "INVITE" and msg["Application-GUID"] == "{5D3E02AB-6190-11d3-BBBB-00C04F795683}":
			# it's incoming file transfer
			fn = msg["Application-File"]
			ft = users.FileTransfer(md, usrs[email], fn, buildpath((os.getcwd(), "received_files", email, fn)), msg["Application-FileSize"], msg["Invitation-Cookie"], 'receive')
			ft.accept(msg["Connectivity"] == "N")
		elif msg["Invitation-Command"] == "ACCEPT":
			# some file transfer was accepted
			users.FileTransfer.dispatchMsg(msg)
	elif re.search("text/x-msmsgsprofile", msg["Content-Type"]):
		conf.myip = msg["ClientIP"]

m.cb.msg = cb_msg

def cb_add(md, type, tid, params):
	"Callback for being add by new user to contact list"

	t = params.split(' ')
	type = t[0]
	if type == 'RL':
		email = t[2]
		print email
		md.useradd(email)
	else:
		pass
	msncb.cb_add(md, type, tid, params)

m.cb.add = cb_add

# get the login email and password from the parameters
try:
	m.email = sys.argv[1]
	m.pwd = sys.argv[2]
except:
	lcp = buildpath((os.getcwd(), "login.conf"))
	if os.path.exists(lcp):
		from base64 import b64decode
		fl = open(lcp, "r")
		m.email = fl.readline().strip()
		m.pwd = b64decode(fl.readline().strip())
	else:
		print "Use: " + sys.argv[0] + " email password"
		print ""
		print "or create file login.conf with 1st line being a login and 2nd line being a base64 code of password"
		sys.exit(1)

print "Logging In..."
m.login()

print "Retrieving contact list and settings"
m.sync()

print "Initializing..."
m.change_nick("El Mago");
m.change_status("online")

usrs['config'].applyconfig()

print "Running..."
while 1:
	t = m.pollable()
	infd = t[0]
	outfd = t[1]

	try:
		fds = select.select(infd, outfd, [], 0)
	except:
		quit()

	for i in fds[0] + fds[1]:
		try:
			m.read(i)
		except ('SocketError', socket.error), err:
			if i != m:
				m.close(i)
			else:
				m.disconnect()
	time.sleep(0.01)
