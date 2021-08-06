# -*- coding: utf-8 -*-
# -*- tab-width: 4; use-tabs: 1 -*-
# vim:tabstop=4:noexpandtab:
"""
The highest, 5000-foot level API for bots.

Allows you to easily write bots that react to events.
"""
from __future__ import division, absolute_import, with_statement
from .nexuiz import NexRcon, Commands
from .utils import callbyline, complexdecorator, stripcolors
import uuid, re
__all__ = 'Bot', 'command', 'recallback', 'loadpassfromconfig'

def match2pkw(match):
	"""patmatch2pkw(match) -> tuple, dict
	Turns an re match object into a tuple and dict for use as positional and 
	keyword arguments.
	"""
	pattern = match.re
	print "match2pkw: %r %r %r" % (match.groups(), pattern.groupindex, match.groupdict())
	p = tuple(t for i,t in enumerate(match.groups()) if i+1 not in pattern.groupindex.values())
	kw = match.groupdict()
	return p, kw

def func_numargs(func):
	"""func_numargs(function) -> int|None
	Figures out the number of positional arguments the function can take, or 
	None if it can take any number.
	"""
	code = func.func_code
	rv = code.co_argcount
	if code.co_flags & 0x04:
		return None
	else:
		return rv

COMMAND_CALL = re.compile(r'^\{(?P<uuid>[-0-9A-Fa-f]{36})\}-(?P<what>[^:]*)(?::(?P<value>.*))?$')
class Bot(NexRcon):
	"""
	A framework to implement a simple bot.
	"""
	#TODO: Remove command aliases when we're done.
	_commands = {}
	_command_args = {}
	_re_callbacks = {}
	def __init__(self, *pargs, **kwargs):
		"""
		See Rcon.__init__().
		"""
		print type(self)
		super(Bot, self).__init__(*pargs, **kwargs)
		self._commands = {}
		self._command_args = {}
		self._re_callbacks = {}
		
		# Handle the decorators for callbacks/commands
		for name in dir(self):
			meth = getattr(self, name)
			if callable(meth): # Ignore non-methods
				self._process_method(name, meth)
	
	def _process_method(self, name, meth):
		"""
		Internal.
		Dispatches method registration.
		"""
		if hasattr(meth, 'command_uuid'):
			self._add_command(name, meth)
		elif hasattr(meth, 'recallback_info'):
			self._add_recallback(name, meth)
		elif hasattr(meth, 'im_func'): # An actual method, get the function out of it
			self._process_method(name, meth.im_func)
	
	def _add_command(self, name, meth):
		"""
		Internal.
		Registers a @commmand.
		"""
		numargs = func_numargs(meth) # Introspectively finds the number of arguments
		
		if numargs is None:
			numargs = 9 # Technical reasons dealing with aliases
		elif getattr(self, name) is not meth:
			# A method, remove one argument
			numargs -= 1
		self._commands[meth.command_uuid] = getattr(self, name) # Call the method, not the function
		self._command_args[meth.command_uuid] = [None]*numargs
	
	def startProtocol(self):
		"""
		Registers commands with the server.
		"""
		super(Bot, self).startProtocol()
		for uuid, meth in self._commands.iteritems():
			if hasattr(meth, 'im_func') and not hasattr(meth, 'command_uuid'):
				meth = meth.im_func
			assert hasattr(meth, 'command_uuid')
			assert uuid == meth.command_uuid
			name = meth.__name__
			numargs = func_numargs(meth) # Introspectively finds the number of arguments
		
			if numargs is None:
				numargs = 9 # Technical reasons dealing with aliases
			elif getattr(self, name) is not meth:
				# A method, remove one argument
				numargs -= 1
			alias = self._make_alias(name, uuid, numargs)
			self.send(alias)
	
	def _add_recallback(self, name, meth):
		"""
		Internal.
		Registers a @recallback.
		"""
		pattern = meth.recallback_info['pattern']
		stripcolors = meth.recallback_info['stripcolors']
		self._re_callbacks[pattern] = stripcolors, getattr(self, name) # Call the method, not the function
	
	def _make_alias(self, name, uuid, numargs=9):
		"""
		Internal.
		Generates the alias command to register a command on the server.
		"""
		print "_make_alias: %r %r %r" % (name, uuid, numargs)  
		host = self.transport.getHost()
		hoststring = "%s:%i" % (host.host, host.port)
		rv = ""
		print "_make_alias: %r" % rv
		for i in xrange(numargs+1):
			rv += 'packet "%s" "{%s}-%i:$%i";' % (hoststring, str(uuid), i, i)
		rv += 'packet "%s" "{%s}-exec";' % (hoststring, str(uuid))
		rv = 'alias %s "%s"' % (name, rv.replace('"', r'\"'))
		return rv
	
	@callbyline
	def textReceived(self, data):
		"""b.textReceived(string) -> None
		Takes data received and parses out callbacks.
		"""
		stripdata = stripcolors(data)
		for pattern, (stripc, call) in self._re_callbacks.iteritems():
			text = stripdata if stripc else data
			m = pattern.search(text)
			if m:
				p, kw = match2pkw(m)
				print "textReceived:exec: %r %r %r %r" % (call, text, p, kw)
				call(text, *p, **kw)
	
	def packetReceived(self, data):
		"""b.packetReceived(string) -> None
		Takes data received and parses out commands.
		"""
		from uuid import UUID
		print "Packet: %r" % data
		m = COMMAND_CALL.search(data)
		if m is None:
			return
		uuid, what, value = m.group('uuid', 'what', 'value')
		print "Packet: %r %r %r" % (uuid, what, value)
		if what.isdigit():
			what = int(what)
		uuid = UUID(uuid)
		try:
			cmd = self._commands[uuid]
			args = self._command_args[uuid]
		except KeyError:
			return
		
		if isinstance(what, int): # Store argument for execution
			what -= 1 # $0 is the command name
			if what >= 0:
				if value == ('$%i'%(what+1)): return # Wasn't actually given
				print "Set: %r %r" % (what, value)
				args[what] = value #TODO: Parse value so it's a plain string
		elif what == 'exec': # Execute method with stored arguments
			print "Exec: %r %r" % (cmd, args)
			try:
				cmd(*(a for a in args if a is not None))
			finally:
				self._command_args[uuid] = [None] * len(args) # Clear arguments so they don't carry over
	
	@classmethod
	def run(cls, password=None):
		"""Bot.run([string]) -> None
		Runs this bot class, using the standard boilerplate code.
		"""
		from twisted.internet import reactor
		import sys
		if password is None:
			password = sys.argv[1]
		proto = cls('127.0.0.1', 26000, password)
		reactor.listenUDP(0, proto)
		proto.start_streaming()
		reactor.run()
	
	# Some convenience functions
	def say(self, text):
		"""b.say(t) <==> b.send(Commands.say(t))
		Convenience function for saying things in chat.
		"""
		return self.send(Commands.say(text))

def command(func):
	r"""command(callable) -> callable
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
	
	Bugs:
	* Detection of omitted arguments not perfect (arguments can't be "$0" or similar)
	"""
	func.command_uuid = uuid.uuid1()
	return func

def loadpassfromconfig(configfile):
	"""loadpassfromconfig(string) -> string
	Scans the file for the rcon password.
	"""
	with open(configfile, 'rU') as config:
		for line in config:
			if 'rcon_password' in line:
				# Possible. Do further parsing
				line = line.strip()
				line = line.split('//', 1)[0] # Remove comments
				bits = line.split('"') # A trick I learned to parse quotes
				parts = []
				for i, bit in enumerate(bits):
					if i % 2 == 0:
						parts += bit.split()
					else:
						parts.append(bit)
				if len(parts) >= 2 and parts[0] == 'rcon_password':
					# Yes, the actual password
					return parts[1]
		else:
			return None # No password found

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
	flags = kwargs.get('flags', 0)
	stripcolors = kwargs.get('stripcolors', False)
	pattern = re.compile(regex, flags)
	func = yield
	func.recallback_info = {'pattern': pattern, 'stripcolors': stripcolors}
	yield func

