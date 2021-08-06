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


	config.py - class for storing config/current state variables
	"""

import users

class config(object):
	def __init__(self):
		self.usrs = {}
		self.allowed_users = []
		self.myip = None
		pass

	def assureuser(self, md, email):
		""" creates dictionary entry for a user if not present """
		if email not in self.usrs:
			self.usrs[email] = users.RealUser(md, email, self)

