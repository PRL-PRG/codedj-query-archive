# -*- coding: utf-8 -*-
# -*- tab-width: 4; use-tabs: 1 -*-
# vim:tabstop=4:noexpandtab:
"""
An example bot.

Currently won't load due to the lack of implementation
"""
from __future__ import division, absolute_import, with_statement
from .bot import Bot, recallback, command, loadpassfromconfig
from .nexuiz import Commands
__all__ = 'SillyBot',

class SillyBot(Bot):
	# Callbacks of varying types - these are regex based
	@recallback('^.* has turned (?P<name>.*) into slag$', stripcolors=True)
	@recallback('^(?P<name>.*) turned into hogt slag$', stripcolors=True)
	# Always have the signature (self, text, ...).
	# For re callbacks, groups are passed as positional and keyword arguments
	def slag(self, text, name):
		# self is from Bot
		self.say("%s will make good bullets. *pour*" % name)
	
	@recallback('^(?P<name>.*) connected$', stripcolors=True)
	def hello(self, text, name):
		self.say("Hello %s" % name)
	
	@command
	def spam(self, *pargs):
		print "Command test: %r" % (pargs,)

def test():
	"""
	Totally non-reusable function for my own purposes.
	"""
	import os
	SillyBot.run(loadpassfromconfig(os.path.expanduser('~/Nexuiz/data/rcon.cfg')))

if __name__ == '__main__':
	# Should figure out how this works exactly
	test()
