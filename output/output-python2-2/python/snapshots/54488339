"""
	imap_check.py - An imap folder checker 

	Copyright 2006 Kenneth Hayber <ken@hayber.us>,
	All rights reserved.
"""

import poplib, sys

from checker import Checker
class POPChecker(Checker):
	def __init__(self, config):
		Checker.__init__(self, config)		
		self.server = config['server']
		self.port = int(config['port'])
		self.username = config['username']
		self.password = config['password']
		self.ssl = (config['ssl'] == 'True')
		self.apop = (config['apop'] == 'True')

		self.uidls = []  # list of uidls from last check
		# save this to a file to prevent all messages being 'new' at startup

	def check(self):
		self.unseen = 0

		try:
			if self.ssl:
				pop = poplib.POP3_SSL(self.server)
			else:
				pop = poplib.POP3(self.server)

			if self.apop:
				pop.apop(self.username, self.password)
			else:
				pop.user(self.username)
				pop.pass_(self.password)
		except:
			self.results = "%s (%s)\n" % (self.name, _('Login Error'))
			self.blocker.trigger()
			return

		# get list of unique message ids
		m = pop.uidl()
		pop.quit()
		yield None

		# remove the unneeded parts from the uidl results (just want the uidls)
		uidls = map(lambda a: a.split()[1], m[1])
		yield None

		# find the new (unseen) uidls
		for x in uidls:
			if not x in self.uidls:
				self.unseen += 1
			yield None

		if self.unseen > 0:
			self.results = "%s (%d/%d)\n" % (self.name, self.unseen, len(uidls))
		else:
			self.results = ""

		self.uidls = uidls
		self.blocker.trigger()
