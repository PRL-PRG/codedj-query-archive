# -*- coding: utf-8 -*-
# -*- tab-width: 4; use-tabs: 1 -*-
# vim:tabstop=4:noexpandtab:
"""
The highest, 5000-foot level API for bots.

Allows you to easily write bots that react to events.
"""
from __future__ import division, absolute_import, with_statement
from .nexuiz import NexRcon, Commands
__all__ = 'Bot', 'command', 'recallback'

class Bot(NexRcon):
	#TODO: Implement
	
	def textReceived(self, data):
		"""b.textReceived(string) -> None
		Takes data received and parses out commands and callbacks.
		"""
		#TODO: Implement
	
	@classmethod
	def run(cls):
		"""Bot.run() -> None
		Runs this bot class, using the standard boilerplate code.
		"""
		from twisted.internet import reactor
		import sys
		proto = cls('127.0.0.1', 26000, sys.argv[1])
		reactor.listenUDP(0, proto)
		proto.start_streaming()
		reactor.run()

def command(func):
	"""command(callable) -> callable
	Registers a method as a new command.
	
	Sends the server an alias for this command which prints a unique string 
	(eg, a GUID) to the console, which is then picked-up.
	
	The method is called whenever the command is executed server-side. The 
	arguments are positional arguments which match what it was called with.
	
	Example:
	>>> class MyBot(Bot):
	... 	@command
	... 	def dosilly(self):
	... 		self.send(Commands.say(":p"))
	"""
	#TODO: Implement
	return func

def recallback(regex):
	"""recallback(string) -> (callable) -> callable
	Registers a method as a regular expression-trigged callback.
	
	The method is called whenever the regex is found (via search) inside the 
	text. The arguments are:
	* The entire matching line (as the first positional arguments)
	* Any numbered groups as additional positional arguments
	* Any named groups as keyword arguments
	
	Example:
	>>> class MyBot(Bot):
	... 	@recallback("nick")
	... 	def dosilly(self, text):
	... 		self.send(Commands.say("What?"))
	"""
	def _(func):
		return func
	#TODO: Implement
	return _

