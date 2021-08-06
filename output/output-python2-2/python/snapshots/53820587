# -*- coding: utf-8 -*-
# -*- tab-width: 4; use-tabs: 1 -*-
# vim:tabstop=4:noexpandtab:
"""
The highest, 5000-foot level API for bots.

Allows you to easily write bots that react to events.
"""
from __future__ import division, absolute_import, with_statement
from .nexuiz import NexRcon, Commands
from .utils import callbyline, complexdecorator
import uuid, re
__all__ = 'Bot', 'command', 'recallback'


COMMAND_CALL = re.compile(r'^\{(?P<uuid>[-0-9A-Fa-f]{36})\}:(?P<arguments>.*)$')
class Bot(NexRcon):
	def __init__(self, *pargs, **kwargs):
		super(Bot, self).__init__(*pargs, **kwargs)
		
	
	def _make_alias(self, name, uuid, numargs=10):
		rv = "alias %s " % name
		host = self.transport.getHost()
		hoststring = "%s:%i" % (host.host, host.port)
		for i in xrange(numargs):
			rv += 'packet "%s" "{%s}-%n:$%n";' % (hoststring, str(uuid), i, i)
		rv += 'packet "%s" "{%s}-exec";'
	
	@callbyline
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
	(eg, a UUID) to the console, which is then picked-up.
	
	The method is called whenever the command is executed server-side. The 
	arguments are positional arguments which match what it was called with.
	
	Due to technical reasons, only 9 arguments are allowed.
	
	Example:
	>>> class MyBot(Bot):
	... 	@command
	... 	def dosilly(self):
	... 		self.send(Commands.say(":p"))
	"""
	func.mkcommand = True
	func.command_uuid = uuid.uuid1()
	return func

@complexdecorator
def recallback(regex, **kwargs):
	"""recallback(string, [**flags]) -> (callable) -> callable
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
	flags = kwargs.get('flags', None)
	stripcolors = kwargs.get('stripcolors', False)
	pattern = re.compile(regex, flags)
	func = yield
	func.recallback_info = {'pattern': pattern, 'stripcolors': stripcolors}
	yield func

