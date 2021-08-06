# -*- coding: utf-8 -*-
# -*- tab-width: 4; use-tabs: 1 -*-
# vim:tabstop=4:noexpandtab:
"""
This handles the raw communication with the Nexuiz server.

Note: If you actually care about the results of commands, it is encouraged that 
you send them one at a time. Commands with large output may cause strange 
issues.
"""
from __future__ import division, absolute_import, with_statement
from twisted.internet.protocol import DatagramProtocol
from twisted.internet.defer import Deferred, inlineCallbacks, returnValue
from twisted.internet import reactor
from Queue import Queue, Empty
import sys, re
from rconbot.utils import quote
__all__ = 'Rcon', 'CommandRefused', 'VariableException', 'NoValue', 'WrongVariable'

SEND_FORMAT = '\xFF\xFF\xFF\xFF%s\0'
RECV_PREFIX = '\xFF\xFF\xFF\xFF'

class CommandRefused(Exception):
	"""
	Command was refused by server; ie the port is closed and no one is listening.
	"""

class VariableException(Exception):
	"""
	There was a problem getting or setting a variable
	"""

class NoValue(Exception):
	"""
	The server returned something, but it wasn't parsable as a value.
	"""

class WrongVariable(Exception):
	"""
	The server returned a value, but it was for the wrong variable.
	"""

class Rcon(DatagramProtocol, object): # Why doesn't twisted use new-style classes?
	"""
	Communicates with a DarkPlaces server using rcon.
	"""
	__password = None
	_host = _port = None
	_deferreds = None
	_streaming = False
	def __init__(self, host, port, password):
		"""Rcon(string, int, string)
		Init's Rcon with the host, port, and rcon password needed.
		"""
		self._host = host
		self._port = port or 26000
		self.__password = password
		self._deferreds = Queue(-1)
		# Make sure we remove ourselves before our transport gets wiped
		reactor.addSystemEventTrigger('before', 'shutdown', self.stop_streaming)
		# For whatever reason, the above line causes errors in 
		# t.i.udp.Port.doRead() and t.i.protocol.AbstractDatagramProtocol.doStop()
	
#	def doStop(self):
#		"""
#		Cleans up streaming when the object is garbage collected, so the server 
#		isn't sending needless packets.
#		"""
#		d = self.stop_streaming()
#		waiting = True
#		def clear(*p): waiting = False
#		d.addBoth(clear)
#		while waiting:
#			reactor.iterate()
#		super(Rcon, self).doStop()
	
	def startProtocol(self):
		"""
		Does some handling dealing with protocol setup.
		"""
		self.transport.connect(self._host, self._port)
		print >> sys.stderr, "we can only send to %s now" % str((self._host, self._port))
		#self.transport.write("hello") # no need for address
		
	# Possibly invoked if there is no server listening on the
	# address to which we are sending.
	def connectionRefused(self):
		"""
		Invoked if the UDP packet is refused (nothing listening on that port)
		"""
		print >> sys.stderr, "No one listening"
		try:
			d = self._deferreds.get_nowait() # If there isn't an item, skip it
		except Empty:
			return
		else:
			# Should we do something like store an exception with the deferred?
			d.errback(fail=CommandRefused())
	
	RCON_IGNORABLE = re.compile('^server received rcon command from ')
	def datagramReceived(self, data, (host, port)):
		"""
		Parses data from the server and dispatches it to the right place.
		"""
#		print 'recv: %r' % data
		if data.startswith(RECV_PREFIX):
			data = data[len(RECV_PREFIX):]
		else:
			return
		if data[0] == 'n':
			# Console data
			data = data[1:] # Remove the prefixing 'n'
			if len(data) and data[-1] == '\0':
				data = data[:-1]
