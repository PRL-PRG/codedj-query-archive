# -*- coding: utf-8 -*-
# -*- tab-width: 4; use-tabs: 1 -*-
# vim:tabstop=4:noexpandtab:
"""
An example bot.

Currently won't load due to the lack of implementation
"""
from __future__ import division, absolute_import, with_statement
from .bot import Bot, recallback
__all__ = 'SillyBot',

class SillyBot(Bot):
	# Callbacks of varying types - these are regex based
	@recallback('^.* has turned (?P<name>.*) into slag$', stripcolors=True)
	@recallback('^(?P<name>.*) turned into hogt slag$', stripcolors=True)
	# Always have the signature (self, text, ...).
	# For re callbacks, groups are passed as positional and keyword arguments
	def slag(self, text, name):
		# self.protocol is from Bot
		self.protocol.say("%s will make good bullets. *pour*")
		

if __name__ == '__main__':
	# Should figure out how this works exactly
	pass
