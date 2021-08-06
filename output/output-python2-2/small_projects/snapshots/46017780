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

	generate_loginconf.py - Generates login.conf file
"""

from base64 import b64encode
from getpass import getpass

login = raw_input("Enter your Windows Live! login: ")
passwd = getpass("Enter your Windows Live! password: ")

print "Generating your login.conf file..."

cf = open("login.conf", "w")
cf.write("%s\n" % login)
cf.write("%s\n" % b64encode(passwd))

print "OK"
raw_input("Press ENTER to quit...")
