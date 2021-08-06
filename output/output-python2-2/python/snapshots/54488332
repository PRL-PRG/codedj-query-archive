"""
	imap_check.py - An imap folder checker 

	Copyright 2006 Kenneth Hayber <ken@hayber.us>,
	All rights reserved.
"""

import imaplib, sys
from checker import Checker

class IMAPChecker(Checker):
	def __init__(self, config=None):
		Checker.__init__(self, config)
		self.protocol = 'IMAP'
		try:
			self.server = config['server']
			self.port = int(config['port'])
			self.ssl = (config['ssl'] == 'True')
			self.username = config['username']
			self.password = config['password']
			self.folders = config['folders'].split(',')
		except:
			self.server = 'localhost'
			self.port = 143
			self.ssl = False
			self.username = ''
			self.password = ''
			self.folders = []

	def check(self):
		"""Check all folders"""
		try:
			if self.ssl:
				im = imaplib.IMAP4_SSL(self.server, self.port)
			else:
				im = imaplib.IMAP4(self.server, self.port)
			im.login(self.username, self.password)
		except:
			self.results = "%s (%s)\n" % (self.name, _('Login Error'))
			self.blocker.trigger()
			return

		yield None #let someone else run for a while
		
		self.results = ""
		self.unseen = 0
		
		for folder in self.folders:
			folder = folder.strip()
			result = im.select(folder, readonly=True)
			if result[0] == 'OK':
				if result[1][0] == '':
					count = 0
				else:
					count = int(result[1][0])
			else:
				count = -1
			if count == -1:
				yield None #let someone else run for a while
	
			result = im.search(None, "UNSEEN")
			if result[0] == 'OK':
				if result[1][0] == '':
					unseen = 0
				else:
					unseen = len(result[1][0].split())
					self.unseen += unseen
			else:
				unseen = -1
			if unseen > 0:
				self.results += "  %s (%d/%d)\n" % (folder, unseen, count)
			yield None #let someone else run for a while

		try:
			im.close()
			im.logout()
		except:
			pass
		
		if len(self.results):
			self.results = "%s:\n%s" % (self.name, self.results)

		# We're done.  Trigger the main task
		self.blocker.trigger()
		