#			print >> sys.stderr, "received %r... from %s:%d" % (data[:64], host, port)
			if len(data) and data[0] == '\x01':
				self.chatReceived(data[1:])
			else:
				if self.RCON_IGNORABLE.search(data) is not None:
					return # Ignore it
				try:
					d = self._deferreds.get_nowait() # If there isn't an item, skip it
				except Empty:
					self.textReceived(data)
				else:
					d.callback(data)
		else:
			self.packetReceived(data)
	
	def send_raw(self, text):
		"""r.send_raw(string) -> int
		Actually sends a command, wrapping it in a QW packet.
		"""
		print >> sys.stderr, "Sending %r" % text
		return self.transport.write(SEND_FORMAT % text)
	
	def format_rcon(self, cmd):
		"""r.format_rcon(str) -> str
		Internal.
		Turns the command into what should be passed to send_raw()
		"""
		return "rcon %s %s" % (self.__password, cmd)
	
	def _send(self, cmd):
		"""r._send(str) -> Deferred
		Wraps a command in rcon and sends it, creating and handling the 
		Deferred.
		"""
		d = Deferred()
		self._deferreds.put(d)
		self.send_raw(self.format_rcon(cmd))
		return d
	
	def _sends(self, cmds):
		"""r._sends([string, ...]) -> Deferred
		Internal.
		Wraps and sends a series of commands to the server, creating and 
		handling the Deferred.
		"""
		# Used by NexRcon
		d = Deferred()
		self._deferreds.put(d)
		self.send_raw(self.format_rcon('\0'.join(cmds)))
		return d
	
	def format_cmd(self, cmd, *pargs):
		"""r.format_cmd(string, ...) -> string
		Internal.
		Formats a command to pass to _send()/_sends()
		"""
		# Used by NexRcon
		return cmd+' '+' '.join(quote(a) for a in pargs)
	
	def cmd(self, cmd, *pargs):
		"""r.cmd(str, ...) -> Deferred
		Sends the command cmd with the arguments pargs.
		"""
		return self._send(self.format_cmd(cmd, *pargs))
	
	def cmds(self, *cmds):
		"""r.cmds((str, ...), ...) -> Deferred, ...
		Sends a single packet with all the commands.
		"""
		return self._sends((self.format_cmd(*cmd) for cmd in cmds))
	
	CVARVAL = re.compile(r'^"(?P<name>.*)" is "(?P<value>.*)" \["(?P<default>.*)"\]\n?$')
	@inlineCallbacks
	def getcvar(self, var):
		"""r.getcvar(string) -> Deferred((string, string))
		Gets the current and default value of a cvar.
		"""
		text = yield self.cmd(var)
		m = self.CVARVAL.search(text)
		if m is None:
			raise NoValue(repr(text))
		name, value, default = m.group('name', 'value', 'default')
		if name != var:
			raise WrongVariable(repr(text))
		returnValue((value, default))
	
	def setcvar(self, var, value):
		"""r.setcvar(string, string) -> Deferred
		Sets a cvar.
		"""
		return self.cmd(var, str(value))
	
	def setcvars(self, **kwargs):
		"""r.setcvars(name=value, ...) -> Deferred
		Sets several cvars.
		"""
		return self.cmds(*(map(str, kv) for kv in kwargs.iteritems()))
	
	@inlineCallbacks
	def start_streaming(self):
		"""r.start_streaming() -> Deferred
		Sets up streaming console.
		"""
		host = self.transport.getHost()
		cur, _ = yield self.getcvar('log_dest_udp')
		cur += " %s:%i" % (host.host, host.port)
		self.setcvars(log_dest_udp=cur.strip())
		self._streaming = True
	
	@inlineCallbacks
	def stop_streaming(self):
		"""r.stop_streaming() -> Deferred
		Tears down streaming console.
		"""
		if self._streaming:
			host = self.transport.getHost()
			hoststring = "%s:%i" % (host.host, host.port)
			cur, _ = yield self.getcvar('log_dest_udp')
			cur = cur.replace(hoststring, '')
			self.setcvars(log_dest_udp=cur)
			self._streaming = False
	
	# Overridables
	def textReceived(self, data):
		"""r.textReceived(string) -> None
		Any data the server sends to us is passed to this function.
		
		Meant to be overridden.
		"""
		sys.stdout.write(data)
	
	def chatReceived(self, data):
		"""r.chatReceived(string) -> None
		Any chats the server receives (and is passed to us) goes here.
		
		Meant to be overridden.
		"""
		sys.stdout.write("Chat: "+data)
	
	def packetReceived(self, data):
		"""r.packetReceived(string) -> None
		Any other packets the server receives go here.
		
		Meant to be overridden.
		"""
		print "Packet: %r" % data


if __name__ == '__main__':
	from twisted.internet import reactor
	import sys
	proto = Rcon(sys.argv[1], int(sys.argv[2]), sys.argv[3])
	reactor.listenUDP(0, proto)
	proto.start_streaming()
	reactor.run()

