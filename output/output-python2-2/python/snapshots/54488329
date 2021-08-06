"""
	checker.py - A base class for mailbox checkers 

	Copyright 2006 Kenneth Hayber <ken@hayber.us>,
	All rights reserved.
"""

class Checker:	
	def __init__(self, config):
		try:
			self.name = config['name']
			self.polltime = int(config['polltime'])
		except:
			self.name = ''
			self.polltime = 10

		self.blocker = None
		self.results = ""
		self.unseen = 0
		self.prev_total = 0

	def check(self):
		yield None
		self.blocker.trigger()
