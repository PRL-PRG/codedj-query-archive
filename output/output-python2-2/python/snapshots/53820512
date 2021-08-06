#!/usr/bin/python
# -*- coding: utf-8 -*-
# -*- tab-width: 4; use-tabs: 1 -*-
# vim:tabstop=4:noexpandtab:
"""
An example bot.
"""
from __future__ import division, absolute_import, with_statement
from rconbot.utils import filesystem
from rconbot.bot import Bot, recallback, command, loadpassfromconfig
from rconbot.nexuiz import Commands
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

if __name__ == '__main__':
	filesystem.addpath('.') # Change this to your nexuiz directory
	SillyBot.run(loadpassfromconfig('server.cfg'))
