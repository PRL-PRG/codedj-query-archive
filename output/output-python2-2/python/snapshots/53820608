# -*- coding: utf-8 -*-
# -*- tab-width: 4; use-tabs: 1 -*-
# vim:tabstop=4:noexpandtab:
"""
This handles the raw communication with the Nexuiz server.
"""
from __future__ import division, absolute_import, with_statement
from twisted.internet.protocol import DatagramProtocol
from twisted.internet import defer
from Queue import Queue, Empty
import sys, re
from rconbot.utils import quote
__all__ = 'Rcon', 'CommandRefused', 'VariableException', 'NoValue', 'WrongVariable'

SEND_FORMAT = '\xFF\xFF\xFF\xFF%s\0'
RECV_PREFIX = '\xFF\xFF\xFF\xFFn'

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

class Rcon(DatagramProtocol):
	__password = None
	_host = _port = None
	_deferreds = None
	_streaming = False
	def __init__(self, host, port, password):
		self._host = host
		self._port = port or 26000
		self.__password = password
		self._deferreds = Queue(-1)
	
	def __del__(self):
		self.stop_streaming()
	
	def startProtocol(self):
		self.transport.connect(self._host, self._port)
		print >> sys.stderr, "we can only send to %s now" % str((self._host, self._port))
		#self.transport.write("hello") # no need for address
		
	# Possibly invoked if there is no server listening on the
	# address to which we are sending.
	def connectionRefused(self):
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
		#print 'recv: %r' % data
		if data.startswith(RECV_PREFIX):
			data = data[len(RECV_PREFIX):]
		else:
			return
		if len(data) and data[-1] == '\0':
			data = data[:-1]
		print >> sys.stderr, "received %r... from %s:%d" % (data[:64], host, port)
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
	
	def send_raw(self, text):
		print >> sys.stderr, "Sending %r" % text
		return self.transport.write(SEND_FORMAT % text)
	
	def format_rcon(self, cmd):
		"""r.format_rcon(str) -> str
		Turns the command into what should be passed to send_raw()
		"""
		return "rcon %s %s" % (self.__password, cmd)
	
	def _send(self, cmd):
		"""r._send(str) -> Deferred
		Wraps a command in rcon to be sent.
		"""
		d = defer.Deferred()
		self._deferreds.put(d)
		self.send_raw(self.format_rcon(cmd))
		return d
	
	def _sends(self, cmds):
		d = defer.Deferred()
		self._deferreds.put(d)
		self.send_raw(self.format_rcon('\0'.join(cmds)))
		return d
	
	def format_cmd(self, cmd, *pargs):
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
	
	VARVAL = re.compile(r'^"(?P<name>.*)" is "(?P<value>.*)" \["(?P<default>.*)"\]\n?$')
	def getvar(self, var):
		"""r.getvar(string) -> Deferred
		Gets the current and default value of a variable.
		"""
		vd = defer.Deferred()
		def parseval(text):
			m = self.VARVAL.search(text)
			if m is None:
				vd.errback(NoValue(repr(text)))
				return text
			name, value, default = m.group('name', 'value', 'default')
			if name != var:
				vd.errback(WrongVariable(repr(text)))
				return text
			vd.callback((value, default))
			return text
		cd = self.cmd(var)
		cd.addCallback(parseval)
		return vd
	
	def setvar(self, var, value):
		"""r.setvar(string, string) -> Deferred
		Sets a variable.
		"""
		return self.cmd(var, str(value))
	
	def setvars(self, **kwargs):
		"""r.setvars(name=value, ...) -> Deferred
		Sets several variables.
		"""
		return self.cmds(*(map(str, kv) for kv in kwargs.iteritems()))
	
	def start_streaming(self):
		"""
		Sets up streaming console.
		"""
		host = self.transport.getHost()
		def set_ldu(values):
			cur, defa = values
			cur += " %s:%i" % (host.host, host.port)
			self.setvars(log_dest_udp=cur)
			self._streaming = True
			return values
		d = self.getvar('log_dest_udp')
		d.addCallback(set_ldu)
	
	def stop_streaming(self):
		"""
		Tears down streaming console.
		"""
		if self._streaming:
			host = self.transport.getHost()
			hoststring = "%s:%i" % (host.host, host.port)
			def set_ldu(values):
				cur, defa = values
				cur = cur.replace(hoststring, '')
				self.setvars(log_dest_udp=cur)
				self._streaming = False
				return values
			d = self.getvar('log_dest_udp')
			d.addCallback(set_ldu)
	
	# Overloadables
	def textReceived(self, data):
		"""
		Any data the server sends to us (TODO: not responses to commands) is 
		passed to this function.
		"""
		sys.stdout.write(data)
	
	def chatReceived(self, data):
		"""
		Any chats the server receives (and is passed to us) goes here.
		"""
		sys.stdout.write("Chat: "+data)


if __name__ == '__main__':
	from twisted.internet import reactor
	import sys
	proto = Rcon(sys.argv[1], int(sys.argv[2]), sys.argv[3])
	reactor.listenUDP(0, proto)
	proto.start_streaming()
	reactor.run()

